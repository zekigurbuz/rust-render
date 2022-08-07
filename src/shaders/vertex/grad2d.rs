pub const SHADER: &str = r#"
    attribute vec4 pos;
    attribute vec4 col;
    uniform mat4 trans;

    varying lowp vec4 vcol;

    void main() {
        vcol = col;
        gl_Position = trans * pos;    
    }
"#;
