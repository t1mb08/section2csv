use csv::Writer;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use serde_json::{self, Value};
use sqlx::PgPool;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::{borrow::Cow, fs::File, io::BufReader};
use std::{io, path::PathBuf};
use structs::fssummary::FastestSectionSummary;
use structs::horse::Horse;
use structs::horsesummary::HorseSummary;
use structs::race::Race;
use structs::racesummary::RaceSummary;
use structs::sectionsummary::SectionSummary;

pub mod structs;

// Race Summary Tags
pub const RACE_SUMMARY: &'static str = "RaceSummary";
pub const EVENT_DATE: &'static str = "EventDate";
pub const MEETING_CODE: &'static str = "MeetingCode";
pub const RACE_CODE: &'static str = "RaceCode";
pub const EVENT_NAME: &'static str = "EventName";
pub const COURSE_NAME: &'static str = "CourseName";
pub const RACE_NAME: &'static str = "RaceName";
pub const FINISH_TIME: &'static str = "FinishTime";
pub const TIME: &'static str = "Time";
pub const TRACK_NAME: &'static str = "TrackName";
pub const TRACK_CONDITION: &'static str = "TrackCondition";
pub const RAIL_POSITION: &'static str = "RailPosition";
pub const FASTEST_SECTIONS: &'static str = "FastestSections";
pub const HORSES: &'static str = "Horses";

// Section Summary Tags
pub const SECTION_SUMMARY: &'static str = "SectionSummary";
pub const CUMULATED_DISTANCE: &'static str = "CumulatedDistance";
pub const MARGIN_DECIMAL: &'static str = "MarginDecimal";
pub const REAL_DISTANCE: &'static str = "RealDistance";

pub const RANK: &'static str = "Rank";
pub const INTERMEDIATE_TIME: &'static str = "IntermediateTime";
pub const SECTION_TIME: &'static str = "SectionTime";
pub const AVG_SPEED: &'static str = "AvgSpeed";
pub const TOP_SPEED: &'static str = "TopSpeed";
pub const AVERAGE_STRIDE_FREQUENCY: &'static str = "AverageStrideFrequency";
pub const AVERAGE_STRIDE_LENGTH: &'static str = "AverageStrideLength";
pub const AVERAGE_DISTANCE_TO_RAIL: &'static str = "AverageDistanceToRail";

// Horse Summary Tags
pub const HORSE_SUMMARY: &'static str = "HorseSummary";
pub const NAME: &'static str = "Name";
pub const HORSE_CODE: &'static str = "HorseCode";
pub const BIB: &'static str = "Bib";
pub const DRAW_NUMBER: &'static str = "DrawNumber";
pub const DISTANCE_TRAVELLED: &'static str = "DistanceTravelled";
pub const DISTANCE_TRAVELED_DIFFERENCE: &'static str = "DistanceTraveledDifference";
pub const FINAL_RANK: &'static str = "FinalRank";
pub const IS_FINISH_TIME_OFFICIAL: &'static str = "IsFinishTimeOfficial";
pub const OFFICIAL_MARGIN_DECIMAL: &'static str = "OfficialMarginDecimal";
pub const FASTEST_SECTION_TIME: &'static str = "FastestSectionTime";
pub const FASTEST_SECTION_INDEX: &'static str = "FastestSectionIndex";
pub const TOP_SPEED_SECTION_INDEX: &'static str = "TopSpeedSectionIndex";
pub const RESULT_STATE: &'static str = "ResultState";
pub const RESULT_SUB_STATE: &'static str = "ResultSubState";

pub const SPEEDS: &'static str = "Speeds";
pub const RANKS: &'static str = "Ranks";
pub const SERIALIZABLE_TUPLE_OF_DOUBLE_DOUBLE: &'static str = "SerializableTupleOfDoubleDouble";
pub const SERIALIZABLE_TUPLE_OF_DOUBLE_INT32: &'static str = "SerializableTupleOfDoubleInt32";
pub const ITEM1: &'static str = "Item1";
pub const ITEM2: &'static str = "Item2";

pub const SECTIONS: &'static str = "Sections";

pub trait ValueProcessor {
    fn get_single_fields(&self, field: &str) -> Option<String>;
    fn set_single_fields(&mut self, field: &str, new: String) -> Result<(), &str>;
}

