use std::ops::RangeInclusive;

use eframe::{
    egui::{
        plot::{Legend, Line, Plot, Points, Value, Values},
        Button, CentralPanel, Color32, CtxRef, DragValue, Grid, RichText, ScrollArea,
        SelectableLabel, SidePanel, Vec2, Window,
    },
    epi::{App, Frame},
};
use rand::{thread_rng, Rng};

use super::divider;

const NODES_RANGE: RangeInclusive<usize> = 1..=15;

#[derive(Default)]
struct Matrix {
    inner: Vec<Vec<bool>>,
    vertices: usize,
}

impl Matrix {
    // Changes the size of the inner vector, freeing the old one and allocating new one
    pub fn resize(&mut self, nodes: usize) {
        self.vertices = nodes;

        self.inner = Vec::with_capacity(self.vertices);

        for i in 0..self.vertices {
            self.inner.push(vec![false; i + 1]);
        }
    }

    // Randomizes edges of the graph
    pub fn randomize(&mut self) {
        let mut rng = thread_rng();

        self.inner.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, state)| {
                if i != j {
                    *state = rng.gen_bool(0.5);
                }
            });
        });
    }

    // Filling all the edges of the graph
    pub fn fill(&mut self) {
        self.inner.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, state)| {
                if i != j {
                    *state = true;
                }
            });
        });
    }

    // Clearing all the edges of the graph
    pub fn empty(&mut self) {
        self.inner
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|val| *val = false))
    }

    // Outputs vertices coords
    fn vertices_iter(&self) -> impl Iterator<Item = Value> {
        let count = self.vertices;
        let radius = match count {
            1 => 0.0,
            2..=8 => 1.0,
            9..=12 => 1.25,
            13..=15 => 1.5,
            _ => 2.0,
        };

        (0..self.vertices).map(move |i| {
            let angle = ((360.0 / count as f64) * i as f64).to_radians();
            Value::new(radius * angle.cos(), radius * angle.sin())
        })
    }

    // Outputs edges
    fn edges(&self) -> Vec<(usize, usize)> {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, state)| if *state { Some((i, j)) } else { None })
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect()
    }
}

// -------------------------------------------------------------------------------------------------

pub struct SeventhLab {
    matrix: Matrix,
    nodes: usize,

    // UI
    matrix_viewer: bool,
    graph_viewer: bool,
}

impl App for SeventhLab {
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        SidePanel::left("graph_left")
            .max_width(256.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|vert| vert.heading("⛭ Controls"));
                ui.separator();
                ui.add_sized(
                    Vec2::new(ui.available_width(), 1.0),
                    DragValue::new(&mut self.nodes)
                        .speed(0.05)
                        .prefix("Nodes: ")
                        .clamp_range(NODES_RANGE),
                );
                if ui
                    .add_sized(
                        Vec2::new(ui.available_size().x, 1.0),
                        Button::new("Regenerate"),
                    )
                    .clicked()
                {
                    self.matrix.resize(self.nodes);
                }
                divider(ui);
                ui.vertical_centered(|vert| vert.heading("👁 Viewers"));
                ui.separator();
                ui.vertical(|vert| {
                    let matrix_viewer_label =
                        SelectableLabel::new(self.matrix_viewer, "Matrix Viewer");
                    let graph_viewer_label =
                        SelectableLabel::new(self.graph_viewer, "Graph Viewer");

                    if vert
                        .add_sized(Vec2::new(vert.available_width(), 1.0), matrix_viewer_label)
                        .clicked()
                    {
                        self.matrix_viewer = !self.matrix_viewer;
                    };
                    if vert
                        .add_sized(Vec2::new(vert.available_width(), 1.0), graph_viewer_label)
                        .clicked()
                    {
                        self.graph_viewer = !self.graph_viewer;
                    };
                });
                divider(ui);
                ui.vertical_centered(|vert| vert.heading("🔃 Operations"));
                ui.separator();
                if ui
                    .add_sized(
                        Vec2::new(ui.available_size().x, 1.0),
                        Button::new("Randomize"),
                    )
                    .clicked()
                {
                    self.matrix.randomize();
                }
                if ui
                    .add_sized(Vec2::new(ui.available_size().x, 1.0), Button::new("Fill"))
                    .clicked()
                {
                    self.matrix.fill();
                }
                if ui
                    .add_sized(Vec2::new(ui.available_size().x, 1.0), Button::new("Empty"))
                    .clicked()
                {
                    self.matrix.empty();
                }
            });

        Window::new("Matrix Viewer")
            .open(&mut self.matrix_viewer)
            .show(ctx, |ui| {
                ScrollArea::horizontal().show(ui, |scroll| {
                    Grid::new("graph_matrix")
                        .striped(true)
                        .show(scroll, |grid| {
                            // Header
                            {
                                grid.label("");
                                for i in 0..self.matrix.vertices {
                                    grid.label(RichText::new((i + 1).to_string()).strong());
                                }
                                grid.end_row();
                            }

                            for i in 0..self.matrix.vertices {
                                grid.label(RichText::new((i + 1).to_string()).strong());
                                for j in 0..self.matrix.vertices {
                                    // Performance preferred in this case
                                    #[allow(clippy::comparison_chain)]
                                    if j == i {
                                        grid.label("-");
                                    } else {
                                        grid.checkbox(
                                            if j < i {
                                                &mut self.matrix.inner[i][j]
                                            } else {
                                                &mut self.matrix.inner[j][i]
                                            },
                                            "",
                                        );
                                    }
                                }
                                grid.end_row();
                            }
                        });
                });
            });

        Window::new("Graph Viewer")
            .open(&mut self.graph_viewer)
            .show(ctx, |ui| {
                Plot::new("graph_viewer")
                    .data_aspect(1.0)
                    .show_axes([false; 2])
                    .legend(Legend::default())
                    .show(ui, |plot| {
                        plot.points(
                            Points::new(Values::from_values_iter(self.matrix.vertices_iter()))
                                .radius(5.0)
                                .color(Color32::YELLOW)
                                .highlight()
                                .name("Vertices"),
                        );

                        {
                            let vertices = self.matrix.vertices_iter().collect::<Vec<Value>>();
                            self.matrix.edges().iter().for_each(|(a, b)| {
                                plot.line(
                                    Line::new(Values::from_values(vec![
                                        vertices[*a],
                                        vertices[*b],
                                    ]))
                                    .color(Color32::GREEN)
                                    .name("Edges"),
                                );
                            });
                        }
                    });
            });

        // Disable transparent background
        CentralPanel::default().show(ctx, |_| {});
    }

    fn name(&self) -> &str {
        "Lab 7"
    }
}

impl Default for SeventhLab {
    fn default() -> Self {
        #[allow(const_item_mutation)]
        let nodes = NODES_RANGE.next().unwrap();
        let mut matrix = Matrix::default();
        matrix.resize(nodes);

        Self {
            matrix,
            nodes,
            matrix_viewer: true,
            graph_viewer: true,
        }
    }
}
