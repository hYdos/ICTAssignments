use std::rc::Rc;
use crate::model::model::SimpleModel;
use crate::shader::shader::Shader;
use crate::utils::{get_gl, log};
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};

pub struct Renderer {
    pub gl: Rc<WebGlRenderingContext>,
}

impl Renderer {
    pub fn new(canvas_id: &str) -> Renderer {
        log("Its Rendering Time.");
        let gl: WebGlRenderingContext = get_gl(canvas_id).unwrap();

        return Renderer {
            gl: Rc::new(gl),
        };
    }

    pub fn render(&mut self, model: &mut SimpleModel) {
        let gl = &self.gl;

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

}
