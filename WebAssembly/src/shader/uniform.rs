use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram, WebGlUniformLocation};

pub struct Uniform {
    location: WebGlUniformLocation,
    name: String,
}

pub struct UnbakedUniform {
    name: String,
    location: WebGlUniformLocation,
}

impl Uniform {
    pub fn new(program: &WebGlProgram, name: &str, gl: &WebGlRenderingContext) -> Uniform {
        Uniform {
            location: gl.get_uniform_location(program, name).unwrap(),
            name: name.to_string(),
        }
    }
}