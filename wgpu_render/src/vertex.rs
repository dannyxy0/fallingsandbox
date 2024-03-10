use bytemuck::{Pod, Zeroable};
use falling_sand::simulation::ElementMatrix;
use std::mem::size_of;
use wgpu::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl<'a> Vertex {
    pub fn desc() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub fn vertices_from_matrix(matrix: &ElementMatrix) -> Vec<Vertex> {
    // clip_space ranges from -1 to 1, so we need to divide 2 by xy
    let spacing_x = 2.0 / matrix.ncols() as f32;
    let spacing_y = 2.0 / matrix.nrows() as f32;

    matrix
        .data
        .as_slice()
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| {
            cell.as_ref().map(|element| {
                let c = element.properties.color();
                let color = [c.red, c.green, c.blue, c.alpha].map(|x| x as f32 / 255.0);

                // origin is the top left position of the rectangle we need to draw
                // position from 0 to 1
                let mut origin_x = (i % matrix.ncols()) as f32 / matrix.ncols() as f32;
                let mut origin_y = (i / matrix.nrows()) as f32 / matrix.nrows() as f32;
                // position from -1 to 1
                origin_x = origin_x * 2.0 - 1.0;
                origin_y = 1.0 - origin_y * 2.0;

                [
                    Vertex {
                        position: [origin_x, origin_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x, origin_y - spacing_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x + spacing_x, origin_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x + spacing_x, origin_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x, origin_y - spacing_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x + spacing_x, origin_y - spacing_y],
                        color,
                    },
                ]
            })
        })
        .flatten()
        .collect()
}
