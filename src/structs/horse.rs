use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Horse {
    pub result: i32,
    pub name: String,
    pub trainer: String,
    pub jockey: String,
    pub barrier: i32,
    pub weight: f64,
    pub margin: f64,
    pub time: String,
    pub progression: String,
    pub steward_comment: String,
    pub starting_price: f64,
}

impl Horse {
    pub fn new() -> Self {
        Self {
            result: 0,
            name: String::new(),
            trainer: String::new(),
            jockey: String::new(),
            barrier: 0,
            weight: 0.0,
            margin: 0.0,
            time: String::new(),
            progression: String::new(),
            steward_comment: String::new(),
            starting_price: 0.0,
        }
    }

    pub fn get_result(&self) -> i32 {
        self.result
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_trainer(&self) -> &str {
        &self.trainer
    }

    pub fn get_jockey(&self) -> &str {
        &self.jockey
    }

    pub fn get_barrier(&self) -> i32 {
        self.barrier
    }

    pub fn get_margin(&self) -> f64 {
        self.margin
    }

    pub fn get_time(&self) -> &str {
        &self.time
    }

    pub fn get_progression(&self) -> &String {
        &self.progression
    }

    pub fn get_steward_comment(&self) -> &str {
        &self.steward_comment
    }

    pub fn get_starting_price(&self) -> f64 {
        self.starting_price
    }

    pub fn set_result(&mut self, new: i32) {
        self.result = new;
    }

    pub fn set_name(&mut self, new: String) {
        self.name = new;
    }

    pub fn set_trainer(&mut self, new: String) {
        self.trainer = new;
    }

    pub fn set_jockey(&mut self, new: String) {
        self.jockey = new;
    }

    pub fn set_barrier(&mut self, new: i32) {
        self.barrier = new;
    }

    pub fn set_margin(&mut self, new: f64) {
        self.margin = new;
    }

    pub fn set_time(&mut self, new: String) {
        self.time = new;
    }

    pub fn set_progression(&mut self, new: String) {
        self.progression = new;
    }

    pub fn set_steward_comment(&mut self, new: String) {
        self.steward_comment = new;
    }

    pub fn set_starting_price(&mut self, new: f64) {
        self.starting_price = new;
    }
}
