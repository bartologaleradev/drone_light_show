use std::collections::HashMap;

/**
 * Struct that represent a Time Marker List
 */
struct TimeMarkers {
    /** The hashmap of time markers where keys represent marker names and
     * values represent time in seconds
     */
    markers: HashMap<String, f32>
}

impl TimeMarkers {
    pub fn new() -> Self {
        Self {
            markers: HashMap::new()
        }
    }

    pub fn new_from(markers: HashMap<String, f32>) -> Self {
        Self { markers }
    }

    pub fn append(&mut self, key: String, value: f32) {
        self.markers.insert(key, value);
    }

    pub fn delete(&mut self, key: &String ) {
        self.markers.remove(key);
    }
}