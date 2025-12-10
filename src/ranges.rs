//! Infinite range types for indexing infinite arrays.


/// Infinite range starting from 1: 1, 2, 3, ...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneToInf;

impl OneToInf {
    pub fn new() -> Self {
        OneToInf
    }
    
    pub fn start(&self) -> usize {
        1
    }
    
    pub fn contains(&self, item: usize) -> bool {
        item >= 1
    }
    
    pub fn index(&self, value: usize) -> usize {
        if value < 1 {
            panic!("{} not in OneToInf", value);
        }
        value - 1
    }
}

impl Default for OneToInf {
    fn default() -> Self {
        OneToInf::new()
    }
}

impl OneToInf {
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        1..
    }
}

impl OneToInf {
    pub fn get(&self, key: usize) -> usize {
        key + 1
    }
}

/// Infinite unit range starting from a given value: start, start+1, start+2, ...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfUnitRange {
    start: usize,
    step: usize,
}

impl InfUnitRange {
    pub fn new(start: usize, step: usize) -> Self {
        InfUnitRange { start, step }
    }
    
    pub fn start(&self) -> usize {
        self.start
    }
    
    pub fn step(&self) -> usize {
        self.step
    }
    
    pub fn contains(&self, item: usize) -> bool {
        if self.step == 1 {
            item >= self.start
        } else {
            (item >= self.start) && ((item - self.start) % self.step == 0)
        }
    }
    
    pub fn index(&self, value: usize) -> usize {
        if !self.contains(value) {
            panic!("{} not in {:?}", value, self);
        }
        (value - self.start) / self.step
    }
}

impl InfUnitRange {
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        let start = self.start;
        let step = self.step;
        (0..).map(move |i| start + i * step)
    }
}

impl InfUnitRange {
    pub fn get(&self, key: usize) -> usize {
        self.start + key * self.step
    }
}

/// Infinite step range: start, start+step, start+2*step, ...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfStepRange {
    start: usize,
    step: usize,
}

impl InfStepRange {
    pub fn new(start: usize, step: usize) -> Self {
        InfStepRange { start, step }
    }
    
    pub fn start(&self) -> usize {
        self.start
    }
    
    pub fn step(&self) -> usize {
        self.step
    }
    
    pub fn contains(&self, item: usize) -> bool {
        let remainder = (item as isize - self.start as isize) % self.step as isize;
        remainder == 0 && item >= self.start
    }
    
    pub fn index(&self, value: usize) -> usize {
        if !self.contains(value) {
            panic!("{} not in {:?}", value, self);
        }
        ((value as isize - self.start as isize) / self.step as isize) as usize
    }
}

impl InfStepRange {
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        let start = self.start;
        let step = self.step;
        (0..).map(move |i| (start as isize + i as isize * step as isize) as usize)
    }
}

impl InfStepRange {
    pub fn get(&self, key: usize) -> usize {
        (self.start as isize + key as isize * self.step as isize) as usize
    }
}

