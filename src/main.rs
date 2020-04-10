extern crate gtk;
extern crate libappindicator;
extern crate device_query;
extern crate confy;

mod app;
mod listener;
mod config;

use app::App;
use listener::listen;
use config::Config;

fn main() {
    let config = Config::load().unwrap();
    
    let app_ref = App::init(config).unwrap();

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