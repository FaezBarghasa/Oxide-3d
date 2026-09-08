//! Interactive 3D Viewport canvas widget with Orbit/Pan/Zoom camera controls.

use iced::mouse::{self, Button, Cursor};
use iced::widget::canvas::{Event, Frame, Geometry, Path, Program, Stroke};
use iced::widget::Action;
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme};
use oxide_render::{Camera, TriMesh};

/// Message emitted by the interactive 3D Viewport widget.
#[derive(Debug, Clone, Copy)]
pub enum ViewportMessage {
    /// Orbit camera drag [dx, dy].
    Orbit {
        /// Horizontal angle delta.
        dx: f32,
        /// Vertical angle delta.
        dy: f32,
    },
    /// Pan camera drag [dx, dy].
    Pan {
        /// Horizontal displacement delta.
        dx: f32,
        /// Vertical displacement delta.
        dy: f32,
    },
    /// Zoom camera delta.
    Zoom {
        /// Magnification / distance factor delta.
        delta: f32,
    },
    /// Selection click at screen coords [x, y].
    Pick {
        /// Coordinates of click on screen.
        screen_pos: [f32; 2],
    },
}

/// Drag interaction state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum DragMode {
    #[default]
    None,
    Orbit,
    Pan,
}

/// Internal state for the interactive 3D viewport canvas.
#[derive(Debug, Default)]
pub struct ViewportState {
    last_cursor: Option<Point>,
    drag_mode: DragMode,
}

/// Interactive 3D Viewport Canvas Program for Iced.
pub struct ViewportWidget<'a> {
    camera: &'a Camera,
    mesh: &'a TriMesh,
}

impl<'a> ViewportWidget<'a> {
    /// Create a new interactive viewport widget displaying a camera and mesh.
    #[must_use]
    pub fn new(camera: &'a Camera, mesh: &'a TriMesh) -> Self {
        Self { camera, mesh }
    }
}

impl<'a> Program<ViewportMessage, Theme, Renderer> for ViewportWidget<'a> {
    type State = ViewportState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<ViewportMessage>> {
        let cursor_position = cursor.position_in(bounds)?;

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(Button::Left) => {
                    state.drag_mode = DragMode::Orbit;
                    state.last_cursor = Some(cursor_position);
                    Some(Action::publish(ViewportMessage::Pick {
                        screen_pos: [cursor_position.x, cursor_position.y],
                    }))
                }
                mouse::Event::ButtonPressed(Button::Middle | Button::Right) => {
                    state.drag_mode = DragMode::Pan;
                    state.last_cursor = Some(cursor_position);
                    Some(Action::capture())
                }
                mouse::Event::ButtonReleased(Button::Left | Button::Middle | Button::Right) => {
                    state.drag_mode = DragMode::None;
                    state.last_cursor = None;
                    Some(Action::capture())
                }
                mouse::Event::CursorMoved { .. } => {
                    let prev = state.last_cursor.replace(cursor_position);
                    if let Some(prev_pos) = prev {
                        let dx = cursor_position.x - prev_pos.x;
                        let dy = cursor_position.y - prev_pos.y;
                        match state.drag_mode {
                            DragMode::Orbit => Some(Action::publish(ViewportMessage::Orbit { dx, dy })),
                            DragMode::Pan => Some(Action::publish(ViewportMessage::Pan { dx, dy })),
                            DragMode::None => None,
                        }
                    } else {
                        None
                    }
                }
                mouse::Event::WheelScrolled { delta } => {
                    let scroll_amount = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => *y * 20.0,
                        mouse::ScrollDelta::Pixels { y, .. } => *y,
                    };
                    Some(Action::publish(ViewportMessage::Zoom {
                        delta: scroll_amount,
                    }))
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Fill modern dark engineering background
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgb(0.11, 0.12, 0.14),
        );

        // Draw coordinate grid lines
        let center_x = bounds.width * 0.5;
        let center_y = bounds.height * 0.5;

        let grid_color = Color::from_rgba(0.25, 0.28, 0.32, 0.4);
        let grid_stroke = Stroke::default().with_color(grid_color).with_width(1.0);

