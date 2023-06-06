use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

use super::{
    runtime::{Application, RuntimeModule},
    window::WintWindows,
};

pub struct BaseApplication {
    is_quit: bool,
}

impl BaseApplication {
    pub fn new() -> BaseApplication {
        BaseApplication { is_quit: false }
    }
}

impl Default for BaseApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeModule for BaseApplication {
    fn initialize(&mut self) {
        self.is_quit = false;

        let event_loop = EventLoop::new();
        let window = WintWindows::new(&event_loop);

        let event_handler = move |event: Event<()>,
                                  _: &EventLoopWindowTarget<()>,
                                  control_flow: &mut ControlFlow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    control_flow.set_exit();
                }
                Event::MainEventsCleared => {
                    // Application update code.

                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.
                    window.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in MainEventsCleared, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.
                }
                _ => (),
            }
        };

        event_loop.run(event_handler);
    }

    fn finalize(&self) {}

    fn tick(&self) {
        println!("BaseApplication::tick()");
    }
}

impl Application for BaseApplication {
    fn is_quit(&self) -> bool {
        self.is_quit
    }
}
