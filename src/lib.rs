//! Typed Home Assistant MQTT discovery payloads.
//!
//! Plain structs and enums with `serde` support that describe the JSON Home
//! Assistant expects on its discovery topics. No MQTT client, no I/O.
//!
//! <https://www.home-assistant.io/integrations/mqtt/>

mod string_enum;

pub mod common;
pub mod component;

pub use common::unit;
pub use common::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    entity_category::EntityCategory,
    entity_name::EntityName,
    message_expiry_interval::{MessageExpiryInterval, TimePeriod},
    origin::Origin,
    qos::Qos,
    state_class::StateClass,
    unit::Unit,
};
pub use component::{Platform, binary_sensor, button, number, select, sensor, switch};
pub use string_enum::UnknownValue;
