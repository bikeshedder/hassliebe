//! The `origin` block that names the sending application.

use serde::{Deserialize, Serialize};

/// Information about the application that sent a discovery message (the `o`
/// block). Recommended for single component discovery, required for device
/// discovery.
///
/// <https://www.home-assistant.io/integrations/mqtt/>
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Origin {
    /// Name of the application that is the origin of the discovered item.
    /// Required.
    pub name: String,
    /// Support URL of the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub support_url: Option<String>,
    /// Software version of the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub sw_version: Option<String>,
}

#[cfg(feature = "builder")]
impl Origin {
    /// Returns a builder for [`Origin`].
    pub fn builder() -> OriginBuilder {
        OriginBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn round_trip() {
        let origin = Origin {
            name: "bla2mqtt".into(),
            support_url: Some("https://bla2mqtt.example.com/support".into()),
            sw_version: Some("2.1".into()),
        };
        let json = serde_json::to_string(&origin).unwrap();
        assert_eq!(
            json,
            r#"{"name":"bla2mqtt","support_url":"https://bla2mqtt.example.com/support","sw_version":"2.1"}"#
        );
        assert_eq!(serde_json::from_str::<Origin>(&json).unwrap(), origin);
    }

    #[test]
    fn name_is_required() {
        assert!(serde_json::from_str::<Origin>(r#"{"sw_version":"2.1"}"#).is_err());
    }

    #[cfg(feature = "builder")]
    #[test]
    fn builder_requires_name() {
        assert!(Origin::builder().sw_version("2.1").build().is_err());
        assert!(Origin::builder().name("bla2mqtt").build().is_ok());
    }
}
