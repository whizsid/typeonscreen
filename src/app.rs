use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};

use std::sync::Arc;
use std::sync::Mutex;

pub struct App {
    app_indicator: AppIndicator,
    pub toggle_item: gtk::MenuItem,
    pub exit_item: gtk::MenuItem,
    typing: bool,
    window: gtk::Window,
    pub scrolled_window: gtk::ScrolledWindow,
    pub text_view: gtk::TextView
}

impl App {
    pub fn init()->Result<Arc<Mutex<App>>,&'static str>{
        if let Err(_) = gtk::init() {
            return Err("Can not initial GTK.");
        }
        
        // Indicator
        let mut m = gtk::Menu::new();
        let mut ai = AppIndicator::new("Type On Screen", "resources/disable.png");
        ai.set_status(AppIndicatorStatus::Active);
        ai.set_menu(&mut m);
        let toggle_item = gtk::MenuItem::new_with_label("Activate Typing");
        m.append(&toggle_item);
        let exit_item = gtk::MenuItem::new_with_label("Exit TypeOnScreen");
        exit_item.connect_activate(move |_| {
            gtk::main_quit();
        });
        m.append(&exit_item);

        m.show_all();

        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        // Hiding window behaviours from the user
        window.set_decorated(false);
        window.maximize();
        window.set_keep_above(true);
        window.set_deletable(false);
        window.set_skip_pager_hint(true);
        window.set_skip_taskbar_hint(true);
        window.connect_delete_event(|_,_|{
            gtk::Inhibit(true)
        });

        // Transparent
        window.set_widget_name("toplevel");
        let css = "
            #toplevel {
                background-color: rgba(0, 250, 250, 0.7);
            }
            #textview text {
                color: #ff0000;
            }
            #textview, #textview text {
                background: transparent;
                font-size: 40px;
                font-family: 'Source Code Pro';
            }
        ";
        let style_provider = gtk::CssProvider::new();
        style_provider.load_from_data(css.as_bytes()).unwrap();

        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().unwrap(),
            &style_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        let screen = window.get_screen().unwrap();
        let visual = screen.get_rgba_visual().unwrap();
        window.set_visual(Some(&visual));

        // Text Box
        let entry = gtk::TextView::new();
        entry.set_justification(gtk::Justification::Center);
        entry.set_widget_name("textview");
        entry.set_wrap_mode(gtk::WrapMode::Word);

        // Scrollable
        let size = window.get_size();
        let scrolled_window = gtk::ScrolledWindow::new(
            Some(&gtk::Adjustment::new(f64::from(size.0),0.0,0.0,0.0,0.0,0.0)),
            Some(&gtk::Adjustment::new(f64::from(size.1),0.0,0.0,0.0,0.0,0.0))
        );

        // Vertical center
        let container = gtk::Box::new(gtk::Orientation::Vertical,0);
        container.pack_start(&entry, true, false,0);
        scrolled_window.add(&container);
        window.add(&scrolled_window);

        let app = Arc::new( Mutex::new(App{
            app_indicator: ai,
            toggle_item,
            exit_item,
            typing: false,
            window,
            scrolled_window,
            text_view: entry
        }));

        // Toggle typing mode
        {
            let app1 = app.clone();

            app.lock().unwrap().toggle_item.connect_activate(move |_|{
                app1.lock().unwrap().toggle_typing();
            });
        }

        // Scroll to bottom when typing
        {
            let app2 = app.clone();

            let text_buffer = app2.lock().unwrap().text_view.get_buffer().unwrap();

            text_buffer.connect_changed(move |_|{
                let app_brw_result = app2.lock();

                match app_brw_result {
                    Ok(app_brw)=>{
                        let adj = app_brw.scrolled_window.get_vadjustment().unwrap();
                        adj.set_value(adj.get_upper());
                    }
                    Err(_)=>{}
                };
            });
        }

        Ok(app)
    }

    pub fn toggle_typing(&mut self){
        match self.typing {
            true=>{
                self.toggle_item.set_label("Activate Typing");
                self.app_indicator.set_icon("resources/disable.png");

                self.window.hide();
            }
            false=>{
                self.toggle_item.set_label("Deactivate Typing");
                self.app_indicator.set_icon("resources/enable.png");
                self.window.show_all();
                
                // Clear text after open again
                let text_buffer = self.text_view.get_buffer().unwrap();
                let mut bounds = text_buffer.get_bounds();
                text_buffer.delete(&mut bounds.0, &mut bounds.1);
            }
        };
        self.typing = !self.typing;
    }
}