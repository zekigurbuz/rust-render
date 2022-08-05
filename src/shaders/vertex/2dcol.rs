pub const SHADER: &str = r#"
    attribute vec4 pos;
    uniform mat4 trans;

    void main() {
        gl_Position = trans * pos;    
    }
"#;
