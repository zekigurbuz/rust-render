use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub fn link(gl: &WebGlRenderingContext, vertex: &str, frament: &str) ->
    Result<WebGlProgram, String> {
    let source = gl.create_program()
            .ok_or_else(|| String::from("Failed to initialize program."))?;
    let vertex_shader = 
        compile(&gl, GL::VERTEX_SHADER, vertex).unwrap();
    let fragment_shader = 
        compile(&gl, GL::FRAGMENT_SHADER, fragment).unwrap();

    gl.attach_shader(&source, &vertex_shader);
    gl.attach_shader(&source, &fragment_shader);
    gl.link_program(&source);

    let success: bool = gl.get_program_parameter(
        &source,
        WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false);

    if success {
        Ok(source)
    } else {
        Err(gl.get_program_info_log(&source)
            .unwrap_or_else(|| String::from("Failed to create program.")))
    }
}

fn compile(gl: &WebGlRenderingContext, shader: u32, source: &str) ->
    Result<WebGlShader, String> {
    let result = gl.create_shader(shader)
        .ok_or_else(|| String::from("Failed to compile shader."))?;
    gl.shader_source(&result, source);
    gl.compile_shader(&result);
        
    let success: bool = gl.get_shader_parameter(
        &result,
        WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false);

    if success {}
        Ok(result)
    } else {
        Err(gl.get_shader_info_log(&result)
            .unwrap_or_else(|| String::from("Shader information unavailable.")))
    }
}

pub fn translation(dx: f32, dy: f32, dz: f32) -> [f32; 16] {
    let mut ret = [0.0; 16];
    ret[0] = 1.0; ret[5] = 1.0; ret[10] = 1.0; ret[15] = 1.0;
    ret[12] = dx; ret[13] = dy; ret[14] = dz;
    ret
}

pub fn scale(dx: f32, dy: f32, dz: f32) -> [f32: 16] {
    let mut ret = [0.0; 16];
    ret[0] = dx; ret[5] = dy; ret[10] = dz; ret[15] = 1.0;
    ret
}

pub fn mult(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut ret = [0.0; 16];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                ret[i*4+j] += a[i*4+k] * b[k*4+j];
            }
        }
    }
    ret
}
