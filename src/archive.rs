

use std::io::Write;

use crate::solution::{Solution};
use serde::{Deserialize, Serialize};
use deflate::deflate_bytes;

#[derive(Serialize, Deserialize)]
pub struct Archive {
    pub solutions: Vec<Solution>,
    pub fitnesses: Vec<Vec<f64>>,
    pub valid: Vec<bool>,
    pub visited: Vec<bool>,
    pub step_size: Vec<f64>,
    pub success_rate: Vec<f64>
}

impl Clone for Archive {
    fn clone(&self) -> Self{
        Archive{
            solutions: self.solutions.clone(),
            fitnesses: self.fitnesses.clone(),
            valid: self.valid.clone(),
            visited: self.visited.clone(),
            step_size: self.step_size.clone(),
            success_rate: self.success_rate.clone()
        }
    }
}





impl Archive {

    pub fn new() -> Self {
        Archive {
            solutions: vec![],
            fitnesses: vec![], 
            valid: vec![],
            visited: vec![],
            step_size: vec![],
            success_rate: vec![]
        }
    }
    pub fn len(&self) -> usize {
        self.solutions.len()
    }
    pub fn append(&self, solution: &Solution, fitness: &Vec<f64>, valid: bool, visited: bool, step_size: f64, success_rate: f64) -> Self{
        let mut new_archive = self.clone();
        new_archive.append_inplace(solution, fitness, valid, visited, step_size, success_rate);
        new_archive
    }
    pub fn append_inplace(&mut self, solution: &Solution, fitness: &Vec<f64>, valid: bool, visited: bool, step_size: f64, success_rate: f64) {
        self.solutions.push(solution.clone());
        self.fitnesses.push(fitness.clone());
        self.valid.push(valid);
        self.visited.push(visited);
        self.step_size.push(step_size);
        self.success_rate.push(success_rate);
    }


    pub fn mask(&self, mask: Vec<bool>) -> Archive {
        let mut new_archive  = self.clone();
        new_archive.mask_inplace(mask);
        new_archive
    }

    pub fn mask_inplace(&mut self, mask: Vec<bool>){
        self.solutions  = self.solutions.iter().zip(mask.iter()).filter(|(s, &m)| m).map(|(s, m)| s.clone()).collect();
        self.fitnesses  = self.fitnesses.iter().zip(mask.iter()).filter(|(f, &m)| m).map(|(f, m)| f.clone()).collect();
        self.valid      = self.valid.iter().zip(mask.iter()).filter(|(&f, &m)| m).map(|(&f, m)| f).collect();
        self.visited    = self.visited.iter().zip(mask.iter()).filter(|(&f, &m)| m).map(|(&f, m)| f).collect();
        self.step_size  = self.step_size.iter().zip(mask.iter()).filter(|(&s, &m)| m).map(|(&s, m)| s).collect();
        self.success_rate  = self.success_rate.iter().zip(mask.iter()).filter(|(&s, &m)| m).map(|(&s, m)| s).collect();
    }

    pub fn save(&self, path: &String) {
        let json_string = serde_json::to_string_pretty(self).unwrap();
    {
        let mut json_file = match std::fs::File::create(String::from("logs/") + path) {
            Err(why) => panic!("Could not create file: {}", why),
            Ok(file) => file
        };



        match json_file.write_all(deflate_bytes(json_string.as_bytes()).as_slice()) {
            Err(why) => panic!("Could not write to file: {}", why),
            Ok(_) => println!("Solution saved to solution.json")
        };
    }



    }

}

#[cfg(test)]
mod test{
    #[test]
    fn test_length(){
        let archive = super::Archive::new();
        assert_eq!(archive.len(), 0);

    }


}









