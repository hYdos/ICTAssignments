use js_sys::{Float32Array, JsString};
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::utils::*;

mod utils;

#[wasm_bindgen]
pub struct WebRosella {
    vertex_shader: Option<WebGlShader>,
    fragment_shader: Option<WebGlShader>,
    program: Option<WebGlProgram>,
    gl: WebGlRenderingContext,
}

#[wasm_bindgen]
impl WebRosella {
    pub fn new(canvas_id: JsString) -> Result<WebRosella, JsValue> {
        log("Starting WebRosella instance");
        let gl = get_gl(canvas_id.to_rust_string().as_str())?;

        Ok(WebRosella {
            vertex_shader: None,
            fragment_shader: None,
            program: None,
            gl,
        })
    }

    pub fn bind_vertices(&self, vertices: &Float32Array) -> Result<(), JsValue> {
        let buffer = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        self.gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices,
            WebGlRenderingContext::STATIC_DRAW,
        );

        Ok(())
    }

    pub fn render(&self, vertices: &Float32Array) -> Result<(), JsValue> {
        self.gl.use_program(Some(self.program.as_ref().ok_or(JsString::from("Program not present"))?));

        self.bind_vertices(vertices)?;

        self.gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(0);

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.length() / 3) as i32,
        );

        Ok(())
    }

    pub fn load_shader(&self, vertex_shader: JsString, fragment_shader: JsString) -> Result<WebRosella, JsValue> {
        let gl = get_gl("canvas")?; // TODO: fix

        let vert_shader = compile_shader(
            &gl,
            WebGlRenderingContext::VERTEX_SHADER,
            vertex_shader.to_rust_string().as_str(),
        )?;
        let frag_shader = compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            fragment_shader.to_rust_string().as_str(),
        )?;
        let program = link_program(&gl, &vert_shader, &frag_shader);

        Ok(WebRosella {
            vertex_shader: None,
            fragment_shader: None,
            program: Some(program?),
            gl,
        })
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

fn compile_shader(
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

fn link_program(
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
