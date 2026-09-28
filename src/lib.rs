//! Typed Home Assistant MQTT discovery payloads.
//!
//! Plain structs and enums with `serde` support that describe the JSON Home
//! Assistant expects on its discovery topics. No MQTT client, no I/O.
//!
//! <https://www.home-assistant.io/integrations/mqtt/>

pub mod common;
pub mod component;
pub mod discovery;

pub use common::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    entity_category::EntityCategory,
    qos::Qos,
    state_class::StateClass,
    unit::UnitOfMeasurement,
};
pub use component::{binary_sensor, number, select, sensor, switch};
