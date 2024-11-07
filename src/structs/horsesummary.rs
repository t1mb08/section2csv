use super::sectionsummary::SectionSummary;
use crate::ValueProcessor;
use crate::*;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct HorseSummary {
    pub name: String,
    pub code: i32,
    pub bib: i32,
    pub draw_number: i32,
    pub distance_travelled: i32,
    pub distance_difference: i32,
    pub final_rank: u8,
    pub time_official: bool,
    pub official_margin: f64,
    pub fastest_section_time: NaiveTime,
    pub fastest_section_index: u8,
    pub top_speed: f64,
    pub top_speed_index: u8,
    pub finish_time: NaiveTime,
    pub result_state: String,
    pub result_substate: String,
    pub speeds: Vec<(i32, f64)>,
    pub ranks: Vec<(i32, f64)>,
    pub sections: Vec<SectionSummary>,
}

impl HorseSummary {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            code: 0,
            bib: 0,
            draw_number: 0,
            distance_travelled: 0,
            distance_difference: 0,
            final_rank: 0,
            time_official: false,
            official_margin: 0.0,
            fastest_section_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            fastest_section_index: 0,
            top_speed: 0.0,
            top_speed_index: 0,
            finish_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            result_state: String::new(),
            result_substate: String::new(),
            speeds: Vec::new(),
            ranks: Vec::new(),
            sections: Vec::new(),
        }
    }

    pub fn get_tuple_field(&self, field: &str) -> Option<&Vec<(i32, f64)>> {
        match field {
            SPEEDS => Some(&self.speeds),
            RANKS => Some(&self.ranks),
            _ => None,
        }
    }

    pub fn add_tuple_field(&mut self, field: &str, new: (i32, f64)) -> Result<(), &str> {
        match field {
            SPEEDS => {
                self.speeds.push(new);
                Ok(())
            }
            RANKS => {
                self.ranks.push(new);
                Ok(())
            }
            _ => Err("Field not found"),
        }
    }

    pub fn add_section(&mut self, new: SectionSummary) {
        self.sections.push(new)
    }
}

impl ValueProcessor for HorseSummary {
    fn get_single_fields(&self, field: &str) -> Option<String> {
        match field {
            NAME => Some(self.name.clone()),
            HORSE_CODE => Some(self.code.to_string()),
            BIB => Some(self.bib.to_string()),
            DRAW_NUMBER => Some(self.draw_number.to_string()),
            DISTANCE_TRAVELLED => Some(self.distance_travelled.to_string()),
            DISTANCE_TRAVELED_DIFFERENCE => Some(self.distance_difference.to_string()),
            FINAL_RANK => Some(self.final_rank.to_string()),
            IS_FINISH_TIME_OFFICIAL => Some(self.time_official.to_string()),
            OFFICIAL_MARGIN_DECIMAL => Some(self.official_margin.to_string()),
            FASTEST_SECTION_TIME => Some(self.fastest_section_time.to_string()),
            FASTEST_SECTION_INDEX => Some(self.fastest_section_index.to_string()),
            TOP_SPEED => Some(self.top_speed.to_string()),
            TOP_SPEED_SECTION_INDEX => Some(self.top_speed_index.to_string()),
            FINISH_TIME => Some(self.finish_time.to_string()),
            RESULT_STATE => Some(self.result_state.clone()),
            RESULT_SUB_STATE => Some(self.result_substate.clone()),
            _ => None,
        }
    }

    fn set_single_fields(&mut self, field: &str, new: String) -> Result<(), &str> {
        match field {
            NAME => {
                self.name = new;
                Ok(())
            }
            HORSE_CODE => {
                if let Ok(value) = new.parse::<i32>() {
                    self.code = value;
                    Ok(())
                } else {
                    Err("Invalid value for code")
                }
            }
            BIB => {
                if let Ok(value) = new.parse::<i32>() {
                    self.bib = value;
                    Ok(())
                } else {
                    Err("Invalid value for bib")
                }
            }
            DRAW_NUMBER => {
                if let Ok(value) = new.parse::<i32>() {
                    self.draw_number = value;
                    Ok(())
                } else {
                    Err("Invalid value for draw_number")
                }
            }
            DISTANCE_TRAVELLED => {
                if let Ok(value) = new.parse::<i32>() {
                    self.distance_travelled = value;
                    Ok(())
                } else {
                    Err("Invalid value for distance_travelled")
                }
            }
            DISTANCE_TRAVELED_DIFFERENCE => {
                if let Ok(value) = new.parse::<i32>() {
                    self.distance_difference = value;
                    Ok(())
                } else {
                    Err("Invalid value for distance_difference")
                }
            }
            FINAL_RANK => {
                if let Ok(value) = new.parse::<u8>() {
                    self.final_rank = value;
                    Ok(())
                } else {
                    Err("Invalid value for final_rank")
                }
            }
            IS_FINISH_TIME_OFFICIAL => {
                if let Ok(value) = new.parse::<bool>() {
                    self.time_official = value;
                    Ok(())
                } else {
                    Err("Invalid value for time_official")
                }
            }
            OFFICIAL_MARGIN_DECIMAL => {
                if let Ok(value) = new.parse::<f64>() {
                    self.official_margin = value;
                    Ok(())
                } else {
                    Err("Invalid value for official_margin")
                }
            }
            FASTEST_SECTION_TIME => {
                let trimmed_value =
                    new.trim_end_matches(|c: char| c == '[' || c == ']' || c == ' ');
                let parsed_value = trimmed_value
                    .split_whitespace()
                    .next()
                    .unwrap_or("00:00:00");

                if let Ok(value) = NaiveTime::parse_from_str(&parsed_value, "%H:%M:%S%.3f") {
                    self.fastest_section_time = value.into();
                    Ok(())
                } else {
                    Err("Invalid value for fastest_section_time")
                }
            }
            FASTEST_SECTION_INDEX => {
                if let Ok(value) = new.parse::<u8>() {
                    self.fastest_section_index = value;

                    Ok(())
                } else {
                    Err("Invalid value for top_speed_index")
                }
            }
            TOP_SPEED => {
                let trimmed_value =
                    new.trim_end_matches(|c: char| c == '[' || c == ']' || c == ' ');
                let parsed_value = trimmed_value.split_whitespace().next().unwrap_or("0.0");

                if let Ok(value) = parsed_value.parse::<f64>() {
                    self.top_speed = value;
                    Ok(())
                } else {
                    Err("Invalid value for top_speed")
                }
            }

            TOP_SPEED_SECTION_INDEX => {
                if let Ok(value) = new.parse::<u8>() {
                    self.top_speed_index = value;
                    Ok(())
                } else {
                    Err("Invalid value for top_speed_index")
                }
            }
            FINISH_TIME => {
                if let Ok(value) = NaiveTime::parse_from_str(&new, "%H:%M:%S%.3f") {
                    self.finish_time = value.into();
                    Ok(())
                } else {
                    Err("Invalid value for fastest_section_time")
                }
            }
            RESULT_STATE => {
                self.result_state = new;
                Ok(())
            }
            RESULT_SUB_STATE => {
                self.result_substate = new;
                Ok(())
            }
            _ => Err("Field not found"),
        }
    }
}
