//! Types shared by all platforms: device and availability blocks, QoS, entity
//! categories, state classes and units.

pub mod availability;
pub mod device;
pub mod entity_category;
pub mod message_expiry_interval;
pub mod nullable;
pub mod origin;
pub mod qos;
pub mod state_class;
pub(crate) mod string_list;
pub mod unit;
