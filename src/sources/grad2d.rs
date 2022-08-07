use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;
use super::super::utils as utils;

pub struct Grad2D {
    source: WebGlProgram,
    col_buffer: WebGlBuffer,
    num_ind: i32,
    buffer: WebGlBuffer,
    opac: WebGlUniformLocation,
    trans: WebGlUniformLocation,
}

impl Grad2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = utils::link(
            &gl,
            super::super::shaders::vertex::grad2d::SHADER,
            super::super::shaders::fragment::grad2d::SHADER)
                .unwrap();

        let rect: [f32; 8] = [
            0.0, 1.0,
            0.0, 0.0,
            1.0, 1.0,
            1.0, 0.0
        ];

        let indices: [u16; 6] = [
            0, 1, 2, 2, 1, 3
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

        let indices_ptr = indices.as_ptr() as u32 / 2;

        let indices_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let indices_js = js_sys::Uint16Array::new(&indices_buffer).subarray(
            indices_ptr,
            indices_ptr + indices.len() as u32);
        let index_buffer = gl.create_buffer()
            .ok_or("Failed to initialize buffer")
            .unwrap();

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices_js,
            GL::STATIC_DRAW);
        

        Self {
            col_buffer: gl.create_buffer().ok_or("Failed to initialize buffer").unwrap(),
            opac: gl.get_uniform_location(&program, "opac").unwrap(),
            trans: gl.get_uniform_location(&program, "trans").unwrap(),
            num_ind: indices_js.length() as i32,
            buffer: rect_buffer,
            source: program
        }
    }

    pub fn render(&self, gl: &WebGlRenderingContext,
                  bottom: f32, top: f32, left: f32, right: f32,
                  height: f32, width: f32) {
        gl.use_program(Some(&self.source));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.col_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        let cols: [f32; 16] = [
            1.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 1.1,
            1.1, 1.1, 1.1, 1.1
        ];

        let cols_memory = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let cols_ptr = cols.as_ptr() as u32 / 4;
        let cols_js = js_sys::Float32Array::new(&cols_memory)
            .subarray(cols_ptr, cols_ptr + cols.len() as u32);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &cols_js, GL::DYNAMIC_DRAW);

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
        gl.draw_elements_with_i32(GL::TRIANGLES, self.num_ind, GL::UNSIGNED_SHORT, 0);
    }
}
