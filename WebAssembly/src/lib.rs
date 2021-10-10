#![allow(unused_variables)]

use crate::render::model::{SimpleModel, Vertex};
use crate::render::renderer::Renderer;
use wasm_bindgen::prelude::*;

use crate::utils::*;

mod render;
pub mod utils;

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
    pub fn new(tf2mode: bool) -> TeamFourTrees {
        return TeamFourTrees {
            renderer: Renderer::new("canvas"),
        };
    }

    #[wasm_bindgen]
    pub fn run(&mut self, tf2e: bool) {
        self.renderer.load_shader(
            include_str!("shaders/simple.v.glsl"),
            include_str!("shaders/simple.f.glsl"),
        );

        let mut model = SimpleModel::new(
            &self.renderer,
            vec![
                Vertex(-0.5, 0.5, 0.0),
                Vertex(0.5, 0.5, 0.0),
                Vertex(-0.5, -0.5, 0.0),
                Vertex(0.5, -0.5, 0.0),
            ],
            vec![
                0, 1, 2, // Triangle 1
                1, 2, 3, // Triangle 2
            ],
        );

        self.renderer.render(&mut model)
    }
}
