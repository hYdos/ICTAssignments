#![allow(unused_variables)]

use crate::model::model::{SimpleModel, Vertex};
use crate::render::renderer::Renderer;
use crate::shader::shader::Shader;
use wasm_bindgen::prelude::*;
use crate::shader::uniform::Uniform;
use web_sys::HtmlCanvasElement;
use crate::utils::*;

mod render;
mod model;
pub mod utils;
pub mod gl;
mod shader;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
struct TeamFourTrees {
    renderer: Renderer,
}

#[wasm_bindgen]
impl TeamFourTrees {
    #[wasm_bindgen(constructor)]
    pub fn new(tf2mode: bool, canvas: HtmlCanvasElement) -> TeamFourTrees {
        return TeamFourTrees {
            renderer: Renderer::new(canvas),
        };
    }

    #[wasm_bindgen]
    pub fn run(&mut self, tf2e: bool) {
        let simple_2d_shader = Shader::new(
            self.renderer.gl.as_ref(),
            include_str!("shaders/simple2D.v.glsl"),
            include_str!("shaders/simple.f.glsl"),
            vec![
                // UnbakedUniform
            ],
        );

        let mut model = SimpleModel::new(
            &self.renderer,
            vec![
                Vertex(-1.0, -1.0, 1.0),
                Vertex(1.0, -1.0, 1.0),
                Vertex(1.0, 1.0, 1.0),
                Vertex(-1.0, 1.0, 1.0),
                Vertex(-1.0, -1.0, -1.0),
                Vertex(-1.0, 1.0, -1.0),
                Vertex(1.0, 1.0, -1.0),
                Vertex(1.0, -1.0, -1.0),
            ],
            vec![
                // Front face
                0, 1, 2,
                3, 4, 5,
                // Back face
                6, 7, 5,
                3, 2, 6,
                // Top face
                4, 7, 1,
                0, 7, 6,
                // Bottom face
                2, 1, 4,
                0, 3, 5,
            ],
            simple_2d_shader,
        );

        self.renderer.render(&mut model)
    }
}
