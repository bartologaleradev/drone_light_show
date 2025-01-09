use std::iter::zip;

use crate::utils::data_format::YawListDataFormat;

/**
 * The simplest representation of a yaw setpoint
 */
#[derive(Clone, Copy)]
pub struct YawSetPoint{
    //The timestamp associated to the yaw setpoint, in seconds
    time: f32,

    //The yaw angle associated to the yaw setopoint, in degrees, CW.
    angle: f32,
}

impl YawSetPoint {
    pub fn new(time: f32, angle: f32) -> Self {
        Self { time, angle }
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }
    /** Increase in delta the angle value */
    pub fn change_angle(&mut self, delta: f32) {
        self.angle += delta;
    }

}

/**
 * Simplest representation of a causal yaw setpoint list in time.
 * 
 * SetPoints are assumed to be linear, i.e. yaw rate is constant between setpoints.
 */
struct YawSetpointList{
    setpoints: Vec<YawSetPoint>
}

impl YawSetpointList {
    pub fn new(setpoints: Vec<YawSetPoint>) -> Self {
        let mut new_points = setpoints;
        new_points.sort_by(|a, b| a.get_time().partial_cmp(&b.get_time()).unwrap());
        Self { setpoints: new_points }
    }

    /** Add a setpoint to the end of the setpoint list */
    pub fn append(&mut self, setpoint: YawSetPoint) {
        if self.setpoints.len() == 0 || self.setpoints.last().unwrap().get_time() >= setpoint.get_time() {
            eprintln!("YawSetPointList.append: New setpoint must come after existing setpoints in time");
        } else {
            self.setpoints.push(setpoint);
        }
    }

    /**
     * Create a Skybrush-compatible instance representation of this instance. 
     */
    pub fn as_skv_dict_format(&self) -> YawListDataFormat {
        YawListDataFormat::new(&self.setpoints, 1)
    }

    /**
     * Translate the yaw setpoints with the given delta angle. The setpoint list will be manipulated in-place
     */
    pub fn shift(&mut self, delta: f32) {
        for index in 0..self.setpoints.len(){
            self.setpoints[index].change_angle(delta); 
        }
    }

    /**
     * Simplify yaw setpoint list in list
     * 
     * Returns:
     *    The simplified yaw setpoint list
     */
    pub fn simplify(&mut self) -> Vec<YawSetPoint>{
        if self.setpoints.len() == 0 { return self.setpoints.clone() }

        //set first yaw in the [0, 360) range and shift entire list accordingly
        let angle = self.setpoints.first().unwrap().get_angle() % 360.0;
        let delta = angle - self.setpoints.first().unwrap().get_angle();

        if delta != 0.0 { self.shift(delta);}

        //remove intermediate points on constant angular speed segments
        let mut new_setpoints: Vec<YawSetPoint> = Vec::new();
        let mut last_angular_speed = -1e12; //todo!() de donde sale este valor?

        for index in 0..self.setpoints.len() {
            let point = self.setpoints[index];
            
            if new_setpoints.len() == 0 {
                new_setpoints.push(point);
            } else {
                let last = new_setpoints.last().unwrap();
                let dt = point.get_time() - last.get_time();

                if dt <= 0.0 {
                    eprintln!("YawSetPointList.simplify: Yaw timestamp are not causal ({} <= {})", point.get_time(), last.get_time());
                    continue;
                }

                //When calculating angular speed, we MUST round timestamps and angles to
                //avoid large numeric errors at division by small numbers.
                let angular_speed = (point.get_angle() - last.get_angle()) / dt;
                if (angular_speed - last_angular_speed).abs() < 1e-6 {
                    let last_index = new_setpoints.len() - 1;
                    new_setpoints[last_index] = point;
                } else {
                    new_setpoints.push(point);
                }

                last_angular_speed = angular_speed;
            }
        }

        self.setpoints = new_setpoints;

        self.setpoints.clone()
    }

    /**
     * Unwraps the yaw angles of the setpoint list "in-place" and ensures that consecutives sampled angles never
     * have a difference of more than 180 degrees.
     */
    pub fn unwrap_yaw(&mut self, threshold: f32, full_cycle: f32) -> Vec<YawSetPoint> {
        let threshold = 180.0;
        let full_cycle = 180.0;

        //We use -2 because len() give us the length of the vector and if we want to iter over it
        //we have to put the limit in length -1, and how we are going to take the index element and
        //the next one in each iteration, -2 is necesarry in order of not create an outbound error
        for index in 0..self.setpoints.len() - 2 {
            let prev = self.setpoints[index];
            let mut curr = self.setpoints[index + 1]; //The reaseon of the previous -2

            let diff = curr.get_angle() - prev.get_angle();
            
            if diff > threshold || diff < -threshold {
                let num_cycles = -(diff / full_cycle);
                curr.change_angle(num_cycles * full_cycle);
                self.setpoints[index + 1 ] = curr;
            }
        }
        
        self.setpoints.clone()
    }
}

