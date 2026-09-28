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
    temperature_unit::TemperatureUnit,
    unit::Unit,
};
pub use component::{
    Component, Platform, alarm_control_panel, binary_sensor, button, camera, climate, cover, date,
    datetime, device_tracker, event, fan, humidifier, image, infrared, lawn_mower, light, lock,
    notify, number, scene, select, sensor, siren, switch, text, time, update, vacuum, valve,
    water_heater,
};
pub use discovery::{DeviceDiscovery, SharedOptions};
pub use string_enum::UnknownValue;
