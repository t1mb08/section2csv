use crate::ValueProcessor;
use crate::*;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FastestSectionSummary {
    pub cumulated_distance: i32,
    pub intermediate_time: NaiveTime,
    pub section_time: NaiveTime,
}

impl FastestSectionSummary {
    pub fn new() -> Self {
        Self {
            cumulated_distance: 0,
            intermediate_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(), // You might want to adjust this initialization
            section_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(), // You might want to adjust this initialization
        }
    }
}

impl ValueProcessor for FastestSectionSummary {
    fn get_single_fields(&self, field: &str) -> Option<String> {
        match field {
            CUMULATED_DISTANCE => Some(self.cumulated_distance.to_string()),
            INTERMEDIATE_TIME => Some(self.intermediate_time.to_string()),
            SECTION_TIME => Some(self.section_time.to_string()),
            _ => None,
        }
    }

    fn set_single_fields(&mut self, field: &str, new: String) -> Result<(), &str> {
        match field {
            CUMULATED_DISTANCE => {
                if let Ok(value) = new.parse::<i32>() {
                    self.cumulated_distance = value;
                    Ok(())
                } else {
                    Err("Invalid value for cumulated_distance")
                }
            }
            INTERMEDIATE_TIME => {
                if let Ok(value) = NaiveTime::parse_from_str(&new, "%H:%M:%S%.3f") {
                    self.intermediate_time = value.into();
                    Ok(())
                } else {
                    Err("Invalid value for intermediate_time")
                }
            }
            SECTION_TIME => {
                if let Ok(value) = NaiveTime::parse_from_str(&new, "%H:%M:%S%.3f") {
                    self.section_time = value.into();
                    Ok(())
                } else {
                    Err("Invalid value for section_time")
                }
            }
            _ => Err("Field not found"),
        }
    }
}
