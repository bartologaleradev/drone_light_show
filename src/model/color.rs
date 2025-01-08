/**
 * Simplest representation of a 3D color in RGB space.
 */
struct Color3D {
    //Red component of the color in the range [0-255]
    r: u8,
    //Green component of the color in the range [0-255]
    g: u8,
    //Blue component of the color in the range [0-255]
    b: u8
}

impl Color3D {
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Self { r, g, b }
    }

    /**
     * Returns a Color4D copy of this color such that the copy is placed
     * at the given number of seconds on the time axis.
     * 
     * Parameters:
     *     t: the number of seconds where the new color should be placed on the time axis
     *     is_fade: flag to specify whether we should fade here from the previous keypoint
     *             (True) or maintain previous color until this moment and change here 
     *              abruptly (False)
     */
    pub fn at_time(&self, t: f32, is_fade: bool) -> Color4D {
        Color4D { t, r: self.r, g: self.g, b: self.b, is_fade }
    }
    /** Convert a Color3D instante to a Blender color array with alpha channel included */
    pub fn as_array(&self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, 1.0]
    }
}

/**
 * Simplest representation of a 4D color in RGB space and time.
 */
#[derive(Clone, Copy)]
pub struct Color4D {
    pub t: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    /* flag to specify wheter we should fade here from the previous keypoint (True) or maintain
    previous color until this moment and change here abruptly (False) */
    is_fade: bool
}

impl Color4D {
    /** Return a Color4D instante to a Blender color vector with alpha channel
     * included and ingnoring the timestamp
     */
    pub fn as_array(&self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, 1.0]
    }
}