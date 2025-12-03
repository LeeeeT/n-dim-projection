use js_sys::{Float32Array, Uint32Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ShapeData {
    vertices: Vec<f32>,
    edges: Vec<u32>,
    dimension: u32,
    vertex_count: u32,
}

#[wasm_bindgen]
impl ShapeData {
    #[wasm_bindgen(getter)]
    pub fn dimension(&self) -> u32 {
        self.dimension
    }

    #[wasm_bindgen(getter)]
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn vertices(&self) -> Float32Array {
        Float32Array::from(self.vertices.as_slice())
    }

    pub fn edges(&self) -> Uint32Array {
        Uint32Array::from(self.edges.as_slice())
    }
}

#[wasm_bindgen]
pub fn build_shape(kind: &str, dimension: u32) -> Result<ShapeData, JsValue> {
    let dim = dimension.max(2) as usize;
    let (vertices, edges, vertex_count) = match kind {
        "cube" => generate_n_cube(dim),
        "simplex" => generate_simplex(dim).map_err(|msg| JsValue::from_str(msg))?,
        "orthoplex" => generate_orthoplex(dim),
        other => {
            return Err(JsValue::from_str(&format!("Unsupported shape '{other}'")));
        }
    };

    Ok(ShapeData {
        vertices,
        edges,
        dimension: dim as u32,
        vertex_count: vertex_count as u32,
    })
}

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

fn generate_n_cube(dimension: usize) -> (Vec<f32>, Vec<u32>, usize) {
    let dim = dimension.max(2);
    let vertex_count = 1usize << dim;
    let mut vertices = vec![0.0_f32; vertex_count * dim];
    for i in 0..vertex_count {
        for bit in 0..dim {
            let value = if (i & (1 << bit)) != 0 { 1.0 } else { -1.0 };
            vertices[i * dim + bit] = value;
        }
    }
    let mut edges = Vec::with_capacity(dim * vertex_count);
    for i in 0..vertex_count {
        for bit in 0..dim {
            if (i & (1 << bit)) == 0 {
                edges.push(i as u32);
                edges.push((i | (1 << bit)) as u32);
            }
        }
    }
    (vertices, edges, vertex_count)
}

fn generate_simplex(dimension: usize) -> Result<(Vec<f32>, Vec<u32>, usize), &'static str> {
    let dim = dimension.max(2);
    let count = dim + 1;
    let centroid = 1.0_f32 / count as f32;
    let mut raw_vertices = Vec::with_capacity(count);
    for i in 0..count {
        let mut vec = vec![-centroid; count];
        vec[i] = 1.0 - centroid;
        raw_vertices.push(vec);
    }

    let basis = gram_schmidt(&raw_vertices[..count - 1], dim)?;
    let mut vertices = vec![0.0_f32; count * dim];
    for (idx, raw) in raw_vertices.iter().enumerate() {
        for axis_idx in 0..dim {
            let coord = dot(raw, &basis[axis_idx]);
            vertices[idx * dim + axis_idx] = coord;
        }
    }

    let mut edges = Vec::with_capacity(count * (count - 1));
    for i in 0..count {
        for j in (i + 1)..count {
            edges.push(i as u32);
            edges.push(j as u32);
        }
    }

    Ok((vertices, edges, count))
}

fn generate_orthoplex(dimension: usize) -> (Vec<f32>, Vec<u32>, usize) {
    let dim = dimension.max(2);
    let vertex_count = dim * 2;
    let mut vertices = vec![0.0_f32; vertex_count * dim];
    for axis in 0..dim {
        let pos_idx = axis * 2;
        let neg_idx = axis * 2 + 1;
        for d in 0..dim {
            let value = if d == axis { 1.0 } else { 0.0 };
            vertices[pos_idx * dim + d] = value;
            vertices[neg_idx * dim + d] = -value;
        }
    }
    let mut edges = Vec::with_capacity(vertex_count * (vertex_count - 2));
    for i in 0..vertex_count {
        for j in (i + 1)..vertex_count {
            if (i / 2) != (j / 2) {
                edges.push(i as u32);
                edges.push(j as u32);
            }
        }
    }
    (vertices, edges, vertex_count)
}

fn gram_schmidt(vectors: &[Vec<f32>], target_dim: usize) -> Result<Vec<Vec<f32>>, &'static str> {
    let mut basis: Vec<Vec<f32>> = Vec::with_capacity(target_dim);
    for vector in vectors {
        let mut w = vector.clone();
        for axis in &basis {
            let proj = dot(&w, axis);
            for (wi, ai) in w.iter_mut().zip(axis.iter()) {
                *wi -= proj * ai;
            }
        }
        let norm = w.iter().map(|value| value * value).sum::<f32>().sqrt();
        if norm <= 1e-9 {
            continue;
        }
        for value in w.iter_mut() {
            *value /= norm;
        }
        basis.push(w);
        if basis.len() == target_dim {
            break;
        }
    }
    if basis.len() < target_dim {
        return Err("Unable to construct orthonormal basis for simplex");
    }
    Ok(basis)
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai * bi).sum()
}
