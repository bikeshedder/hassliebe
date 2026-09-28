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
  classes, nullable `name` (`Nullable`), `availability` (was `availabilty`),
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
- Remove the shared `DeviceClass`; each platform has its own device class enum.
- Add `Button` and `ButtonDeviceClass`.
- Add `Text` and `TextMode`.
- Add `Cover` and `CoverDeviceClass`.
- Add `Light` (default schema only), `LightSchema` and `OnCommandType`.
- Add `Component`, the platform-tagged entry of device discovery, with
  `Removed` and (feature `json`) `Other` for unmodeled platforms.
- Add `DeviceDiscovery` and `SharedOptions` for device-based discovery;
  deserializing (feature `json`) applies shared options like Home Assistant.
- Add discovery topic helpers (`discovery_topic`, `component_topic`,
  `device_topic`, `is_valid_id`) and the status topic constants.
- Add `abbreviations::{expand_component_payload, expand_device_payload}`
  (feature `json`) to expand abbreviated keys and the `~` base topic.
- The README is the crate documentation; its example is a doctest.
- `Platform::SCHEMA` for platforms with several config schemas; `Component`
  picks the struct by platform and schema.
- Add `Date`.
- Fields Home Assistant accepts as a single value or a list read both forms for
  any element type (`common::one_or_many`).
- Add `Time`.
- Add `DateTime`.
- Add `Scene`.
- Add `Notify`.
- Add `Event` and `EventDeviceClass`.
- `Select.options` also reads a single string.
- Add `Camera` and `CameraImageEncoding`.
- Add `Image` and `ImageEncoding`; the builder checks the `image_topic`/`url_topic` rules.
- Add `DeviceTracker` and `SourceType`; the builder requires `state_topic` or `json_attributes_topic`.
- Add `Lock`.
- Add `Valve` and `ValveDeviceClass`.
- Add `Siren` and `SirenTone`.
- Add `LawnMower`.
- Add `Update` and `UpdateDeviceClass`.
- Add `AlarmControlPanel` and `AlarmControlPanelFeature`.
- Add `Vacuum` and `VacuumFeature`; the builder requires `unique_id` with `clean_segments_command_topic`.
- Add `Fan`; the builder requires `preset_mode_command_topic` and `preset_modes` together.
- Add `Humidifier` and `HumidifierDeviceClass`; the builder requires `mode_command_topic` and `modes` together.
- Add `TemperatureUnit` (`C`/`F`) for climate and water heater devices.
- Add `WaterHeater` and `WaterHeaterMode`.
- Add `Climate` and `HvacMode`; the builder checks the preset mode and target humidity topic pairs.
- Add `JsonLight`, `JsonLightSchema` and `ColorMode`; `light` is now a directory module.
- Add `TemplateLight` and `TemplateLightSchema`.
- Add `InfraredEmitter` and `InfraredReceiver` with their schema tags.
- Add `DeviceTrigger` (platform `device_automation`) and `DeviceAutomationType`; the `type` key is the field `r#type`.
- Add `Tag` (tag scanner).
