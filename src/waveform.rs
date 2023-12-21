use pixels_graphics_lib::buffer_graphics_lib::prelude::Coord;
use pixels_graphics_lib::graphics_shapes::coord;
use pixels_graphics_lib::graphics_shapes::lerp::inv_flerp;

pub fn to_waveform(data: Vec<f32>, width: usize, height: usize) -> Vec<(Coord, Coord)> {
    if data.is_empty() {
        return vec![];
    }
    let chunk_len = data.len() / width;
    let height = height as f32 / 2.0;
    let (total_min, total_max) = min_max(&data);

    //total_min is lowest value in sample, total_max is highest

    //chunk_len is number of values per pixel

    //code:
    //for each chunk
    //convert to min and max
    //convert to percent of total min and max
    //convert to pixel value
    //convert to coord

    data.chunks_exact(chunk_len)
        .map(min_max)
        .map(|(min, max)| (inv_flerp(total_min, total_max, min), inv_flerp(total_min, total_max, max)))
        .map(|(min, max)| ((min * height) as usize, (max * height) as usize))
        .enumerate()
        .map(|(x, (min, max))| (coord!(x,min), coord!(x,max)))
        .collect()
}

fn min_max(nums: &[f32]) -> (f32,f32) {
    let min = *nums.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
    let max = *nums.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    (min, max)
}