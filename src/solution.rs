use std::{collections::HashMap, io::{Read, Write}, usize};
use nalgebra::{Vector3};
use serde::{Deserialize, Serialize};
use rand::Rng;
use nalgebra_glm::{DVec3};

use crate::camera;



pub mod traits {
    use std::collections::HashMap;
    use nalgebra_glm::{DVec3};
    pub trait Solution {
        fn p<'a>(&'a self) -> &'a Vec<DVec3>;
        fn s<'a>(&'a self) -> &'a Vec<DVec3>;
        fn m<'a>(&'a self) -> &'a Vec<f64>;
        fn f<'a>(&'a self) -> &'a Vec<usize>;
        fn o<'a>(&'a self) -> &'a Vec<f64>;
        fn l<'a>(&'a self) -> &'a DVec3;
        fn d<'a>(&'a self) -> &'a Vec<Option<HashMap<String, String>>>;
    }


    


}
#[derive(Serialize, Deserialize)]
pub struct Solution {
    pub p: Vec<DVec3>,
    pub s: Vec<DVec3>,
    pub m: Vec<f64>,
    pub f: Vec<usize>,
    pub o: Vec<f64>,
    pub l: DVec3,
    pub d: Vec<Option<HashMap<String, String>>>
}

impl traits::Solution for Solution{
    fn p<'a>(&'a self) -> &'a Vec<DVec3> {
        &self.p
    }

    fn s<'a>(&'a self) -> &'a Vec<DVec3> {
        &self.s
    }

    fn m<'a>(&'a self) -> &'a Vec<f64> {
        &self.m
    }

    fn f<'a>(&'a self) -> &'a Vec<usize> {
        &self.f
    }

    fn o<'a>(&'a self) -> &'a Vec<f64> {
        &self.o
    }

    fn l<'a>(&'a self) -> &'a DVec3 {
        &self.l
    }

    fn d<'a>(&'a self) -> &'a Vec<Option<HashMap<String, String>>> {
        &self.d
    }
}


impl Solution {
    pub fn new() -> Self {
        Solution{
            p: vec![],
            s: vec![],
            m: vec![],
            f: vec![],
            o: vec![],
            l: DVec3::new(0., 0., 0.),
            d: vec![]
        }
    }
    pub fn random(module_amount: usize, size_limit: (f64, f64, f64), space_limit: (f64, f64, f64), max_mass: f64) -> Solution {

        let mut rng = rand::thread_rng();
        let positions: Vec<DVec3> = (0..module_amount).map(|_v| DVec3::new(rng.gen(), rng.gen(), rng.gen())).collect();
        let sizes: Vec<DVec3> = (0..module_amount).map(|_v| DVec3::new(rng.gen(), rng.gen(), rng.gen())).collect();
        let masses: Vec<f64> = (0..module_amount).map(|_v| rng.gen::<f64>() * max_mass).collect();
        let functionality: Vec<usize> = (0..module_amount).map(|_v| 0).collect();
        let orientation: Vec<f64> = (0..module_amount).map(|_v| 0.).collect();


        Solution {
            p: positions,
            s: sizes,
            m: masses,
            f: functionality,
            o: orientation,
            l: DVec3::new(space_limit.0, space_limit.1, space_limit.2),
            d: (0..module_amount).map(|_v| None).collect()
        }
    }


    pub fn random_cameras(&mut self, camera_amount: usize, view_angle: f64) {
        let mut rng = rand::thread_rng();
        for _i in 0..camera_amount {
            let camera_i = rng.gen_range(0..self.len());
            println!("{}", camera_i);
            let mut hm = HashMap::<String, String>::new();
            hm.insert(String::from("view_angle"), view_angle.to_string());
            // hm["view_angle"] = ;
            self.f[camera_i] = 1;
            self.d[camera_i] = Some(hm);
        }

        




    }
    pub fn remove(&self, index: usize) -> Self {
        let mut solution = self.clone();
        solution.remove_inplace(index);
        return solution;
    }

    pub fn remove_inplace(&mut self, index: usize) {
        self.p.remove(index);
        self.s.remove(index);
        self.m.remove(index);
        self.f.remove(index);
        self.o.remove(index);
        self.d.remove(index);
    }
    


