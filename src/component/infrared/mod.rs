//! MQTT infrared: <https://www.home-assistant.io/integrations/infrared.mqtt/>
//!
//! Infrared has two config schemas with their own structs.

mod emitter;
mod receiver;

pub use emitter::{InfraredEmitter, InfraredEmitterSchema};
pub use receiver::{InfraredReceiver, InfraredReceiverSchema};
