mod utils;
mod app;
mod item;

use app::App;


fn main() {
    let mut app = App::new();
    app.run();
}
