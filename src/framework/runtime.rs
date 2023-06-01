pub trait RuntimeModule {
    fn initialize(&mut self) -> i8;

    fn finalize(&self);

    fn tick(&self);
}

pub trait Application : RuntimeModule {
    fn is_quit(&self) -> bool;
}
