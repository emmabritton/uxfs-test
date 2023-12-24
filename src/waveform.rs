use pixels_graphics_lib::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Waveform {
    pub duration: f32,
    points: Vec<(Coord, Coord)>,
}

impl Waveform {
    pub fn new(data: Vec<f32>, sample_rate: usize, width: usize, height: usize) -> Self {
        let duration = data.len() as f32 / sample_rate as f32;
        let points = to_waveform(data, width, height);
        Waveform { duration, points }
    }

    pub fn render_line(&self, graphics: &mut Graphics, color: Color) {
        for (top, bottom) in &self.points {
            graphics.draw_line(top, bottom, color);
        }
    }
}

fn to_waveform(data: Vec<f32>, width: usize, height: usize) -> Vec<(Coord, Coord)> {
    if data.is_empty() {
        return vec![];
    }
    let chunk_len = data.len() / width;
    let (total_min, total_max) = min_max(&data);
    if chunk_len == 0 {
        return vec![];
    }

    data.chunks_exact(chunk_len)
        .map(min_max)
        .map(|(min, max)| {
            (
                inv_flerp(total_min, total_max, min),
                inv_flerp(total_min, total_max, max),
            )
        })
        .map(|(min, max)| {
            (
                (min * height as f32) as usize,
                (max * height as f32) as usize,
            )
        })
        .enumerate()
        .map(|(x, (min, max))| (coord!(x, min), coord!(x, max)))
        .collect()
}

fn min_max(nums: &[f32]) -> (f32, f32) {
    let min = *nums.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
    let max = *nums.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    (min, max)
}
