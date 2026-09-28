#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod string_enum;

#[cfg(feature = "json")]
pub mod abbreviations;
pub mod common;
pub mod component;
pub mod discovery;

pub use common::unit;
pub use common::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    entity_category::EntityCategory,
    message_expiry_interval::{MessageExpiryInterval, TimePeriod},
    nullable::Nullable,
    origin::Origin,
    qos::Qos,
    state_class::StateClass,
    unit::Unit,
};
pub use component::{
    Component, Platform, binary_sensor, button, camera, cover, date, datetime, event, image, light,
    notify, number, scene, select, sensor, switch, text, time,
};
pub use discovery::{DeviceDiscovery, SharedOptions};
pub use string_enum::UnknownValue;
