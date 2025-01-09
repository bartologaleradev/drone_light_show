use crate::utils::data_format::CvDataFormat;

use super::point::{Point3D, Point4D};

/**
 * Simplest representation of a causal trajectory in space and time.
 * 
 * Positions between given Point4D elements are assumed to be linearly
 * interpolated both in space and time
 */
 struct Trajectory {
    points: Vec<Point4D>
}

impl Trajectory {
    pub fn new(points: Vec<Point4D>) -> Self {
        let mut sorted_points = points;
        sorted_points.sort_by(|a, b| a.get_t().partial_cmp(&b.get_t()).unwrap());

        Self { points: sorted_points }
    }

    /** Add a point to the end of the trajectory */
    pub fn append(&mut self, point: Point4D) {
        if self.points.last().take().unwrap().get_t() >= point.get_t() {
            eprint!("Trajector.append: New point must come after existing trajectory in time");
        } else {
            self.points.push(point);
        }
    }

    /** Create a data format element */
    pub fn as_skyc_format_data(&self, version: u8) -> CvDataFormat {
        CvDataFormat::SkyCvDataFormat(&self.points, version)
    }

    /** Return de duration of the trajectory in seconds */
    pub fn duration(&self) -> f32 {
        if self.points.len() < 2 { return 0.0 }

        self.points.last().unwrap().get_t() - self.points.first().unwrap().get_t()
    }

    /** Shift all points of the trajectory in-place
     * 
     * Parameters:
     *     offset: the spacial offset to add to each point in the trajectory
     */
    pub fn shift_in_place(&self, offset: Point3D) -> Self {
        let mut offset_points: Vec<Point4D> = Vec::new();

        for point in self.points.clone() {
            offset_points.push(Point4D::new(
                point.get_t(),
                point.get_x() + offset.get_x(),
                point.get_y() + offset.get_y(),
                point.get_z() + offset.get_z() 
            ));
        }

        Self { points: offset_points }
    }

    /** 
     * Shift all timestamp of the trajctory in-place
     * 
     * Parameters:
     *     delta: the time delta to add to the timestamp of each point in the trajectory
     */
    pub fn shift_time_in_place(&self, delta: f32) -> Self {
        let mut delta_points: Vec<Point4D> = Vec::new();

        for point in self.points.clone() {
            delta_points.push(Point4D::new(
                point.get_t() + delta,
                point.get_x(),
                point.get_y(),
                point.get_z()
            ));
        }
        Self { points: delta_points }
    }

    pub fn simplify_in_place(&self) -> Self {
        if self.points.len() == 0 { return Self {points: self.points.clone()} }

        let mut new_points: Vec<Point4D> = Vec::new();
        let first_point = self.points.first().unwrap();
        let mut fake_last_point = Point4D::new(
            first_point.get_t() - 1.0,
            first_point.get_x() - 1.0,
            first_point.get_y() - 1.0,
            first_point.get_z() - 1.0);
        
        let mut keep_next = false;
        for point in self.points.clone() {
            let prev_is_same = (
                fake_last_point.get_t() == point.get_t() &&
                fake_last_point.get_x() == point.get_x() &&
                fake_last_point.get_y() == point.get_y() &&
                fake_last_point.get_z() == point.get_z()
            );

            if keep_next || !prev_is_same {
                new_points.push(point);
            } else {
                let last_index = new_points.len() - 1;
                new_points[last_index] = point;
            }

            keep_next = !prev_is_same;
            fake_last_point = point
        }

        Self { points: new_points }
    }
}