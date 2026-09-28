# hassliebe

*A love-hate relationship with Home Assistant, in typed Rust.*

`hassliebe` contains typed Rust structs for the
[Home Assistant MQTT discovery](https://www.home-assistant.io/integrations/mqtt/)
payloads, with `serde` support. Nothing more: no MQTT client, no async runtime,
no opinions about how you talk to your broker.

Use it to announce devices to Home Assistant from any MQTT client, or to parse and
inspect discovery payloads.

> **Status:** early development. The API will change.

## Example

```rust
use hassliebe::{Device, sensor::{Sensor, SensorDeviceClass}, StateClass, unit::UnitOfPower};

fn main() -> Result<(), serde_json::Error> {
    let sensor = Sensor {
        name: Some("Power".into()),
        unique_id: Some("wallbox_garage_power".into()),
        state_topic: "wallbox/garage/power".into(),
        device_class: Some(SensorDeviceClass::Power),
        state_class: Some(StateClass::Measurement),
        unit_of_measurement: Some(UnitOfPower::Watt.into()),
        device: Some(Device {
            identifiers: vec!["wallbox_garage".into()],
            name: Some("Wallbox Garage".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let topic = hassliebe::discovery::component_topic("homeassistant", &sensor, None, "wallbox_garage_power");
    let payload = serde_json::to_string(&sensor)?;
    // publish `payload` retained on `topic` with the MQTT client of your choice
    Ok(())
}
```

## Features

| Feature   | Default | Description |
|-----------|---------|-------------|
| `builder` | yes     | `derive_builder` builders for all structs |
| `json`    | no      | Reading device discovery payloads, expanding abbreviated keys (`stat_t`, `dev_cla`, …) and the `~` base topic |

## Why the name?

*Hassliebe* is German for "love-hate relationship". It also happens to start
with `hass`.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
