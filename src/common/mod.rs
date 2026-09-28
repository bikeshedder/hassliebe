//! Types shared by all platforms: device and availability blocks, QoS, entity
//! categories, state classes and units.

pub mod availability;
pub mod device;
pub mod entity_category;
pub mod message_expiry_interval;
pub mod nullable;
pub(crate) mod one_or_many;
pub mod origin;
pub mod qos;
pub mod state_class;
pub mod temperature_unit;
pub mod unit;
