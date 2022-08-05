use wasm_bindgen::JsCase;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;
use super::super::utils as utils;

pub struct 2DCol {
    source: WebGlProgram,
    num_vert: usize,
    buffer: WebGlBuffer,
    col: WebGlUniformLocation,
    opac: WebGlUniformLocation,
    trans: WebGlUniformLocation,
}

impl 2DCol {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = utils::link(
            &gl,
            super::super::shaders::vertex::2dcol::SHADER,
            super::super::shaders::fragment::2dcol::SHADER)
                .unwrap();

        let rect: [f32, 12] = [
            0.0, 1.0,
            0.0, 0.0,
            1.0, 1.0,
            1.0, 1.0
            0.0, 0.0,
            1,0, 0,0
        ];
        let rect_ptr = rect.as_ptr() as u32 / 4;

        let buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let rect_js = js_sys::Float32Array::new(&buffer).subarray(
            rect_ptr,
            rect_ptr + rect.len() as u32);
        let rect_buffer = gl.create_buffer()
            .ok_or("Failed to initialize buffer")
            .unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&rect_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &rect_js,
            GL::STATIC_DRAW);

        Self {
            col: gl.get_uniform_location(&program, "uColor").unwrap(),
            opac: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            trans: gl.get_uniform_location(&program, "uTransform").unwrap(),
            num_vert: rect.len(),
            buffer: rect_buffer,
            source: program
        }
    }

    pub fn render(&self, gl: &WebGLRenderingContext,
                  bottom: f32, top: f32, left: f32, right: f32,
                  height: f32, width: f32) {
        gl.use_program(&self.source);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);
        gl.uniform4f(Some(&self.col), 0.0, 0.5, 0.5, 1.0);
        gl.uniform1f(Some(&self.opac), 1.0);

        let translation = utils::translation(
            2.0 * left / width - 1.0,
            2.0 * bottom / height - 1.0,
            0.0);

        let scale = utils::scale(
            2.0 * (right - left) / width,
            2.0 * (top - bottom) / height,
            0.0);

        let transform = utils::mult(scale, translation);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.trans), false, &transform);
        gl.draw_arrays(GL::TRIANGLES, 0, (self.num_verts / 2) as i32);
    }
}
