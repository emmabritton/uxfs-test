use pixels_graphics_lib::buffer_graphics_lib::prelude::Coord;
use pixels_graphics_lib::graphics_shapes::coord;
use pixels_graphics_lib::graphics_shapes::lerp::inv_flerp;

pub fn to_waveform(data: Vec<f32>, width: usize, height: usize) -> Vec<Coord> {
    pixelize(align(normalize(data), height), width)
}

/// Normalizes a slice of numbers so they map to 0.0..1.0
fn normalize(data: Vec<f32>) -> Vec<f32> {
    if data.is_empty() {
        return vec![];
    }
    let min = *data.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
    let max = *data.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

    data.iter().map(|num| inv_flerp(min, max, *num)).collect()
}

/// Converts data to data * height
pub fn align(data: Vec<f32>, height: usize) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }
    let height = height as f32;
    data.iter()
        .map(|num| (height * num).round() as usize)
        .collect()
}

/// Converts data to pixels
pub fn pixelize(data: Vec<usize>, width: usize) -> Vec<Coord> {
    let chunk_len = data.len()/width;
    data.chunks_exact(chunk_len)
        .map(|nums| nums.iter().sum::<usize>() / chunk_len)
        .enumerate()
        .map(|(i, y)| coord!(i, y))
        .collect()
}
