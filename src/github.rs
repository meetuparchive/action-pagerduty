use serde::Deserialize;
use std::{error::Error, fs::File, path::Path};

#[derive(Deserialize)]
pub struct Event {
    pub action: String,
    pub check_suite: CheckSuite,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Conclusion {
    Success,
    Failure,
    Neutral,
    Cancelled,
    TimedOut,
    ActionRequired,
}

#[derive(Deserialize, Debug)]
pub struct App {
    pub name: String,
    pub id: usize,
    //pub check_runs_url: String
}

#[derive(Deserialize)]
pub struct CheckSuite {
    pub conclusion: Conclusion,
    pub app: App,
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
