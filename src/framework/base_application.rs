use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

use super::{
    runtime::{Application, RuntimeModule},
    window::WintWindows,
    render::State,
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

// #[async_trait]
impl RuntimeModule for BaseApplication {
    fn initialize(&mut self) {
        self.is_quit = false;

        let event_loop = EventLoop::new();
        let window = WintWindows::new(&event_loop);
        let mut state = futures_lite::future::block_on(State::new(&window.window));

        let event_handler = move |event: Event<()>,
                                  _: &EventLoopWindowTarget<()>,
                                  control_flow: &mut ControlFlow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id: _,
                } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("WindowEvent::CloseRequested");
                            control_flow.set_exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
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
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        // Recreate the swap_chain if lost
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
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
