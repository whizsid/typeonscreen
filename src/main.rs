extern crate gtk;
extern crate libappindicator;

mod app;
use app::App;

fn main() {

    App::init().unwrap();

    gtk::main();
}
