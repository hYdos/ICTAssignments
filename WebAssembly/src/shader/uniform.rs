use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram, WebGlUniformLocation};

pub(crate) struct Uniform {
    location: WebGlUniformLocation,
    name: String,
}

impl Uniform {
    pub fn new(program: &WebGlProgram, name: &str, gl: &WebGlRenderingContext) -> Uniform {
        Uniform {
            location: gl.get_uniform_location(program, name).unwrap(),
            name: name.to_string(),
        }
    }
}