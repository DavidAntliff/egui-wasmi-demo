use egui::{emath, Color32, Frame, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};
use egui::emath::RectTransform;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
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
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("egui-eframe-demo");


            Frame::canvas(ui.style()).show(ui, |ui| {
                //self.ui_content(ui);

                let (mut response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

                let to_screen = emath::RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                    response.rect,
                );

                let lines: Vec<Vec<Pos2>> = vec![
                    vec![Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)],
                    vec![Pos2::new(0.0, 1.0), Pos2::new(1.0, 0.0)],
                ];
                let stroke = Stroke::new(1.0, Color32::from_rgb(25, 200, 100));

                let mut shapes = lines
                    .iter()
                    .filter(|line| line.len() >= 2)
                    .map(|line| {
                        let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                        egui::Shape::line(points, stroke)
                    }).collect::<Vec<_>>();

                shapes.push(egui::Shape::rect_stroke(
                    response.rect,
                    20.0,
                    Stroke::new(1.0, Color32::LIGHT_BLUE),
                    StrokeKind::Inside
                ));

                shapes.push(egui::Shape::rect_filled(
                    to_screen.transform_rect(Rect::from_two_pos(Pos2::new(1.1, 0.1), Pos2::new(1.4, 0.4))),
                    20.0,
                    Color32::LIGHT_BLUE,
                ));

                // Add multiple shapes at once
                painter.extend(shapes);

                // Add a single shape
                painter.add(
                    egui::Shape::rect_filled(
                        to_screen.transform_rect(Rect::from_two_pos(Pos2::new(1.2, 0.6), Pos2::new(1.5, 0.9))),
                        0.0,
                        Color32::from_rgb(200, 100, 25),
                    )
                );


                // Let's try a grid of squares
                const ROWS: usize = 16;
                const COLS: usize = 16;
                const GRID_MARGIN: f32 = 0.0175;  // Margin around entire grid
                const CELL_GAP: f32 = 0.01125;  // Gap between cells

                const GRID_ORIGIN: Pos2 = Pos2::new(0.1, 0.1);
                const GRID_DIMS: Vec2 = Vec2::new(0.8, 0.8);

                // Grid local coordinates: (0, 0) to (1, 1)
                let grid_local_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1.0, 1.0));

                // Where the grid lives in the logical canvas coordinates
                let grid_canvas_rect = Rect::from_min_size(GRID_ORIGIN, GRID_DIMS);

                // Transform: grid local (0-1) -> canvas logical (0.25-0.75)
                let grid_to_canvas = RectTransform::from_to(grid_local_rect, grid_canvas_rect);

                // Calculate cell size: available space after margins and gaps
                let total_gap_x = CELL_GAP * (COLS - 1) as f32;
                let total_gap_y = CELL_GAP * (ROWS - 1) as f32;
                let cell_width = (1.0 - 2.0 * GRID_MARGIN - total_gap_x) / COLS as f32;
                let cell_height = (1.0 - 2.0 * GRID_MARGIN - total_gap_y) / ROWS as f32;

                painter.add(egui::Shape::rect_filled(
                    to_screen.transform_rect(grid_canvas_rect),
                    0.0,
                    Color32::BLACK,
                ));

                let mut grid_shapes = Vec::new();
                for row in 0..ROWS {
                    for col in 0..COLS {
                        let x0 = GRID_MARGIN + col as f32 * (cell_width + CELL_GAP);
                        let y0 = GRID_MARGIN + row as f32 * (cell_height + CELL_GAP);

                        // Grid local rect for this cell
                        let cell_local = Rect::from_min_size(
                            Pos2::new(x0, y0),
                            Vec2::new(cell_width, cell_height),
                        );

                        // Transform to canvas coords, then to screen coords
                        let cell_canvas = grid_to_canvas.transform_rect(cell_local);
                        let cell_screen = to_screen.transform_rect(cell_canvas);

                        let color = Color32::from_rgb(
                            (row * 255 / ROWS) as u8,
                            (col * 255 / COLS) as u8,
                            150,
                        );
                        grid_shapes.push(egui::Shape::rect_filled(cell_screen, 1.0, color));
                    }
                }

                painter.extend(grid_shapes);
            });
        });
    }

    ///// Called by the framework to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}
