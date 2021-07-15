use std::usize;

use crate::solution::{Solution};
use itertools::Itertools;

use nalgebra_glm::{DVec3, abs, less_than,all};



pub fn count_collisions(solution: &Solution) -> usize {
    let radius: Vec<DVec3> = solution.s.iter().map(|&v| v * 0.5).collect();
    let center = solution.center();
    let mut collision_count: usize = 0;
    for i in 0..solution.len(){
        let max_radius = radius.iter().map(|&v| v + radius[i]).collect_vec();
        let deltas = center
            .iter()
            .enumerate()
            .filter(|(j, _)| i != j.clone())
            .map(|(_, c)| c)
            .map(|&c| abs(&(c- center[i]))).collect_vec();

        let intersection_result = deltas
            .iter()
            .zip(max_radius.iter())
            .enumerate()
            .filter(|(j, _)| i != j.to_owned())
            .map(|(_, (d,r))| (d,r))
            .any(|(d, r)| all(&less_than(d, r)));
        if intersection_result {
            collision_count += 1
        }        
    }
    collision_count
}

pub fn calculate_collision_fitness(solution: &Solution) -> Vec<f64> {
    let coll = count_collisions(solution);
    // println!("collisions: {}", coll);
    
    vec![
        1.0 - (coll as f64 /  (solution.len() as f64))
    ]
}