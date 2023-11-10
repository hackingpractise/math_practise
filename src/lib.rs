use anyhow::Result as AResult;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, Clone)]
pub(crate) struct SumReults {
    score: i32,
    time_taken: u128,
}

pub(crate) const ITERATIONS: i32 = 10;
pub(crate) const BUFFER: usize = 32_768;

pub mod functions;
pub mod record;

pub fn entry() -> AResult<record::RecordBuilder> {
    let result = functions::cube(ITERATIONS)?;
    let time: Duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let record = record::RecordBuilder::new()
        .add_score(result.score)
        .add_time_taken(result.time_taken)
        .add_time_recorded(time.as_nanos());
    Ok(record)
}
