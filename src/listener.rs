use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::mpsc::channel;
use gtk::timeout_add;
use std::time::Duration;
use std::thread;

pub fn listen<F>( mut cb: F) -> std::sync::mpsc::Sender<()>
where
    F: FnMut() -> () + 'static,
{
    let (sender, receiver) = channel::<()>();

    let (exit_sender, exit_receiver) = channel::<()>();

    thread::spawn( move ||{

        let device_state = DeviceState::new();
        let mut started = false;

        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            
            match exit_receiver.recv_timeout(Duration::from_millis(1)) {
                Ok(_)=>{
                    break;
                }
                Err(_)=>{

                }
            };

            match keys.last() {
                Some(key)=>{
                    match key {
                        Keycode::LControl=>{
                            started = true;
                        }
                        Keycode::RControl=>{
                            started = true;
                        }
                        Keycode::F6=>{
                            if started {
                                sender.send(()).unwrap();
                            }
                            started = false;
                        }
                        _=>{
                            started = false;
                        }
                    }
                }

                None=>{}
            };
        }
    });

    timeout_add(100, move ||{
        match receiver.recv_timeout(Duration::from_millis(10)) {
            Ok(())=>{
                cb();
            }
            Err(_)=>{}
        }

        gtk::prelude::Continue(true)
    });
        
    exit_sender
}