---
description: 'Top-level OKF summary: 36 concepts across 1 domains and 3 modules'
git_branch: master
git_repo: Oxide-3d
okf_version: '0.2'
timestamp: '2026-09-27T20:45:20Z'
title: oxide-server — Knowledge Summary
type: Index
---

# oxide-server — Knowledge Summary

> OKF v0.2 bundle | 36 concepts | 1 domains | 3 modules

## Stats

| Type | Count |
|------|-------|
| Dependency | 16 |
| Function | 9 |
| Class | 8 |
| Module | 3 |

| Language | Concepts |
|----------|----------|
| rust | 20 |
| manifest | 16 |

## Domain Map

Use these links to navigate the bundle or prime an AI agent with focused context.

### [src](src/index.md) — 20 concepts

- [src/handlers](src/handlers/index.md) (16 concepts) — REST API Handlers for Oxide-3D Web Platform.
- [src/web_ui](src/web_ui/index.md) (2 concepts) — Production Web Client for Oxide-3D CAD/DCC Platform.
- [src/main](src/main/index.md) (2 concepts) — Oxide-3D Web & Collaboration Server.

## Dependencies

> Full list at [`_dependencies/index.md`](/_dependencies/index.md) or `okf lookup --type Dependency`

| Ecosystem | Packages |
|----------|----------|
| cargo | 16 |

## Key Concepts

Highest-value concepts across all domains (Classes and Functions with rich descriptions).

| Concept | Type | Module | Description |
|---------|------|--------|-------------|
| [get_web_app_html](/src/web_ui/get_web_app_html.md) | Function | `src/web_ui.rs` | Returns the complete HTML/JS/CSS client application string. |
| [execute_command_handler](/src/handlers/execute_command_handler.md) | Function | `src/handlers.rs` | CAD Command execution interpreter handler. |

## Usage with OpenCode

```bash
# Prime full context
RUN cat ./okf_bundle/SUMMARY.md

# Prime specific domain
RUN cat ./okf_bundle/src/index.md

# Find a concept
RUN find ./okf_bundle -name '<ConceptName>.md' | xargs cat
```
