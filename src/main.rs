pub mod framework;
use crate::framework::runtime::RuntimeModule;
use crate::framework::runtime::Application;

fn main() {
    let mut app = framework::base_application::BaseApplication::new();
    
    app.initialize();

    while !app.is_quit() {
        app.tick();
    }

    app.finalize();
}
