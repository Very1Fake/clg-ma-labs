use eframe::{
    egui::{global_dark_light_mode_switch, CtxRef, TopBottomPanel, Ui, Vec2},
    epi::{App as EApp, Frame},
};

use seven::SeventhLab;
use six::SixthLab;

mod seven;
mod six;

#[inline]
fn divider(ui: &mut Ui) {
    ui.separator();
    ui.add_space(16.0);
}

// -------------------------------------------------------------------------------------------------

#[derive(PartialEq)]
enum Anchor {
    Sixth,
    Seventh,
}

impl Anchor {
    fn as_str(&self) -> &str {
        match self {
            Anchor::Sixth => "Lab 6",
            Anchor::Seventh => "Lab 7",
        }
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::Sixth
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Default)]
struct Labs {
    sixth_lab: SixthLab,
    seventh_lab: SeventhLab,
}

impl Labs {
    fn iter_mut(&mut self) -> impl Iterator<Item = (Anchor, &mut dyn EApp)> {
        vec![
            (Anchor::Sixth, &mut self.sixth_lab as &mut dyn EApp),
            (Anchor::Seventh, &mut self.seventh_lab as &mut dyn EApp),
        ]
        .into_iter()
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct LabsApp {
    labs: Labs,
    selected_lab: Anchor,
    full_width: bool,
    debug: bool,
}

impl EApp for LabsApp {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        ctx.set_debug_on_hover(self.debug);

        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                global_dark_light_mode_switch(ui);
                #[cfg(target_arch = "wasm32")]
                if ui
                    .selectable_label(self.full_width, "🖵")
                    .on_hover_text("Full width mode")
                    .clicked()
                {
                    self.full_width = !self.full_width;
                }
                if ui
                    .selectable_label(self.debug, "🐞")
                    .on_hover_text("Debug on hover")
                    .clicked()
                {
                    self.debug = !self.debug;
                }
                ui.separator();
                self.labs.iter_mut().for_each(|(anchor, _)| {
                    if ui
                        .selectable_label(anchor == self.selected_lab, anchor.as_str())
                        .clicked()
                    {
                        self.selected_lab = anchor;
                    }
                });
            })
        });

        self.labs
            .iter_mut()
            .find(|(anchor, _)| anchor == &self.selected_lab)
            .unwrap()
            .1
            .update(ctx, frame);
    }

    fn name(&self) -> &str {
        "MA: Labs"
    }

    fn max_size_points(&self) -> Vec2 {
        if self.full_width {
            Vec2::new(f32::INFINITY, f32::INFINITY)
        } else {
            Vec2::new(1024.0, 2048.0)
        }
    }
}
