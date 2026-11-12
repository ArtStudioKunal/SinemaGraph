# SinemaGraph upgrades: UI/UX and AI Studio

Status: **Design and implementation roadmap** (not yet implemented).
Base: `rebrand/sinemagraph` in the existing `ArtStudioKunal/filmcraft` fork.
Preserve existing project files, manual editing workflows, native engine commands, undo/redo, and upstream notices.

## Product goals

### 1. Workspace and docking
- Keep docking controls adjacent to each panel's title/tabs (not at the opposite edge).
- Dock, tab, rearrange, resize, float, maximize, minimize and restore relevant panels.
- Save, rename, duplicate and reset user workspaces; recover missing or off-screen floating panels.
- Support AI Studio workspace preset and a normal editing preset.
- Maintain mouse, keyboard, multi-monitor, high-DPI, dark/light and reduced-motion accessibility.
- Avoid modifying control geometry unless interaction tests explicitly cover the change.

### 2. Dedicated, optional AI Studio panel
Add a dockable **AI Studio** panel via Window > AI Studio and an optional workspace preset.

The panel provides:
- Sequence settings: width, height, FPS (rational frame rate), sequence duration (fixed or match narration), pixel aspect ratio, and sequence name.
- Script: write/paste in an editor, or import TXT/DOCX/PDF (format support to be implemented/tested).
- Narration audio: import selected supported audio formats such as WAV/MP3/M4A; optional script/audio alignment.
- Current project/bin picker and scene-to-bin matching preview.
- Local model configuration: Ollama endpoint (default loopback), installed compatible model selection, connection check, hardware/runtime diagnostics.
- Actions: Validate Inputs, Analyze Project, Review Scene Map, Generate Draft, Preview Edit Plan, Apply to New Sequence, Cancel.
- Progress, warnings, missing media, and a diagnostic report. An offline project remains usable without Ollama.

### 3. Script / narration / footage workflow
1. Read supplied script and/or transcribe narration with timestamps. If both are supplied, match script sections to narration.
2. Parse narrative into **scene descriptions and timing constraints**. Do not invent source footage.
3. Index the current project's bins and media references (read-only). Prefer bins named by scene number/subject (e.g. `Scene 01 - Exterior`, `Scene 02 - Family`).
4. Match script scenes to bins using scene numbers, bin names, metadata and optionally visual content. Expose ambiguous/missing matches for user correction.
5. Sample/analyze video shots; retain source media ID, source-in/out, visual labels and confidence. Do not require sending private media to the internet.
6. Estimate shot lengths from voiceover word timings, narration pauses, target duration and user settings.
7. Generate a **structured, schema-validated edit plan** using existing sequence and clip commands. Validate IDs, source ranges, frame-rate precision, track permissions, clipping, availability and max durations.
8. Show the scene map and proposed timeline edits, with manual overrides. Default to approval before applying.
9. Apply as a **new draft sequence**, preserve original clips and projects; make edits undoable where possible.
10. Provide a report: script-scene coverage, selected source ranges, unfulfilled requirements, exact timeline duration, failed operations.
11. Export remains the native SinemaGraph export workflow.

### 4. AI division of responsibility
- **Ollama** is a model-serving runtime, not the editing engine.
- **Gemma or another compatible local model** interprets text and available visual summaries and produces scene plans / constrained edit-plan JSON.
- **Speech analysis**: use the project's existing Whisper-oriented speech infrastructure where appropriate, or a verified compatible implementation; do not assume every Gemma-on-Ollama build handles native audio.
- **Shot detection / scoring**: deterministic scene-cut and audio analysis plus appropriate local visual-model sampling.
- **Rust engine**: performs real editing after exact plan validation; the model must not execute arbitrary filesystem operations or edit raw project documents.
- Do not block UI during processing; cancellation must be safe. Keep external communication disabled by default except user-configured local Ollama.
- Never overwrite the active timeline without explicit approval.

## v1 acceptance test
Given an existing project with four named scene bins containing several clips each, a script and an optional voiceover, a user can:
- Set a vertical sequence at 1080x1920, 30 FPS, 30 seconds, square pixels.
- Select the local model and run project validation.
- Preview script-scene -> named-bin matches and correct any errors.
- Generate a reviewable edit plan referencing source clip IDs and precise in/out points.
- Apply a draft into a new sequence, play it, save/reopen the project, undo changes, and export.
- If a scene or audio file is missing, get a clear error/warning rather than fabricated footage or a crash.
- Repeat using a fixed seeded plan and obtain deterministic command execution.

## Proposed schedule (planning estimates; not delivery commitments)
Assumes one experienced full-time developer, working Windows build agents, acceptable local AI hardware, and no major regressions.

| Relative week | Milestone | Demonstrable output |
| --- | --- | --- |
| 1–2 | Complete Windows acceptance of existing rebrand and harden docking | Passing CI + downloadable Windows EXE |
| 3–4 | Workspace UX and AI Studio panel shell | Working dockable AI panel with persisted settings |
| 5–7 | Ollama connection, script/audio ingestion and project-bin inspection | Scene map and validation without editing |
| 8–11 | Shot analysis, timing engine and schema-validated draft plan | First script/voiceover-to-timeline proof of concept |
| 12–14 | Preview/approval, undo, exception handling and regression tests | AI Studio MVP candidate and Windows test builds |
| 15–20+ | Real-project quality evaluation, speed, UX and packaging | Stabilized beta after user acceptance |

The 12–14 week MVP and 15–20+ week beta estimates are indicative; actual dates depend on the engineering capacity and hardware. The existing rebrand Windows EXE is a separate current artifact; an AI Studio-enabled EXE cannot be declared complete until these stages ship.

## Required tests and release controls
- Run formatter, Clippy and affected Rust tests per CI policy; preserve existing 5,057-test baseline and licensing compliance.
- Add UI interaction and docking regression tests (panel controls beside correct title; no unclickable UI due to styling).
- Add AI input, schema, invalid media, missing bin, mismatched script/audio length, rate conversion, cancellation, empty-project and rollback tests.
- Use a small reproducible golden test project with four named bins and voiceover.
- Build a Windows x64 EXE artifact for every implementation milestone; require Windows smoke tests (imports, UI, playback, timeline editing, save/reopen, export) before marking ready.
- Keep new AI features optional and avoid touching the rebranding PR until separate acceptance gates pass.
