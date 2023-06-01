use super::runtime::{Application, RuntimeModule};

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
    fn initialize(&mut self) -> i8 {
        self.is_quit = false;

        0
    }

    fn finalize(&self) {
        
    }

    fn tick(&self) {
        
    }
}

impl Application for BaseApplication {
    fn is_quit(&self) -> bool {
        self.is_quit
    }
}