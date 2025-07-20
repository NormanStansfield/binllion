// モジュールファイルの読み込み
mod app;
mod bin_data;
mod bin_data_index;
mod constants;
mod interfaces;
mod key_event_handler;
mod mini_buf;
mod notice_provider;
mod tui;
mod write_mode;

use app::App;
use interfaces::AppTrait as _;

// main 関数s
fn main() {
    let mut app = App::new();

    app.run();
}
