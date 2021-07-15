use std::{collections::btree_map::Range, ops::Add};


pub struct FloatIterator {
    current: f64,
    end: f64, 
    step: f64
    
}

impl FloatIterator{
    pub fn new(begin: f64, end: f64, step: f64) -> Self {
        FloatIterator {
            current: begin,
            end: end,
            step: step
        }
    }
}

impl Iterator for FloatIterator {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let v = self.current.clone();
            self.current += self.step;
            return Some(v);
        }
        None
    }
}

impl Clone for FloatIterator{
    fn clone(&self) -> Self {
        FloatIterator{
            current: self.current,
            end: self.end,
            step: self.step 
        }
    }
}