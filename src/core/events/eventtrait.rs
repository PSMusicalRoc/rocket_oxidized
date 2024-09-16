#[derive(Debug, PartialEq, Eq)]
pub enum RocketEventTypes {
    NoneType = 0,
    KeyEvent,
    MouseEvent,
    QuitEvent
}

pub trait RocketEvent {
    
    fn is_handled(&self) -> bool;

    fn handle(&mut self);

    fn get_event_type(&self) -> RocketEventTypes;
}