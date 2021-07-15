use core::f64;
use std::borrow::Borrow;
use approx::relative_eq;
use itertools::Itertools;
// use kiss3d::nalgebra::Vector;
use nalgebra::{Vector3, Point3};
use nalgebra_glm::{DVec3, distance, matrix_comp_mult, comp_min};
use rayon::prelude::*;


use crate::solution::Solution;

pub fn calculate_average_cluster_distance(solution: &Solution) -> f64 {
    let center_points = solution.center();
    let avg = center_points
        .iter()
        .fold(Vector3::<f64>::zeros(),|lhs, rhs| lhs + rhs)
        / solution.len() as f64;
    let delta: Vec<f64> = center_points
        .iter()
        .map(|c| distance(c, &avg)).collect();
    delta.iter().fold(0.0,|lhs, &rhs| lhs + rhs) / (solution.len() as f64)
}


pub fn calculate_cluster_fitness(solution: &Solution) -> Vec<f64> {
    let diagonal = distance(&(DVec3::zeros()), &solution.l);
    let distance = calculate_average_cluster_distance(solution);
    vec![1.0 - (distance / diagonal)]
}


pub fn calculate_center_of_mass(solution: &Solution) -> DVec3 {
    let centers = solution.center();
    centers.iter()
        .zip(solution.m.iter())
        .map(|(&c, &m)| c * m)
        .fold(DVec3::zeros(), |lhs, rhs| lhs + rhs) / solution.m.iter().fold(0., |lhs, &rhs| lhs + rhs)
}


// pub fn calculate_average_distance(solution: &Solution) -> f64 {
//     solution.center().iter().enumerate().map(|(lhs_i,lhs )|
//         solution.center().iter().enumerate()
//             .filter(|(rhs_i, _)| rhs_i.clone() != lhs_i)
//             .map(|(_, rhs)| 
//             distance(lhs, rhs)        
//         ).sum::<f64>()
//     ).sum::<f64>() / (solution.len() * solution.len()) as f64
// }


// pub fn average_distance_fitness(solution: &Solution) -> Vec<f64> {
//     vec![1. - (calculate_average_distance(solution) distance(&DVec3::zeros(), &solution.l))]
// }




pub fn calculate_center_of_mass_fitness(solution: &Solution) -> Vec<f64> {
    let diagonal = distance(&DVec3::zeros(), &solution.l);
    let center_of_mass = calculate_center_of_mass(solution);
    let ideal_center_of_mass = matrix_comp_mult(&DVec3::new(0.5, 0., 0.5), &solution.l);
    let d =  distance(&ideal_center_of_mass, &center_of_mass);
    vec![1.0 - (d / diagonal)]
}

pub fn count_out_of_bounds(solution: &Solution) -> usize {
    solution.p.iter()
        .zip(solution.s.iter())
        .filter(|(&p, &s) | comp_min(&p) < 0. || (p+s) > solution.l.to_owned())
        .count()
}

pub fn calculate_space_fitness(solution: &Solution) -> Vec<f64>{
    vec![1.0 - (count_out_of_bounds(solution) as f64 / solution.len() as f64)]
}

pub fn mo_fitness(solution: &Solution, dominators: Vec<fn(&Solution) -> Vec<f64>>) -> Vec<f64> {
    dominators
        .iter()
        .map(|f| f(solution))
        .flatten()
        .collect()
}


pub fn mo_fitness_and_validity(solution: &Solution, dominators: &Vec<(bool, fn(&Solution) -> Vec<f64>)>) -> (bool, Vec<f64>) {
    let results: Vec<(bool, Vec<f64>)> = dominators
        .iter()
        .map(|(req, f)| (req, f(solution)))
        .map(|(req, f_vec)| (!req.clone() ||  f_vec.iter().all(|&f| relative_eq!(f, 1.0, epsilon = f64::EPSILON)), f_vec))
        .collect();

        // results.iter().enumerate().map(|(i,(&obligated, _))| !obligated || results.iter().map(|r|) )
        (results.iter().all(|r| r.0), results.iter().map(|r| r.1.clone()).flatten().collect())
}


pub fn fitness_distance(fitness: &Vec<f64>) -> f64 {
    fitness.iter().map(|f| f * f).sum::<f64>().sqrt()
}

pub fn average_fitness_distance(fitnesses: &Vec<Vec<f64>>) -> f64{
    fitnesses.iter().map(fitness_distance).sum::<f64>() / fitnesses.len() as f64

}