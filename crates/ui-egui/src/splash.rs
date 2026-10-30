//! The opening animation the desktop app plays while it starts: a 940 x 625 splash card, centred
//! over the dimmed window, with the ARTIXO artwork on the right and the name, start-up steps and a
//! progress bar on the left. A click skips it. Tests and the control (agent) mode never show it.

use egui::{Align2, Color32, ColorImage, Context, Id, Order, Sense, Stroke, TextureHandle, TextureOptions, pos2, vec2};

use crate::theme::Tokens;

static LOGO: &[u8] = include_bytes!("../../../assets/app-icon/hicolor/512x512/apps/com.artixoone.effect.png");

/// When the animation began (the host keeps one and passes it to [`show`] every frame).
pub type Start = web_time::Instant;

/// Seconds the animation lasts (including the 0.5 s fade in and the 0.5 s fade out).
const TOTAL: f64 = 3.6;
const FADE: f64 = 0.5;
const PINK: Color32 = Color32::from_rgb(0xda, 0x22, 0x84);
const CARD_BG: Color32 = Color32::from_rgb(0x14, 0x0b, 0x17);
/// The splash card's size in points.
const CARD: egui::Vec2 = egui::vec2(940.0, 625.0);
const STEPS: [&str; 5] = ["Starting engine…", "Loading effects…", "Preparing the GPU compositor…", "Opening workspace…", "Ready"];

fn smooth(x: f64) -> f32 {
    let x = x.clamp(0.0, 1.0);
    (x * x * (3.0 - 2.0 * x)) as f32
}

fn ease_out(x: f64) -> f32 {
    let x = x.clamp(0.0, 1.0);
    (1.0 - (1.0 - x).powi(3)) as f32
}

/// The logo as a mipmapped texture, decoded once. `None` when it can't be decoded.
fn logo_texture(ctx: &Context) -> Option<TextureHandle> {
    let id = Id::NULL.with("artixo-splash-logo");
    if let Some(cached) = ctx.data(|d| d.get_temp::<Option<TextureHandle>>(id)) {
        return cached;
    }
    let tex = image::load_from_memory_with_format(LOGO, image::ImageFormat::Png).ok().map(|img| {
        let img = img.to_rgba8();
        let ci = ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.as_raw());
        ctx.load_texture("artixo-splash-logo", ci, TextureOptions::LINEAR.with_mipmap_mode(Some(egui::TextureFilter::Linear)))
    });
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    tex
}

