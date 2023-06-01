pub mod framework;
use crate::framework::runtime::RuntimeModule;
use crate::framework::runtime::Application;

fn main() {
    let mut app = framework::base_application::BaseApplication::new();
    
    if app.initialize() != 0 {
        println!("App initializ≥e failed!");
        return;
    }

    while !app.is_quit() {
        app.tick();
    }

    app.finalize();
}
