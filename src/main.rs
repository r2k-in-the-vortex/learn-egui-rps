#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

const GAMEWIDTH: f32 = 640.0;
const GAMEHEIGHT: f32 = 480.0;
const ICONSIZE: f32 = 25.0;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([GAMEWIDTH, GAMEHEIGHT])
            .with_min_inner_size([GAMEWIDTH, GAMEHEIGHT])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    let mut game: Box<web_rock_paper_scissors::RockPaperScissors<'_>> = Box::new(
        web_rock_paper_scissors::RockPaperScissors::atsize(GAMEWIDTH, GAMEHEIGHT, ICONSIZE),
    );
    web_rock_paper_scissors::RockPaperScissors::game_restart(&mut game);

    eframe::run_native(
        "Rock Paper Scissors",
        native_options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            game
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    let mut game: Box<web_rock_paper_scissors::RockPaperScissors<'_>> = Box::new(
        web_rock_paper_scissors::RockPaperScissors::atsize(GAMEWIDTH, GAMEHEIGHT, ICONSIZE),
    );
    web_rock_paper_scissors::RockPaperScissors::game_restart(&mut game);

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    // This gives us image support:
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    game
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
