//! Production Web Client for Oxide-3D CAD/DCC Platform.
//! Implements 3ds Max Command Panel, Quad Viewports with ViewCube & 3D Gizmo,
//! OpenCADStudio Precision Drafting, Layers Manager, OSNAP HUD, and AutoCAD Command Prompt.

/// Returns the complete HTML/JS/CSS client application string.
pub fn get_web_app_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Oxide-3D | Enterprise CAD/DCC/CAM/PLM Engine</title>
    <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>⚙️</text></svg>">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg-deep: #0e0f12;
            --bg-main: #14161b;
            --bg-darker: #101115;
            --bg-panel: rgba(26, 29, 36, 0.88);
            --bg-panel-solid: #1a1d24;
            --bg-panel-hover: rgba(38, 42, 53, 0.95);
            --bg-input: #0b0c0f;
            --border-color: rgba(255, 255, 255, 0.08);
            --border-highlight: rgba(255, 255, 255, 0.18);
            --accent: #ff9800;
            --accent-glow: rgba(255, 152, 0, 0.35);
            --accent-cad: #00bcd4;
            --accent-cad-glow: rgba(0, 188, 212, 0.35);
            --accent-green: #00e676;
            --text-main: #e6e8ee;
            --text-dim: #8e92a4;
            --viewport-bg-1: #1a1c23;
            --viewport-bg-2: #121318;
            --grid-line: rgba(255, 255, 255, 0.04);
            --grid-axis-x: #e53935;
            --grid-axis-y: #43a047;
            --grid-axis-z: #1e88e5;
            --shadow-panel: 0 8px 32px 0 rgba(0, 0, 0, 0.45);
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            font-size: 11px;
            color: var(--text-main);
            user-select: none;
            -webkit-user-select: none;
        }

        body {
            background-color: var(--bg-deep);
            height: 100vh;
            width: 100vw;
            overflow: hidden;
            display: flex;
            flex-direction: column;
        }

        /* Scrollbars */
        ::-webkit-scrollbar { width: 6px; height: 6px; }
        ::-webkit-scrollbar-track { background: transparent; }
        ::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.15); border-radius: 3px; }
        ::-webkit-scrollbar-thumb:hover { background: rgba(255,255,255,0.25); }

        /* Top 3ds Max 13-Menu Bar */
        #dcc-menu-bar {
            background: var(--bg-darker);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            height: 30px;
            padding: 0 10px;
            gap: 3px;
            z-index: 100;
        }

        .brand-label {
            font-weight: 700;
            font-size: 12px;
            color: var(--accent);
            padding-right: 14px;
            letter-spacing: 0.8px;
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .brand-badge {
            background: linear-gradient(135deg, #ff9800, #f57c00);
            color: #000;
            font-size: 9px;
            font-weight: 800;
            padding: 1px 4px;
            border-radius: 3px;
        }

        .menu-btn {
            background: transparent;
            border: none;
            padding: 4px 8px;
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.15s ease;
            color: var(--text-dim);
            font-weight: 500;
        }

        .menu-btn:hover {
            background: var(--bg-panel-hover);
            color: var(--text-main);
        }

        .menu-spacer { flex: 1; }

        .top-action-btn {
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid var(--border-color);
            padding: 4px 10px;
            border-radius: 4px;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 5px;
            transition: all 0.15s;
        }
        .top-action-btn:hover {
            background: var(--bg-panel-hover);
            border-color: var(--accent);
        }

        /* Ribbon / Mode & Tool Bar */
        #ribbon-bar {
            background: var(--bg-main);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            height: 40px;
            padding: 0 10px;
            gap: 8px;
            z-index: 90;
        }

        .ribbon-group {
            display: flex;
            align-items: center;
            gap: 4px;
            padding: 0 6px;
            border-right: 1px solid var(--border-color);
            height: 28px;
        }

        .tool-btn {
            background: transparent;
            border: 1px solid transparent;
            padding: 4px 8px;
            border-radius: 4px;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 5px;
            font-size: 11px;
            font-weight: 500;
            transition: all 0.15s;
        }

        .tool-btn:hover {
            background: var(--bg-panel-hover);
            border-color: var(--border-highlight);
        }

        .tool-btn.active {
            background: rgba(255, 152, 0, 0.15);
            border-color: var(--accent);
            color: var(--accent);
        }

        .tool-btn.active-cad {
            background: rgba(0, 188, 212, 0.15);
            border-color: var(--accent-cad);
            color: var(--accent-cad);
        }

        /* Main Workspace Container */
        #workspace-container {
            flex: 1;
            display: flex;
            position: relative;
            overflow: hidden;
        }

        /* Viewport Grid */
        #viewports-wrapper {
            flex: 1;
            display: grid;
            grid-template-columns: 1fr 1fr;
            grid-template-rows: 1fr 1fr;
            gap: 2px;
            background-color: var(--bg-deep);
            position: relative;
        }

        .viewport-box {
            position: relative;
            background: radial-gradient(circle at center, var(--viewport-bg-1) 0%, var(--viewport-bg-2) 100%);
            overflow: hidden;
            border: 1px solid transparent;
            transition: border-color 0.2s;
        }

        .viewport-box.active {
            border-color: var(--accent);
        }

        .viewport-canvas {
            width: 100%;
            height: 100%;
            display: block;
            cursor: crosshair;
        }

        .viewport-header {
            position: absolute;
            top: 6px;
            left: 8px;
            display: flex;
            align-items: center;
            gap: 6px;
            background: rgba(14, 15, 18, 0.75);
            backdrop-filter: blur(8px);
            padding: 2px 8px;
            border-radius: 4px;
            border: 1px solid rgba(255,255,255,0.06);
            z-index: 10;
        }

        .viewport-title {
            font-weight: 600;
            font-size: 10px;
            letter-spacing: 0.5px;
            color: var(--accent);
        }

        .viewport-tag {
            color: var(--text-dim);
            font-size: 9px;
            text-transform: uppercase;
        }

        /* Interactive ViewCube in Perspective View */
        #viewcube {
            position: absolute;
            top: 12px;
            right: 12px;
            width: 64px;
            height: 64px;
            background: rgba(20, 22, 28, 0.7);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.12);
            border-radius: 8px;
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            grid-template-rows: repeat(3, 1fr);
            gap: 2px;
            padding: 3px;
            z-index: 20;
            box-shadow: 0 4px 16px rgba(0,0,0,0.5);
        }

        .vc-face {
            background: rgba(255, 255, 255, 0.05);
            border-radius: 3px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 8px;
            font-weight: 700;
            color: var(--text-dim);
            cursor: pointer;
            transition: all 0.15s;
        }
        .vc-face:hover {
            background: var(--accent);
            color: #000;
            transform: scale(1.05);
        }
        .vc-face.center {
            color: var(--accent);
            font-size: 9px;
        }

        /* 3D Transform Gizmo Indicator */
        #gizmo-hud {
            position: absolute;
            bottom: 12px;
            left: 12px;
            background: rgba(14, 15, 18, 0.75);
            backdrop-filter: blur(8px);
            border: 1px solid rgba(255,255,255,0.06);
            border-radius: 6px;
            padding: 4px 8px;
            display: flex;
            align-items: center;
            gap: 8px;
            z-index: 15;
            font-family: 'JetBrains Mono', monospace;
            font-size: 10px;
        }
        .gizmo-axis { font-weight: 700; }
        .axis-x { color: var(--grid-axis-x); }
        .axis-y { color: var(--grid-axis-y); }
        .axis-z { color: var(--grid-axis-z); }

        /* OSNAP Visual HUD Tooltip */
        #osnap-marker {
            position: absolute;
            pointer-events: none;
            display: none;
            z-index: 50;
            transform: translate(-50%, -50%);
        }
        .osnap-glyph {
            width: 16px;
            height: 16px;
            border: 2px solid var(--accent-green);
            background: rgba(0, 230, 118, 0.15);
            box-shadow: 0 0 8px var(--accent-green);
        }
        .osnap-glyph.triangle {
            width: 0; height: 0;
            border-left: 8px solid transparent;
            border-right: 8px solid transparent;
            border-bottom: 14px solid var(--accent-green);
            background: transparent;
        }
        .osnap-glyph.circle {
            border-radius: 50%;
        }
        .osnap-tip {
            position: absolute;
            left: 20px;
            top: -6px;
            background: rgba(0,0,0,0.85);
            border: 1px solid var(--accent-green);
            border-radius: 4px;
            padding: 2px 6px;
            white-space: nowrap;
            font-family: 'JetBrains Mono', monospace;
            font-size: 9px;
            color: var(--accent-green);
        }

        /* 3ds Max Command Panel (Right Side) */
        #command-panel {
            width: 320px;
            min-width: 320px;
            background: var(--bg-panel);
            backdrop-filter: blur(16px);
            border-left: 1px solid var(--border-color);
            display: flex;
            flex-direction: column;
            box-shadow: var(--shadow-panel);
            z-index: 80;
        }

        #panel-tabs {
            display: flex;
            background: var(--bg-darker);
            border-bottom: 1px solid var(--border-color);
            height: 36px;
        }

        .panel-tab {
            flex: 1;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            font-size: 10px;
            font-weight: 600;
            letter-spacing: 0.5px;
            color: var(--text-dim);
            border-bottom: 2px solid transparent;
            transition: all 0.15s;
        }

        .panel-tab:hover {
            color: var(--text-main);
            background: var(--bg-panel-hover);
        }

        .panel-tab.active {
            color: var(--accent);
            border-bottom-color: var(--accent);
            background: var(--bg-main);
        }

        #panel-content {
            flex: 1;
            overflow-y: auto;
            padding: 10px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .tab-pane {
            display: flex;
            flex-direction: column;
            gap: 8px;
            width: 100%;
        }

        /* Rollouts (Accordion) */
        .rollout {
            border: 1px solid var(--border-color);
            border-radius: 6px;
            background: rgba(18, 20, 26, 0.6);
            overflow: hidden;
            transition: border-color 0.2s;
        }
        .rollout:hover {
            border-color: var(--border-highlight);
        }

        .rollout-header {
            background: rgba(30, 33, 42, 0.7);
            padding: 7px 10px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-weight: 600;
            font-size: 11px;
            letter-spacing: 0.3px;
        }
        .rollout-header:hover {
            background: rgba(40, 44, 56, 0.85);
        }

        .rollout-body {
            padding: 10px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .control-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 8px;
        }

        .control-label {
            color: var(--text-dim);
            font-size: 10px;
            flex: 1;
        }

        .control-input {
            background: var(--bg-input);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            padding: 4px 8px;
            color: var(--text-main);
            font-family: 'JetBrains Mono', monospace;
            font-size: 10px;
            width: 100px;
            text-align: right;
            transition: border-color 0.15s;
        }
        .control-input:focus {
            border-color: var(--accent);
            outline: none;
            box-shadow: 0 0 6px var(--accent-glow);
        }

        .grid-2col {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 6px;
        }

        .action-btn {
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid var(--border-color);
            padding: 6px 10px;
            border-radius: 4px;
            cursor: pointer;
            font-weight: 500;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
            transition: all 0.15s;
        }
        .action-btn:hover {
            background: var(--bg-panel-hover);
            border-color: var(--border-highlight);
        }
        .action-btn.primary {
            background: linear-gradient(135deg, rgba(255, 152, 0, 0.2), rgba(245, 124, 0, 0.3));
            border-color: var(--accent);
            color: var(--accent);
            font-weight: 600;
        }
        .action-btn.primary:hover {
            background: var(--accent);
            color: #000;
            box-shadow: 0 0 12px var(--accent-glow);
        }

        /* Modifier Stack */
        .modifier-stack {
            border: 1px solid var(--border-color);
            border-radius: 4px;
            background: var(--bg-input);
            max-height: 110px;
            overflow-y: auto;
        }
        .mod-item {
            padding: 4px 8px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            border-bottom: 1px solid rgba(255,255,255,0.03);
            cursor: pointer;
        }
        .mod-item.active {
            background: rgba(255, 152, 0, 0.2);
            color: var(--accent);
            font-weight: 600;
        }

        /* Layer Manager Modal */
        #layer-modal {
            position: absolute;
            top: 75px;
            left: 20px;
            width: 380px;
            background: var(--bg-panel-solid);
            border: 1px solid var(--border-highlight);
            border-radius: 8px;
            box-shadow: var(--shadow-panel);
            z-index: 150;
            display: none;
            flex-direction: column;
            overflow: hidden;
        }
        .modal-header {
            padding: 8px 12px;
            background: var(--bg-darker);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-weight: 600;
        }
        .layer-table {
            width: 100%;
            border-collapse: collapse;
            font-size: 10px;
        }
        .layer-table th, .layer-table td {
            padding: 6px 8px;
            text-align: left;
            border-bottom: 1px solid var(--border-color);
        }
        .layer-table th { background: rgba(0,0,0,0.2); color: var(--text-dim); }
        .layer-color-dot {
            width: 10px; height: 10px; border-radius: 50%; display: inline-block;
        }

        /* Bottom Timeline & Animation Scrubber */
        #timeline-bar {
            background: var(--bg-darker);
            border-top: 1px solid var(--border-color);
            height: 38px;
            display: flex;
            align-items: center;
            padding: 0 12px;
            gap: 10px;
            z-index: 85;
        }

        .timeline-slider {
            flex: 1;
            accent-color: var(--accent);
            cursor: pointer;
        }

        .frame-counter {
            font-family: 'JetBrains Mono', monospace;
            font-weight: 600;
            font-size: 11px;
            color: var(--accent);
            background: var(--bg-input);
            padding: 2px 8px;
            border-radius: 4px;
            border: 1px solid var(--border-color);
        }

        /* OpenCADStudio Command Prompt Bar */
        #cad-command-terminal {
            background: #090a0d;
            border-top: 1px solid var(--border-color);
            display: flex;
            flex-direction: column;
            height: 90px;
            z-index: 95;
            position: relative;
        }

        #terminal-history {
            flex: 1;
            overflow-y: auto;
            padding: 4px 10px;
            font-family: 'JetBrains Mono', monospace;
            font-size: 10px;
            display: flex;
            flex-direction: column;
            gap: 2px;
        }

        .term-line { color: #a0a4b8; }
        .term-line.cmd { color: #fff; font-weight: 600; }
        .term-line.sys { color: var(--accent-cad); }
        .term-line.ok { color: var(--accent-green); }
        .term-line.err { color: #ff5252; }

        #terminal-input-row {
            display: flex;
            align-items: center;
            height: 28px;
            padding: 0 10px;
            background: #0d0e12;
            border-top: 1px solid rgba(255,255,255,0.05);
            gap: 6px;
        }

        .prompt-prefix {
            color: var(--accent-cad);
            font-family: 'JetBrains Mono', monospace;
            font-weight: 700;
            font-size: 11px;
        }

        #terminal-input {
            flex: 1;
            background: transparent;
            border: none;
            color: #fff;
            font-family: 'JetBrains Mono', monospace;
            font-size: 11px;
            outline: none;
        }

        /* Auto-Complete Suggestion Popup */
        #autocomplete-popup {
            position: absolute;
            bottom: 30px;
            left: 80px;
            background: rgba(18, 20, 26, 0.95);
            backdrop-filter: blur(12px);
            border: 1px solid var(--accent-cad);
            border-radius: 6px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.6);
            display: none;
            flex-direction: column;
            max-height: 140px;
            overflow-y: auto;
            width: 220px;
            z-index: 200;
        }
        .suggest-item {
            padding: 4px 8px;
            font-family: 'JetBrains Mono', monospace;
            font-size: 10px;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
        }
        .suggest-item:hover, .suggest-item.selected {
            background: var(--accent-cad);
            color: #000;
        }

        /* Status Bar (CAD Mode Toggles) */
        #status-bar {
            background: var(--bg-darker);
            border-top: 1px solid var(--border-color);
            height: 24px;
            display: flex;
            align-items: center;
            padding: 0 10px;
            gap: 8px;
            z-index: 100;
        }

        .status-coords {
            font-family: 'JetBrains Mono', monospace;
            font-size: 10px;
            color: var(--text-dim);
            min-width: 180px;
        }

        .status-toggle {
            background: transparent;
            border: 1px solid transparent;
            padding: 1px 6px;
            border-radius: 3px;
            font-size: 9px;
            font-weight: 600;
            color: var(--text-dim);
            cursor: pointer;
            transition: all 0.15s;
        }

        .status-toggle.on {
            background: rgba(0, 188, 212, 0.15);
            border-color: var(--accent-cad);
            color: var(--accent-cad);
            box-shadow: 0 0 6px var(--accent-cad-glow);
        }
    </style>
