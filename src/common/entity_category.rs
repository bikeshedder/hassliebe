//! Entity categories.

use crate::string_enum::string_enum;

string_enum! {
    /// The category of an entity. Entities with a category are not exposed to
    /// voice assistants and not included in area-wide actions.
    ///
    /// <https://www.home-assistant.io/integrations/sensor.mqtt/#entity_category>
    pub enum EntityCategory {
        /// An entity that allows changing the configuration of a device.
        Config = "config",
        /// An entity exposing a configuration parameter or diagnostics of a
        /// device.
        Diagnostic = "diagnostic",
    }
}
