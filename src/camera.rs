use std::collections::HashMap;

use crate::solution::{Solution};
use nalgebra::{Point1, Point3, Vector3, distance};
use nalgebra_glm::{DVec3};

pub fn cartesian_to_spherical(coord: &Vector3<f64>) -> Vector3<f64> {
    let r = distance(&Point3::from(Vector3::<f64>::zeros()), &Point3::<f64>::from(coord.to_owned()));
    if coord[0] == 0.0 && coord[1] == 0.0 {
        return DVec3::new(r, 0., 0.);
    }
    let mut thetha = (coord[0] / coord[1]).atan();
    let phi = (distance(&Point1::<f64>::new(coord.x), &Point1::<f64>::new(coord.y)) / coord.z).atan();
    if (coord.x < 0.0 && coord.y >= 0.0 || thetha == 0.0){
        thetha = std::f64::consts::PI;
    }
    else if(coord.x < 0.0 && coord.y < 0.0 &&  thetha.signum() > 0.0){
        thetha -= std::f64::consts::PI;
    }
    else if(coord.x < 0.0 && coord.y > 0.0 &&  thetha.signum() < 0.0){
        thetha += std::f64::consts::PI;
    }
    DVec3::new(r, thetha, phi)
}



fn count_boxes_in_sight(solution: &Solution, index: usize) -> usize {
    let box_radia : Vec<Vector3<f64>> = solution.s.iter().map(|&c|  c * 0.5).collect();
    let camera_position = solution.p[index] + box_radia[index];
    let orientation = solution.o[index];
    let d: &HashMap<String, String> = match &solution.d[index] {
        None => panic!("Could not load data for camera at index {}", index),
        Some(data) => data
    };
    if ! d.contains_key("view_angle") {
        panic!("Data for camera does not contain viewangle");
    }
    let view_angle_string = &d[&String::from("view_angle")];
    let view_angle = match  view_angle_string.parse::<f64>() {
        Ok(value) => value,
        Err(err) => panic!("Could not parse view_angle variable for camera {}. The failed value was: '{}'", index, view_angle_string)
    };
    // view_angle * 0.5
    let orientation_vector = DVec3::new(0., orientation, 0.);
    let view_angle_vector = DVec3::new(0., view_angle, view_angle);

    let in_sight =  solution.center().iter()
        .map(|c|  c - camera_position)
        .map(|c| cartesian_to_spherical(&c))
        .map(|c| c - orientation_vector)
        .map(|c| c - view_angle_vector)
        .filter(|c| c[1] < 0.0 && c[2] < 0.0)
        .count();
    return in_sight;
}


pub fn camera_fitness(solution: &Solution) -> Vec<f64> {
    let result: Vec<f64> = solution.f.iter()
        .enumerate()
        .filter(|(i, &v)| v == (1 as usize))
        .map(|(i, &v)| count_boxes_in_sight(solution, i))
        .map(|count| count as f64 / ((solution.len() as f64) - 1.))
        .collect();
    vec![1. - (result.iter().sum::<f64>() / result.len() as f64) as f64]
}