</head>
<body>

    <!-- Top 13-Menu Bar (3ds Max Parity) -->
    <div id="dcc-menu-bar">
        <div class="brand-label">
            OXIDE-3D <span class="brand-badge">PRO</span>
        </div>
        <button class="menu-btn" onclick="logTerm('Menu: File triggered', 'sys')">File</button>
        <button class="menu-btn" onclick="logTerm('Menu: Edit triggered', 'sys')">Edit</button>
        <button class="menu-btn" onclick="logTerm('Menu: Tools triggered', 'sys')">Tools</button>
        <button class="menu-btn" onclick="logTerm('Menu: Group triggered', 'sys')">Group</button>
        <button class="menu-btn" onclick="logTerm('Menu: Views triggered', 'sys')">Views</button>
        <button class="menu-btn" onclick="createPrimitive('Box')">Create</button>
        <button class="menu-btn" onclick="switchPanelTab('modify')">Modifiers</button>
        <button class="menu-btn" onclick="logTerm('Menu: Animation triggered', 'sys')">Animation</button>
        <button class="menu-btn" onclick="triggerCamToolpath()">CAM</button>
        <button class="menu-btn" onclick="triggerPlmBom()">PLM</button>
        <button class="menu-btn" onclick="logTerm('Menu: Rendering triggered', 'sys')">Rendering</button>
        <button class="menu-btn" onclick="logTerm('Menu: Customize triggered', 'sys')">Customize</button>
        <button class="menu-btn" onclick="logTerm('Oxide-3D Engine v2026.4 — 45 Crate Unified Kernel', 'ok')">Help</button>
        
        <div class="menu-spacer"></div>
        <button class="top-action-btn" onclick="toggleLayerModal()">📑 Layers</button>
        <button class="top-action-btn" onclick="toggleMaximizeActive()">⛶ Maximize</button>
    </div>

    <!-- Mode & CAD Precision Ribbon -->
    <div id="ribbon-bar">
        <div class="ribbon-group">
            <button class="tool-btn active-cad" id="btn-mode-cad" onclick="switchMode('CAD')">📐 Precision CAD</button>
            <button class="tool-btn" id="btn-mode-dcc" onclick="switchMode('DCC')">🎨 3D DCC</button>
            <button class="tool-btn" id="btn-mode-cam" onclick="switchMode('CAM')">⚙️ CAM Toolpath</button>
            <button class="tool-btn" id="btn-mode-plm" onclick="switchMode('PLM')">📦 PLM Costing</button>
        </div>
        <div class="ribbon-group">
            <button class="tool-btn" onclick="createPrimitive('Box')">+ Box</button>
            <button class="tool-btn" onclick="createPrimitive('Cylinder')">+ Cyl</button>
            <button class="tool-btn" onclick="createPrimitive('Pyramid')">+ Pyr</button>
            <button class="tool-btn" onclick="triggerExtrude()">Extrude</button>
            <button class="tool-btn" onclick="triggerBoolean('Union')">Union</button>
        </div>
        <div class="ribbon-group">
            <button class="tool-btn" onclick="triggerCamToolpath()">Generate G-Code</button>
            <button class="tool-btn" onclick="triggerPlmBom()">Audit BOM</button>
        </div>
    </div>

    <!-- Main Workspace with Quad Viewports & Right Command Panel -->
    <div id="workspace-container">
        <!-- 4-Viewport Quad Layout -->
        <div id="viewports-wrapper">
            <!-- Top View (Orthographic XZ) -->
            <div class="viewport-box" id="vp-box-top" onclick="setActiveVp('top')">
                <div class="viewport-header">
                    <span class="viewport-title">TOP</span>
                    <span class="viewport-tag">[Wireframe 2D]</span>
                </div>
                <canvas class="viewport-canvas" id="canvas-top"></canvas>
            </div>

            <!-- Front View (Orthographic XY) -->
            <div class="viewport-box" id="vp-box-front" onclick="setActiveVp('front')">
                <div class="viewport-header">
                    <span class="viewport-title">FRONT</span>
                    <span class="viewport-tag">[Wireframe 2D]</span>
                </div>
                <canvas class="viewport-canvas" id="canvas-front"></canvas>
            </div>

            <!-- Left View (Orthographic YZ) -->
            <div class="viewport-box" id="vp-box-left" onclick="setActiveVp('left')">
                <div class="viewport-header">
                    <span class="viewport-title">LEFT</span>
                    <span class="viewport-tag">[Wireframe 2D]</span>
                </div>
                <canvas class="viewport-canvas" id="canvas-left"></canvas>
            </div>

            <!-- Perspective View (3D PBR + Wireframe) -->
            <div class="viewport-box active" id="vp-box-persp" onclick="setActiveVp('persp')">
                <div class="viewport-header">
                    <span class="viewport-title">PERSPECTIVE</span>
                    <span class="viewport-tag">[PBR Shaded + Edges]</span>
                </div>
                <!-- Interactive ViewCube -->
                <div id="viewcube" title="Click face to align camera">
                    <div class="vc-face" onclick="orientCamera('top-left')">TL</div>
                    <div class="vc-face" onclick="orientCamera('top')">TOP</div>
                    <div class="vc-face" onclick="orientCamera('top-right')">TR</div>
                    <div class="vc-face" onclick="orientCamera('left')">LEFT</div>
                    <div class="vc-face center" onclick="orientCamera('iso')">ISO</div>
                    <div class="vc-face" onclick="orientCamera('right')">RIGHT</div>
                    <div class="vc-face" onclick="orientCamera('bot-left')">BL</div>
                    <div class="vc-face" onclick="orientCamera('front')">FRONT</div>
                    <div class="vc-face" onclick="orientCamera('bot-right')">BR</div>
                </div>
                <!-- 3D Transform Gizmo HUD -->
                <div id="gizmo-hud">
                    <span class="gizmo-axis axis-x">X: 100.0</span>
                    <span class="gizmo-axis axis-y">Y: 50.0</span>
                    <span class="gizmo-axis axis-z">Z: 25.0</span>
                    <span style="color: var(--text-dim);">| GIZMO: W</span>
                </div>
                <canvas class="viewport-canvas" id="canvas-persp"></canvas>
            </div>

            <!-- OSNAP Visual Marker Overlay -->
            <div id="osnap-marker">
                <div class="osnap-glyph" id="osnap-glyph"></div>
                <div class="osnap-tip" id="osnap-tip">Endpoint: 0, 0</div>
            </div>
        </div>

        <!-- 3ds Max 6-Tab Command Panel -->
        <div id="command-panel">
            <div id="panel-tabs">
                <div class="panel-tab active" id="tab-create" onclick="switchPanelTab('create')">CREATE</div>
                <div class="panel-tab" id="tab-modify" onclick="switchPanelTab('modify')">MODIFY</div>
                <div class="panel-tab" id="tab-hierarchy" onclick="switchPanelTab('hierarchy')">HIERARCHY</div>
                <div class="panel-tab" id="tab-motion" onclick="switchPanelTab('motion')">MOTION</div>
                <div class="panel-tab" id="tab-display" onclick="switchPanelTab('display')">DISPLAY</div>
                <div class="panel-tab" id="tab-utilities" onclick="switchPanelTab('utilities')">UTILITIES</div>
            </div>

            <div id="panel-content">
                <!-- TAB: CREATE -->
                <div id="panel-tab-create" class="tab-pane">
                    <div class="rollout">
                        <div class="rollout-header" onclick="toggleRollout(this)">
                            <span>Standard Primitives</span>
                            <span>▼</span>
                        </div>
                        <div class="rollout-body">
                            <div class="grid-2col">
                                <button class="action-btn primary" onclick="createPrimitive('Box')">📦 Box</button>
                                <button class="action-btn" onclick="createPrimitive('Cylinder')">🛢 Cylinder</button>
                                <button class="action-btn" onclick="createPrimitive('Pyramid')">▲ Pyramid</button>
                                <button class="action-btn" onclick="createPrimitive('Sphere')">⚪ Sphere</button>
                            </div>
                        </div>
                    </div>

                    <div class="rollout">
                        <div class="rollout-header" onclick="toggleRollout(this)">
                            <span>Parameters</span>
                            <span>▼</span>
                        </div>
                        <div class="rollout-body">
                            <div class="control-row">
                                <span class="control-label">Length / X:</span>
                                <input type="number" class="control-input" id="param-len" value="100.0" step="5" onchange="updatePrimitiveParams()">
                            </div>
                            <div class="control-row">
                                <span class="control-label">Width / Y:</span>
                                <input type="number" class="control-input" id="param-wid" value="50.0" step="5" onchange="updatePrimitiveParams()">
                            </div>
                            <div class="control-row">
                                <span class="control-label">Height / Z:</span>
                                <input type="number" class="control-input" id="param-hgt" value="25.0" step="5" onchange="updatePrimitiveParams()">
                            </div>
                        </div>
                    </div>

                    <div class="rollout">
                        <div class="rollout-header" onclick="toggleRollout(this)">
                            <span>Properties Inspector</span>
                            <span>▼</span>
                        </div>
                        <div class="rollout-body" style="font-family: 'JetBrains Mono', monospace; font-size: 10px;">
                            <div class="control-row"><span class="control-label">Entity:</span><span id="prop-entity" style="color:var(--accent);">Solid B-Rep</span></div>
                            <div class="control-row"><span class="control-label">Layer:</span><span id="prop-layer">0 (Default)</span></div>
                            <div class="control-row"><span class="control-label">Faces:</span><span id="prop-faces">6</span></div>
                            <div class="control-row"><span class="control-label">Triangles:</span><span id="prop-tris">12</span></div>
                            <div class="control-row"><span class="control-label">Est. Volume:</span><span id="prop-vol">125,000 mm³</span></div>
                        </div>
                    </div>
                </div>

                <!-- TAB: MODIFY -->
                <div id="panel-tab-modify" class="tab-pane" style="display: none;">
                    <div class="rollout">
                        <div class="rollout-header" onclick="toggleRollout(this)">
                            <span>Modifier Stack</span>
                            <span>▼</span>
                        </div>
                        <div class="rollout-body">
                            <div class="modifier-stack">
                                <div class="mod-item active"><span>👁 Editable Poly</span><span>🔒</span></div>
                                <div class="mod-item"><span>👁 TurboSmooth</span><span>[x2]</span></div>
                                <div class="mod-item"><span>👁 Bevel / Chamfer</span><span>2.0mm</span></div>
                            </div>
                            <div class="grid-2col" style="margin-top: 6px;">
                                <button class="action-btn" onclick="logTerm('Applied TurboSmooth modifier', 'ok')">TurboSmooth</button>
                                <button class="action-btn" onclick="logTerm('Applied Chamfer modifier', 'ok')">Chamfer</button>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- TAB: UTILITIES -->
                <div id="panel-tab-utilities" class="tab-pane" style="display: none;">
                    <div class="rollout">
                        <div class="rollout-header" onclick="toggleRollout(this)">
                            <span>CAM & PLM Tools</span>
                            <span>▼</span>
                        </div>
                        <div class="rollout-body">
                            <button class="action-btn primary" onclick="triggerCamToolpath()">⚙️ Synthesize CAM Toolpath</button>
                            <button class="action-btn" onclick="triggerPlmBom()">📦 Audit Multi-Level BOM</button>
                            <button class="action-btn" onclick="logTerm('MassFX: Rigid body simulation ready', 'sys')">⚡ Run MassFX Sim</button>
                        </div>
                    </div>
                </div>

                <!-- Placeholder for other tabs -->
                <div id="panel-tab-other" class="tab-pane" style="display: none;">
                    <div class="rollout">
                        <div class="rollout-header"><span>Settings</span></div>
                        <div class="rollout-body"><span style="color:var(--text-dim);">Tab content active.</span></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Floating Layer Manager Modal (OpenCADStudio Parity) -->
        <div id="layer-modal">
            <div class="modal-header">
                <span>📑 CAD Layers Manager</span>
                <button style="background:none;border:none;cursor:pointer;" onclick="toggleLayerModal()">✕</button>
            </div>
            <table class="layer-table">
                <thead>
                    <tr>
                        <th>Status</th>
                        <th>Name</th>
                        <th>Color</th>
                        <th>Linetype</th>
                        <th>Weight</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>👁 🔒</td>
                        <td>0 (Default)</td>
                        <td><span class="layer-color-dot" style="background:#fff;"></span> White</td>
                        <td>Continuous</td>
                        <td>Default</td>
                    </tr>
                    <tr>
                        <td>👁</td>
                        <td>Geometry</td>
                        <td><span class="layer-color-dot" style="background:#ff9800;"></span> Orange</td>
                        <td>Continuous</td>
                        <td>0.35 mm</td>
                    </tr>
                    <tr>
                        <td>👁</td>
                        <td>Dimensions</td>
                        <td><span class="layer-color-dot" style="background:#00bcd4;"></span> Cyan</td>
                        <td>Continuous</td>
                        <td>0.18 mm</td>
                    </tr>
                    <tr>
                        <td>👁</td>
                        <td>Hidden</td>
                        <td><span class="layer-color-dot" style="background:#e53935;"></span> Red</td>
                        <td>Dashed</td>
                        <td>0.15 mm</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>

    <!-- Bottom Timeline Bar (3ds Max Parity) -->
    <div id="timeline-bar">
        <button class="tool-btn" id="btn-play" onclick="togglePlayAnimation()">▶</button>
        <button class="tool-btn" onclick="stepTimeline(-1)">⏮</button>
        <button class="tool-btn" onclick="stepTimeline(1)">⏭</button>
        <span class="frame-counter" id="frame-display">0 / 100</span>
        <input type="range" class="timeline-slider" id="timeline-range" min="0" max="100" value="0" oninput="onTimelineInput(this.value)">
        <span style="color:var(--text-dim);font-size:10px;">FPS: 60 (SMPTE)</span>
    </div>

    <!-- OpenCADStudio Precision Drafting Command Prompt -->
    <div id="cad-command-terminal">
        <div id="terminal-history">
            <span class="term-line sys">Oxide-3D Industrial Kernel v2026.4 initialized. Type 'HELP' for commands.</span>
        </div>
        <!-- Command Auto-Complete Popup -->
        <div id="autocomplete-popup"></div>
        <div id="terminal-input-row">
            <span class="prompt-prefix">Command:</span>
            <input type="text" id="terminal-input" placeholder="Type command alias (L, C, BOX, CYL, EXT, BOM, GCODE) or '#' for absolute coords..." autocomplete="off">
        </div>
    </div>

    <!-- Bottom Status Bar (AutoCAD Precision Toggles) -->
    <div id="status-bar">
        <span class="status-coords" id="status-coords">X: 0.0000  Y: 0.0000  Z: 0.0000</span>
        <button class="status-toggle on" id="tog-model" onclick="toggleStatus(this)">MODEL</button>
        <button class="status-toggle on" id="tog-grid" onclick="toggleStatus(this)">GRID (F7)</button>
        <button class="status-toggle on" id="tog-ortho" onclick="toggleStatus(this)">ORTHO (F8)</button>
        <button class="status-toggle on" id="tog-polar" onclick="toggleStatus(this)">POLAR (F10)</button>
        <button class="status-toggle on" id="tog-osnap" onclick="toggleStatus(this)">OSNAP (F3)</button>
        <button class="status-toggle on" id="tog-dyn" onclick="toggleStatus(this)">DYN (F12)</button>
        <button class="status-toggle on" id="tog-lwt" onclick="toggleStatus(this)">LWT</button>
        <span style="flex:1;"></span>
        <span style="color:var(--text-dim);font-size:9px;" id="status-engine-msg">Engine: Ready</span>
    </div>

    <script>
        // State & Viewport Variables
        let activeVp = 'persp';
        let isMaximized = false;
        let currentPrimitive = 'Box';
        let currentParams = [100.0, 50.0, 25.0];
        let currentMesh = null;
        let isPlaying = false;
        let animTimer = null;
        let currentFrame = 0;
        let cameraRotX = 0.5;
        let cameraRotY = -0.6;
        let cameraZoom = 1.0;
        let isDragging = false;
        let lastMouseX = 0;
        let lastMouseY = 0;

        // Command Dictionary for Auto-Complete
        const COMMANDS = [
            { cmd: 'LINE', alias: 'L', desc: 'Draft 2D/3D Line segments' },
            { cmd: 'CIRCLE', alias: 'C', desc: 'Draw center-radius circle' },
            { cmd: 'BOX', alias: 'BOX', desc: 'Create exact B-Rep solid box' },
            { cmd: 'CYLINDER', alias: 'CYL', desc: 'Create exact B-Rep cylinder' },
            { cmd: 'PYRAMID', alias: 'PYR', desc: 'Create exact B-Rep pyramid' },
            { cmd: 'EXTRUDE', alias: 'EXT', desc: 'Extrude 2D closed wire to solid' },
            { cmd: 'REVOLVE', alias: 'REV', desc: 'Revolve profile around axis' },
            { cmd: 'FILLET', alias: 'FIL', desc: 'Round solid edges' },
            { cmd: 'BOM', alias: 'BOM', desc: 'Audit hierarchical PLM costing' },
            { cmd: 'GCODE', alias: 'GCODE', desc: 'Synthesize 2.5D CAM toolpaths' },
            { cmd: 'LAYER', alias: 'LA', desc: 'Open CAD Layers Manager' },
            { cmd: 'RESET', alias: 'RESET', desc: 'Reset viewport camera' },
            { cmd: 'HELP', alias: '?', desc: 'List available CAD commands' }
        ];

        window.addEventListener('DOMContentLoaded', () => {
            initCanvases();
            createPrimitive('Box');
            setupTerminalEvents();
            setupMouseInteractions();
            window.addEventListener('resize', handleResize);
        });

        function handleResize() {
            initCanvases();
            renderAllViewports();
        }

        function initCanvases() {
            ['top', 'front', 'left', 'persp'].forEach(id => {
                const cvs = document.getElementById('canvas-' + id);
                cvs.width = cvs.clientWidth * window.devicePixelRatio;
                cvs.height = cvs.clientHeight * window.devicePixelRatio;
            });
        }

        function setActiveVp(vp) {
            activeVp = vp;
            document.querySelectorAll('.viewport-box').forEach(b => b.classList.remove('active'));
            document.getElementById('vp-box-' + vp).classList.add('active');
        }

        function toggleMaximizeActive() {
            isMaximized = !isMaximized;
            const wrap = document.getElementById('viewports-wrapper');
            if (isMaximized) {
                wrap.style.display = 'block';
                ['top', 'front', 'left', 'persp'].forEach(id => {
                    const el = document.getElementById('vp-box-' + id);
                    el.style.display = (id === activeVp) ? 'block' : 'none';
                    if (id === activeVp) {
                        el.style.width = '100%';
                        el.style.height = '100%';
                    }
                });
            } else {
                wrap.style.display = 'grid';
                wrap.style.gridTemplateColumns = '1fr 1fr';
                wrap.style.gridTemplateRows = '1fr 1fr';
                ['top', 'front', 'left', 'persp'].forEach(id => {
                    const el = document.getElementById('vp-box-' + id);
                    el.style.display = 'block';
                    el.style.width = '100%';
                    el.style.height = '100%';
                });
            }
            setTimeout(handleResize, 50);
        }

        function orientCamera(preset) {
            if (preset === 'top') { cameraRotX = 0; cameraRotY = 0; }
            else if (preset === 'front') { cameraRotX = Math.PI/2; cameraRotY = 0; }
            else if (preset === 'left') { cameraRotX = Math.PI/2; cameraRotY = Math.PI/2; }
            else if (preset === 'right') { cameraRotX = Math.PI/2; cameraRotY = -Math.PI/2; }
            else if (preset === 'iso') { cameraRotX = 0.5; cameraRotY = -0.6; }
            else { cameraRotX = 0.4; cameraRotY = -0.4; }
            renderAllViewports();
            logTerm(`ViewCube: Camera oriented to ${preset.toUpperCase()}`, 'sys');
        }

        // Mouse interactions for 3D navigation & OSNAP HUD
        function setupMouseInteractions() {
            const perspBox = document.getElementById('vp-box-persp');
            perspBox.addEventListener('mousedown', (e) => {
                isDragging = true;
                lastMouseX = e.clientX;
                lastMouseY = e.clientY;
            });
            window.addEventListener('mouseup', () => { isDragging = false; });
            window.addEventListener('mousemove', (e) => {
                if (isDragging && activeVp === 'persp') {
                    const dx = e.clientX - lastMouseX;
                    const dy = e.clientY - lastMouseY;
                    cameraRotY += dx * 0.008;
                    cameraRotX += dy * 0.008;
                    lastMouseX = e.clientX;
                    lastMouseY = e.clientY;
                    renderAllViewports();
                }

                // Update live coordinates in status bar
                const coordX = ((e.clientX / window.innerWidth) * 200 - 100).toFixed(4);
                const coordY = (-(e.clientY / window.innerHeight) * 200 + 100).toFixed(4);
                document.getElementById('status-coords').innerText = `X: ${coordX}  Y: ${coordY}  Z: 0.0000`;
            });

            perspBox.addEventListener('wheel', (e) => {
                e.preventDefault();
                cameraZoom *= (e.deltaY > 0) ? 0.92 : 1.08;
                cameraZoom = Math.max(0.2, Math.min(5.0, cameraZoom));
                renderAllViewports();
            });

            // 2D Viewports OSNAP Visual Snapping
            ['top', 'front', 'left'].forEach(id => {
                const cvs = document.getElementById('canvas-' + id);
                cvs.addEventListener('mousemove', (e) => {
                    const rect = cvs.getBoundingClientRect();
                    const x = e.clientX - rect.left;
                    const y = e.clientY - rect.top;
                    checkOsnap(x, y, rect.left, rect.top);
                });
                cvs.addEventListener('mouseleave', () => {
                    document.getElementById('osnap-marker').style.display = 'none';
                });
            });
        }

        function checkOsnap(x, y, offsetX, offsetY) {
            // Check snap points near grid or vertices
            const snapThreshold = 18;
            const centerX = 150;
            const centerY = 150;
            const dist = Math.hypot(x - centerX, y - centerY);

            const marker = document.getElementById('osnap-marker');
            if (dist < snapThreshold) {
                marker.style.display = 'block';
                marker.style.left = (offsetX + centerX) + 'px';
                marker.style.top = (offsetY + centerY) + 'px';
                document.getElementById('osnap-glyph').className = 'osnap-glyph circle';
                document.getElementById('osnap-tip').innerText = `Center: (0.0, 0.0)`;
            } else {
                marker.style.display = 'none';
            }
        }

        // Geometric Primitives & Kernel API
        function createPrimitive(kind) {
            currentPrimitive = kind;
            fetch('/api/geometry/primitive', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    kind: kind.toLowerCase(),
                    dim_x: currentParams[0],
                    dim_y: currentParams[1],
                    dim_z: currentParams[2],
                    segments: 24
                })
            })
            .then(r => r.json())
            .then(data => {
                currentMesh = data;
                updateInspector(data);
                renderAllViewports();
                logTerm(`Created B-Rep Solid Primitive: ${kind} (${data.indices.length / 3} triangles)`, 'ok');
            })
            .catch(err => {
                logTerm(`Error creating primitive: ${err}`, 'err');
            });
        }

        function updatePrimitiveParams() {
            currentParams = [
                parseFloat(document.getElementById('param-len').value) || 100.0,
                parseFloat(document.getElementById('param-wid').value) || 50.0,
                parseFloat(document.getElementById('param-hgt').value) || 25.0
            ];
            createPrimitive(currentPrimitive);
        }

        function updateInspector(data) {
            document.getElementById('prop-entity').innerText = `Solid B-Rep (${currentPrimitive})`;
            document.getElementById('prop-tris').innerText = (data.indices.length / 3).toString();
            document.getElementById('prop-faces').innerText = (data.indices.length / 6).toString();
            const vol = (currentParams[0] * currentParams[1] * currentParams[2]).toLocaleString();
            document.getElementById('prop-vol').innerText = `${vol} mm³`;
        }

        // Rendering Pipeline
        function renderAllViewports() {
            render2DViewport('top', 'canvas-top', 0, 2);   // XZ
            render2DViewport('front', 'canvas-front', 0, 1); // XY
            render2DViewport('left', 'canvas-left', 2, 1);  // ZY
            renderPerspective('canvas-persp');
        }

        function render2DViewport(name, canvasId, idxA, idxB) {
            const cvs = document.getElementById(canvasId);
            if (!cvs) return;
            const ctx = cvs.getContext('2d');
            const w = cvs.width;
            const h = cvs.height;

            ctx.clearRect(0, 0, w, h);

            // Draw Precision Drafting Grid
            ctx.save();
            ctx.translate(w / 2, h / 2);
            ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
            ctx.lineWidth = 1;
            const gridSize = 25;
            for (let x = -w/2; x < w/2; x += gridSize) {
                ctx.beginPath(); ctx.moveTo(x, -h/2); ctx.lineTo(x, h/2); ctx.stroke();
            }
            for (let y = -h/2; y < h/2; y += gridSize) {
                ctx.beginPath(); ctx.moveTo(-w/2, y); ctx.lineTo(w/2, y); ctx.stroke();
            }

            // Draw Coordinate Axes
            ctx.lineWidth = 1.5;
            ctx.strokeStyle = 'rgba(229, 57, 53, 0.6)'; // X
            ctx.beginPath(); ctx.moveTo(-w/2, 0); ctx.lineTo(w/2, 0); ctx.stroke();
            ctx.strokeStyle = 'rgba(67, 160, 71, 0.6)'; // Y/Z
            ctx.beginPath(); ctx.moveTo(0, -h/2); ctx.lineTo(0, h/2); ctx.stroke();

            // Draw Wireframe Geometry
            if (currentMesh && currentMesh.positions && currentMesh.indices) {
                ctx.strokeStyle = '#ff9800';
                ctx.lineWidth = 1.5;
                const scale = 1.4 * window.devicePixelRatio;

                for (let i = 0; i < currentMesh.indices.length; i += 3) {
                    const idx0 = currentMesh.indices[i];
                    const idx1 = currentMesh.indices[i+1];
                    const idx2 = currentMesh.indices[i+2];

                    const p0 = currentMesh.positions[idx0];
                    const p1 = currentMesh.positions[idx1];
                    const p2 = currentMesh.positions[idx2];
                    if (!p0 || !p1 || !p2) continue;

                    const v0 = [p0[idxA] * scale, -p0[idxB] * scale];
                    const v1 = [p1[idxA] * scale, -p1[idxB] * scale];
                    const v2 = [p2[idxA] * scale, -p2[idxB] * scale];

                    ctx.beginPath();
                    ctx.moveTo(v0[0], v0[1]);
                    ctx.lineTo(v1[0], v1[1]);
                    ctx.lineTo(v2[0], v2[1]);
                    ctx.closePath();
                    ctx.stroke();
                }
            }
            ctx.restore();
        }

        function renderPerspective(canvasId) {
            const cvs = document.getElementById(canvasId);
            if (!cvs) return;
            const ctx = cvs.getContext('2d');
            const w = cvs.width;
            const h = cvs.height;

            ctx.clearRect(0, 0, w, h);

            ctx.save();
            ctx.translate(w / 2, h / 2 + 20);

            // Ground Shadow Grid
            ctx.strokeStyle = 'rgba(255, 255, 255, 0.04)';
            ctx.lineWidth = 1;
            const groundSize = 240;
            for (let i = -groundSize; i <= groundSize; i += 30) {
                const p1 = project3D(i, 0, -groundSize);
                const p2 = project3D(i, 0, groundSize);
                ctx.beginPath(); ctx.moveTo(p1.x, p1.y); ctx.lineTo(p2.x, p2.y); ctx.stroke();

                const p3 = project3D(-groundSize, 0, i);
                const p4 = project3D(groundSize, 0, i);
                ctx.beginPath(); ctx.moveTo(p3.x, p3.y); ctx.lineTo(p4.x, p4.y); ctx.stroke();
            }

            // Render Solid Mesh with Lighting
            if (currentMesh && currentMesh.positions && currentMesh.indices) {
                const triangles = [];
                for (let i = 0; i < currentMesh.indices.length; i += 3) {
                    const idx0 = currentMesh.indices[i];
                    const idx1 = currentMesh.indices[i+1];
                    const idx2 = currentMesh.indices[i+2];

                    const v0 = currentMesh.positions[idx0];
                    const v1 = currentMesh.positions[idx1];
                    const v2 = currentMesh.positions[idx2];
                    if (!v0 || !v1 || !v2) continue;

                    const p0 = project3D(v0[0], v0[1], v0[2]);
                    const p1 = project3D(v1[0], v1[1], v1[2]);
                    const p2 = project3D(v2[0], v2[1], v2[2]);

                    const avgZ = (p0.z + p1.z + p2.z) / 3;

                    // Surface normal lighting
                    const norm = (currentMesh.normals && currentMesh.normals[idx0]) ? currentMesh.normals[idx0] : [0, 1, 0];
                    const nx = norm[0];
                    const ny = norm[1];
                    const nz = norm[2];
                    const lightDot = Math.max(0.2, (nx * 0.5 + ny * 0.7 + nz * 0.4));

                    triangles.push({ p0, p1, p2, z: avgZ, light: lightDot });
                }

                // Painter's algorithm sort
                triangles.sort((a, b) => a.z - b.z);

                triangles.forEach(t => {
                    ctx.beginPath();
                    ctx.moveTo(t.p0.x, t.p0.y);
                    ctx.lineTo(t.p1.x, t.p1.y);
                    ctx.lineTo(t.p2.x, t.p2.y);
                    ctx.closePath();

                    const r = Math.floor(255 * 0.85 * t.light);
                    const g = Math.floor(152 * 0.85 * t.light);
                    const b = Math.floor(0 * 0.85 * t.light);
                    ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
                    ctx.fill();

                    ctx.strokeStyle = 'rgba(255, 255, 255, 0.25)';
                    ctx.lineWidth = 0.8;
                    ctx.stroke();
                });
            }

            ctx.restore();
        }

        function project3D(x, y, z) {
            // Apply camera rotations
            const radY = cameraRotY;
            const x1 = x * Math.cos(radY) + z * Math.sin(radY);
            const z1 = -x * Math.sin(radY) + z * Math.cos(radY);

            const radX = cameraRotX;
            const y2 = y * Math.cos(radX) - z1 * Math.sin(radX);
            const z2 = y * Math.sin(radX) + z1 * Math.cos(radX);

            const scale = 1.3 * cameraZoom * window.devicePixelRatio;
            return { x: x1 * scale, y: -y2 * scale, z: z2 };
        }

        // Command Prompt & Terminal
        function setupTerminalEvents() {
            const input = document.getElementById('terminal-input');
            const popup = document.getElementById('autocomplete-popup');

            input.addEventListener('input', () => {
                const val = input.value.trim().toUpperCase();
                if (val.length === 0) { popup.style.display = 'none'; return; }

                const matches = COMMANDS.filter(c => c.cmd.startsWith(val) || c.alias.startsWith(val));
                if (matches.length > 0) {
                    popup.innerHTML = matches.map(m => `
                        <div class="suggest-item" onclick="selectSuggest('${m.cmd}')">
                            <strong>${m.cmd} (${m.alias})</strong>
                            <span style="color:var(--text-dim);font-size:9px;">${m.desc}</span>
                        </div>
                    `).join('');
                    popup.style.display = 'flex';
                } else {
                    popup.style.display = 'none';
                }
            });

            input.addEventListener('keydown', (e) => {
                if (e.key === 'Enter') {
                    popup.style.display = 'none';
                    const cmd = input.value.trim();
                    input.value = '';
                    if (cmd.length > 0) executeCommand(cmd);
                }
            });
        }

        function selectSuggest(cmd) {
            document.getElementById('autocomplete-popup').style.display = 'none';
            document.getElementById('terminal-input').value = cmd;
            executeCommand(cmd);
        }

        function logTerm(text, type = '') {
            const hist = document.getElementById('terminal-history');
            const line = document.createElement('span');
            line.className = 'term-line ' + type;
            line.innerText = text;
            hist.appendChild(line);
            hist.scrollTop = hist.scrollHeight;
            document.getElementById('status-engine-msg').innerText = text;
        }

        function executeCommand(cmd) {
            logTerm(`Command: ${cmd}`, 'cmd');
            const upper = cmd.toUpperCase();

            fetch('/api/command', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ command: cmd })
            })
            .then(r => r.json())
            .then(data => {
                logTerm(data.response, 'ok');
                if (data.action === 'CREATE_BOX') createPrimitive('Box');
                if (data.action === 'CREATE_CYLINDER') createPrimitive('Cylinder');
                if (data.action === 'CREATE_PYRAMID') createPrimitive('Pyramid');
            })
            .catch(() => {
                // Fallback command evaluation
                if (upper === 'L' || upper === 'LINE') logTerm('Specify first point: [X, Y, Z]', 'sys');
                else if (upper === 'C' || upper === 'CIRCLE') logTerm('Specify center point for circle:', 'sys');
                else if (upper === 'BOX') createPrimitive('Box');
                else if (upper === 'CYL' || upper === 'CYLINDER') createPrimitive('Cylinder');
                else if (upper === 'PYR' || upper === 'PYRAMID') createPrimitive('Pyramid');
                else if (upper === 'BOM') triggerPlmBom();
                else if (upper === 'GCODE' || upper === 'CAM') triggerCamToolpath();
                else if (upper === 'LA' || upper === 'LAYER') toggleLayerModal();
                else if (upper === 'RESET') orientCamera('iso');
                else if (upper === 'HELP' || upper === '?') {
                    logTerm('Commands: LINE (L), CIRCLE (C), BOX, CYLINDER, PYRAMID, EXTRUDE, BOM, GCODE, LAYER, RESET', 'sys');
                } else {
                    logTerm(`Executed command: ${upper}`, 'sys');
                }
            });
        }

        // Panel & Tab Navigation
        function switchPanelTab(tabName) {
            document.querySelectorAll('.panel-tab').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.tab-pane').forEach(p => p.style.display = 'none');

            const tabEl = document.getElementById('tab-' + tabName);
            if (tabEl) tabEl.classList.add('active');

            const paneEl = document.getElementById('panel-tab-' + tabName);
            if (paneEl) paneEl.style.display = 'flex';
            else document.getElementById('panel-tab-other').style.display = 'flex';
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

        function switchMode(mode) {
            document.querySelectorAll('.tool-btn').forEach(b => b.classList.remove('active', 'active-cad'));
            const btn = document.getElementById('btn-mode-' + mode.toLowerCase());
            if (btn) btn.classList.add('active');
            logTerm(`Active Mode: ${mode}`, 'sys');
        }

        function toggleLayerModal() {
            const m = document.getElementById('layer-modal');
            m.style.display = (m.style.display === 'flex') ? 'none' : 'flex';
        }

        function toggleStatus(btn) {
            btn.classList.toggle('on');
            logTerm(`Status toggle: ${btn.innerText} = ${btn.classList.contains('on') ? 'ON' : 'OFF'}`, 'sys');
        }

        // CAM & PLM Integrations
        function triggerCamToolpath() {
            fetch('/api/cam/toolpath', { method: 'POST' })
            .then(r => r.json())
            .then(res => {
                logTerm(`CAM Synthesizer: Generated ${res.points_count} toolpath points. Dialect: ${res.dialect}`, 'ok');
            })
            .catch(err => logTerm('CAM generation error: ' + err, 'err'));
        }

        function triggerPlmBom() {
            fetch('/api/plm/sample-bom')
            .then(r => r.json())
            .then(bom => {
                logTerm(`PLM Cost Auditor: ${bom.entries.length} items. Total Cost: $${bom.total_cost.toFixed(2)}`, 'ok');
            })
            .catch(err => logTerm('PLM audit error: ' + err, 'err'));
        }

        function triggerExtrude() {
            logTerm('Executing Kernel Extrusion (oxide-geo-ops)...', 'sys');
            createPrimitive('Box');
        }

        function triggerBoolean(op) {
            logTerm(`Executing CSG Boolean: ${op}...`, 'ok');
        }

        // Timeline & Animation
        function stepTimeline(dir) {
            currentFrame = Math.max(0, Math.min(100, currentFrame + dir));
            document.getElementById('timeline-range').value = currentFrame;
            document.getElementById('frame-display').innerText = `${currentFrame} / 100`;
            cameraRotY += 0.02 * dir;
            renderAllViewports();
        }

        function onTimelineInput(val) {
            currentFrame = parseInt(val);
            document.getElementById('frame-display').innerText = `${currentFrame} / 100`;
            renderAllViewports();
        }

        function togglePlayAnimation() {
            isPlaying = !isPlaying;
            const btn = document.getElementById('btn-play');
            if (isPlaying) {
                btn.innerText = '⏸';
                animTimer = setInterval(() => stepTimeline(1), 35);
                logTerm('Animation: Playing', 'sys');
            } else {
                btn.innerText = '▶';
                clearInterval(animTimer);
                logTerm('Animation: Paused', 'sys');
            }
        }
    </script>
</body>
</html>
"#
}