/// Draw one frame of the opening animation. `start` holds the time the animation began (set on the
/// first call). Returns `false` once it has finished.
pub fn show(ctx: &Context, start: &mut Option<Start>) -> bool {
    // The wall clock, not `ctx.input(|i| i.time)`: the animation must keep running (and end) however
    // the host paces its frames.
    let begun = *start.get_or_insert_with(web_time::Instant::now);
    let t = begun.elapsed().as_secs_f64();
    if t >= TOTAL {
        return false;
    }
    let alpha = smooth(t / FADE).min(smooth((TOTAL - t) / FADE));
    let appear = ease_out(t / 0.9);
    let screen = ctx.content_rect();
    let mut skip = false;
    egui::Area::new(Id::NULL.with("artixo-splash")).order(Order::Tooltip).fixed_pos(screen.min).interactable(true).show(ctx, |ui| {
        let (full, resp) = ui.allocate_exact_size(screen.size(), Sense::click_and_drag());
        let p = ui.painter_at(full);
        // The window behind is dimmed.
        p.rect_filled(full, 0.0, Color32::BLACK.gamma_multiply(0.62 * alpha));

        // The card: 940 x 625 points, smaller only if the window is.
        let fit = ((full.width() - 40.0) / CARD.x).min((full.height() - 40.0) / CARD.y).min(1.0).max(0.3);
        let size = CARD * fit * (0.96 + 0.04 * appear);
        let card = egui::Rect::from_center_size(full.center(), size);
        for i in 0..6 {
            // A soft shadow.
            p.rect_filled(card.expand(4.0 + i as f32 * 5.0), 20.0 + i as f32 * 4.0, Color32::BLACK.gamma_multiply(0.07 * alpha));
        }
        p.rect_filled(card, 18.0, CARD_BG.gamma_multiply(alpha));
        p.rect_stroke(card, 18.0, Stroke::new(1.0, PINK.gamma_multiply(0.35 * alpha)), egui::StrokeKind::Inside);
        let u = fit; // one unit of the layout, in points

        // Right: the artwork (centre-cropped to the panel), easing in.
        let art = egui::Rect::from_min_max(pos2(card.max.x - 540.0 * u, card.min.y + 20.0 * u), pos2(card.max.x - 20.0 * u, card.max.y - 20.0 * u));
        p.rect_filled(art, 14.0 * u, PINK.gamma_multiply(alpha));
        if let Some(tex) = logo_texture(ui.ctx()) {
            let zoom = 1.0 + 0.07 * (1.0 - appear);
            let aspect = art.width() / art.height();
            let half_w = (aspect * 0.5 / zoom).min(0.5);
            let half_h = 0.5 / zoom;
            let uv = egui::Rect::from_min_max(pos2(0.5 - half_w, 0.5 - half_h), pos2(0.5 + half_w, 0.5 + half_h));
            egui::Image::new((tex.id(), art.size())).uv(uv).corner_radius(14.0 * u).tint(Color32::WHITE.gamma_multiply(alpha * smooth(t / 0.6))).paint_at(ui, art);
        }

        // Left: small mark, name, tagline.
        let x0 = card.min.x + 40.0 * u;
        let name_in = smooth((t - 0.35) / 0.6);
        let tag_in = smooth((t - 0.7) / 0.6);
        let mark = egui::Rect::from_min_size(pos2(x0, card.min.y + 40.0 * u), vec2(64.0, 64.0) * u);
        if let Some(tex) = logo_texture(ui.ctx()) {
            p.image(tex.id(), mark, egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE.gamma_multiply(alpha * name_in));
        }
        p.text(
            pos2(mark.max.x + 16.0 * u, mark.center().y - 11.0 * u + (1.0 - name_in) * 8.0),
            Align2::LEFT_CENTER,
            "ARTIXO Effect",
            Tokens::semibold(26.0 * u),
            Color32::WHITE.gamma_multiply(alpha * name_in),
        );
        p.text(
            pos2(mark.max.x + 16.0 * u, mark.center().y + 17.0 * u),
            Align2::LEFT_CENTER,
            "Motion graphics & visual effects",
            Tokens::ui(12.5 * u),
            Color32::from_rgb(0xe9, 0xc2, 0xd9).gamma_multiply(alpha * tag_in),
        );

        // A ring of three dots circling while it loads.
        let spin = t as f32 * 2.6;
        let ring_c = pos2(x0 + 14.0 * u, card.center().y + 10.0 * u);
        for k in 0..3 {
            let a = spin + k as f32 * std::f32::consts::TAU / 3.0;
            let d = ring_c + vec2(a.cos(), a.sin()) * 11.0 * u;
            p.circle_filled(d, (3.4 - k as f32 * 0.6) * u, PINK.gamma_multiply(alpha));
        }

        // Start-up step and progress bar.
        let progress = smooth((t - 0.3) / (TOTAL - 1.2));
        let step = ((progress * STEPS.len() as f32) as usize).min(STEPS.len() - 1);
        let bar_w = card.width() - 540.0 * u - 100.0 * u;
        let bar_y = card.max.y - 92.0 * u;
        p.text(pos2(x0, bar_y - 18.0 * u), Align2::LEFT_CENTER, STEPS[step], Tokens::ui(12.5 * u), Color32::WHITE.gamma_multiply(0.66 * alpha));
        let track = egui::Rect::from_min_size(pos2(x0, bar_y), vec2(bar_w, 3.0));
        p.rect_filled(track, 1.5, Color32::WHITE.gamma_multiply(0.12 * alpha));
        p.rect_filled(egui::Rect::from_min_size(track.min, vec2(bar_w * progress, 3.0)), 1.5, PINK.gamma_multiply(alpha));

        // Footer.
        let foot = Color32::WHITE.gamma_multiply(0.42 * alpha);
        p.text(pos2(x0, card.max.y - 52.0 * u), Align2::LEFT_CENTER, "\u{a9} 2026 ARTIXO ONE. All rights reserved.", Tokens::ui(11.0 * u), foot);
        p.text(pos2(x0, card.max.y - 34.0 * u), Align2::LEFT_CENTER, format!("Version {}", env!("CARGO_PKG_VERSION")), Tokens::ui(11.0 * u), foot);

        skip = resp.clicked();
    });
    if skip && t < TOTAL - FADE {
        *start = web_time::Instant::now().checked_sub(std::time::Duration::from_secs_f64(TOTAL - FADE));
    }
    ctx.request_repaint();
    true
}
