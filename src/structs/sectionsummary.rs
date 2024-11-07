use crate::ValueProcessor;
use crate::*;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SectionSummary {
    pub cumulated_distance: i32,
    pub margin_decimal: f64,
    pub real_distance: f64,
    pub rank: i32,
    pub intermediate_time: NaiveTime,
    pub section_time: NaiveTime,
    pub avg_speed: f64,
    pub top_speed: f64,
    pub avg_stride_freq: f64,
    pub average_stride_length: f64,
    pub avg_distance_rail: f64,
}

impl SectionSummary {
    pub fn new() -> Self {
        Self {
            cumulated_distance: 0, // Initialize to default value
            margin_decimal: 0.0,   // Initialize to default value
            real_distance: 0.0,    // Initialize to default value
            rank: 0,               // Initialize to default value
            intermediate_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(), // You might want to adjust this initialization
            section_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(), // You might want to adjust this initialization
            avg_speed: 0.0,                                          // Initialize to default value
            top_speed: 0.0,                                          // Initialize to default value
            avg_stride_freq: 0.0,                                    // Initialize to default value
            average_stride_length: 0.0,                              // Initialize to default value
            avg_distance_rail: 0.0,                                  // Initialize to default value
        }
    }
}

impl ValueProcessor for SectionSummary {
    fn get_single_fields(&self, field: &str) -> Option<String> {
        match field {
            CUMULATED_DISTANCE => Some(self.cumulated_distance.to_string()),
            MARGIN_DECIMAL => Some(self.margin_decimal.to_string()),
            REAL_DISTANCE => Some(self.real_distance.to_string()),
            RANK => Some(self.rank.to_string()),
            INTERMEDIATE_TIME => Some(self.intermediate_time.to_string()),
            SECTION_TIME => Some(self.section_time.to_string()),
            AVG_SPEED => Some(self.avg_speed.to_string()),
            TOP_SPEED => Some(self.top_speed.to_string()),
            AVERAGE_STRIDE_FREQUENCY => Some(self.avg_stride_freq.to_string()),
            AVERAGE_STRIDE_LENGTH => Some(self.average_stride_length.to_string()),
            AVERAGE_DISTANCE_TO_RAIL => Some(self.avg_distance_rail.to_string()),
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
            MARGIN_DECIMAL => {
                if let Ok(value) = new.parse::<f64>() {
                    self.margin_decimal = value;
                    Ok(())
                } else {
                    Err("Invalid value for margin_decimal")
                }
            }
            REAL_DISTANCE => {
                if let Ok(value) = new.parse::<f64>() {
                    self.real_distance = value;
                    Ok(())
                } else {
                    Err("Invalid value for real_distance")
                }
            }
            RANK => {
                if let Ok(value) = new.parse::<i32>() {
                    self.rank = value;
                    Ok(())
                } else {
                    Err("Invalid value for rank")
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
            AVG_SPEED => {
                if let Ok(value) = new.parse::<f64>() {
                    self.avg_speed = value;
                    Ok(())
                } else {
                    Err("Invalid value for avg_speed")
                }
            }
            TOP_SPEED => {
                if let Ok(value) = new.parse::<f64>() {
                    self.top_speed = value;
                    Ok(())
                } else {
                    Err("Invalid value for top_speed")
                }
            }
            AVERAGE_STRIDE_FREQUENCY => {
                if let Ok(value) = new.parse::<f64>() {
                    self.avg_stride_freq = value;
                    Ok(())
                } else {
                    Err("Invalid value for avg_stride_freq")
                }
            }
            AVERAGE_STRIDE_LENGTH => {
                if let Ok(value) = new.parse::<f64>() {
                    self.average_stride_length = value;
                    Ok(())
                } else {
                    Err("Invalid value for average_stride_length")
                }
            }
            AVERAGE_DISTANCE_TO_RAIL => {
                if let Ok(value) = new.parse::<f64>() {
                    self.avg_distance_rail = value;
                    Ok(())
                } else {
                    Err("Invalid value for avg_distance_rail")
                }
            }
            _ => Err("Field not found"),
        }
    }
}
