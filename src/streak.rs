use chrono::{Local, NaiveDate};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use crate::exercise::Exercise;

/// File name for storing streak data in the user's home directory
const STREAK_FILE_NAME: &str = ".rustlings_streak.json";

/// Represents the persisted streak state
#[derive(Serialize, Deserialize, Debug)]
struct StreakData {
    last_date: String,
    streak: u32,
}

/// Get the full path to the streak file in the user's home directory
fn streak_file_path() -> PathBuf {
    let mut path = home_dir().expect("Could not locate home directory");
    path.push(STREAK_FILE_NAME);
    path
}

/// Update the current streak progress
pub fn update_streak() -> io::Result<()> {
    let today = Local::now().date_naive();
    let today_str = today.to_string();
    let path = streak_file_path();

    let mut data = match fs::read_to_string(&path) {
        Ok(content) => {
            serde_json::from_str::<StreakData>(&content).unwrap_or_else(|_| StreakData {
                last_date: today_str.clone(),
                streak: 1,
            })
        }
        Err(e) if e.kind() == ErrorKind::NotFound => StreakData {
            last_date: today_str.clone(),
            streak: 1,
        },
        Err(e) => return Err(e),
    };

    let last_date = NaiveDate::parse_from_str(&data.last_date, "%Y-%m-%d")
        .unwrap_or(today);
    let days_diff = (today - last_date).num_days();

    match days_diff {
        1 => {
            data.streak += 1;
            data.last_date = today_str;
        }
        d if d > 1 => {
            data.streak = 1;
            data.last_date = today_str;
        }
        _ => return Ok(()), // already up to date, nothing to change
    }

    fs::write(&path, serde_json::to_string_pretty(&data)?)?;
    Ok(())
}

/// display the current streak progress
pub fn display_streak(exercises: &[Exercise]) -> io::Result<()> {

    let num_done = exercises.iter().filter(|e| e.looks_done()).count();

    if num_done == 0 {
        println!("🚀 Let's get started! Your Rust journey begins now.");
        return Ok(()); // Don't read or write streak file
    }

    let today = Local::now().date_naive();
    let path = streak_file_path();

    let data = match fs::read_to_string(&path) {
        Ok(content) => {
            serde_json::from_str::<StreakData>(&content).unwrap_or_else(|_| StreakData {
                last_date: today.to_string(),
                streak: 1,
            })
        }
        Err(_) => {
            println!("📅 No streak data found. Start solving exercises to begin your streak!");
            return Ok(());
        }
    };

    let last_date = NaiveDate::parse_from_str(&data.last_date, "%Y-%m-%d")
        .unwrap_or(today);
    let days_diff = (today - last_date).num_days();

    match days_diff {
        0 => {
            println!("✅ Current streak: {} 🔥", data.streak);
        }
        1 => {
            println!("🔥 You're on a {}-day streak! Don’t forget to complete an exercise today!", data.streak);
        }
        _ => {
            println!("😴 Your streak was {} days. Time to start again!", data.streak);
        }
    }

    Ok(())
}
