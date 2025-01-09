/**
 * Simplest representation of a 3D point in space
 */
#[derive(Clone, Copy)]
pub struct Point3D {
    //x coordinate in meters
    x: f32,

    //y coordinate in meters
    y: f32,

    //z coordinate in meters
    z: f32
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }

    /**
     * Return a Point4D copy of this point such that the copy is placed at the given number
     * of seconds on the time axis
     * 
     * Parameter:
     *    t: the number of seconds where the new point should be place on the time axis.
     */
    pub fn at_time(&self, t: f32) -> Point4D {
        Point4D {
            t,
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn as_vector(&self) -> Vec<f32> {
        self.as_vector()
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_z(&self) -> f32 {
        self.z
    }
}

impl CloudMember<Point3D> for Point3D {
    fn to_cloud_point(&self) -> Point3D {
        Point3D{
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}


/**
 * Simpliest representation of a 4D point in space an time
 */
#[derive(Clone, Copy)]
pub struct Point4D {
    //time in seconds
    t: f32,

    //x coordinate in meters
    x: f32,

    //y coordinate in meters
    y: f32,

    //z coordinate in meters
    z: f32
}

impl Point4D {
    pub fn new(t: f32, x: f32, y: f32, z: f32) -> Self {
        Self { t, x, y, z }
    }

    /**
     * Returns a Point3D instance that is at the same coordinate as this instance
     */
    pub fn as_3d(&self) -> Point3D {
        Point3D{
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn as_vector(&self) -> Vec<f32> {
        self.as_vector()
    }

    pub fn get_t(&self) -> f32 {
        self.t
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_z(&self) -> f32 {
        self.z
    }
}
impl CloudMember<Point3D> for Point4D {
    fn to_cloud_point(&self) -> Point3D {
        self.as_3d()
    }   
}
pub trait CloudMember <T>{
    fn to_cloud_point(&self) -> T;
}

/**
 * Simpliest representation of a list/group/cloud of Point3D points
 */
struct PointCloud {
    points: Vec<Point3D>
}

impl PointCloud {
    pub fn new<T: CloudMember<Point3D>>(points: &Vec<T>) -> Self {
        let my_points: Vec<Point3D> = points.iter().map(|point| point.to_cloud_point()).collect();
        Self { points: my_points }
    }

    pub fn get_item(&self, index: usize) -> Point3D {
        if index >= self.points.len(){
            *self.points.last().unwrap()
        }else{
            self.points[index]
        }
    }

    pub fn append<T: CloudMember<Point3D>>(&mut self, point: T) {
        self.points.push(point.to_cloud_point());
    }

    pub fn count(&self) -> usize {
        self.points.len()
    }
}