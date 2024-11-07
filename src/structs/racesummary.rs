use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use super::{fssummary::FastestSectionSummary, horsesummary::HorseSummary};
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct RaceSummary {
    pub event_date: NaiveDate,
    pub meeting_code: i32,
    pub race_number: i32,
    pub race_code: i32,
    pub event_name: String,
    pub course_name: String,
    pub race_name: String,
    pub finish_time: NaiveTime, // 00:01:56.900"
    pub track_name: String,
    pub track_condition: String,
    pub rail_position: String,
    pub fastest_sections: Vec<FastestSectionSummary>,
    pub horses: Vec<HorseSummary>,
}

impl RaceSummary {
    pub fn new() -> Self {
        Self {
            event_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), // Initialize to a default date
            meeting_code: 0,
            race_code: 0,
            race_number: 0,
            event_name: String::new(),
            course_name: String::new(),
            race_name: String::new(),
            finish_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(), // Initialize to midnight
            track_name: String::new(),
            track_condition: String::new(),
            rail_position: String::new(),
            fastest_sections: Vec::new(),
            horses: Vec::new(),
        }
    }

    pub fn get_fastest_sections(&self) -> &Vec<FastestSectionSummary> {
        &self.fastest_sections
    }

    pub fn set_fastest_sections(&mut self, sum: Vec<FastestSectionSummary>) {
        self.fastest_sections = sum;
    }

    pub fn add_fastest_section(&mut self, sum: FastestSectionSummary) {
        self.fastest_sections.push(sum);
    }

    pub fn get_horses(&self) -> &Vec<HorseSummary> {
        &self.horses
    }

    pub fn add_horse(&mut self, sum: HorseSummary) {
        self.horses.push(sum);
    }
}

impl ValueProcessor for RaceSummary {
    fn get_single_fields(&self, field: &str) -> Option<String> {
        match field {
            EVENT_DATE => Some(self.event_date.to_string()),
            MEETING_CODE => Some(self.meeting_code.to_string()),
            RACE_CODE => Some(self.race_code.to_string()),
            EVENT_NAME => Some(self.event_name.clone()),
            COURSE_NAME => Some(self.course_name.clone()),
            RACE_NAME => Some(self.race_name.clone()),
            FINISH_TIME => Some(self.finish_time.to_string()),
            TRACK_NAME => Some(self.track_name.clone()),
            TRACK_CONDITION => Some(self.track_condition.clone()),
            RAIL_POSITION => Some(self.rail_position.clone()),
            "race_number" => Some(self.race_number.to_string()),
            _ => None,
        }
    }

    fn set_single_fields(&mut self, field: &str, new: String) -> Result<(), &str> {
        match field {
            EVENT_DATE => {
                match NaiveDateTime::parse_from_str(&new, "%Y-%m-%dT%H:%M:%S") {
                    Ok(datetime) => {
                        // Extract date part from datetime
                        let date = datetime.date();
                        self.event_date = date;
                        Ok(())
                    }
                    Err(_err) => Err("Invalid value for event_date"),
                }
            }
            MEETING_CODE => {
                if let Ok(value) = new.parse::<i32>() {
                    self.meeting_code = value;
                    Ok(())
                } else {
                    Err("Invalid value for meeting_code")
                }
            }
            RACE_CODE => {
                if let Ok(value) = new.parse::<i32>() {
                    self.race_code = value;
                    Ok(())
                } else {
                    Err("Invalid value for race_code")
                }
            }
            "race_number" => {
                if let Ok(value) = new.parse::<i32>() {
                    self.race_number = value;
                    Ok(())
                } else {
                    Err("Invalid value for race_code")
                }
            }
            EVENT_NAME => {
                self.event_name = new;
                Ok(())
            }
            COURSE_NAME => {
                self.course_name = new;
                Ok(())
            }
            RACE_NAME => {
                self.race_name = new;
                Ok(())
            }
            FINISH_TIME => {
                if let Ok(value) = NaiveTime::parse_from_str(&new, "%H:%M:%S%.3f") {
                    self.finish_time = value;
                    Ok(())
                } else {
                    Err("Invalid value for finish_time")
                }
            }
            TRACK_NAME => {
                self.track_name = new;
                Ok(())
            }
            TRACK_CONDITION => {
                self.track_condition = new;
                Ok(())
            }
            RAIL_POSITION => {
                self.rail_position = new;
                Ok(())
            }
            _ => Err("Field not found"),
        }
    }
}
