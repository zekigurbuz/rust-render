pub const SHADER: &str = r#"
    precision mediump float;
    
    uniform vec4 col;
    uniform float opac;

    void main() {
        gl_FragColor = vec4(col.r, col.g, col.b, col.a * opac);
    }
"#;
