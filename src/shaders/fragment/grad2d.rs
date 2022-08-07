pub const SHADER: &str = r#"
    precision mediump float;
    
    uniform float opac;

    varying lowp vec4 vcol;

    void main() {
        gl_FragColor = vec4(vcol.r, vcol.g, vcol.b, vcol.a * opac);
    }
"#;
