use egui::emath::RectTransform;
use egui::{Color32, Frame, Pos2, Rect, Sense, Vec2, emath};

#[derive(Default)]
pub struct TemplateApp {}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        update_ui(ctx, _frame);
    }
}

fn update_ui(ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        Frame::canvas(ui.style()).show(ui, |ui| {
            //self.ui_content(ui);

            let (response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                response.rect,
            );

            // Let's try a grid of squares
            const ROWS: usize = 16;
            const COLS: usize = 16;
            const GRID_MARGIN: f32 = 0.0175; // Margin around entire grid
            const CELL_GAP: f32 = 0.01125; // Gap between cells

            const GRID_ORIGIN: Pos2 = Pos2::new(0.0, 0.0);
            const GRID_DIMS: Vec2 = Vec2::new(1.0, 1.0);

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
                    let cell_local =
                        Rect::from_min_size(Pos2::new(x0, y0), Vec2::new(cell_width, cell_height));

                    // Transform to canvas coords, then to screen coords
                    let cell_canvas = grid_to_canvas.transform_rect(cell_local);
                    let cell_screen = to_screen.transform_rect(cell_canvas);

                    let color = Color32::from_rgb(20, 20, 20);
                    grid_shapes.push(egui::Shape::rect_filled(cell_screen, 1.0, color));
                }
            }

            painter.extend(grid_shapes);
        });
    });
}
