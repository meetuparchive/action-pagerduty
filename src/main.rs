mod github;
mod pagerduty;

use crate::pagerduty::{Action, Event, PagerDuty, Payload};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

// https://help.github.com/en/articles/virtual-environments-for-github-actions#default-environment-variables
#[derive(Deserialize)]
struct Config {
    pd_token: String,
    pd_integration_key: String,
    github_event_path: String,
}

fn run<P>(
    config: Config,
    pagerduty: P,
) -> Result<(), Box<dyn Error>>
where
    P: PagerDuty,
{
    let Config {
        pd_token,
        pd_integration_key,
        github_event_path,
    } = config;

    let event = github::parse(github_event_path)?;
    println!(
        "conclusion of checksuite was {:?}",
        event.check_suite.conclusion
    );

    pagerduty.notify(
        pd_token,
        Event {
            routing_key: pd_integration_key,
            event_action: Action::Trigger,
            payload: Some(Payload {
                summary: "this is just a test".into(),
                ..Payload::default()
            }),
        },
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run(envy::from_env()?, Client::new())?;
    Ok(())
}
