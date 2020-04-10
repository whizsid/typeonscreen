extern crate gtk;
extern crate libappindicator;
extern crate device_query;

mod app;
mod listener;

use app::App;
use listener::listen;

fn main() {
    let app_ref = App::init().unwrap();

    let exit_sender = listen(move ||{
        match app_ref.try_lock() {
            Ok(mut app)=>{
                app.toggle_typing();
            }
            Err(_)=>{}
        };
    });

    gtk::main();

    exit_sender.send(()).unwrap();
}