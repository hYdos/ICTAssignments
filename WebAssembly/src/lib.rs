#![allow(unused_variables)]

/**
    A quick web gl version of a vulkan render engine i wrote in kotlin.
    probably wont be a like what so ever but ill just use the name
    because im not that creative :)
*/

use js_sys::{Array, Boolean, Float32Array, JsString};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::utils::{*};

mod utils;

#[wasm_bindgen]
struct WebRosella {
    vertex_shader: Option<WebGlShader>,
    fragment_shader: Option<WebGlShader>,
    program: Option<WebGlProgram>,
    gl: WebGlRenderingContext,
}

#[wasm_bindgen]
impl WebRosella {
    pub fn new(canvas_id: JsString) -> WebRosella {
        let canvas_name = canvas_id.to_rust_string();
        log("Starting WebRosella instance");
        let gl: WebGlRenderingContext = get_gl(canvas_name.as_str()).unwrap();

        let vert_shader = compile_shader(
            &gl,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
    "#,
        );
        let frag_shader = compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
        void main() {
            gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
        }
    "#,
        );
        let program = link_program(&gl, &vert_shader.unwrap(), &frag_shader.unwrap());

        return WebRosella {
            vertex_shader: Option::None,
            fragment_shader: Option::None,
            program: Option::Some(program.unwrap()),
            gl,
        };
    }

    pub fn bind_vertices(&self, vertices: &Float32Array) {
        let gl = &self.gl;

        let buffer = gl.create_buffer().ok_or("failed to create buffer");
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer.unwrap()));

        unsafe {
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertices,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
    }

    pub fn render(&self, vertices: &Float32Array) {
        let gl: &WebGlRenderingContext = &self.gl;
        gl.use_program(Some(&self.program.as_ref().unwrap()));

        self.bind_vertices(vertices);

        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.length() / 3) as i32,
        );
    }

    pub fn add_vertices(self) {}

    pub fn load_shader(self) {}
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
        .unwrap_or(false) {
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
        .unwrap_or(false) {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}