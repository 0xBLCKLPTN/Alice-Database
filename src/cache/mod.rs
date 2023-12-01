use crate::configurator::*;

#[derive(Debug, Clone)]
pub struct Cache<T> {
    pub name: String,
    pub data: Vec<T>,
    pub capacity: Option<usize>,
}

impl<T: Clone> Cache<T> {
    pub fn new(name: &str) -> Cache<T> {
        let mut data = Vec::new();
        Cache { name: name.to_string(), data, capacity: None}
    }
    pub fn new_with_capacity(name: &str, capacity: usize) -> Cache<T> {
        let mut data: Vec<T> = Vec::with_capacity(capacity);
        Cache { name: name.to_string(), data, capacity: Some(capacity)}
    }
    pub fn push_back(&mut self, item: T) {
        self.data.push(item);
    }
    pub fn get_first(&mut self) -> T {
        let elem: T = self.data[0].clone();
        self.data.pop();
        return elem;
    }
    pub fn get_elems(&self) -> usize {
        return self.data.len();
    }
    pub fn clear(&mut self) {
        self.data.clear()
    }
}

pub fn setup_cache<T: Clone>(name: &str ,config: Cache_config) -> Cache<T> {
    return match config.capacity {
        Some(cap) => Cache::new_with_capacity(name, cap as usize),
        _ => Cache::new(name),
    };
}