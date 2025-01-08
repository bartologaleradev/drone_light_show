/**
 * Simplifies a sequence of points to a similar sequence with fewer points, using a disntance function
 * and an acceptable error term.
 * 
 * The function uses the Ramer-Douglas-Peucker algoritm for simplifying the line segments.
 * 
 * Parameters:
 *    points: a sequence of points. Each point my be an arbitrary object as long as the distance function
 *           can deal with it appropriately.
 *    eps: the error term; a point is considered redundant with respect to two other points if the point is
 *         closer to the line formed by the two other points than this error term.
 *    distance_func: a function that receives a list of points and two additional points, and returns the 
 *          distance of each point in the list from the line formed by the two additional points.
 * 
 * Returns:
 *    the simplified sequence of points. This will be of the same class as the input sequence. It is assumed
 *    that an instance of the sequence may be constructed from a list of items.
 */
pub fn simplify_path<T: Copy> (points: Vec<T>, eps: f32, distance_func: fn(&Vec<T>, &T, &T) -> Vec<f32>) -> Vec<T> {
    
    let vec:Vec<T> = Vec::new();

    
    if points.len() == 0 {
        return vec
    } else {
        return simplify_line(&points, eps, distance_func)
    }
}

fn simplify_line<T: Copy>(points: &Vec<T>, eps: f32, distance_func: fn(&Vec<T>, &T, &T)-> Vec<f32>) -> Vec<T> {
    let vec: Vec<T> = Vec::new();
    if points.len() == 0 { return vec }

    let start = *points.first().unwrap();
    let end = *points.last().unwrap();
    let dist = distance_func(points, &start, &end);
    let index = get_max_value_position(&dist);
    let dmax = dist[index];
    if dmax <= eps {
        return vec![start, end];
    } else {
        let pre = simplify_line(&points[0..(index + 1)].to_vec(), eps, distance_func);
        let post = simplify_line(&points[index..points.len()].to_vec(), eps, distance_func);
        return [pre, post].concat()
    }
}

fn get_max_value_position(points: &Vec<f32>) -> usize {
    let mut index: usize = 0;
    let mut max: f32 = points[0];

    for item in points {
        if *item > max {
            max = *item;
        }
        index += 1;
    }

    index
}