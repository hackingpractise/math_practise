/// Holds capacity for buffers.
const FILE_NAME: &str = "records.json";

use anyhow::{Context, Result as AResult};
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, OpenOptions};
use std::io::{self, BufWriter, Write};
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, RandGen)]
pub struct Record {
    pub index: u32,
    pub score: i32,
    pub time_recorded: u128,
    pub time_taken: u128,
}
impl Record {
    pub fn new(score: i32, time_taken: u128, time_recorded: u128, index: u32) -> Self {
        Self {
            index,
            score,
            time_taken,
            time_recorded,
        }
    }
    pub fn add(&self) -> io::Result<()> {
        // TODO
        if !std::path::Path::exists(FILE_NAME.as_ref()) {
            fs::File::create(FILE_NAME).unwrap();
        }
        let contents = fs::read_to_string(FILE_NAME).unwrap();
        let mut vec_records = if contents.trim().is_empty() {
            vec![]
        } else {
            serde_json::from_str(&contents).unwrap()
        };
        vec_records.push(self.to_owned());
        let json_val: String = serde_json::to_string_pretty(&vec_records).unwrap();
        write_to_file(FILE_NAME, json_val.as_bytes()).unwrap();
        Ok(())
    }
    pub fn last_entry() -> Option<Record> {
        let list = read_from_file(FILE_NAME).unwrap();
        list.last().cloned()
    }
}
#[derive(Debug, Default, Clone)]
pub struct RecordBuilder {
    score: Option<i32>,
    time_taken: Option<u128>,
    time_recorded: Option<u128>,
    index: Option<u32>,
}

impl RecordBuilder {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
    pub fn add_index(self, index: u32) -> Self {
        Self {
            index: Some(index),
            ..self
        }
    }
    pub fn add_time_taken(self, time_taken: u128) -> Self {
        Self {
            time_taken: Some(time_taken),
            ..self
        }
    }
    pub fn add_time_recorded(self, time_recorded: u128) -> Self {
        Self {
            time_recorded: Some(time_recorded),
            ..self
        }
    }
    pub fn add_score(self, score: i32) -> Self {
        Self {
            score: Some(score),
            ..self
        }
    }
    pub fn build(self) -> Record {
        Record {
            score: self.score.unwrap(),
            time_recorded: self.time_recorded.unwrap(),
            time_taken: self.time_taken.unwrap(),
            index: self.index.unwrap(),
        }
    }
}

pub fn read_from_file(filename: &str) -> AResult<Vec<Record>> {
    if !std::path::Path::exists(filename.as_ref()) {
        fs::File::create(filename).unwrap();
    }
    let json: String = fs::read_to_string(filename)
        .with_context(|| format!("Unable to read file: {}", filename))?;
    let records: Vec<Record> = serde_json::from_str(json.trim()).unwrap_or_default();

    Ok(records)
}
pub fn write_to_file(filename: &str, data: &[u8]) -> io::Result<usize> {
    let file: fs::File = OpenOptions::new().write(true).open(filename)?;
    let mut writer = BufWriter::new(file);
    writer.write(data)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn write() {
        let record = Record {
            index: 0,
            time_taken: 0,
            time_recorded: 0,
            score: 0,
        };
        record.add();
    }
    #[test]
    fn test_last_entry() {
        let last_record: Record = rand::random();
        last_record.add().unwrap();
        let last_entry = Record::last_entry().unwrap();

        assert_eq!(last_record, last_entry);
    }
}
