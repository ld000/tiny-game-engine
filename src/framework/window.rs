use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

pub struct WintWindows {
    pub window: winit::window::Window,
}

impl Default for WintWindows {
    fn default() -> Self {
        Self::new(&EventLoop::new())
    }
}

impl WintWindows {
    pub fn new(event_loop: &EventLoop<()>) -> WintWindows {
        let mut builder = WindowBuilder::new();

        let logical_size = LogicalSize::new(800, 600);
        builder = builder.with_inner_size(logical_size);

        let window = builder.build(event_loop).unwrap();

        WintWindows { window }
    }
}
