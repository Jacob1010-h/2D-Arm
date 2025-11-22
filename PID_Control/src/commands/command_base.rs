pub trait Boxable: CommandBase + 'static {
    fn boxed(self) -> Box<dyn CommandBase>;
}

// Implement `Boxable` for all types that implement `CommandBase`
impl<T: CommandBase + 'static> Boxable for T {
    fn boxed(self) -> Box<dyn CommandBase> {
        Box::new(self)
    }
}

pub trait CommandBase {
    /// Called once the command is first scheduled
    fn initialize(&mut self);

    /// Called on every scheduler tick which the command is active.
    fn execute(&mut self);

    /// Return true when the command is complete.
    fn is_finished(&mut self) -> bool;

    /// Called once when the command ends or is interrupted.
    fn end(&mut self, interrupted: bool);
    
    /// Overrites the name
    fn set_name(&mut self, name: &str);
    
    /// Gets the curr name that is set
    fn name(&mut self) -> String;
    

}
