pub trait RuntimeModule {
    fn initialize(&mut self);

    fn finalize(&self);

    fn tick(&self);
}

pub trait Application : RuntimeModule {
    fn is_quit(&self) -> bool;
}
