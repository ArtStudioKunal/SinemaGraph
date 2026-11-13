//! Sinemagraph Import workspace: a responsive launch surface with gently animated
//! cards and the same import/recent-project commands used by the original editor.

use egui::{Align2, Rect, Sense, Stroke, StrokeKind, pos2, vec2};
use serde_json::json;

use crate::FilmcraftApp;
use crate::theme::{Tokens, blend};

pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let p = ui.painter().clone();
    p.rect_filled(rect, t.radius, t.panel_bg);
    let left = rect.left() + 32.0;
    let top = rect.top() + 30.0;
    let width = (rect.width() - 64.0).max(120.0);

    p.rect_filled(Rect::from_min_size(pos2(left, top), vec2(44.0, 3.0)), 2.0, t.accent);
    p.text(pos2(left, top + 28.0), Align2::LEFT_CENTER, "SINEMAGRAPH", Tokens::semibold(11.0), t.accent);
    p.text(pos2(left, top + 67.0), Align2::LEFT_CENTER, tl!("Import"), Tokens::semibold(27.0), t.text);
    p.text(
        pos2(left, top + 98.0),
        Align2::LEFT_CENTER,
        tl!("Add media to your project. Drop files or folders anywhere in the window, or choose a source below."),
        Tokens::ui(13.0),
        t.text_dim,
    );

    let ctx = ui.ctx().clone();
    let cards = [
        (tl!("Browse files…"), "file.import", tl!("Movies, audio and stills from disk"), crate::icons::Icon::Folder),
        (tl!("Demo footage"), "file.importDemoFootage", tl!("Six procedural 1080p clips with sound"), crate::icons::Icon::Film),
        (tl!("Demo project"), "file.openDemoProject", tl!("A cut sequence with transitions, effects and music"), crate::icons::Icon::Sequence),
        (tl!("Bars and Tone"), "file.newBarsAndTone", tl!("SMPTE HD bars with 1 kHz reference tone"), crate::icons::Icon::Grid),
    ];

    let gap = 14.0;
    let columns = if width >= 1100.0 {
        4
    } else if width >= 760.0 {
        3
    } else if width >= 490.0 {
        2
    } else {
        1
    };
    let card_width = (width - gap * (columns - 1) as f32) / columns as f32;
    let card_height = 160.0;
    let cards_top = top + 132.0;
    for (i, (title, command, subtitle, icon)) in cards.iter().enumerate() {
        let column = i % columns;
        let row = i / columns;
        let r =
            Rect::from_min_size(pos2(left + column as f32 * (card_width + gap), cards_top + row as f32 * (card_height + gap)), vec2(card_width, card_height));
        let resp = ui.interact(r, egui::Id::unique(("imp", *command)), Sense::click());
        app.auto.add(&format!("import.{command}"), r, title);
        let hover = ui.ctx().animate_bool(resp.id, resp.hovered());
        let fill = if i == 0 { blend(t.tab_bg, t.accent, 0.13 + 0.08 * hover) } else { blend(t.tab_bg, t.hover, hover * 0.85) };
        p.rect_filled(r, 14.0, fill);
        p.rect_stroke(
            r,
            14.0,
            Stroke::new(1.0, if i == 0 || resp.hovered() { blend(t.separator, t.accent, 0.45 + 0.3 * hover) } else { t.separator }),
            StrokeKind::Inside,
        );

        let icon_bg = Rect::from_min_size(r.min + vec2(18.0, 18.0), vec2(48.0, 48.0));
        p.rect_filled(icon_bg, 12.0, blend(t.panel_bg, t.accent, if i == 0 { 0.23 } else { 0.12 + 0.12 * hover }));
        crate::icons::paint(&p, icon_bg.shrink(11.0), *icon, if i == 0 || resp.hovered() { t.accent_hover } else { t.icon });

        let clipped = p.with_clip_rect(r.shrink(12.0));
        clipped.text(pos2(r.left() + 18.0, r.top() + 99.0), Align2::LEFT_CENTER, *title, Tokens::semibold(14.0), t.text);
        let mut sub = egui::text::LayoutJob::simple_singleline(subtitle.to_string(), Tokens::ui(11.5), t.text_dim);
        sub.wrap.max_width = (card_width - 36.0).max(60.0);
        sub.wrap.max_rows = 2;
        let galley = clipped.layout_job(sub);
        clipped.galley(pos2(r.left() + 18.0, r.top() + 116.0), galley, t.text_dim);

        if resp.clicked() {
            if let Err(error) = crate::menus::invoke(app, &ctx, command, json!({})) {
                app.ui.status = error;
            } else {
                app.ui.mode = crate::state::Mode::Edit;
            }
        }
    }
    let rows = cards.len().div_ceil(columns);
    recent(app, ui, rect, cards_top + rows as f32 * (card_height + gap) + 28.0 - rect.top());
}

/// Recent projects listed on Home, as in File ▸ Open Recent.
const RECENT_ROWS: usize = 6;
const RECENT_ROW_H: f32 = 28.0;

/// Home's Recent list at `y`: the recent projects (newest first, file name with its folder beside
/// it), a click opens one.
fn recent(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect, y: f32) {
    let paths: Vec<String> = app.session.prefs.general.recent_projects.iter().take(RECENT_ROWS).cloned().collect();
    if paths.is_empty() {
        return;
    }
    let t = app.tokens;
    ui.painter().text(rect.min + vec2(24.0, y), Align2::LEFT_CENTER, tl!("Recent"), Tokens::semibold(15.0), t.text);
    let mut open = None;
    for (i, path) in paths.iter().enumerate() {
        let row =
            Rect::from_min_size(rect.min + vec2(16.0, y + 16.0 + i as f32 * RECENT_ROW_H), vec2(560.0_f32.min(rect.width() - 32.0).max(0.0), RECENT_ROW_H));
        let name = crate::menus::recent_name(path);
        let resp = ui.interact(row, egui::Id::unique(("imp-recent", i)), Sense::click()).on_hover_text(path.as_str());
        app.auto.add(&format!("import.recent.{i}"), row, &name);
        if resp.hovered() {
            ui.painter().rect_filled(row, 4.0, t.hover);
        }
        let p = ui.painter().with_clip_rect(row);
        crate::icons::paint(&p, Rect::from_center_size(pos2(row.min.x + 16.0, row.center().y), vec2(14.0, 14.0)), crate::icons::Icon::Sequence, t.text_dim);
        let name_r = p.text(pos2(row.min.x + 32.0, row.center().y), Align2::LEFT_CENTER, &name, Tokens::ui(13.0), t.text);
        if let Some(folder) = std::path::Path::new(path).parent() {
            p.text(pos2(name_r.max.x + 12.0, row.center().y), Align2::LEFT_CENTER, folder.to_string_lossy(), Tokens::ui(11.5), t.text_dim);
        }
        if resp.clicked() {
            open = Some(path.clone());
        }
    }
    if let Some(path) = open {
        let ctx = ui.ctx().clone();
        // unsaved changes: Save / Don't Save / Cancel first, asked over the project they were made in;
        // Home switches to Edit once the chosen project opens, and stays after Cancel
        if crate::panels::clip_dialogs::ask_before_open(app, &path).is_some() {
            if let Some(info) = app.ui.clip_dialog.as_mut().and_then(|d| d.info.as_object_mut()) {
                info.insert("editAfterOpen".into(), json!(true));
            }
        } else if crate::menus::invoke(app, &ctx, "file.open", json!({"path": path})).is_ok() {
            app.ui.mode = crate::state::Mode::Edit;
        }
    }
}
