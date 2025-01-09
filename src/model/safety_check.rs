use std::collections::HashMap;

use super::plane::Coordinate3D;

/**
 * Safety Check parameters
 */
struct SafetyCheckParams {
    max_altitude: f32,
    max_velocity_xy: f32,
    max_velocity_z: f32,
    min_distance: f32,
    max_velocity_z_up: f32,
    min_nav_altitude: f32,
}

impl SafetyCheckParams {
    pub fn new() -> Self {
        Self {
            max_altitude: 150.0,
            max_velocity_xy: 8.0,
            max_velocity_z: 3.0,
            min_distance: 3.0,
            max_velocity_z_up: 0.0,
            min_nav_altitude: 2.5,
        }
    }

    pub fn new_from(
        max_altitude: f32,
        max_velocity_xy: f32,
        max_velocity_z: f32,
        min_distance: f32,
        max_velocity_z_up: f32,
        min_nav_altitude: f32,
    ) -> Self {
        Self {
            max_altitude,
            max_velocity_xy,
            max_velocity_z,
            min_distance,
            max_velocity_z_up,
            min_nav_altitude,
        }
    }

    pub fn as_dict(&self) -> HashMap<&'static str, f32> {
        HashMap::from([
            ("maxAltitude", self.max_altitude),
            ("maxVelocityXY", self.max_velocity_xy),
            ("maxVelocityZ", self.max_velocity_z),
            ("minDistance", self.min_distance),
            ("minNavAltitude", self.min_nav_altitude),
            ("maxVelocityZUp", self.max_velocity_z_up),
        ])
    }
}

/**
 * Instance of this struct hold the result of a single safety check
 */
struct SafetyCheckResult {
    pub drones_over_max_altitude: Vec<Coordinate3D>,
    pub drones_over_max_velocity_xy: Vec<Coordinate3D>,
    pub drones_over_max_velocity_z: Vec<Coordinate3D>,
    pub drones_below_min_nav_altitude: Vec<Coordinate3D>,
    pub closest_pair: Vec<(Coordinate3D, Coordinate3D)>,
    pub min_distance: Vec<f32>,
    pub min_altitude: Vec<f32>,
    pub all_close_pairs: Vec<(Coordinate3D, Coordinate3D)>,
}

impl SafetyCheckResult {
    /**
     * Creates a new SafetyCheckResult with all its params empty
     */
    pub fn new_empty() -> Self {
        Self {
            drones_over_max_altitude: Vec::new(),
            drones_over_max_velocity_xy: Vec::new(),
            drones_over_max_velocity_z: Vec::new(),
            drones_below_min_nav_altitude: Vec::new(),
            closest_pair: Vec::new(),
            min_distance: Vec::new(),
            min_altitude: Vec::new(),
            all_close_pairs: Vec::new(),
        }
    }

    /**
     * Creates a new SafetyCheckResult element from all its params
     */
    pub fn new_full_from(
        drones_over_max_altitude: Vec<Coordinate3D>,
        drones_over_max_velocity_xy: Vec<Coordinate3D>,
        drones_over_max_velocity_z: Vec<Coordinate3D>,
        drones_below_min_nav_altitude: Vec<Coordinate3D>,
        closest_pair: Vec<(Coordinate3D, Coordinate3D)>,
        min_distance: Vec<f32>,
        min_altitude: Vec<f32>,
        all_close_pairs: Vec<(Coordinate3D, Coordinate3D)>,
    ) -> Self {
        Self {
            drones_over_max_altitude,
            drones_over_max_velocity_xy,
            drones_over_max_velocity_z,
            drones_below_min_nav_altitude,
            closest_pair,
            min_distance,
            min_altitude,
            all_close_pairs,
        }
    }

    /**
     * Create a new SafetyCheckResult element initializing the optionals params as an empty instance of Vec
     */
    pub fn new_partial_from(
        drones_over_max_altitude: Vec<Coordinate3D>,
        drones_over_max_velocity_xy: Vec<Coordinate3D>,
        drones_over_max_velocity_z: Vec<Coordinate3D>,
        drones_below_min_nav_altitude: Vec<Coordinate3D>,
        all_close_pairs: Vec<(Coordinate3D, Coordinate3D)>,
    ) -> Self {
        Self {
            drones_over_max_altitude,
            drones_over_max_velocity_xy,
            drones_over_max_velocity_z,
            drones_below_min_nav_altitude,
            closest_pair: Vec::new(),
            min_distance: Vec::new(),
            min_altitude: Vec::new(),
            all_close_pairs,
        }
    }

    pub fn clear(&mut self) {
        self.drones_over_max_altitude.clear();
        self.drones_over_max_velocity_xy.clear();
        self.drones_over_max_velocity_z.clear();
        self.drones_below_min_nav_altitude.clear();
        self.all_close_pairs.clear();
        self.closest_pair.clear();
        self.min_distance.clear();
        self.min_altitude.clear();
    }
}