pub fn parse_start<T: Clone + ValueProcessor>(
    e: BytesStart,
    tag: &mut String,
    values: &mut T, // Where the Values are being stored
) {
    // Get Any Attributes
    e.attributes()
        .with_checks(false)
        .map(|x| x.unwrap().clone())
        .map(|x| {
            let key = String::from_utf8(x.key.0.to_owned()).unwrap();
            let value =
                String::from_utf8(<Cow<'_, [u8]> as Clone>::clone(&x.value).into_owned()).unwrap();
            (key, value)
        })
        .for_each(|(mut key, value)| {
            if key == TIME {
                key = tag.clone();
            }
            let text = values.get_single_fields(key.as_str());
            let _ = match text {
                Some(_x) => values.set_single_fields(key.as_str(), value),
                None => Ok(()),
            };
        });
}

pub fn parse_text<T: Clone + ValueProcessor>(
    e: BytesText,
    tag: &mut String,
    values: &mut T, // Where the Values are being stored
    txt: &mut String,
) {
    *txt = e.unescape().unwrap().into_owned();
    let tag_name_str = tag.as_str();
    let text = values.get_single_fields(tag_name_str);

    // Store Text to Tag
    let _ = match text {
        Some(_x) => values.set_single_fields(tag_name_str, txt.to_string()),
        None => Ok(()),
    };
}

pub fn parse_end(tag: &mut String) {
    *tag = String::new();
}

pub fn proccess_basic(reader: &mut Reader<BufReader<File>>) -> RaceSummary {
    let mut race: RaceSummary = RaceSummary::new();
    let mut fastests = FastestSectionSummary::new();
    let mut horses = HorseSummary::new();
    let mut item1: i32 = 0;
    let mut item2: f64 = 0.0;
    let mut sectionsummary = SectionSummary::new();

    let mut buf = Vec::new();
    let mut txt: String = String::new();
    let mut tag: String = String::new();

    let mut tagflag: Vec<String> = Vec::new();
    let mut structflag = RACE_SUMMARY;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Decl(_)) => {}
            Ok(Event::Start(e)) => {
                tag = String::from_utf8((*e.name().0).to_vec()).unwrap();
                tagflag.push(tag.to_string());

                match tag.as_str() {
                    FASTEST_SECTIONS => structflag = FASTEST_SECTIONS,
                    HORSES => structflag = HORSES,
                    SECTIONS => structflag = SECTIONS,
                    _ => {}
                }

                match structflag {
                    FASTEST_SECTIONS => parse_start(e, &mut tag, &mut fastests),
                    HORSES => parse_start(e, &mut tag, &mut horses),
                    SECTIONS => parse_start(e, &mut tag, &mut sectionsummary),
                    _ => {
                        parse_start(e, &mut tag, &mut race);
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                tag = String::from_utf8((*e.name().0).to_vec()).unwrap();
                match structflag {
                    FASTEST_SECTIONS => parse_start(e, &mut tag, &mut fastests),
                    HORSES => parse_start(e, &mut tag, &mut horses),
                    SECTIONS => parse_start(e, &mut tag, &mut sectionsummary),
                    _ => {
                        parse_start(e, &mut tag, &mut race);
                    }
                }
            }
            Ok(Event::Text(e)) => match tagflag.last() {
                Some(x) => match x.as_str() {
                    ITEM1 => {
                        item1 = e.unescape().unwrap().into_owned().parse().unwrap();
                    }
                    ITEM2 => {
                        item2 = e.unescape().unwrap().into_owned().parse().unwrap();
                    }
                    _ => match structflag {
                        FASTEST_SECTIONS => parse_text(e, &mut tag, &mut fastests, &mut txt),
                        HORSES => parse_text(e, &mut tag, &mut horses, &mut txt),
                        SECTIONS => parse_text(e, &mut tag, &mut sectionsummary, &mut txt),
                        _ => {
                            parse_text(e, &mut tag, &mut race, &mut txt);
                        }
                    },
                },
                None => {
                    parse_text(e, &mut tag, &mut race, &mut txt);
                }
            },
            Ok(Event::End(_e)) => match tagflag.pop() {
                Some(x) => match x.as_str() {
                    SECTION_SUMMARY => {
                        if tagflag.last().unwrap() == FASTEST_SECTIONS {
                            race.add_fastest_section(fastests.clone());
                            fastests = FastestSectionSummary::new();
                        } else {
                            horses.add_section(sectionsummary.clone())
                        }
                    }
                    HORSE_SUMMARY => {
                        race.add_horse(horses.clone());
                        horses = HorseSummary::new();
                    }
                    SERIALIZABLE_TUPLE_OF_DOUBLE_DOUBLE | SERIALIZABLE_TUPLE_OF_DOUBLE_INT32 => {
                        match tagflag.last().unwrap().as_str() {
                            SPEEDS => {
                                let _ =
                                    horses.add_tuple_field(SPEEDS, (item1.clone(), item2.clone()));
                            }
                            RANKS => {
                                let _ =
                                    horses.add_tuple_field(RANKS, (item1.clone(), item2.clone()));
                            }
                            _ => {}
                        }
                    }
                    SECTIONS => structflag = HORSES,
                    _ => {}
                },
                None => {}
            },

            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
    race
}

pub fn unzip() {
    for file in fs::read_dir("./data")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
    {
        let file = fs::File::open(file.unwrap()).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let mut outpath = PathBuf::from("./unzipped_sectionals");
            match file.enclosed_name() {
                Some(path) => outpath.push(path.to_owned()),
                None => continue,
            };

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {i} comment: {comment}");
                }
            }

            if (*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.display());
                fs::create_dir_all(&outpath).unwrap();
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();

                // Get and Set permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;

                    if let Some(mode) = file.unix_mode() {
                        fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                    }
                }
            }
        }
    }
}
