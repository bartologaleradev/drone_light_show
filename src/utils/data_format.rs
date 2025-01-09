use crate::model::{point::Point4D, yaw::YawSetPoint};

type SkyCvV0PointFormat = [f32;4];
pub enum DataFormat {
    SkyCvV0([f32; 4]),
    SkyCvV1((f32, [f32;3]))
}
pub struct CvDataFormat {
    points: Vec<DataFormat>,
    version: u8
}

impl CvDataFormat {
    pub fn SkyCvDataFormat(points: &Vec<Point4D>, version: u8) -> Self {
        let destructured_points: Vec<DataFormat>;

        match version {
            0 => destructured_points = CvDataFormat::sky_cv_data_format_v0(&points),
            1 => destructured_points = CvDataFormat::sky_cv_data_format_v1(&points),
            _ => destructured_points = CvDataFormat::sky_cv_data_format_v1(&points)
        }

        Self {points: destructured_points, version}
    }

    /**
     * Special representation to be used when sending a trajectory for rendering to .skyc into the
     * Skybrush Studio server. This representation indicates to the Skybrush Studio server that the
     * points are samples and it is allowed to simplify the trajectory further by eliminating unneed points
     */
    fn sky_cv_data_format_v0(points: &Vec<Point4D>) -> Vec<DataFormat> {
        let mut v0_points: Vec<DataFormat> = Vec::new();
        
        for point in points {
            v0_points.push(DataFormat::SkyCvV0([
                point.get_t(),
                point.get_x(),
                point.get_y(),
                point.get_z()
            ]));
        }

        v0_points
    }

    /** Standard representation for rendering to .skyc into the Skybrush Studio Server */
    fn sky_cv_data_format_v1(points: &Vec<Point4D>) -> Vec<DataFormat> {
        let mut v1_points: Vec<DataFormat> = Vec::new();

        for point in points {
            v1_points.push(DataFormat::SkyCvV1(
                (
                point.get_t(),
                [
                    point.get_x(),
                    point.get_y(),
                    point.get_z()
                ]
            )
        ));
        }

        v1_points
    }
}

pub struct YawListDataFormat {
    setpoints: Vec<YawSetPoint>,
    version: u8
}

impl YawListDataFormat {
    pub fn new(setpoints: &Vec<YawSetPoint>, version: u8) -> Self {
        Self { setpoints: setpoints.clone(), version }
    }
}