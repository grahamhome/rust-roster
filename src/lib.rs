use std::collections::{BTreeMap, BTreeSet};

pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> School {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0.entry(grade).or_default().insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.0.get(&grade).unwrap_or(&BTreeSet::new()).iter().cloned().collect()
    }
}
