use approx::relative_eq;
use itertools::Itertools;

use crate::archive::Archive;
use nalgebra::{DVector, distance};
use crate::iterators::{FloatIterator};
use rayon::prelude::*;
use crate::fitness::{average_fitness_distance, fitness_distance};
pub fn dominates(archive: &Archive, new_fitness: &Vec<f64>, valid: bool) -> bool {
    let population_size = archive.len();
    // println!("get_pareto_front population_size {}", population_size);
    
    // archive.fitness.
    archive.fitnesses.iter()
        .zip(archive.valid.iter())
        .zip(archive.visited.iter())
        .filter(|(_, &v)| valid == v || v)
        // .filter(|(_, &v)| v || amount_valid <= 0 )
        .any(|((rhs, _), _)|
            (!new_fitness.iter().zip(rhs.iter()).any(|(&lhs, &rhs)| lhs > rhs))
            && !new_fitness.iter().zip(rhs.iter()).all(|(&l,&r)| l >= r || relative_eq!(l, r, epsilon = f64::EPSILON))
        )

    



    // for (i, rhs) in archive.fitnesses.iter().enumerate() {
    //     let all = new_fitness
    //         .iter()
    //         .zip(rhs)
    //         .any(|(&lhs, &rhs)| lhs > rhs);

    //     if all {
    //         // println!("All and any");
    //         pareto_front[i] = false;
    //     }
    // }
    // pareto_front.iter().any(|&v| v)
}



pub fn get_pareto_front(archive: &Archive) -> Archive {
    let pareto_front: Vec<bool> = archive.fitnesses.iter()
        .zip(archive.valid.iter())
        .zip(archive.visited.iter())
        .map(|((rhs, &rhs_v), &visited )|
        (!visited || visited == rhs_v)
        && !archive.fitnesses.iter().zip(archive.valid.iter()).any(|((lhs), &lhs_v)|
            ((lhs_v == rhs_v) || lhs_v)
            && lhs.iter().zip(rhs.iter()).all(|(&l,&r)| l >= r || relative_eq!(l, r, epsilon = f64::EPSILON))
            && lhs.iter().zip(rhs.iter()).any(|(&l,&r)| l > r)
            
        )
    ).collect();
    archive.mask(pareto_front)
}



pub fn clean(archive: &Archive) -> Archive {
    let avg_fitness_distance  = average_fitness_distance(&archive.fitnesses);
    let pareto_front: Vec<bool> = archive.fitnesses.iter()
        .zip(archive.valid.iter())
        .zip(archive.visited.iter())
        .map(|((rhs, &rhs_v), &visited )|
        (!visited || visited == rhs_v)
        &&  fitness_distance(rhs ) > (avg_fitness_distance * 0.33)
    ).collect();
    archive.mask(pareto_front)
}