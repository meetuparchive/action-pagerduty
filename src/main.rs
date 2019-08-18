mod github;
mod pagerduty;

use crate::pagerduty::{Action, Client, Event, PagerDuty, Payload};
use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::error::Error;

// https://help.github.com/en/articles/virtual-environments-for-github-actions#default-environment-variables
#[derive(Deserialize)]
struct Config {
    pd_token: String,
    pd_integration_key: String,
    github_event_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Config {
        pd_token,
        pd_integration_key,
        github_event_path,
    } = envy::from_env()?;

    let event = github::parse(github_event_path)?;
    println!(
        "conclusion of checksuite was {}",
        event.check_suite.conclusion
    );

    let client = Client {
        pd_token,
        client: HttpClient::new(),
    };
    client.send(Event {
        routing_key: pd_integration_key,
        event_action: Action::Trigger,
        payload: Some(Payload {
            summary: "this is just a test".into(),
            ..Payload::default()
        }),
    })?;

    Ok(())
}
