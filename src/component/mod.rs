//! One module per MQTT platform, each with its config struct.
//!
//! <https://www.home-assistant.io/integrations/#search/mqtt>

pub mod binary_sensor;
pub mod button;
pub mod cover;
pub mod number;
pub mod select;
pub mod sensor;
pub mod switch;
pub mod text;

/// Implemented by every platform config struct.
pub trait Platform {
    /// The platform name Home Assistant uses in discovery topics and in the
    /// `platform` key of device discovery components, e.g. `"sensor"`.
    const PLATFORM: &'static str;
}
