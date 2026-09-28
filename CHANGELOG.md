# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Initial import of the discovery types from `nrg-hass`.
- Move modules to `common` and `component`; re-export common types and platform
  modules at the crate root.
- `Qos` serializes as the integers `0`, `1`, `2` and deserializes only those.
- String enums (`AvailabilityMode`, `EntityCategory`, …) are `#[non_exhaustive]`, deserialize, and keep
  unknown values in an `Other(String)` variant.
- Add `StateClass::MeasurementAngle`.
- Replace `UnitOfMeasurement` with `Unit` and the `UnitOf*` classes from Home
  Assistant. Adds all current units, fixes `ft³/min` and serializes the micro sign
  as U+03BC.
- Structs derive `Deserialize` and `PartialEq`; builders fail on missing required
  fields.
- `Device.identifiers` and `Device.connections` are plain `Vec`s; `identifiers`
  also reads a single string. The `Device` builder requires `name` and an
  identifier or connection.
- Add `Origin`.
- Add `MessageExpiryInterval` (seconds or a `TimePeriod`).
- Replace the `Discovery` trait with `Platform`; `unique_id` is optional.
- Sync `Sensor` with Home Assistant: `SensorDeviceClass` with all 62 device
  classes, nullable `name` (`EntityName`), `availability` (was `availabilty`),
  `origin`, `message_expiry_interval`, `visible_by_default`; legacy top-level
  availability fields removed.
- Sync `BinarySensor` with Home Assistant: its own `BinarySensorDeviceClass`
  (29 values), `group` as a list, same common fields as `Sensor`.
- Sync `Switch` with Home Assistant: `SwitchDeviceClass`, `command_topic`
  required, `state_topic` optional.
- Sync `Number` with Home Assistant: `NumberDeviceClass` (58 values), open
  `NumberMode`, `payload_reset`, `group`, alphabetical fields.
- Sync `Select` with Home Assistant: `options` required, typed
  `entity_category`.
