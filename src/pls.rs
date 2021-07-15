


use std::thread::current;

use crate::pareto::clean;
use crate::solution::{self, Solution};
use crate::archive::{Archive};
use itertools::{Itertools, iproduct};
use nalgebra::{DVector, Vector3, Point3};
use nalgebra_glm::DVec3;
use rayon::prelude::*;
use crate::iterators::{FloatIterator};


pub fn d_pls(
    initial_solution: &Solution,
    continue_operator: fn(&Archive) -> bool,
    fitness_operator: fn(&Solution) -> (bool, Vec<f64>),
    pick_operator: fn(&Archive) -> Option<usize>,
    dominate_operator: fn(&Archive, &Vec<f64>, bool) -> bool,
    pareto_front_operator: fn(&Archive) -> Archive,
    clean_operator: fn(&Archive) -> Archive
    
) -> Archive {
    let box_amount = initial_solution.len();
    let space = initial_solution.l;
    let mut archive = Archive::new();
    {
        let (initial_vadility, initial_fitness) = fitness_operator(&initial_solution);
        archive.append_inplace(
            &initial_solution, 
            &initial_fitness,
            initial_vadility, 
            false,
            1.0,
            -1.0
        );
    }
    let mut iteration = 0;
    while continue_operator(&archive) {
        iteration += 1;
        let solution_index_option = pick_operator(&archive);
        if solution_index_option.is_none(){
            break;
        }
        let solution_index = solution_index_option.unwrap();
        let target_solution = archive.solutions[solution_index].clone();
        archive.visited[solution_index] = true;


        let it: Vec<(usize, DVec3, Solution, bool, Vec<f64>)> = iproduct!(0..box_amount, 0..space[0] as usize, 0..space[1] as usize, 0..space[2] as usize)
            .par_bridge()
            .map(|(b, x,y,z)| (b, Vector3::<f64>::new(x.clone() as f64,y.clone() as f64,z.clone() as f64)))
            .map(|(box_index, coord)|{
                let mut current_solution = target_solution.clone();
                current_solution.p[box_index] = coord;
                let (current_valid, current_fitness) = fitness_operator(&current_solution);            
                return (box_index, coord, current_solution, current_valid, current_fitness);
            })
            .filter(|(_,_,_,current_valid, current_fitness)|  !dominate_operator(&archive, &current_fitness, current_valid.clone()))
            .collect();
        for (box_index, coord, current_solution , current_valid, current_fitness) in it.iter() {
            
            let dominated = dominate_operator(&archive, &current_fitness, current_valid.clone());
            if !dominated {
                archive.append_inplace(&current_solution, &current_fitness, current_valid.clone(), false, 1.0, -1.0);
            }
        }
        println!("Archive size before pareto: {}",  archive.len());
        archive = pareto_front_operator(&archive);
        println!("Iteration_count: {}, Archive size: {}, visited: {}, valid: {}", iteration,  archive.len(), archive.visited.iter().filter(|&val| val.to_owned()).count(), archive.valid.iter().filter(|&val| val.to_owned()).count());
        print!("Max fitness");
        for fitness_i in 0..archive.fitnesses[0].len(){
            print!("{}, ", archive.fitnesses.iter().map(|f| f[fitness_i]).fold(0.0, |lhs, rhs| if lhs > rhs { lhs} else{ rhs }));
        }
        print!("\n");
        println!()
    }
    archive.mask_inplace(archive.valid.clone());
    archive = clean_operator(&archive);
    println!("Result size: {}", archive.len());
    return archive;
}



