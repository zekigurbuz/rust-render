extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod state;
mod setup;
mod shaders;
mod sources;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    gl: WebGlRenderingContext,
    col2d_program: sources::Col2D,
    grad2d_program: sources::Grad2D,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = setup::initialize().unwrap();
        Self {
            col2d_program: sources::Col2D::new(&gl),
            grad2d_program: sources::Grad2D::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        state::update(_height, _width, _time);
        Ok(())
    }

    pub fn render(&self) {
        let state = state::state();
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.col2d_program.render(&self.gl, state.bottom, state.top, state.left, state.right, state.height, state.width);
        self.grad2d_program.render(&self.gl, state.bottom + 20., state.top - 20., state.left + 20., state.right - 20., state.height, state.width);
    }
}
