use super::eventtrait::RocketEvent;

type RocketEventHandler = fn(&mut dyn RocketEvent);

pub struct RocketEventSystem {
    handlers: Vec<RocketEventHandler>
}

impl RocketEventSystem {

    pub fn new() -> Self {
        RocketEventSystem {
            handlers: Vec::new()
        }
    }

    pub fn add_handler(&mut self, handler: RocketEventHandler) {
        self.handlers.push(handler);
    }

    pub fn remove_handler(&mut self, handler: RocketEventHandler) {
        let mut v: Vec<usize> = vec![];
        let mut offset: usize = 0;
        for (i, h) in self.handlers.iter().enumerate() {
            if *h == handler {
                v.push(i - offset);
                offset += 1;
            }
        }
        for i in v.iter() {
            self.handlers.remove(*i);
        }
    }

    pub fn handle_event(&self, event: &mut dyn RocketEvent) {
        for handler in self.handlers.iter() {
            (*handler)(event);
        }
    }

}
