/**
 * Struct representing a single camera in the scene.
 * 
 * This struct is a simplified representation of the properties that a typical
 * camera may have in a real 3D software like Blender (aplicacion de graficos)
 */
struct Camera {
    name: String, //The name of the camera
    position: [f32; 3], //The position of the camera in 3D space
    orientation: [f32; 4], //The orientation of the camera using Blender quaternions
}

impl Camera {
    pub fn new(name: String, position: [f32; 3], orientation: [f32; 4]) -> Self {
        Self { name: name, position: position, orientation: orientation }
    }
}