pub fn c_pls(
    initial_solution: &Solution,
    continue_operator: fn(&Archive) -> bool,
    fitness_operator: fn(&Solution) -> (bool, Vec<f64>),
    pick_operator: fn(&Archive) -> Option<usize>,
    dominate_operator: fn(&Archive, &Vec<f64>, bool) -> bool,
    pareto_front_operator: fn(&Archive) -> Archive,
    step_size_operator: fn(f64, f64, usize, usize) -> f64,
    noise_operator: fn(DVec3, f64) -> DVec3,
    clean_operator: fn(&Archive) -> Archive
) -> Archive {
    let box_amount = initial_solution.len();
    let space = initial_solution.l;
    let mut archive = Archive::new();
    {
        let (initial_vadility, initial_fitness) = fitness_operator(&initial_solution);
        archive.append_inplace(
            &initial_solution, 
            &initial_fitness,
            initial_vadility, 
            false,
            1.0,
            0.1
        );
    }
    let mut iteration = 0;
    while continue_operator(&archive) {
        iteration += 1;
        let solution_index_option = pick_operator(&archive);
        if solution_index_option.is_none(){
            break;
        }
        let solution_index = solution_index_option.unwrap();
        let target_solution = archive.solutions[solution_index].clone();
        let current_stepsize = archive.step_size[solution_index];
        // let start_index = archive.len();
        // let mut current_solution_iterations: usize = 0;
        let mut dominated_count: usize = 0;
        archive.visited[solution_index] = true;
        let x_iterator = FloatIterator::new(0.0, space[0] as f64, current_stepsize);
        let y_iterator = FloatIterator::new(0.0, space[1] as f64, current_stepsize);
        let z_iterator = FloatIterator::new(0.0, space[2] as f64, current_stepsize);
        println!("Start iteration {} with stepsize {}", iteration, current_stepsize);

        let it: Vec<(usize, DVec3, Solution, bool, Vec<f64>)> = iproduct!(0..box_amount, x_iterator, y_iterator, z_iterator)
            .map(|(b, x,y,z)| (b, Vector3::<f64>::new(x.clone() as f64,y.clone() as f64,z.clone() as f64)))
            .map(|(box_index, coord)|{
                let mut current_solution = target_solution.clone();
                current_solution.p[box_index] = noise_operator(coord, current_stepsize);
                let (current_valid, current_fitness) = fitness_operator(&current_solution);
                return (box_index, coord, current_solution, current_valid, current_fitness);
            })
            .filter(|(_,_,_,current_valid, current_fitness)|  !dominate_operator(&archive, &current_fitness, current_valid.clone()))
            .collect();

        let iteration_count = ((box_amount as f64) * ((space[0] as f64) / current_stepsize) * ((space[1] as f64) / current_stepsize) * ((space[2] as f64) / current_stepsize)).round() as usize;

        println!("Iterationcount: {}", iteration_count);

        



        for (box_index, coord, current_solution , current_valid, current_fitness)  in it.iter(){
            let dominated = dominate_operator(&archive, &current_fitness, current_valid.clone());
            if !dominated {
                dominated_count += 1;
                archive.append_inplace(&current_solution, &current_fitness, current_valid.clone(), false, -1.0, -1.0);
            }
        } 

        print!("Fitness: ");
        for i  in 0..archive.fitnesses[0].len() {
            print!("{},", archive.fitnesses.iter().map(|x| x[i]).fold(-1., |lhs, rhs| if lhs > rhs { lhs } else{ rhs }));
        }
        print!("\n");
        
        
        let new_stepsize = step_size_operator(archive.success_rate[solution_index], current_stepsize, iteration_count, dominated_count);
        let current_successrate = (dominated_count as f64) / (iteration_count as f64);
        archive.step_size = archive.step_size.iter().map(|&s| if s < 0.0 { new_stepsize } else { s }).collect();
        archive.success_rate = archive.success_rate.iter().map(|&s| if s < 0.0 { current_successrate } else { s }).collect();
        // archive = pareto_front_operator(&archive);
        archive = clean_operator(&archive);
        println!("Iteration_count: {}, Archive size: {}, visited: {}, valid: {}", iteration,  archive.len(), archive.visited.iter().filter(|&val| val.to_owned()).count(), archive.valid.iter().filter(|&val| val.to_owned()).count());
        


        archive.save(&(iteration.to_string() + ".json"));
    }
    
    println!("Result size: {}", archive.len());
    return archive;
}



