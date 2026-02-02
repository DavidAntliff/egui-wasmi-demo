use egui::emath::RectTransform;
use egui::{Color32, Frame, Pos2, Rect, Sense, Vec2};
use std::collections::VecDeque;
use wasmi::{Engine, Linker, Memory, Module, Store, TypedFunc};
use web_time::{Duration, Instant};

const FPS: f32 = 30.0;
const FPS_WINDOW_SIZE: usize = 60;

pub struct DemoApp {
    counter: u64,
    frame_times: VecDeque<Instant>,
    guest_state: GuestState,
}

pub struct GuestState {
    _engine: Engine,
    store: Store<()>,
    _linker: Linker<()>,
    memory: Memory,

    // Guest exports
    buffer_ptr: TypedFunc<(), i32>,
    update: TypedFunc<u64, ()>,
}

impl DemoApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let wasm_bytes =
            include_bytes!("../../guest/target/wasm32-unknown-unknown/release/guest.wasm");
        log::info!("Initialising engine...");
        let engine = Engine::default();
        log::info!("Initialising module...");
        let module = Module::new(&engine, wasm_bytes).expect("Failed to create module");
        log::info!("Initialising store...");
        let mut store = Store::new(&engine, ());
        log::info!("Initialising linker...");
        let linker = Linker::<()>::new(&engine);

        // // Allocate some memory for the guest to use
        // let memory =
        //     Memory::new(&mut store, MemoryType::new(1, Some(1))).expect("Should create memory");
        //
        // // Create an export
        // linker
        //     .define("env", "memory", memory)
        //     .expect("Should define env::memory");

        log::info!("Instantiating instance...");
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("Failed to instantiate module");

        let memory = instance
            .get_memory(&store, "memory")
            .expect("Failed to get guest memory");

        // log::info!("Fetching 'mem_write' function...");
        // let mem_write = instance
        //     .get_typed_func::<(u32, u32), ()>(&mut store, "mem_write")
        //     .expect("Failed to get 'mem_write' function");
        //
        // log::info!("Fetching 'mem_read' function...");
        // let _mem_read = instance
        //     .get_typed_func::<u32, u32>(&mut store, "mem_read")
        //     .expect("Failed to get 'mem_read' function");
        //
        // log::info!("Calling 'mem_write' function...");
        // mem_write
        //     .call(&mut store, (0, 0xff))
        //     .expect("Failed to call 'mem_write' function");
        // mem_write
        //     .call(&mut store, (3, 0xaa))
        //     .expect("Failed to call 'mem_write' function");

        log::info!("Fetching 'buffer_ptr' function...");
        let buffer_ptr = instance
            .get_typed_func::<(), i32>(&mut store, "buffer_ptr")
            .expect("Failed to get 'buffer_ptr' function");

        log::info!("Fetching 'update' function...");
        let update_func = instance
            .get_typed_func::<u64, ()>(&mut store, "update")
            .expect("Failed to get 'update' function");

        log::info!("Fetching 'init' function...");
        let init_func = instance
            .get_typed_func::<(), ()>(&mut store, "init")
            .expect("Failed to get 'init' function");

        log::info!("Calling 'init' function...");
        init_func
            .call(&mut store, ())
            .expect("Failed to call 'init' function");

        Self {
            counter: 0,
            frame_times: VecDeque::with_capacity(FPS_WINDOW_SIZE),
            guest_state: GuestState {
                _engine: engine,
                store,
                _linker: linker,
                memory,
                buffer_ptr,
                update: update_func,
            },
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.guest_state
            .update
            .call(&mut self.guest_state.store, self.counter)
            .expect("Failed to call 'update' function");

        self.update_ui(ctx, _frame);
        ctx.request_repaint_after(Duration::from_millis((1000.0 / FPS) as u64));
        //ctx.request_repaint();
        self.counter += 1;

        self.frame_times.push_back(Instant::now());
        if self.frame_times.len() > FPS_WINDOW_SIZE {
            self.frame_times.pop_front();
        }
    }

    fn update_ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            let fps = if self.frame_times.len() >= 2 {
                let oldest = self.frame_times.front().expect("Should be Some");
                let newest = self.frame_times.back().expect("Should be Some");
                let duration = newest.duration_since(*oldest).as_secs_f64();
                if duration > 0.0 {
                    (self.frame_times.len() - 1) as f64 / duration
                } else {
                    0.0
                }
            } else {
                0.0
            };
            ui.label(format!("FPS: {fps:.1}"));
        });

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

                let to_screen = RectTransform::from_to(
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

                // Get the pixel buffer as a byte offset into WASM guest memory
                let pixel_buffer =
                    self.guest_state
                        .buffer_ptr
                        .call(&mut self.guest_state.store, ())
                        .expect("Failed to call 'buffer_ptr'") as usize;

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

                        let mut color_buf = [0u8; 3];
                        let pixel_id = (row * COLS + col) * 3usize;
                        let offset = pixel_buffer + pixel_id;

                        self.guest_state
                            .memory
                            .read(&self.guest_state.store, offset, &mut color_buf)
                            .expect("Should read pixel buffer memory");

                        let color = Color32::from_rgb(color_buf[0], color_buf[1], color_buf[2]);

                        // let color = if row * COLS + col == (counter as usize % (ROWS * COLS)) {
                        //     Color32::from_rgb(200, 200, 200)
                        // } else {
                        //     Color32::from_rgb(20, 20, 20)
                        // };

                        grid_shapes.push(egui::Shape::rect_filled(cell_screen, 1.0, color));
                    }
                }

                painter.extend(grid_shapes);
            });
        });
    }
}

impl eframe::App for DemoApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Self::update(self, ctx, _frame);
    }
}
