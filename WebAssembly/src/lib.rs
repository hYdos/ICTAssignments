#![allow(unused_variables)]
/**
    A quick web gl version of a vulkan render engine i wrote in kotlin.
    probably wont be a like what so ever but ill just use the name
    because im not that creative :)
*/

use js_sys::{Boolean, JsString};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::utils::{*};

mod utils;

#[wasm_bindgen]
struct WebRosella {
    is_ready: bool,
    vertex_shader: Option<WebGlShader>,
    fragment_shader: Option<WebGlShader>,
    program: Option<WebGlProgram>,
}

#[wasm_bindgen]
impl WebRosella {
    pub fn new(canvas_id: JsString) -> WebRosella {
        log(canvas_id.to_rust_string().as_str());
        let gl: WebGlRenderingContext = get_gl(canvas_id.to_rust_string().as_str()).unwrap();

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
        gl.use_program(Some(&program.unwrap()));

        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

        let buffer = gl.create_buffer().ok_or("failed to create buffer");
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer.unwrap()));

        // after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() / 3) as i32,
        );

        return WebRosella {
            is_ready: false,
            vertex_shader: Option::None,
            fragment_shader: Option::None,
            program: Option::None
        }
    }

    pub fn render(self) {
    }

    pub fn add_vertices(self) {
    }

    pub fn load_shader(self) {
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