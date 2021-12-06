use crate::{Client, FromMap, TwilioError, POST};
use serde::Deserialize;
use std::collections::BTreeMap;

pub struct OutboundMessagingServiceMessage<'a> {
    pub messaging_service_sid: &'a str,
    pub to: &'a str,
    pub body: &'a str,
}

impl<'a> OutboundMessagingServiceMessage<'a> {
    pub fn new(messaging_service_sid: &'a str, to: &'a str, body: &'a str) -> OutboundMessagingServiceMessage<'a> {
        OutboundMessagingServiceMessage { messaging_service_sid, to, body }
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MessagingServiceMessageStatus {
    queued,
    sending,
    sent,
    failed,
    delivered,
    undelivered,
    receiving,
    received,
}

#[derive(Debug, Deserialize)]
pub struct MessagingServiceMessage {
    pub messaging_service_sid: String,
    pub to: String,
    pub body: Option<String>,
    pub sid: String,
    pub status: Option<MessagingServiceMessageStatus>,
}

impl Client {
    pub async fn send_messaging_service_message(&self, msg: OutboundMessagingServiceMessage<'_>) -> Result<MessagingServiceMessage, TwilioError> {
        let opts = [("To", &*msg.to), ("MessagingServiceSid", &*msg.messaging_service_sid), ("Body", &*msg.body)];
        self.send_request(POST, "Messages", &opts).await
    }
}

impl FromMap for MessagingServiceMessage {
    fn from_map(mut m: BTreeMap<String, String>) -> Result<Box<MessagingServiceMessage>, TwilioError> {
        let messaging_service_sid = match m.remove("MessagingServiceSid") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let to = match m.remove("To") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let sid = match m.remove("MessageSid") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let body = m.remove("Body");
        Ok(Box::new(MessagingServiceMessage {
            messaging_service_sid,
            to,
            sid,
            body,
            status: None,
        }))
    }
}
