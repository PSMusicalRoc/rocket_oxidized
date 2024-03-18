// std library includes
use std::sync::Mutex;

pub struct EngineStatus {
    pub running:bool
}

static ENGINE_STATUS:Mutex<EngineStatus> = Mutex::new(EngineStatus{
    running: true
});

pub fn is_engine_running() -> bool {
    return ENGINE_STATUS.lock().unwrap().running;
}

pub fn engine_stop() {
    let mut m = ENGINE_STATUS.lock().unwrap();
    *m = EngineStatus{
        running: false,
        ..*m
    };
}