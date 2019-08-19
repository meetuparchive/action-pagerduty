mod github;
mod pagerduty;

use github::Conclusion;
use pagerduty::{Action, Event, PagerDuty, Payload};
use reqwest::Client;
use serde::Deserialize;
use std::{error::Error, process::exit};

// https://developer.github.com/actions/creating-github-actions/accessing-the-runtime-environment/#exit-codes-and-statuses
const NEUTRAL_EXIT: i32 = 78;

// https://help.github.com/en/articles/virtual-environments-for-github-actions#default-environment-variables
#[derive(Deserialize)]
struct Config {
    pd_token: String,
    pd_integration_key: String,
    github_event_path: String,
}

fn action(conclusion: Conclusion) -> Action {
    match conclusion {
        Conclusion::Success | Conclusion::Neutral => Action::Resolve,
        _ => Action::Trigger,
    }
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
    let conclusion = event.check_suite.conclusion;
    println!(
        "conclusion of checksuite for {:#?} was {:?}",
        event.check_suite.app, conclusion
    );

    if conclusion == Conclusion::Cancelled {
        exit(NEUTRAL_EXIT)
    }

    pagerduty.notify(
        pd_token,
        Event {
            routing_key: pd_integration_key,
            event_action: action(conclusion),
            payload: Some(Payload {
                summary: "this is just a test".into(),
                ..Payload::default()
            }),
        },
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run(envy::from_env()?, Client::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_is_based_on_conclusion() {
        for (given, expect) in vec![
            (Conclusion::ActionRequired, Action::Trigger),
            (Conclusion::Cancelled, Action::Trigger),
            (Conclusion::Failure, Action::Trigger),
            (Conclusion::Neutral, Action::Resolve),
            (Conclusion::Success, Action::Resolve),
            (Conclusion::TimedOut, Action::Trigger),
        ] {
            assert_eq!(action(given), expect)
        }
    }
}
