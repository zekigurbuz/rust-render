use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref STATE: Mutex<Arc<State>> = Mutex::new(Arc::new(State::new()));
}

pub fn update(height: f32, width: f32, time: f32) {
    let dim = height.min(width) * 0.9 * 0.5;
    let mut data = STATE.lock().unwrap();
    *data = Arc::new(State {
        height: height,
        width: width,
        bottom: height * 0.5 - dim,
        top: height * 0.5 + dim,
        left: width * 0.5 - dim,
        right: width * 0.5 + dim,
        time: time,
        ..*data.clone()
    });
}

pub fn state() -> Arc<State> {
    STATE.lock().unwrap().clone()
}

pub struct State {
    pub height: f32,
    pub width: f32,
    pub bottom: f32,
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub time: f32
}

impl State {
    fn new() -> Self {
        Self {
            height: 0.0,
            width: 0.0,
            bottom: 0.0,
            top: 0.0,
            left: 0.0,
            right: 0.0,
            time: 0.0
        }
    }
}
