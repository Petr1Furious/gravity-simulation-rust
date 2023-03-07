mod gui;
mod utils;
mod simulation;

mod app;
use app::App;

pub fn main() {
    let app = App::new();
    app.start();
}