        let grid_size = 40.0;
        let mut x = center_x % grid_size;
        while x < bounds.width {
            frame.stroke(
                &Path::line(Point::new(x, 0.0), Point::new(x, bounds.height)),
                grid_stroke,
            );
            x += grid_size;
        }

        let mut y = center_y % grid_size;
        while y < bounds.height {
            frame.stroke(
                &Path::line(Point::new(0.0, y), Point::new(bounds.width, y)),
                grid_stroke,
            );
            y += grid_size;
        }

        // Project and render 3D mesh triangles
        let view_proj = self.camera.build_view_projection_matrix();

        // Project all vertices to screen space
        let screen_pts: Vec<Option<(Point, f32)>> = self
            .mesh
            .vertices
            .iter()
            .map(|v| {
                let p = glam::Vec4::new(v.position[0], v.position[1], v.position[2], 1.0);
                let clip = view_proj * p;
                if clip.w > 0.01 {
                    let ndc_x = clip.x / clip.w;
                    let ndc_y = clip.y / clip.w;
                    let sx = (ndc_x * 0.5 + 0.5) * bounds.width;
                    let sy = (-ndc_y * 0.5 + 0.5) * bounds.height;
                    Some((Point::new(sx, sy), clip.z / clip.w))
                } else {
                    None
                }
            })
            .collect();

        // Sort triangles by depth (back to front) for pseudo-depth rendering
        let mut tri_indices = Vec::new();
        for chunk in self.mesh.indices.chunks_exact(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let opt0 = screen_pts.get(i0).copied().flatten();
            let opt1 = screen_pts.get(i1).copied().flatten();
            let opt2 = screen_pts.get(i2).copied().flatten();

            if let (Some((p0, z0)), Some((p1, z1)), Some((p2, z2))) = (opt0, opt1, opt2) {
                // Backface culling in screen space (2D cross product)
                let edge1 = Point::new(p1.x - p0.x, p1.y - p0.y);
                let edge2 = Point::new(p2.x - p0.x, p2.y - p0.y);
                let cross = edge1.x * edge2.y - edge1.y * edge2.x;

                if cross > 0.0 {
                    let avg_z = (z0 + z1 + z2) / 3.0;
                    tri_indices.push((avg_z, [p0, p1, p2], i0));
                }
            }
        }

        // Sort back-to-front
        tri_indices.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let edge_stroke = Stroke::default()
            .with_color(Color::from_rgb(0.85, 0.88, 0.92))
            .with_width(1.5);

        for (_, [p0, p1, p2], vert_idx) in tri_indices {
            let col = self.mesh.vertices[vert_idx].color;
            let fill_color = Color::from_rgba(col[0], col[1], col[2], col[3]);

            let path = Path::new(|builder| {
                builder.move_to(p0);
                builder.line_to(p1);
                builder.line_to(p2);
                builder.close();
            });

            frame.fill(&path, fill_color);
            frame.stroke(&path, edge_stroke);
        }

        // Draw 3D Orientation Axes Gizmo in top-right corner
        let gizmo_center = Point::new(bounds.width - 50.0, 50.0);
        let axes = [
            (glam::Vec3::X, Color::from_rgb(0.9, 0.2, 0.2)), // X - Red
            (glam::Vec3::Y, Color::from_rgb(0.2, 0.8, 0.2)), // Y - Green
            (glam::Vec3::Z, Color::from_rgb(0.2, 0.4, 0.9)), // Z - Blue
        ];

        let rot = glam::Mat3::from_mat4(self.camera.build_view_projection_matrix());
        for (axis, color) in axes {
            let p_3d = rot * axis;
            let end = Point::new(
                gizmo_center.x + p_3d.x * 25.0,
                gizmo_center.y - p_3d.y * 25.0,
            );
            frame.stroke(
                &Path::line(gizmo_center, end),
                Stroke::default().with_color(color).with_width(3.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

/// Convenience helper to create a canvas element backed by the ViewportWidget.
pub fn viewport_canvas<'a, Message: 'a>(
    camera: &'a Camera,
    mesh: &'a TriMesh,
    map_fn: impl Fn(ViewportMessage) -> Message + 'a,
) -> Element<'a, Message> {
    iced::widget::Canvas::new(ViewportWidget::new(camera, mesh))
        .width(Length::Fill)
        .height(Length::Fill)
        .into_element()
        .map(map_fn)
}
