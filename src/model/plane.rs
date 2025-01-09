use std::fmt::Error;

/**
 * Struct that specifies a plane in 3D space.
 * 
 * The plane is defined as the set of points that satisfy the equation of the
 * form a*x + b*y + c*z = d, where (a, b, c) is the normal vector of the plane
 * and d is the offset
 */

 pub type Coordinate3D = [f32; 3];

struct Plane {
    //The normal vector of the plane
    normal: Coordinate3D,
    //The offset parameter of the plane equation
    offset: f32
}


impl Plane {
    /**
    * Constructs a plane from three points.
    * 
    * Args:
    *    p: the first point
    *    q: the second point
    *    r: the third point
    */
    pub fn from_points(p: Coordinate3D, q: Coordinate3D, r: Coordinate3D) -> Result<Self, &'static str> {
        let pq = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
        let pr = [r[0] - p[0], r[1] - p[1], r[2] - p[2]];
        let normal = [
            pq[1] * pr[2] - pq[2] * pr[1],
            pq[0] * pr[2] - pq[2] * pr[0],
            pq[0] * pr[1] - pq[1] * pr[0]
        ];

        if normal[0] == 0.0 && normal[1] == 0.0 && normal[2] == 0.0 {
            return Err("The given points are collinear")
        }

        Ok(Self { normal, offset: 0.0 })
    } 

    /**
     * Construct a plane from its normal vector and an arbitrary point on the plane.
     * 
     * Args:
     *    normal: the normal vector.
     *    point: the point on the plane
     */
    pub fn form_normal_and_point(normal: Coordinate3D, point: Coordinate3D) -> Self {
        let offset = point[0] * normal[0] + point[1] * normal[1] + point[2] * normal[2];
        
        Self {
            normal,
            offset
        }
    }

    /**
     * Returns whether the given point is on the front side of the plane.
     * Points that lie exactly on the plane are considered to be on the front side.
     */
    pub fn is_front(&self, point: Coordinate3D) -> bool {
        let x = self.normal[0] * point[0] + self.normal[1] * point[1] + self.normal[2] * point[2];

        x >= self.offset
    }
}