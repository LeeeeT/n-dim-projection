use js_sys::{Float32Array, Uint32Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rotate_project(
    vertices: &Float32Array,
    planes: &Uint32Array,
    angles: &Float32Array,
    projection: &Float32Array,
    dimension: u32,
    half_w: f32,
    half_h: f32,
    scale: f32,
) -> Float32Array {
    let dim = dimension as usize;
    let mut vertex_data = vertices.to_vec();
    let plane_data = planes.to_vec();
    let angle_data = angles.to_vec();
    let proj_data = projection.to_vec();

    let vertex_count = vertex_data.len() / dim;
    let mut output = vec![0.0_f32; vertex_count * 3];

    let mut rotations = Vec::with_capacity(angle_data.len());
    for (idx, chunk) in plane_data.chunks(2).enumerate() {
        if chunk.len() < 2 {
            continue;
        }
        let angle = angle_data.get(idx).copied().unwrap_or(0.0);
        if angle == 0.0 {
            continue;
        }
        let i = chunk[0] as usize;
        let j = chunk[1] as usize;
        if i >= dim || j >= dim {
            continue;
        }
        rotations.push((i, j, angle.cos(), angle.sin()));
    }

    for &(i, j, c, s) in &rotations {
        for v in 0..vertex_count {
            let base = v * dim;
            let xi = vertex_data[base + i];
            let xj = vertex_data[base + j];
            vertex_data[base + i] = xi * c - xj * s;
            vertex_data[base + j] = xi * s + xj * c;
        }
    }

    for v in 0..vertex_count {
        let base = v * dim;
        let mut proj_x = 0.0_f32;
        let mut proj_y = 0.0_f32;
        for d in 0..dim {
            let weight_x = proj_data[d];
            let weight_y = proj_data[dim + d];
            let coord = vertex_data[base + d];
            proj_x += coord * weight_x;
            proj_y += coord * weight_y;
        }

        let screen_x = half_w + proj_x * scale;
        let screen_y = half_h + proj_y * scale;
        let depth = if dim <= 2 {
            vertex_data.get(base + dim - 1).copied().unwrap_or(0.0)
        } else {
            let tail = &vertex_data[base + 2..base + dim];
            let sum: f32 = tail.iter().sum();
            sum / (tail.len() as f32)
        };

        let base = v * 3;
        output[base] = screen_x;
        output[base + 1] = screen_y;
        output[base + 2] = depth;
    }

    Float32Array::from(output.as_slice())
}
