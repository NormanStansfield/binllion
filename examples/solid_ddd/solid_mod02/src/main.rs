pub mod interfaces;
use interfaces::AppTrait as _;

mod driver;

mod car;

mod app;
use app::App;

mod ui;

 fn main() {
    let app = App::new();
    app.run();
}
