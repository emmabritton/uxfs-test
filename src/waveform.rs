use pixels_graphics_lib::prelude::PixelFont::Standard4x4;
use pixels_graphics_lib::prelude::*;

#[derive(Debug, Clone)]
pub struct Waveform {
    pub duration: f32,
    points: Vec<(Coord, Coord)>,
    error: bool,
    center: TextPos,
}

impl Waveform {
    pub fn new(data: Vec<f32>, sample_rate: usize, width: usize, height: usize) -> Self {
        let duration = data.len() as f32 / sample_rate as f32;
        let (error, points) = if data.iter().any(|v| v.is_nan() || v.is_infinite()) {
            (true, vec![])
        } else {
            (false, to_waveform(data, width, height))
        };
        Waveform {
            duration,
            points,
            error,
            center: TextPos::Px((width / 2) as isize, (height / 2) as isize),
        }
    }

    pub fn render_line(&self, graphics: &mut Graphics, color: Color) {
        if self.error {
            graphics.draw_text(
                "ERROR RENDERING WAVEFORM",
                self.center,
                (color, Standard4x4, Positioning::Center),
            )
        }
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
