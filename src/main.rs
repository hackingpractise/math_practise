use ansi_term::Colour::{Green, Red};
use math_practice::record;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
enum Time {
    Day(u32, u64),
    Hour(u32, u64),
    Min(u32, u64),
    Seconds(u64),
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds(x) => write!(f, "{} seconds", x),
            Self::Min(x, y) => write!(f, "{} minutes and {} seconds", x, y),
            Self::Hour(x, y) => write!(f, "{} hour(s) and {} seconds", x, y),
            Self::Day(x, y) => write!(f, "{} day(s) and {} seconds", x, y),
        }
    }
}

fn main() {
    let record_builder = math_practice::entry().unwrap();

    let record = if let Some(val_rec) = record::Record::last_entry() {
        record_builder.add_index(val_rec.index + 1).build()
    } else {
        record_builder.add_index(1).build()
    };

    // Print the improvement.
    if let Some(rec) = record::Record::last_entry() {
        if record.time_recorded < rec.time_recorded {
            panic!("Help!Help")
        }
        let time_since_last = record.time_recorded - rec.time_recorded;
        let duration = Duration::from_nanos(time_since_last as u64);
        let time = map_time(duration);
        if rec.score > record.score {
            println!(
                "What happened, you lost {} more point than {} ago.\nDo better.",
                Red.paint((rec.score - record.score).to_string()),
                time
            );
        } else if rec.score == record.score {
            println!("Congatuations you maintained your score since {} ago", time);
        } else {
            println!(
                "Hurrah you improved by {1} since last time {0} ago",
                time,
                Green.paint((record.score - rec.score).to_string())
            );
        }

        if rec.time_taken > record.time_taken {
            let time_improved = rec.time_taken - record.time_taken;
            let duration_tt = Duration::from_nanos(time_improved as u64);
            println!(
                "Your improved your speed, took {} seconds than last time.",
                Green.paint((duration_tt.as_millis() as f64 / 1000_f64).to_string())
            );
        } else {
            let time_drop = record.time_taken - rec.time_taken;
            let duration_td = Duration::from_nanos(time_drop as u64);
            println!(
                "Your speed droped, you took {} more seconds than last time.",
                Red.paint((duration_td.as_millis() as f64 / 1000_f64).to_string())
            );
        }
        let duration_ttt = Duration::from_nanos(record.time_taken as u64);
        println!(
            "You took {} seconds.",
            duration_ttt.as_millis() as f64 / 1000_f64
        );
    } else {
        println!("This is your first attempt.Do more to have a history.");
    }

    record.add().unwrap();
}

fn map_time(duration: Duration) -> Time {
    let x = duration.as_secs();
    match x {
        86400.. => Time::Day(x as u32 / 86400_u32, x % 86400),
        3600.. => Time::Hour(x as u32 / 3600_u32, x % 3600),
        60.. => Time::Min(x as u32 / 60, x % 60),
        0.. => Time::Seconds(x),
    }
}
