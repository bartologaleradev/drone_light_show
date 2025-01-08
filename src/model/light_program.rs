use crate::utils::simplify_path;

use super::color::Color4D;

/**
 * Simpliest representation of a causal light program in space and time.
 * 
 * The color between given points is linearly interpolated or kept constant
 * from past according to the is_fade property of each color4D element.
 */
struct LightProgram {
    colors: Vec<Color4D>
}

impl LightProgram {
    pub fn new (colors: Vec<Color4D>) -> Self {
        let mut sorted_colors = colors;
        sorted_colors.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Self {
            colors: sorted_colors
        }
    }

    /**
     * Add a color at the end of the light code
     */
    pub fn append(&mut self, color: Color4D ){
        if self.colors.last().unwrap().t > color.t {
            eprint!("Function LightProgram.append: Can't add an earlier point than the last one");
        } else {
            self.colors.push(color);
        }
    }

    /**
     * Simplifies the light code by removing unnecesary keypoints from it.
     * 
     * Return:
     *     LightProgram instance with the simplified light code
     */
    pub fn simplify(&self) -> LightProgram {
        let new_items = simplify_path(self.colors.clone(), 4.0, simplify_color_distante_func);
        LightProgram::new(new_items)
    }
}

/*Distance function for LightProgram.simplify() */
fn simplify_color_distante_func(keypoints: &Vec<Color4D>, start: &Color4D, end: &Color4D) -> Vec<f32> {
    let timespan = end.t - start.t;
    let timespan2: f32;
    if timespan > 0.0  {timespan2 = timespan} else {timespan2 = 0.5}

    let mut result: Vec<f32> = vec![];

    for point in keypoints {
        let ratio = (point.t - start.t) / timespan2;

        let interp: [f32; 3] = [
            start.r as f32 + ratio * (end.r - start.r) as f32,
            start.g as f32 + ratio * (end.g - start.g) as f32,
            start.b as f32 + ratio * (end.b - start.b) as f32
        ];

        let diff = [
            (interp[0] - point.r as f32).abs(),
            (interp[1] - point.g as f32).abs(),
            (interp[2] - point.b as f32).abs()
        ];

        let mut max: f32 = diff[0];
        for number in 1..3 {
            if diff[number] > max {
                max = diff[number];
            }
        }

        result.push(max);
    }

    result
}