//! Typed Home Assistant MQTT discovery payloads.
//!
//! Plain structs and enums with `serde` support that describe the JSON Home
//! Assistant expects on its discovery topics. No MQTT client, no I/O.
//!
//! <https://www.home-assistant.io/integrations/mqtt/>

// Remove once the first enum is generated with the macro.
#[cfg_attr(not(test), expect(unused_macros, dead_code))]
#[expect(unused_imports)]
mod string_enum;

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
