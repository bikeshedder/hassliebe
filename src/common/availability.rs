use serde::Serialize;

use crate::string_enum::string_enum;

#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option)))]
pub struct Availability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    pub topic: String,
    /// This field is not available for all MQTT integrations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

string_enum! {
    /// How the `availability` topics are combined into one availability state.
    /// Home Assistant's default is `latest`.
    ///
    /// <https://www.home-assistant.io/integrations/sensor.mqtt/#availability_mode>
    pub enum AvailabilityMode {
        /// Available only if all configured topics report available.
        All = "all",
        /// Available if at least one configured topic reports available.
        Any = "any",
        /// The last availability message received decides.
        Latest = "latest",
    }
}
