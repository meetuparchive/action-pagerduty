use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::error::Error;

pub trait PagerDuty {
    fn notify(
        &self,
        pd_token: String,
        event: Event,
    ) -> Result<StatusCode, Box<dyn Error>>;
}

impl PagerDuty for Client {
    fn notify(
        &self,
        pd_token: String,
        event: Event,
    ) -> Result<StatusCode, Box<dyn Error>> {
        // https://v2.developer.pagerduty.com/docs/send-an-event-events-api-v2
        Ok(self
            .post("https://events.pagerduty.com/v2/enqueue")
            .header("Authorization", format!("Token token={}", pd_token))
            .json(&event)
            .send()?
            .status())
    }
}

// https://v2.developer.pagerduty.com/docs/authentication#api-token-authentication

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Trigger,
    Resolve,
}

impl Default for Action {
    fn default() -> Self {
        Action::Trigger
    }
}

#[derive(Serialize, Default)]
pub struct Event {
    pub routing_key: String,
    pub event_action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

#[derive(Serialize, Default)]
pub struct Payload {
    pub summary: String,
    pub source: String,
    pub severity: String,
    pub links: Vec<Link>,
}

#[derive(Serialize, Default)]
pub struct Link {
    pub href: String,
    pub text: String,
}
