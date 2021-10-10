use crate::render::renderer::Renderer;
use std::rc::Rc;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vertex(pub f32, pub f32, pub f32);

pub struct SimpleModel {
    vertex_fmt_size: i32, // Size each vertex takes up individually within the ArrayBuffer
    vertex_buffer: WebGlBuffer, // The models vertices
    index_buffer: WebGlBuffer, // The models indices
    index_count: usize,   // Amount of indices
    gl: Rc<WebGlRenderingContext>,
}

impl SimpleModel {
    pub fn new(renderer: &Renderer, vertices: Vec<Vertex>, indices: Vec<u16>) -> SimpleModel {
        let gl = renderer.gl.clone();
        SimpleModel {
            vertex_fmt_size: 8, // 2 Floats (4 Bytes Each). Total of 8.
            vertex_buffer: create_vertex_buffer(&gl, &vertices),
            index_buffer: create_index_buffer(&gl, &indices),
            index_count: indices.len(),
            gl,
        }
    }

    pub fn get_count(&self) -> usize {
        self.index_count
    }

    /// Binds relevant data for rendering this Model
    pub fn bind(&mut self) {
        bind_buffer(self.gl.as_ref(), &self.vertex_buffer);
        bind_element_buffer(self.gl.as_ref(), &self.index_buffer);
    }
}

fn bind_buffer(gl: &WebGlRenderingContext, buffer: &WebGlBuffer) {
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(buffer));
}

fn bind_element_buffer(gl: &WebGlRenderingContext, buffer: &WebGlBuffer) {
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(buffer));
}

fn u16_array_buffer_data(gl: &WebGlRenderingContext, buffer: &WebGlBuffer, data: &Vec<u16>) {
    bind_element_buffer(gl, buffer);
    unsafe {
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &js_sys::Uint16Array::view(&data),
            WebGlRenderingContext::STATIC_DRAW,
        )
    }
}

fn f_array_buffer_data(gl: &WebGlRenderingContext, buffer: &WebGlBuffer, data: &Vec<f32>) {
    bind_buffer(gl, buffer);
    unsafe {
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &js_sys::Float32Array::view(&data),
            WebGlRenderingContext::STATIC_DRAW,
        )
    }
}

/// Creates a Index Buffer.
fn create_index_buffer(gl: &WebGlRenderingContext, indices: &Vec<u16>) -> WebGlBuffer {
    let buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
    u16_array_buffer_data(gl, &buffer, indices);
    return buffer
}

fn create_vertex_buffer(gl: &WebGlRenderingContext, vertices: &Vec<Vertex>) -> WebGlBuffer {
    let buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();

    let mapped: Vec<f32> = vertices.iter().flat_map(|v| [v.0, v.1, v.2]).collect();
    f_array_buffer_data(gl, &buffer, &mapped);
    return buffer
}
