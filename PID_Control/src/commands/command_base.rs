pub trait Boxable: CommandBase {
    fn boxed(self) -> Box<dyn CommandBase>;
}

// Implement `Boxable` for all types that implement `CommandBase`
impl<T: CommandBase + 'static> Boxable for T {
    fn boxed(self) -> Box<dyn CommandBase> {
        Box::new(self)
    }
}

pub trait CommandBase {
    fn execute(&mut self);
    fn is_finished(&mut self) -> bool;
    fn stop(&mut self);
    fn initialize(&mut self);
    fn set_name(&mut self, name: &str);
    fn get_name(&mut self) -> String;
}
