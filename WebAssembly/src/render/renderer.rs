use std::rc::Rc;
use crate::render::model::SimpleModel;
use crate::render::shader::{compile_shader, link_program};
use crate::utils::{get_gl, log};
use web_sys::WebGlProgram;
use web_sys::{WebGlRenderingContext, WebGlShader};

pub struct Renderer {
    vertex_shader: Option<WebGlShader>,
    fragment_shader: Option<WebGlShader>,
    program: Option<WebGlProgram>,
    pub gl: Rc<WebGlRenderingContext>,
}

impl Renderer {
    pub fn new(canvas_id: &str) -> Renderer {
        log("Its Rendering Time.");
        let gl: WebGlRenderingContext = get_gl(canvas_id).unwrap();

        return Renderer {
            vertex_shader: Option::None,
            fragment_shader: Option::None,
            program: Option::None,
            gl: Rc::new(gl),
        };
    }

    pub fn render(&mut self, model: &mut SimpleModel) {
        let gl = &self.gl;

        if let Some(program) = &self.program {
            gl.use_program(Some(&program));
        } else {
            gl.use_program(None);
        }

        model.bind();

        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.clear_color(0.0, 1.0, 0.0, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            (model.get_count()) as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    pub fn load_shader(&mut self, vertex_shader: &str, fragment_shader: &str) {
        let vert_shader = compile_shader(
            &self.gl,
            WebGlRenderingContext::VERTEX_SHADER,
            vertex_shader,
        );
        let frag_shader = compile_shader(
            &self.gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            fragment_shader,
        );
        let program = link_program(&self.gl, &vert_shader.unwrap(), &frag_shader.unwrap());

        self.program = Option::Some(program.unwrap())
    }
}
