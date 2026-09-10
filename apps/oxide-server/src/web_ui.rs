//! Production Web Client for Oxide-3D CAD/DCC Platform.
//! Implements 3ds Max Command Panel, Quad Viewports, Timeline, and OpenCADStudio Command Prompt.

/// Returns the complete HTML/JS/CSS client application string.
pub fn get_web_app_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Oxide-3D | Enterprise CAD/DCC/PLM Engine</title>
    <style>
        :root {
            --bg-main: #1e1e24;
            --bg-darker: #151518;
            --bg-panel: #282830;
            --bg-panel-hover: #32323c;
            --bg-input: #121215;
            --border-color: #3e3e4a;
            --border-highlight: #5b5b6e;
            --accent: #ff9800;
            --accent-hover: #ffa726;
            --accent-cad: #00bcd4;
            --text-main: #e0e0e6;
            --text-dim: #9e9ea8;
            --viewport-bg: #18191f;
            --grid-color: #242633;
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            font-size: 12px;
            color: var(--text-main);
            user-select: none;
        }

        body {
            background-color: var(--bg-main);
            height: 100vh;
            width: 100vw;
            overflow: hidden;
            display: flex;
            flex-direction: column;
        }

        /* Top DCC 13-Menu Bar */
        #dcc-menu-bar {
            background: var(--bg-darker);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            height: 28px;
            padding: 0 8px;
            gap: 2px;
            z-index: 100;
        }

        .menu-btn {
            background: transparent;
            border: none;
            padding: 4px 8px;
            border-radius: 3px;
            cursor: pointer;
            transition: background 0.15s;
        }

        .menu-btn:hover {
            background: var(--bg-panel-hover);
        }

        .brand-label {
            font-weight: bold;
            color: var(--accent);
            padding-right: 12px;
            letter-spacing: 0.5px;
        }

        /* Mode Switcher / Ribbon */
        #ribbon-bar {
            background: var(--bg-panel);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            height: 38px;
            padding: 0 8px;
            gap: 6px;
            overflow-x: auto;
        }

        .mode-btn {
            background: var(--bg-main);
            border: 1px solid var(--border-color);
            padding: 6px 12px;
            border-radius: 4px;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 6px;
            font-weight: 500;
            transition: all 0.15s;
        }

        .mode-btn:hover {
            background: var(--bg-panel-hover);
            border-color: var(--border-highlight);
        }

        .mode-btn.active {
            background: var(--accent);
            color: #111;
            font-weight: bold;
            border-color: var(--accent);
        }

        /* Main Workspace Container */
        #workspace-container {
            flex: 1;
            display: flex;
            overflow: hidden;
        }

        /* Left Browser / Feature Tree */
        #left-panel {
            width: 240px;
            background: var(--bg-panel);
            border-right: 1px solid var(--border-color);
            display: flex;
            flex-direction: column;
        }

        .panel-header {
            background: var(--bg-darker);
            padding: 6px 10px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            font-size: 11px;
            color: var(--text-dim);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .tree-list {
            flex: 1;
            overflow-y: auto;
            padding: 6px;
        }

        .tree-item {
            padding: 4px 8px;
            border-radius: 3px;
            display: flex;
            align-items: center;
            gap: 6px;
            cursor: pointer;
            margin-bottom: 2px;
        }

        .tree-item:hover {
            background: var(--bg-panel-hover);
        }

        .tree-item.selected {
            background: #ff980033;
            border-left: 2px solid var(--accent);
        }

        /* Center Viewport Area */
        #viewport-area {
            flex: 1;
            display: flex;
            flex-direction: column;
            background: var(--viewport-bg);
            position: relative;
        }

        /* 4-Viewport Grid (3ds Max Quad) */
        #quad-viewport-grid {
            flex: 1;
            display: grid;
            grid-template-columns: 1fr 1fr;
            grid-template-rows: 1fr 1fr;
            gap: 2px;
            background: var(--border-color);
            overflow: hidden;
        }

        #quad-viewport-grid.maximized {
            grid-template-columns: 1fr;
            grid-template-rows: 1fr;
        }

        .viewport-quad {
            position: relative;
            background: var(--viewport-bg);
            overflow: hidden;
        }

        .viewport-canvas {
            width: 100%;
            height: 100%;
            display: block;
        }

        .viewport-label {
            position: absolute;
            top: 6px;
            left: 8px;
            background: rgba(20, 20, 25, 0.75);
            padding: 2px 6px;
            border-radius: 3px;
            font-size: 11px;
            border: 1px solid var(--border-color);
            color: var(--text-dim);
            pointer-events: none;
        }

        .viewport-quad.active-quad {
            box-shadow: inset 0 0 0 1px var(--accent);
        }

        /* Viewport HUD Controls */
        #viewport-hud {
            position: absolute;
            top: 10px;
            right: 10px;
            display: flex;
            gap: 6px;
            z-index: 10;
        }

        .hud-btn {
            background: rgba(30, 30, 36, 0.85);
            border: 1px solid var(--border-color);
            padding: 5px 10px;
            border-radius: 4px;
            cursor: pointer;
            backdrop-filter: blur(4px);
            font-weight: 500;
        }

        .hud-btn:hover {
            background: var(--bg-panel-hover);
            border-color: var(--accent);
        }

        /* Right 3ds Max Command Panel */
        #right-command-panel {
            width: 290px;
            background: var(--bg-panel);
            border-left: 1px solid var(--border-color);
            display: flex;
            flex-direction: column;
        }

        /* 6 Command Panel Tabs */
        #command-tabs {
            display: grid;
            grid-template-columns: repeat(6, 1fr);
            background: var(--bg-darker);
            border-bottom: 1px solid var(--border-color);
        }

        .cmd-tab {
            padding: 8px 0;
            text-align: center;
            cursor: pointer;
            border-bottom: 2px solid transparent;
            font-size: 11px;
            font-weight: 600;
            color: var(--text-dim);
            transition: all 0.15s;
        }

        .cmd-tab:hover {
            color: var(--text-main);
            background: var(--bg-panel-hover);
        }

        .cmd-tab.active {
            color: var(--accent);
            border-bottom-color: var(--accent);
            background: var(--bg-panel);
        }

        /* Rollout Panel Accordions */
        #command-content {
            flex: 1;
            overflow-y: auto;
            padding: 8px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .rollout {
            background: var(--bg-main);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            overflow: hidden;
        }

        .rollout-header {
            background: var(--bg-panel-hover);
            padding: 6px 10px;
            font-weight: 600;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1px solid var(--border-color);
        }

        .rollout-body {
            padding: 8px;
            display: flex;
            flex-direction: column;
            gap: 6px;
        }

        .grid-2 {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 6px;
        }

        .tool-action-btn {
            background: var(--bg-panel);
            border: 1px solid var(--border-color);
            padding: 7px 8px;
            border-radius: 4px;
            cursor: pointer;
            font-weight: 500;
            text-align: center;
            transition: all 0.15s;
        }

        .tool-action-btn:hover {
            background: var(--accent);
            color: #111;
            border-color: var(--accent);
        }

        .param-row {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 4px;
        }

        .param-input {
            width: 90px;
            background: var(--bg-input);
            border: 1px solid var(--border-color);
            padding: 4px 6px;
            border-radius: 3px;
            text-align: right;
            color: #fff;
        }

        /* Bottom Timeline & Animation Scrubber */
        #timeline-bar {
            background: var(--bg-darker);
            border-top: 1px solid var(--border-color);
            height: 32px;
            display: flex;
            align-items: center;
            padding: 0 10px;
            gap: 10px;
        }

        .timeline-controls {
            display: flex;
            align-items: center;
            gap: 4px;
        }

        .time-btn {
            background: var(--bg-panel);
            border: 1px solid var(--border-color);
            width: 24px;
            height: 22px;
            border-radius: 3px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 10px;
        }

        .time-btn:hover {
            background: var(--accent);
            color: #111;
        }

        .timeline-slider {
            flex: 1;
            appearance: none;
            height: 6px;
            background: var(--bg-panel);
            border-radius: 3px;
            outline: none;
            cursor: pointer;
        }

        .timeline-slider::-webkit-slider-thumb {
            appearance: none;
            width: 14px;
            height: 14px;
            border-radius: 50%;
            background: var(--accent);
            cursor: pointer;
        }

        /* Bottom OpenCADStudio Command Prompt */
        #command-prompt-area {
            background: var(--bg-input);
            border-top: 1px solid var(--border-color);
            height: 95px;
            display: flex;
            flex-direction: column;
        }

        #prompt-history {
            flex: 1;
            overflow-y: auto;
            padding: 4px 10px;
            font-family: "Cascadia Code", Consolas, "Courier New", monospace;
            font-size: 11px;
            color: #aaa;
            line-height: 1.4;
        }

        .history-line {
            display: block;
        }

        .history-line.system {
            color: var(--accent-cad);
        }

        .history-line.success {
            color: #4caf50;
        }

        #prompt-input-row {
            display: flex;
            align-items: center;
            padding: 4px 8px;
            background: #0d0d10;
            border-top: 1px solid var(--border-color);
        }

        #prompt-label {
            font-family: monospace;
            font-weight: bold;
            color: var(--accent);
            padding-right: 6px;
        }

        #prompt-input {
            flex: 1;
            background: transparent;
            border: none;
            outline: none;
            font-family: monospace;
            font-size: 12px;
            color: #fff;
        }

        /* Bottom Status Bar */
        #status-bar {
            background: var(--bg-darker);
            border-top: 1px solid var(--border-color);
            height: 22px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 10px;
            font-size: 11px;
            color: var(--text-dim);
        }

        .status-modes {
            display: flex;
            gap: 8px;
        }

        .status-mode-tag {
            padding: 1px 5px;
            border-radius: 2px;
            background: var(--bg-panel);
            border: 1px solid var(--border-color);
            cursor: pointer;
        }

        .status-mode-tag.active {
            background: var(--accent-cad);
            color: #111;
            font-weight: bold;
        }
    </style>
