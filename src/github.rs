use serde::Deserialize;
use std::{error::Error, fs::File, path::Path};

#[derive(Deserialize)]
pub struct Event {
    pub action: String,
    pub check_suite: CheckSuite,
}

#[derive(Deserialize)]
pub struct CheckSuite {
    pub conclusion: String,
}

// https://developer.github.com/v3/activity/events/types/#webhook-event-name-1
pub fn parse<P>(path: P) -> Result<Event, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(serde_json::from_reader(File::open(path)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_parse() -> Result<(), Box<dyn Error>> {
        let _ = parse("tests/data/check_suite_completed.json")?;
        Ok(())
    }
}