    pub fn len(&self) -> usize {
        self.p.len()
        
    }
    pub fn center(&self) -> Vec<Vector3<f64>> {
        self.p.iter().zip(self.s.iter())
            .map(|(p, s)|  p + (s * 0.5))
            .collect()
        
    }
    pub fn save_solution(&self, file: &str) {
        let json_string = serde_json::to_string_pretty(self).unwrap();
        {
            let mut json_file = match std::fs::File::create(file) {
                Err(why) => panic!("Could not create file: {}", why),
                Ok(file) => file
            };
    
    
            match json_file.write_all(json_string.as_bytes()) {
                Err(why) => panic!("Could not write to file: {}", why),
                Ok(_) => println!("Solution saved to solution.json")
            };
        }


    }




}
pub fn load_solution(file: &str) -> Option<Solution> {
    
    let json_string  = std::fs::read_to_string(file).expect("File not found");
    let jsonObject: Solution =  serde_json::from_str(json_string.as_str()).expect("Json not parsable");
    Some(jsonObject)
     


}




impl Clone for Solution{
    fn clone(&self) -> Self {
        Solution {
            p: self.p.clone(),
            s: self.s.clone(),
            m: self.m.clone(),
            f: self.f.clone(),
            o: self.o.clone(),
            l: self.l.clone(),
            d: self.d.clone()
        }
    }
}



// struct FleightweigtSolution<'a> {
//     parent: &'a Solution,
//     p: Option<Vec<DVec3>>,
//     s: Option<Vec<DVec3>>,
//     m: Option<Vec<f64>>,
//     f: Option<Vec<usize>>,
//     o: Option<Vec<f64>>,
//     l: Option<DVec3>,
//     d: Option<Vec<Option<HashMap<String, String>>>>
// }





// impl<'b> traits::Solution for FleightweigtSolution<'b> {
//     fn p<'a>(&'a self) -> &'a Vec<DVec3> {
//         match &self.p {
//             Some(p) => p,
//             None => (& self.parent.p)
//         }
//     }

//     fn s<'a>(&'a self) -> &'a Vec<DVec3> {
//         match &self.s {
//             Some(s) => s,
//             None => (& self.parent.s)
//         }
//     }

//     fn m<'a>(&'a self) -> &'a Vec<f64> {
//         match &self.m {
//             Some(s) => m,
//             None => (& self.parent.m)
//         }
//     }

//     fn f<'a>(&'a self) -> &'a Vec<usize> {
//         match &self.f {
//             Some(s) => f,
//             None => (& self.parent.f)
//         }
//     }

//     fn o<'a>(&'a self) -> &'a Vec<f64> {
//         match &self.o {
//             Some(s) => (o),
//             None => (& self.parent.o)
//         }
//     }

//     fn l<'a>(&'a self) -> &'a DVec3 {
//         match &self.l {
//             Some(s) => l,
//             None => (& self.parent.l)
//         }
//     }

//     fn d<'a>(&'a self) -> &'a Vec<Option<HashMap<String, String>>> {
//         match &self.d {
//             Some(s) => d,
//             None => (& self.parent.d)
//         }
//     }
// }







#[cfg(test)]
mod test{
    // use kiss3d::nalgebra::{SimdBool, };
    use nalgebra_glm::{DVec3};



    
    #[test]
    fn test_len(){
        let new = super::Solution::new();
        assert_eq!(new.len(), 0);
    }

    #[test]
    fn test_random(){
        let sol = super::Solution::random(10, (1., 1., 1.), (10., 10., 10.), 1.);
        assert_eq!(sol.len(), 10);
        assert!( sol.p.iter().all(|v| DVec3::zeros() <= v.to_owned()));
        assert!( sol.p.iter().zip(sol.s.iter()).all(|(p, s)| DVec3::new(10., 10., 10.) >= p + s));
        assert!( sol.s.iter().all(|s| DVec3::new(1., 1., 1.) >=  s.to_owned()));
        assert!( sol.s.iter().all(|s| DVec3::zeros() <=  s.to_owned()));

    }


}