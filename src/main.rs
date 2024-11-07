use csv::Writer;
use quick_xml::reader::Reader;
use sectionals::proccess_basic;
use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Hello, world!");
    let mut count = 1;

    // Open the final CSV file
    let csv_file_path = "all_race_summaries.csv";
    let mut wtr = Writer::from_path(csv_file_path)?;

    // Write the CSV headers
    let mut headers = vec![
        "event_date",
        "meeting_code",
        "race_number",
        "race_code",
        "event_name",
        "course_name",
        "race_name",
        "finish_time",
        "track_name",
        "track_condition",
        "rail_position",
        "horse_name",
        "horse_code",
        "bib",
        "draw_number",
        "distance_travelled",
        "distance_difference",
        "final_rank",
        "time_official",
        "official_margin",
        "fastest_section_time",
        "fastest_section_index",
        "top_speed",
        "top_speed_index",
        "horse_finish_time",
        "result_state",
        "result_substate",
        // Headers for the last 3 sections
        // New headers for section summary fields
        "last_600_rank",
        "last_600_section_time",
        "last_600_total_time",
        "last_600_real_distance",
        "last_600_avg_speed",
        "last_600_top_speed",
        "last_600_avg_stride_freq",
        "last_600_average_stride_length",
        "last_600_avg_distance_rail",
        "last_400_rank",
        "last_400_section_time",
        "last_400_total_time",
        "last_400_real_distance",
        "last_400_avg_speed",
        "last_400_top_speed",
        "last_400_avg_stride_freq",
        "last_400_average_stride_length",
        "last_400_avg_distance_rail",
        "last_200_rank",
        "last_200_section_time",
        "last_200_total_time",
        "last_200_real_distance",
        "last_200_avg_speed",
        "last_200_top_speed",
        "last_200_avg_stride_freq",
        "last_200_average_stride_length",
        "last_200_avg_distance_rail",
        "total_distance",
    ];

    // Write headers to CSV
    wtr.write_record(headers)?;

    for entry in fs::read_dir("../unzipped_sectionals")? {
        let entry = entry?;
        let file_path = entry.path();

        // Check if the entry is a file
        if !file_path.is_file() {
            continue;
        }

        // Set up reader
        let file = fs::File::open(&file_path)?;
        let race_number: i32 = file_path
            .to_str()
            .unwrap()
            .split("_")
            .last()
            .unwrap()
            .strip_prefix("R")
            .unwrap()
            .strip_suffix(".xml")
            .unwrap()
            .parse()
            .unwrap();
        let mut reader = Reader::from_reader(BufReader::new(file));
        reader.trim_text(true);

        let mut race = proccess_basic(&mut reader);
        race.race_number = race_number;

        if race.race_code == 0 {
            // Write to error.txt
            let mut error_file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("error.txt")?;
            writeln!(error_file, "{:?}", file_path)?;
            continue;
        }
        println!(
            "Writing: {}:{} {}           TOTAL:{}",
            race.event_date, race.course_name, race.race_number, count
        );

        println!("{:#?}", race);

        // Write each horse's data in the race summary
        for horse in race.horses {
            let mut row = vec![
                race.event_date.to_string(),
                race.meeting_code.to_string(),
                race.race_number.to_string(),
                race.race_code.to_string(),
                race.event_name.clone(),
                race.course_name.clone(),
                race.race_name.clone(),
                race.finish_time.to_string(),
                race.track_name.clone(),
                race.track_condition.clone(),
                race.rail_position.clone(),
                horse.name.clone(),
                horse.code.to_string(),
                horse.bib.to_string(),
                horse.draw_number.to_string(),
                horse.distance_travelled.to_string(),
                horse.distance_difference.to_string(),
                horse.final_rank.to_string(),
                horse.time_official.to_string(),
                horse.official_margin.to_string(),
                horse.fastest_section_time.to_string(),
                horse.fastest_section_index.to_string(),
                horse.top_speed.to_string(),
                horse.top_speed_index.to_string(),
                horse.finish_time.to_string(),
                horse.result_state.clone(),
                horse.result_substate.clone(),
            ];

            // Get the last 3 sections or fewer if there are not enough sections
            let section_count = horse.sections.len();
            let start = if section_count > 3 {
                section_count - 3
            } else {
                0
            };
            let last_sections = &horse.sections[start..];

            // Add each horse's last 3 section data (or fill with defaults if fewer)
            for section in last_sections {
                row.push(section.rank.to_string());
                row.push(section.section_time.to_string());
                row.push(section.intermediate_time.to_string());
                row.push(section.real_distance.to_string());
                row.push(section.avg_speed.to_string());
                row.push(section.top_speed.to_string());
                row.push(section.avg_stride_freq.to_string());
                row.push(section.average_stride_length.to_string());
                row.push(section.avg_distance_rail.to_string());
            }

            // Fill in with default values if fewer than 3 sections
            for _ in last_sections.len()..3 {
                row.push("0".to_string()); // Default real_distance
                row.push("N/A".to_string()); // Default avg_speed
                row.push("N/A".to_string()); // Default top_speed
                row.push("N/A".to_string()); // Default avg_stride_freq
                row.push("N/A".to_string()); // Default average_stride_length
                row.push("N/A".to_string()); // Default avg_distance_rail
            }

            // Add the cumulative distance from the last section or default if none
            if let Some(last_section) = last_sections.last() {
                row.push(last_section.cumulated_distance.to_string());
            } else {
                row.push("0".to_string()); // Default cumulative distance
            }

            // Ensure we still have 55 elements in the row
            while row.len() < 55 {
                row.push("N/A".to_string()); // Fill remaining with default
            }

            wtr.write_record(row)?;
        }

        wtr.flush()?;
        count += 1;
    }

    Ok(())
}