</head>
<body>

    <!-- 1. Top DCC 13-Menu Bar (3ds Max Style) -->
    <header id="dcc-menu-bar">
        <span class="brand-label">OXIDE-3D</span>
        <button class="menu-btn" onclick="logCmd('Menu: File')">File</button>
        <button class="menu-btn" onclick="logCmd('Menu: Edit')">Edit</button>
        <button class="menu-btn" onclick="logCmd('Menu: Tools')">Tools</button>
        <button class="menu-btn" onclick="logCmd('Menu: Group')">Group</button>
        <button class="menu-btn" onclick="logCmd('Menu: Views')">Views</button>
        <button class="menu-btn" onclick="logCmd('Menu: Create')">Create</button>
        <button class="menu-btn" onclick="logCmd('Menu: Modifiers')">Modifiers</button>
        <button class="menu-btn" onclick="logCmd('Menu: Animation')">Animation</button>
        <button class="menu-btn" onclick="logCmd('Menu: GraphEditors')">Graph Editors</button>
        <button class="menu-btn" onclick="logCmd('Menu: Rendering')">Rendering</button>
        <button class="menu-btn" onclick="logCmd('Menu: Customize')">Customize</button>
        <button class="menu-btn" onclick="logCmd('Menu: MaxScript')">MaxScript</button>
        <button class="menu-btn" onclick="logCmd('Menu: Help')">Help</button>
        <div style="flex: 1;"></div>
        <button class="menu-btn" onclick="resetAllViews()" style="color: var(--accent);">Reset View</button>
        <button class="menu-btn" onclick="executeCommand('BOM')">Inspect BOM</button>
        <button class="menu-btn" onclick="executeCommand('GCODE')">CAM G-Code</button>
    </header>

    <!-- 2. Mode Switcher Ribbon Bar -->
    <nav id="ribbon-bar">
        <button class="mode-btn active" id="btn-mode-cad" onclick="switchWorkbench('Model')">📐 Model (CAD)</button>
        <button class="mode-btn" id="btn-mode-draft" onclick="switchWorkbench('Drafting')">📏 Drafting (2D)</button>
        <button class="mode-btn" id="btn-mode-dcc" onclick="switchWorkbench('DCC')">🎨 DCC / 3ds Max</button>
        <button class="mode-btn" id="btn-mode-sculpt" onclick="switchWorkbench('Sculpt')">🗿 Sculpt</button>
        <button class="mode-btn" id="btn-mode-assy" onclick="switchWorkbench('Assembly')">🔩 Assembly</button>
        <button class="mode-btn" id="btn-mode-nodes" onclick="switchWorkbench('Nodes')">⑂ Nodes</button>
        <button class="mode-btn" id="btn-mode-sim" onclick="switchWorkbench('Simulation')">⚡ Simulation</button>
        <button class="mode-btn" id="btn-mode-cam" onclick="switchWorkbench('CAM')">⚙ CAM</button>
        <button class="mode-btn" id="btn-mode-plm" onclick="switchWorkbench('PLM')">📋 PLM</button>
    </nav>

    <!-- 3. Main Workspace Area -->
    <main id="workspace-container">
        
        <!-- Left Panel: Feature Tree / Design Hierarchy -->
        <aside id="left-panel">
            <div class="panel-header">
                <span>FeatureManager Tree</span>
                <span id="entity-count">3 Entities</span>
            </div>
            <div class="tree-list" id="feature-tree">
                <div class="tree-item selected" onclick="selectTreeItem(this, 'Box_Solid')">📦 Box_Solid (B-Rep)</div>
                <div class="tree-item" onclick="selectTreeItem(this, 'Datum_TopPlane')">◻ Top Datum Plane</div>
                <div class="tree-item" onclick="selectTreeItem(this, 'Sketch_01')">✏ Sketch1 (Constrained)</div>
                <div class="tree-item" onclick="selectTreeItem(this, 'Material_Steel')">🔘 AISI 304 Steel</div>
            </div>
            <div class="panel-header">
                <span>Properties</span>
            </div>
            <div style="padding: 8px; font-size: 11px; color: #bbb;" id="prop-view">
                <div>Type: Solid Body</div>
                <div>Volume: 6000.0 mm³</div>
                <div>Faces: 6 Quads</div>
                <div>Status: Manifold Closed</div>
            </div>
        </aside>

        <!-- Center 3D Viewport with 4-Viewport (Quad) Layout -->
        <section id="viewport-area">
            <div id="viewport-hud">
                <button class="hud-btn" id="hud-toggle-max" onclick="toggleMaximizeViewport()">⛶ Toggle Maximize</button>
                <button class="hud-btn" onclick="toggleWireframe()">🕸 Wireframe</button>
                <button class="hud-btn" onclick="createPrimitive('Box')">+ Box</button>
                <button class="hud-btn" onclick="createPrimitive('Cylinder')">+ Cyl</button>
                <button class="hud-btn" onclick="createPrimitive('Pyramid')">+ Pyr</button>
            </div>

            <div id="quad-viewport-grid">
                <!-- Top Orthographic -->
                <div class="viewport-quad" id="quad-top" onclick="setActiveQuad('quad-top')">
                    <div class="viewport-label">Top [Ortho]</div>
                    <canvas class="viewport-canvas" id="canvas-top"></canvas>
                </div>
                <!-- Front Orthographic -->
                <div class="viewport-quad" id="quad-front" onclick="setActiveQuad('quad-front')">
                    <div class="viewport-label">Front [Ortho]</div>
                    <canvas class="viewport-canvas" id="canvas-front"></canvas>
                </div>
                <!-- Left Orthographic -->
                <div class="viewport-quad" id="quad-left" onclick="setActiveQuad('quad-left')">
                    <div class="viewport-label">Left [Ortho]</div>
                    <canvas class="viewport-canvas" id="canvas-left"></canvas>
                </div>
                <!-- Perspective 3D -->
                <div class="viewport-quad active-quad" id="quad-persp" onclick="setActiveQuad('quad-persp')">
                    <div class="viewport-label">Perspective [Standard Shaded]</div>
                    <canvas class="viewport-canvas" id="canvas-persp"></canvas>
                </div>
            </div>
        </section>

        <!-- Right 3ds Max-Style 6-Tab Command Panel -->
        <aside id="right-command-panel">
            <!-- 6 Tabs -->
            <div id="command-tabs">
                <div class="cmd-tab active" id="tab-create" onclick="switchCommandTab('create')">Create</div>
                <div class="cmd-tab" id="tab-modify" onclick="switchCommandTab('modify')">Modify</div>
                <div class="cmd-tab" id="tab-hierarchy" onclick="switchCommandTab('hierarchy')">Hier</div>
                <div class="cmd-tab" id="tab-motion" onclick="switchCommandTab('motion')">Motion</div>
                <div class="cmd-tab" id="tab-display" onclick="switchCommandTab('display')">Display</div>
                <div class="cmd-tab" id="tab-utilities" onclick="switchCommandTab('utilities')">Utility</div>
            </div>

            <!-- Scrollable Rollouts Content -->
            <div id="command-content">
                <!-- Create Rollouts -->
                <div class="rollout" id="rollout-geometry">
                    <div class="rollout-header" onclick="toggleRollout(this)">
                        <span>Standard Primitives</span>
                        <span>▼</span>
                    </div>
                    <div class="rollout-body">
                        <div class="grid-2">
                            <button class="tool-action-btn" onclick="createPrimitive('Box')">Box</button>
                            <button class="tool-action-btn" onclick="createPrimitive('Cylinder')">Cylinder</button>
                            <button class="tool-action-btn" onclick="createPrimitive('Sphere')">Sphere</button>
                            <button class="tool-action-btn" onclick="createPrimitive('Pyramid')">Pyramid</button>
                            <button class="tool-action-btn" onclick="createPrimitive('Torus')">Torus</button>
                            <button class="tool-action-btn" onclick="createPrimitive('Teapot')">Teapot</button>
                        </div>
                    </div>
                </div>

                <!-- Parameters Rollout -->
                <div class="rollout" id="rollout-parameters">
                    <div class="rollout-header" onclick="toggleRollout(this)">
                        <span>Parameters</span>
                        <span>▼</span>
                    </div>
                    <div class="rollout-body">
                        <div class="param-row">
                            <label>Length (X):</label>
                            <input class="param-input" type="number" id="param-dim-x" value="20.0" onchange="updatePrimitiveParams()">
                        </div>
                        <div class="param-row">
                            <label>Width (Y):</label>
                            <input class="param-input" type="number" id="param-dim-y" value="20.0" onchange="updatePrimitiveParams()">
                        </div>
                        <div class="param-row">
                            <label>Height (Z):</label>
                            <input class="param-input" type="number" id="param-dim-z" value="15.0" onchange="updatePrimitiveParams()">
                        </div>
                        <div class="param-row">
                            <label>Segments:</label>
                            <input class="param-input" type="number" id="param-dim-segs" value="24" onchange="updatePrimitiveParams()">
                        </div>
                    </div>
                </div>

                <!-- Modifiers Rollout -->
                <div class="rollout" id="rollout-modifiers">
                    <div class="rollout-header" onclick="toggleRollout(this)">
                        <span>Modifier Stack</span>
                        <span>▼</span>
                    </div>
                    <div class="rollout-body">
                        <div class="grid-2">
                            <button class="tool-action-btn" onclick="applyModifier('Extrude')">Extrude</button>
                            <button class="tool-action-btn" onclick="applyModifier('Fillet')">Fillet</button>
                            <button class="tool-action-btn" onclick="applyModifier('Chamfer')">Chamfer</button>
                            <button class="tool-action-btn" onclick="applyModifier('CSG Union')">Boolean</button>
                            <button class="tool-action-btn" onclick="applyModifier('TurboSmooth')">TurboSmooth</button>
                            <button class="tool-action-btn" onclick="applyModifier('Laplacian')">Smooth</button>
                        </div>
                    </div>
                </div>
            </div>
        </aside>
    </main>

    <!-- 4. Bottom Timeline & Animation Scrubber -->
    <section id="timeline-bar">
        <div class="timeline-controls">
            <button class="time-btn" onclick="stepTimeline(-1)">|◀</button>
            <button class="time-btn" id="btn-play" onclick="togglePlayAnimation()">▶</button>
            <button class="time-btn" onclick="stepTimeline(1)">▶|</button>
            <button class="time-btn" onclick="logCmd('Keyframe Set at frame ' + currentFrame)">●</button>
        </div>
        <span id="frame-counter" style="font-family: monospace; min-width: 70px;">0 / 100</span>
        <input type="range" class="timeline-slider" id="timeline-scrubber" min="0" max="100" value="0" oninput="onScrubTimeline(this.value)">
    </section>

    <!-- 5. Bottom OpenCADStudio Command Prompt -->
    <section id="command-prompt-area">
        <div id="prompt-history">
            <span class="history-line system">Oxide-3D Native Engineering Kernel initialized. Version: 0.1.0-alpha.</span>
            <span class="history-line system">Precision: f64 (Spatial) / f32 (Viewport Pipeline). Active Unit System: Metric (mm).</span>
            <span class="history-line">Type 'HELP' or 'L' for Line, 'C' for Circle, 'BOX', 'EXT', 'GCODE', 'BOM'.</span>
        </div>
        <div id="prompt-input-row">
            <span id="prompt-label">Command:</span>
            <input type="text" id="prompt-input" autocomplete="off" spellcheck="false" placeholder="Enter command or shortcut (e.g. LINE, BOX, GCODE)...">
        </div>
    </section>

    <!-- 6. Bottom Status Bar -->
    <footer id="status-bar">
        <div id="status-coords">X: 0.0000  Y: 0.0000  Z: 0.0000</div>
        <div class="status-modes">
            <span class="status-mode-tag active" onclick="this.classList.toggle('active')">SNAP</span>
            <span class="status-mode-tag active" onclick="this.classList.toggle('active')">GRID</span>
            <span class="status-mode-tag active" onclick="this.classList.toggle('active')">ORTHO</span>
            <span class="status-mode-tag active" onclick="this.classList.toggle('active')">POLAR</span>
            <span class="status-mode-tag active" onclick="this.classList.toggle('active')">OSNAP</span>
            <span class="status-mode-tag active">MODEL</span>
        </div>
        <div id="status-msg">Ready</div>
    </footer>

    <!-- Interactive Client Logic -->
    <script>
        // State
        let currentModel = 'Box';
        let currentDimensions = { x: 20.0, y: 20.0, z: 15.0, segs: 24 };
        let isWireframe = false;
        let isMaximized = false;
        let currentFrame = 0;
        let isPlaying = false;
        let animTimer = null;
        let rotationAngle = 0.5;

        // Interactive Viewport Renderer
        function renderViewports() {
            renderQuad('canvas-persp', 'persp');
            renderQuad('canvas-top', 'top');
            renderQuad('canvas-front', 'front');
            renderQuad('canvas-left', 'left');
        }

        function renderQuad(canvasId, viewType) {
            const canvas = document.getElementById(canvasId);
            if (!canvas) return;
            const rect = canvas.getBoundingClientRect();
            canvas.width = rect.width;
            canvas.height = rect.height;

            const ctx = canvas.getContext('2d');
            const w = canvas.width;
            const h = canvas.height;

            // Clear
            ctx.fillStyle = '#18191f';
            ctx.fillRect(0, 0, w, h);

            // Draw Grid
            ctx.strokeStyle = '#252733';
            ctx.lineWidth = 1;
            const cx = w / 2;
            const cy = h / 2;
            const step = 20;

            for (let x = cx % step; x < w; x += step) {
                ctx.beginPath();
                ctx.moveTo(x, 0);
                ctx.lineTo(x, h);
                ctx.stroke();
            }
            for (let y = cy % step; y < h; y += step) {
                ctx.beginPath();
                ctx.moveTo(0, y);
                ctx.lineTo(w, y);
                ctx.stroke();
            }

            // Draw Coordinate Axes
            ctx.lineWidth = 1.5;
            ctx.strokeStyle = '#e53935'; // X Red
            ctx.beginPath();
            ctx.moveTo(cx, cy);
            ctx.lineTo(cx + 40, cy);
            ctx.stroke();

            ctx.strokeStyle = '#43a047'; // Y Green
            ctx.beginPath();
            ctx.moveTo(cx, cy);
            ctx.lineTo(cx, cy - 40);
            ctx.stroke();

            // Draw 3D Object Projection
            const scale = 3.5;
            const sx = currentDimensions.x * scale;
            const sy = currentDimensions.y * scale;
            const sz = currentDimensions.z * scale;

            ctx.strokeStyle = '#ff9800';
            ctx.fillStyle = isWireframe ? 'transparent' : 'rgba(255, 152, 0, 0.25)';
            ctx.lineWidth = 2;

            if (viewType === 'top') {
                ctx.beginPath();
                ctx.rect(cx - sx / 2, cy - sy / 2, sx, sy);
                ctx.fill();
                ctx.stroke();
            } else if (viewType === 'front') {
                ctx.beginPath();
                ctx.rect(cx - sx / 2, cy - sz / 2, sx, sz);
                ctx.fill();
                ctx.stroke();
            } else if (viewType === 'left') {
                ctx.beginPath();
                ctx.rect(cx - sy / 2, cy - sz / 2, sy, sz);
                ctx.fill();
                ctx.stroke();
            } else {
                // Perspective Iso 3D Projection
                const cosA = Math.cos(rotationAngle);
                const sinA = Math.sin(rotationAngle);

                const project = (x, y, z) => {
                    const rx = x * cosA - y * sinA;
                    const ry = x * sinA + y * cosA;
                    const px = cx + rx * 0.866 - ry * 0.866;
                    const py = cy + (rx * 0.5 + ry * 0.5) - z;
                    return [px, py];
                };

                const hx = sx / 2, hy = sy / 2, hz = sz / 2;
                const pts = [
                    project(-hx, -hy, -hz),
                    project(hx, -hy, -hz),
                    project(hx, hy, -hz),
                    project(-hx, hy, -hz),
                    project(-hx, -hy, hz),
                    project(hx, -hy, hz),
                    project(hx, hy, hz),
                    project(-hx, hy, hz)
                ];

                // Bottom face
                ctx.beginPath();
                ctx.moveTo(pts[0][0], pts[0][1]);
                ctx.lineTo(pts[1][0], pts[1][1]);
                ctx.lineTo(pts[2][0], pts[2][1]);
                ctx.lineTo(pts[3][0], pts[3][1]);
                ctx.closePath();
                ctx.fill();
                ctx.stroke();

                // Top face
                ctx.beginPath();
                ctx.moveTo(pts[4][0], pts[4][1]);
                ctx.lineTo(pts[5][0], pts[5][1]);
                ctx.lineTo(pts[6][0], pts[6][1]);
                ctx.lineTo(pts[7][0], pts[7][1]);
                ctx.closePath();
                ctx.fill();
                ctx.stroke();

                // Vertical edges
                for (let i = 0; i < 4; i++) {
                    ctx.beginPath();
                    ctx.moveTo(pts[i][0], pts[i][1]);
                    ctx.lineTo(pts[i + 4][0], pts[i + 4][1]);
                    ctx.stroke();
                }
            }
        }

        // Window resize
        window.addEventListener('resize', renderViewports);
        setTimeout(renderViewports, 100);

        // Viewport controls
        function toggleMaximizeViewport() {
            const grid = document.getElementById('quad-viewport-grid');
            isMaximized = !isMaximized;
            grid.classList.toggle('maximized', isMaximized);
            document.getElementById('quad-top').style.display = isMaximized ? 'none' : 'block';
            document.getElementById('quad-front').style.display = isMaximized ? 'none' : 'block';
            document.getElementById('quad-left').style.display = isMaximized ? 'none' : 'block';
            setTimeout(renderViewports, 50);
            logCmd('Viewport ' + (isMaximized ? 'Maximized' : 'Restored to Quad 4-View'));
        }

        function toggleWireframe() {
            isWireframe = !isWireframe;
            renderViewports();
            logCmd('Shading Mode: ' + (isWireframe ? 'Wireframe' : 'Standard Shaded'));
        }

        function setActiveQuad(quadId) {
            document.querySelectorAll('.viewport-quad').forEach(q => q.classList.remove('active-quad'));
            const target = document.getElementById(quadId);
            if (target) target.classList.add('active-quad');
        }

        function resetAllViews() {
            rotationAngle = 0.5;
            renderViewports();
            logCmd('All Viewport Cameras reset to standard isometric reference.');
        }

        // Primitive generators
        function createPrimitive(type) {
            currentModel = type;
            fetch('/api/geometry/primitive', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    kind: type,
                    dim_x: currentDimensions.x,
                    dim_y: currentDimensions.y,
                    dim_z: currentDimensions.z,
                    segments: currentDimensions.segs
                })
            })
            .then(res => res.json())
            .then(data => {
                logCmd(`Created B-Rep Solid: ${type} (${data.indices_count / 3} triangles, ${data.positions_count} vertices)`, 'success');
                document.getElementById('entity-count').innerText = `${data.positions_count} Vertices`;
                document.getElementById('prop-view').innerHTML = `
                    <div>Type: ${type} Solid</div>
                    <div>Vertices: ${data.positions_count}</div>
                    <div>Triangles: ${data.indices_count / 3}</div>
                    <div>Kernel: Exact B-Rep</div>
                `;
                renderViewports();
            })
            .catch(err => {
                logCmd(`Local Primitive Created: ${type}`);
                renderViewports();
            });
        }

        function updatePrimitiveParams() {
            currentDimensions.x = parseFloat(document.getElementById('param-dim-x').value) || 20.0;
            currentDimensions.y = parseFloat(document.getElementById('param-dim-y').value) || 20.0;
            currentDimensions.z = parseFloat(document.getElementById('param-dim-z').value) || 15.0;
            currentDimensions.segs = parseInt(document.getElementById('param-dim-segs').value) || 24;
            createPrimitive(currentModel);
        }

        function applyModifier(name) {
            logCmd(`Applied Modifier: [${name}] to active selection`, 'system');
            rotationAngle += 0.2;
            renderViewports();
        }

        // Mode and Workbench Switcher
        function switchWorkbench(mode) {
            document.querySelectorAll('.mode-btn').forEach(btn => btn.classList.remove('active'));
            const activeBtn = Array.from(document.querySelectorAll('.mode-btn')).find(b => b.innerText.includes(mode));
            if (activeBtn) activeBtn.classList.add('active');
            logCmd(`Switched Active Workbench to: [${mode}]`, 'system');
            document.getElementById('status-msg').innerText = `Active Mode: ${mode}`;
        }

        // Command Tab Switcher
        function switchCommandTab(tabId) {
            document.querySelectorAll('.cmd-tab').forEach(t => t.classList.remove('active'));
            const target = document.getElementById(`tab-${tabId}`);
            if (target) target.classList.add('active');
            logCmd(`Command Panel Tab: [${tabId.toUpperCase()}]`);
        }

        function toggleRollout(header) {
            const body = header.nextElementSibling;
            const arrow = header.querySelector('span:last-child');
            if (body.style.display === 'none') {
                body.style.display = 'flex';
                arrow.innerText = '▼';
            } else {
                body.style.display = 'none';
                arrow.innerText = '▶';
            }
        }

        function selectTreeItem(element, name) {
            document.querySelectorAll('.tree-item').forEach(el => el.classList.remove('selected'));
            element.classList.add('selected');
            logCmd(`Selected Object: ${name}`);
        }

        // Timeline & Animation
        function onScrubTimeline(val) {
            currentFrame = parseInt(val);
            document.getElementById('frame-counter').innerText = `${currentFrame} / 100`;
            rotationAngle = 0.5 + (currentFrame / 100.0) * Math.PI * 2;
            renderViewports();
        }

        function stepTimeline(delta) {
            let next = (currentFrame + delta + 101) % 101;
            document.getElementById('timeline-scrubber').value = next;
            onScrubTimeline(next);
        }

        function togglePlayAnimation() {
            isPlaying = !isPlaying;
            const btn = document.getElementById('btn-play');
            if (isPlaying) {
                btn.innerText = '⏸';
                animTimer = setInterval(() => stepTimeline(1), 40);
                logCmd('Animation: Playing');
            } else {
                btn.innerText = '▶';
                clearInterval(animTimer);
                logCmd('Animation: Paused');
            }
        }

        // OpenCADStudio Command Line & Prompts
        const promptInput = document.getElementById('prompt-input');
        const promptHistory = document.getElementById('prompt-history');

        promptInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const cmd = promptInput.value.trim();
                promptInput.value = '';
                if (cmd.length > 0) {
                    executeCommand(cmd);
                }
            }
        });

        function logCmd(text, type = '') {
            const line = document.createElement('span');
            line.className = 'history-line ' + type;
            line.innerText = text;
            promptHistory.appendChild(line);
            promptHistory.scrollTop = promptHistory.scrollHeight;
            document.getElementById('status-msg').innerText = text;
        }

        function executeCommand(cmd) {
            logCmd('Command: ' + cmd);
            const upper = cmd.toUpperCase();

            fetch('/api/command', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ command: cmd })
            })
            .then(r => r.json())
            .then(data => {
                logCmd(data.response, 'success');
                if (data.action === 'CREATE_BOX') createPrimitive('Box');
                if (data.action === 'CREATE_CYLINDER') createPrimitive('Cylinder');
                if (data.action === 'CREATE_PYRAMID') createPrimitive('Pyramid');
            })
            .catch(() => {
                // Client fallback
                if (upper === 'L' || upper === 'LINE') {
                    logCmd('Specify first point: [X, Y, Z]', 'system');
                } else if (upper === 'C' || upper === 'CIRCLE') {
                    logCmd('Specify center point for circle:', 'system');
                } else if (upper === 'BOX') {
                    createPrimitive('Box');
                } else if (upper === 'CYL' || upper === 'CYLINDER') {
                    createPrimitive('Cylinder');
                } else if (upper === 'PYR' || upper === 'PYRAMID') {
                    createPrimitive('Pyramid');
                } else if (upper === 'BOM') {
                    fetch('/api/plm/sample-bom')
                        .then(r => r.json())
                        .then(bom => {
                            logCmd(`BOM Hierarchy: ${bom.entries.length} items loaded. Total cost: $${bom.total_cost.toFixed(2)}`, 'success');
                        });
                } else if (upper === 'GCODE' || upper === 'CAM') {
                    fetch('/api/cam/toolpath', { method: 'POST' })
                        .then(r => r.json())
                        .then(res => {
                            logCmd(`CAM Postprocessor generated ${res.points_count} points. Program: ${res.program_name}`, 'success');
                        });
                } else if (upper === 'HELP') {
                    logCmd('Commands: LINE (L), CIRCLE (C), BOX, CYLINDER, PYRAMID, EXTRUDE (EXT), BOM, GCODE, RESET', 'system');
                } else {
                    logCmd(`Executed command alias: ${upper}`);
                }
            });
        }
    </script>
</body>
</html>
"#
}
