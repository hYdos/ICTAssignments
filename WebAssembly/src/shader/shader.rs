use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};
use crate::shader::uniform::{Uniform};

pub(crate) struct Shader {
    program: Option<WebGlProgram>,
    uniforms: Vec<Uniform>
}

impl Shader {
    pub(crate) fn bind(&self, gl: &WebGlRenderingContext) {
        if let Some(program) = &self.program {
            gl.use_program(Some(&program));
        } else {
            gl.use_program(None);
        }
    }
}

impl Shader {

    pub fn new(gl: &WebGlRenderingContext, vertex_shader: &str, fragment_shader: &str, uniforms: Vec<Uniform>) -> Shader {
        let vert_shader = compile_shader(
            gl,
            WebGlRenderingContext::VERTEX_SHADER,
            vertex_shader,
        );
        let frag_shader = compile_shader(
            gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            fragment_shader,
        );
        let program = link_program(gl, &vert_shader.unwrap(), &frag_shader.unwrap());

        Shader {
            program: Option::Some(program.unwrap()),
            uniforms
        }
    }
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}