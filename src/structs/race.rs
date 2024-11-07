use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use serde_json::Number;

use super::horse::Horse;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Race {
    pub abandoned: bool,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub track: String,
    pub name: String,
    pub race_number: i32,
    pub distance: i32,
    pub grade: String,
    pub rail: String,
    pub conditions: String, // Potentiall Struct
    pub total_prizemoney: i32,
    pub prizemoney_structure: Vec<i32>,
    pub horse: Vec<Horse>,
}
impl Race {
    pub fn new() -> Self {
        Self {
            abandoned: false,                                   // Default date (1970-01-01)
            date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), // Default date (1970-01-01)
            time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),    // Default time (00:00:00)
            track: String::new(),
            name: String::new(),
            race_number: 0,
            distance: 0,
            grade: String::new(),
            rail: String::new(),
            conditions: String::new(),
            total_prizemoney: 0,
            prizemoney_structure: Vec::new(),
            horse: Vec::new(),
        }
    }

    // Getters
    pub fn get_abandoned(&self) -> bool {
        self.abandoned
    }

    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    pub fn get_time(&self) -> &NaiveTime {
        &self.time
    }

    pub fn get_track(&self) -> &str {
        &self.track
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_distance(&self) -> i32 {
        self.distance
    }

    pub fn get_grade(&self) -> &str {
        &self.grade
    }

    pub fn get_rail(&self) -> &str {
        &self.rail
    }

    pub fn get_conditions(&self) -> &str {
        &self.conditions
    }

    pub fn get_total_prizemoney(&self) -> &i32 {
        &self.total_prizemoney
    }

    pub fn get_prizemoney_structure(&self) -> &Vec<i32> {
        &self.prizemoney_structure
    }

    pub fn get_horses(&self) -> &Vec<Horse> {
        &self.horse
    }

    // Setters
    pub fn set_abandoned(&mut self, bool: bool) {
        self.abandoned = bool;
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        self.date = date;
    }

    pub fn set_time(&mut self, time: NaiveTime) {
        self.time = time;
    }

    pub fn set_track(&mut self, track: String) {
        self.track = track;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_race_number(&mut self, number: i32) {
        self.race_number = number;
    }

    pub fn set_distance(&mut self, distnace: i32) {
        self.distance = distnace;
    }

    pub fn set_grade(&mut self, grade: String) {
        self.grade = grade;
    }

    pub fn set_rail(&mut self, rail: String) {
        self.rail = rail;
    }

    pub fn set_conditions(&mut self, conditions: String) {
        self.conditions = conditions;
    }

    pub fn set_total_prizemoney(&mut self, prizemoney: i32) {
        self.total_prizemoney = prizemoney;
    }

    pub fn set_prizemoney_structure(&mut self, prizemoney: Vec<i32>) {
        self.prizemoney_structure = prizemoney;
    }

    pub fn set_horses(&mut self, horses: Vec<Horse>) {
        self.horse = horses;
    }

    pub fn add_horses(&mut self, horses: Horse) {
        self.horse.push(horses);
    }
}
