//! The MQTT message expiry interval.

use serde::{Deserialize, Serialize};

/// MQTT message expiry interval that Home Assistant sets on the command
/// payloads it publishes, so retained messages are not kept forever.
///
/// The docs do not specify the format; Home Assistant accepts integer seconds
/// or a time period (`valid_message_expiry_interval` in
/// `homeassistant/components/mqtt/schemas.py`).
///
/// <https://www.home-assistant.io/integrations/mqtt/>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageExpiryInterval {
    /// Interval in seconds.
    Seconds(u32),
    /// Interval as a time period, e.g. `{"hours": 1}`.
    Period(TimePeriod),
}

impl From<u32> for MessageExpiryInterval {
    fn from(seconds: u32) -> Self {
        Self::Seconds(seconds)
    }
}

impl From<TimePeriod> for MessageExpiryInterval {
    fn from(period: TimePeriod) -> Self {
        Self::Period(period)
    }
}

/// A time period as Home Assistant's `cv.time_period_dict` accepts it. The
/// parts are added up; Home Assistant requires at least one of them.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimePeriod {
    /// Days.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<f64>,
    /// Hours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    /// Milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milliseconds: Option<f64>,
    /// Minutes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<f64>,
    /// Seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn seconds() {
        let interval: MessageExpiryInterval = serde_json::from_str("3600").unwrap();
        assert_eq!(interval, MessageExpiryInterval::Seconds(3600));
        assert_eq!(serde_json::to_string(&interval).unwrap(), "3600");
    }

    #[test]
    fn period() {
        let json = r#"{"hours":1.0,"minutes":30.0}"#;
        let interval: MessageExpiryInterval = serde_json::from_str(json).unwrap();
        assert_eq!(
            interval,
            MessageExpiryInterval::Period(TimePeriod {
                hours: Some(1.0),
                minutes: Some(30.0),
                ..Default::default()
            })
        );
        assert_eq!(serde_json::to_string(&interval).unwrap(), json);
    }

    #[test]
    fn rejects_negative_and_strings() {
        assert!(serde_json::from_str::<MessageExpiryInterval>("-1").is_err());
        assert!(serde_json::from_str::<MessageExpiryInterval>(r#""1h""#).is_err());
    }
}
