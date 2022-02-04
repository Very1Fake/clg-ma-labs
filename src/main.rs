use std::ops::RangeInclusive;

use eframe::{
    egui::{
        global_dark_light_mode_switch, text::LayoutJob, Align, Button, CentralPanel, DragValue,
        Grid, SidePanel, TextFormat, TextStyle, TopBottomPanel,
    },
    epi::{App as EApp, NativeOptions},
    run_native,
};
use rand::{thread_rng, Rng};

type StoreType = u8;
const LENGTH_RANGE: RangeInclusive<usize> = 2..=10;

#[derive(Default, Debug)]
struct Matrix {
    pub inner: Vec<Vec<StoreType>>,
    pub a_min: Vec<StoreType>,
    pub b_max: Vec<StoreType>,

    pub rows: usize,
    pub columns: usize,
}

impl Matrix {
    const VALUE_RANGE: RangeInclusive<StoreType> = 2..=12;

    // Recalculate min(Ai) and max(Bi)
    pub fn recalc(&mut self) {
        self.a_min
            .iter_mut()
            .enumerate()
            .for_each(|(i, val)| *val = *self.inner.get(i).unwrap().iter().min().unwrap());

        self.b_max
            .iter_mut()
            .enumerate()
            .for_each(|(i, val)| *val = *self.inner.iter().flatten().skip(i).step_by(self.columns).max().unwrap());
    }

    // Randomize matrix values
    pub fn randomize(&mut self) {
        let mut rng = thread_rng();

        self.inner.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|val| {
                *val = rng.gen_range(Self::VALUE_RANGE);
            })
        });

        self.recalc();
    }

    pub fn resize(&mut self, length: (usize, usize)) {
        self.rows = length.0;
        self.columns = length.1;

        self.inner = vec![vec![0; length.1]; length.0];
        self.a_min = vec![0; length.0];
        self.b_max = vec![0; length.1];

        self.randomize();
    }
}

struct App {
    matrix: Matrix,
    length: (usize, usize),
}

impl EApp for App {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.label("Lab 6");
            })
        });

        // Side panel
        SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| ui.heading("\u{26ED} Controls"));
                ui.separator();
                ui.label("Matrix size");
                ui.horizontal_top(|ui| {
                    let s = [ui.available_size_before_wrap().x / 2.1, 1.0];
                    ui.add_sized(
                        s,
                        DragValue::new(&mut self.length.0)
                            .speed(1.0)
                            .prefix("M: ")
                            .clamp_range(LENGTH_RANGE),
                    );
                    ui.add_sized(
                        s,
                        DragValue::new(&mut self.length.1)
                            .speed(1.0)
                            .prefix("N: ")
                            .clamp_range(LENGTH_RANGE),
                    );
                });
                ui.separator();
                if ui
                    .add_sized([ui.available_size().x, 1.0], Button::new("Regenerate"))
                    .clicked()
                {
                    self.matrix.resize(self.length);
                }
                if ui
                    .add_sized([ui.available_size().x, 1.0], Button::new("Randomize"))
                    .clicked()
                {
                    self.matrix.randomize();
                }
            });

        // Main section
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Matrix View");
            // Matrix grid
            Grid::new("matrix").striped(true).show(ui, |grid| {
                // Index text formatting
                #[inline]
                fn formatted_text(prefix: &str, index: usize) -> LayoutJob {
                    let mut job =
                        LayoutJob::single_section(prefix.to_string(), TextFormat::default());
                    job.append(
                        index.to_string().as_str(),
                        0.0,
                        TextFormat {
                            style: TextStyle::Small,
                            valign: Align::BOTTOM,
                            ..Default::default()
                        },
                    );
                    job
                }

                // Header
                {
                    grid.label("");
                    for i in 0..self.matrix.columns {
                        grid.label(formatted_text("B", i));
                    }
                    grid.label("min(Ai)");
                    grid.end_row();
                }

                self.matrix.inner.iter().enumerate().for_each(|(i, row)| {
                    grid.label(formatted_text("A", i));
                    row.iter().for_each(|num| {
                        grid.label(num.to_string());
                    });
                    grid.label(self.matrix.a_min.get(i).unwrap().to_string());
                    grid.end_row();
                });

                // Footer
                {
                    grid.label("max(Bi)");
                    self.matrix.b_max.iter().for_each(|val| {
                        grid.label(val.to_string());
                    });
                    grid.end_row();
                }
            });
        });
    }

    fn name(&self) -> &str {
        "MA: Lab 6"
    }
}

impl App {
    fn new() -> Self {
        #[allow(const_item_mutation)]
        let min_len = LENGTH_RANGE.next().unwrap();
        let length = (min_len, min_len);
        let mut matrix = Matrix::default();
        matrix.resize(length);

        App { matrix, length }
    }
}

fn main() {
    run_native(Box::new(App::new()), NativeOptions::default());
}
