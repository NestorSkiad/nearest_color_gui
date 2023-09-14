use egui::{Color32, Sense, Vec2};
use crate::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    r: u8,
    g: u8,
    b: u8,
    #[serde(skip)]
    standard_colors: Vec<StandardColor>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            standard_colors: get_standard_colors(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { r, g, b, standard_colors } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Nearest color name");
            ui.add(egui::Slider::new(r, 0..=255).text("red"));
            ui.add(egui::Slider::new(g, 0..=255).text("green"));
            ui.add(egui::Slider::new(b, 0..=255).text("blue"));
            let (rect, _response) = ui.allocate_at_least(Vec2::new(200.0, 200.0), Sense::hover());
            ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(*r, *g, *b));

            ui.label(format!("R{r} G{g} B{b}"));

            let nearest_color = nearest_color_single(&standard_colors, [*r as i64, *g as i64, *b as i64]);
            ui.heading(format!("Nearest color: {}", nearest_color.name));

            let (rect, _response) = ui.allocate_at_least(Vec2::new(200.0, 200.0), Sense::hover());
            ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(nearest_color.color[0] as u8, nearest_color.color[1] as u8, nearest_color.color[2] as u8));
            ui.label(format!("Values: {}", color_to_str(nearest_color.color)));
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
