use std::ops::RangeInclusive;

use eframe::{
    egui::{
        text::LayoutJob, Align, Button, CentralPanel, DragValue, Grid, ScrollArea, SidePanel,
        TextFormat, TextStyle, Vec2,
    },
    epi::App,
};
use rand::{thread_rng, Rng};

use super::divider;

type StoreType = u8;
const LENGTH_RANGE: RangeInclusive<usize> = 2..=10;

#[derive(Default, Debug)]
struct Game {
    pub matrix: Vec<Vec<StoreType>>,

    // Calculation result
    pub a_min: Vec<StoreType>,
    pub b_max: Vec<StoreType>,
    pub max_min: StoreType, // max(min(Ai))
    pub min_max: StoreType, // min(max(Bj))
    pub saddle_point: Option<(usize, usize)>,

    pub rows: usize,
    pub columns: usize,
}

impl Game {
    const VALUE_RANGE: RangeInclusive<StoreType> = 2..=12;

    // Changes the size of the inner matrix, freeing the old one and allocating new one
    pub fn resize(&mut self, length: (usize, usize)) {
        self.rows = length.0;
        self.columns = length.1;

        self.matrix = vec![vec![0; length.1]; length.0];
        self.a_min = vec![0; length.0];
        self.b_max = vec![0; length.1];

        self.randomize();
    }

    // Recalculate min(Ai) and max(Bj)
    pub fn recalc(&mut self) {
        self.a_min
            .iter_mut()
            .enumerate()
            .for_each(|(i, val)| *val = *self.matrix.get(i).unwrap().iter().min().unwrap());

        self.b_max.iter_mut().enumerate().for_each(|(i, val)| {
            *val = *self
                .matrix
                .iter()
                .flatten()
                .skip(i)
                .step_by(self.columns)
                .max()
                .unwrap()
        });

        self.max_min = *self.a_min.iter().max().unwrap();
        self.min_max = *self.b_max.iter().min().unwrap();

        if self.max_min == self.min_max {
            let mut points = Vec::new();
            self.matrix.iter().enumerate().for_each(|(i, row)| {
                row.iter().enumerate().for_each(|(j, val)| {
                    if *val == self.max_min {
                        if self.a_min[i] == self.max_min && self.b_max[j] == self.min_max {
                            points.push((i, j));
                        }
                    }
                })
            });
            self.saddle_point = Some(*points.first().unwrap());
        } else {
            self.saddle_point = None;
        }
    }

    // Randomize matrix values
    pub fn randomize(&mut self) {
        let mut rng = thread_rng();

        self.matrix.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|val| {
                *val = rng.gen_range(Self::VALUE_RANGE);
            })
        });

        self.recalc();
    }
}

pub struct SixthLab {
    game: Game,
    length: (usize, usize),
}

impl App for SixthLab {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        // Side panel
        SidePanel::left("left_panel")
            .max_width(256.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|vert| vert.heading("⛭ Controls"));
                ui.separator();
                ui.vertical_centered(|vert| vert.label("Matrix size"));
                ui.vertical(|vert| {
                    vert.add_sized(
                        Vec2::new(vert.available_size_before_wrap().x, 1.0),
                        DragValue::new(&mut self.length.0)
                            .speed(0.05)
                            .prefix("M: ")
                            .clamp_range(LENGTH_RANGE),
                    );
                    vert.add_sized(
                        Vec2::new(vert.available_size_before_wrap().x, 1.0),
                        DragValue::new(&mut self.length.1)
                            .speed(0.05)
                            .prefix("N: ")
                            .clamp_range(LENGTH_RANGE),
                    );
                });
                divider(ui);
                ui.vertical_centered(|vert| vert.heading("✔ Results"));
                ui.separator();
                ui.vertical(|vert| {
                    vert.label(format!("max(min(Ai)) = {}", self.game.max_min));
                    vert.label(format!("min(max(Bj)) = {}", self.game.min_max));
                    match self.game.saddle_point {
                        Some(point) => {
                            vert.label(format!("V = {}", self.game.min_max));
                            vert.label(format!("Saddle Point: {point:?}"));
                        }
                        None => {
                            vert.label("No Pure Strategy Solutions");
                        }
                    }
                });
                divider(ui);
                ui.vertical_centered(|vert| vert.heading("🔃 Results"));
                ui.separator();
                if ui
                    .add_sized(
                        Vec2::new(ui.available_size().x, 1.0),
                        Button::new("Regenerate"),
                    )
                    .clicked()
                {
                    self.game.resize(self.length);
                }
                if ui
                    .add_sized(
                        Vec2::new(ui.available_size().x, 1.0),
                        Button::new("Randomize"),
                    )
                    .clicked()
                {
                    self.game.randomize();
                }
            });

        // Main section
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Matrix View");
            // Matrix grid
            ScrollArea::both().show(ui, |scroll| {
                Grid::new("matrix").striped(true).show(scroll, |grid| {
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
                        {
                            let mut job = LayoutJob::default();
                            job.append(
                                "A",
                                0.0,
                                TextFormat {
                                    style: TextStyle::Small,
                                    valign: Align::BOTTOM,
                                    ..Default::default()
                                },
                            );
                            job.append(" / ", 0.0, Default::default());
                            job.append(
                                "B",
                                0.0,
                                TextFormat {
                                    style: TextStyle::Small,
                                    valign: Align::TOP,
                                    ..Default::default()
                                },
                            );
                            grid.label(job);
                        }
                        for i in 0..self.game.columns {
                            grid.label(formatted_text("B", i));
                        }
                        grid.label("min(Ai)");
                        grid.end_row();
                    }

                    self.game.matrix.iter().enumerate().for_each(|(i, row)| {
                        grid.label(formatted_text("A", i));
                        row.iter().for_each(|num| {
                            grid.label(num.to_string());
                        });
                        grid.label(self.game.a_min.get(i).unwrap().to_string());
                        grid.end_row();
                    });

                    // Footer
                    {
                        grid.label("max(Bj)");
                        self.game.b_max.iter().for_each(|val| {
                            grid.label(val.to_string());
                        });
                        grid.end_row();
                    }
                });
            });
        });
    }

    fn name(&self) -> &str {
        "Lab 6"
    }
}

impl Default for SixthLab {
    fn default() -> Self {
        #[allow(const_item_mutation)]
        let min_len = LENGTH_RANGE.next().unwrap();
        let length = (min_len, min_len);
        let mut matrix = Game::default();
        matrix.resize(length);

        Self {
            game: matrix,
            length,
        }
    }
}
