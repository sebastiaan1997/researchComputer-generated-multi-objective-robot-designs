use std::io::Write;

use fitness::mo_fitness_and_validity;
use itertools::Itertools;
use rand::Rng;

pub mod solution;
pub mod archive;
pub mod fitness;
pub mod collision;
pub mod pls;
pub mod pareto;
pub mod iterators;
pub mod camera;

pub mod log;

// use rand::distributions::{Normal, Distribution};
use nalgebra_glm::{DVec3, distance, matrix_comp_mult, comp_min};
use rayon::iter::ParallelIterator;

use crate::solution::load_solution;
// pub mod vecmath;
fn apply_noise(coord: DVec3, step_size: f64) -> DVec3 {
    let mut rng = rand::thread_rng();
    // let normal = Normal::new(2.0, 3.0);

    // let factor = 0.5;
    let noise = DVec3::new(rng.gen_range(-step_size..step_size), rng.gen_range(-step_size..step_size), rng.gen_range(-step_size..step_size)) * step_size;
    // let v: f64 = normal.sample(&mut rand::thread_rng());
    coord.component_mul(&noise)
}


fn no_noise(coord: DVec3, _: f64) -> DVec3 {
    coord
}



fn update_stepsize(parent_successrate: f64, current_stepsize: f64, total_iterations: usize, dominated: usize,) -> f64{
    let lambda = 1;
    let learning_rate = 0.8;

    // let lambda = 1 as usize;
    let d = 1.+ (5. * (lambda as f64));
    let success_rate = (dominated as f64) / (total_iterations as f64);
    let psucc = (1.-learning_rate) * (success_rate)  + (learning_rate * success_rate);

    
    println!("Successrate in update:{}", success_rate);
    let value = current_stepsize * (1./d  * ((parent_successrate - psucc) / (1.0 -psucc))).exp();
    if value > 1.0 {
        1.0
    }
    else {
        value
    }
    
    
    
}

static mut COUNT: usize = 0;
fn should_continiue(archive: &archive::Archive) -> bool{
    unsafe {
        COUNT += 1;
        if COUNT >= 10_000{
            return false;
        }
    }
    !archive.visited.iter().all(|&v| v)
    
    
    // println!("Solution size: {}, Visited size: {}", archive.visited.len(),  archive.visited.len());

    
}
fn pick_operator(archive: &archive::Archive) -> Option<usize> {
    let mut rng = rand::thread_rng();

    // let amount_valid = archive.valid.iter().filter(|&v| v.clone()).count();
    // let amount_visited = archive.visited.iter().filter(|&v| v.clone()).count();
    // let max_fitness= archive.success_rate.iter()
    //     .zip(archive.visited.iter())
    //     .zip(archive.valid.iter())
    //     .enumerate()
    //     .filter(|(_,(_,&valid))| valid || amount_valid <= 0)
    //     .filter(|(_, (v, _))| !v.1.clone())
    //     .map(|(i,(v, _))| (i,v))
    //     .max_by(|lhs, rhs| lhs.0.clone().partial_cmp(rhs.0).unwrap()).unwrap();

    // // let median = sorted_fitnesses[(sorted_fitnesses.len() / 2)].1;
    // Some(max_fitness.0)
        




    
    let non_visited: Vec<usize> = archive.visited.iter()
        .zip(archive.valid.iter())
        .enumerate()
        .filter(|(i, (&visited, _))| !visited)
        // .filter(|(i, (_, &valid))| valid || amount_valid == 0)
        .map(|(i, _)| i)
        .collect();
    if non_visited.len() <= 0 {
        return None;
    }
    let index = rng.gen_range(0..non_visited.len());
    Some(non_visited[index])
}

fn fitness_operator(solution: &solution::Solution) -> (bool, Vec<f64>){
    let fitness_functions: Vec<(bool, fn(&solution::Solution) -> Vec<f64>)> = vec![
        (false, fitness::calculate_cluster_fitness),
        (false, fitness::calculate_center_of_mass_fitness),
        (true, collision::calculate_collision_fitness),
        (true, fitness::calculate_space_fitness),
        (false, camera::camera_fitness),
        // (false, fitness::average_distance_fitness)
    ];
    mo_fitness_and_validity(solution, &fitness_functions)
}


pub fn main(){
    let load_solution_from_file = true;
    let save_inital_solution = false;
    let continious = true ;

    let mut solution = if load_solution_from_file {
        load_solution("initial_solution.json").unwrap()
    }
    else {
        solution::Solution::random(10, (1.,1.,1.), (10., 10., 10.), 1.)
    };
    if save_inital_solution {
        solution.save_solution("initial_solution.json"); 
    }
    
    solution.random_cameras(2, 0.5 * std::f64::consts::PI);
    println!("Starting d-pls");
    
    
    let result = if continious {
        pls::c_pls(
            &solution,
            should_continiue,
            fitness_operator,
            pick_operator,
            pareto::dominates,
            pareto::get_pareto_front,
            update_stepsize,
            no_noise,
            pareto::get_pareto_front
        )
    }
    else{
        pls::d_pls(
            &solution,
            should_continiue,
            fitness_operator,
            pick_operator,
            pareto::dominates,
            pareto::get_pareto_front,
            pareto::get_pareto_front
        )
    };

    let json_string = serde_json::to_string_pretty(&result).unwrap();
    {
        let mut json_file = match std::fs::File::create("solution.json") {
            Err(why) => panic!("Could not create file: {}", why),
            Ok(file) => file
        };

        
        match json_file.write_all(json_string.as_bytes()) {
            Err(why) => panic!("Could not write to file: {}", why),
            Ok(_) => println!("Solution saved to solution.json")
        };
    }

    

    






}