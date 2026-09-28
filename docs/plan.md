# Work plan

Status: 2026-09-28. Baseline for all checks: Home Assistant core `dev` and
home-assistant.io `current` as of this date (HA 2026.10 cycle).

Tick items off as they land. Each unchecked item is meant to be one commit;
the bold title is the commit message. Every item ends with the full command set from
`AGENTS.md` passing, unless the item says otherwise.

---

## 1. Inventory of the imported `nrg-hass` code

### Build state

The imported code does **not compile** in any feature combination:

- default / `--all-features`: `serde_json` is optional but used unconditionally
  in `discovery.rs::announce()` and `state.rs::publish_state()`.
- `--no-default-features`: 28 errors, `derive_builder` is used unconditionally
  (`use derive_builder::Builder`, `impl X { fn builder() }`).
- `Cargo.toml` still depends on `rumqttc` and enables `serde/rc` (only needed for
  `Arc<Device>`). `Cargo.toml.new` is the intended manifest without both.

### Delete

| Item | Reason |
|---|---|
| `src/state.rs` (`State` trait, `publish_state`) | I/O, `rumqttc` |
| `src/config.rs` (`HomeAssistantConfig`) | application config, not a payload |
| `discovery.rs`: `announce()`, `DiscoveryPayload` | I/O; also inserts `platform` into single-component payloads, which HA docs say is "only allowed in device discovery messages" |
| `src/client.rs`, `src/mixins.rs`, `src/availability.rs` | empty files |
| `impl State for …` in all five components | depend on `state.rs`; contain `unwrap()` |
| `select.rs`: `OptionsItem` | not part of any HA schema |
| `models/device_class.rs` (shared `DeviceClass`) | replaced by per-platform enums (after all users migrated) |
| `Cargo.toml.new` | merged into `Cargo.toml` |

### Rework

| Item | Problem |
|---|---|
| Module layout `src/models/*` | must become `src/common/*` + `src/component/*` |
| `Discovery` trait | replaced by `Platform { const PLATFORM }`; topic building moves to free functions |
| All structs | only `Serialize`; missing `Deserialize`, `PartialEq`, `#[serde(default)]`, doc comments; builder not feature-gated |
| All components | field typo `availabilty`; ~~`Arc<Device>` instead of `Device`~~ (done in Phase A); `unique_id: String` required (optional in HA) |
| `Sensor.name: String` | optional and nullable in HA |
| `Number` | fields not alphabetical (`availability_topic`/`availability_mode`, `min`/`max`); missing `group` |
| `Select` | `entity_category: Option<String>` instead of `EntityCategory`; field order |
| All components | legacy top-level `availability_topic`, `availability_template`, `payload_available`, `payload_not_available` are dropped (decision 10) |
| `BinarySensor.group: Option<String>` | HA: list of strings (single string coerced); `Sensor`/`Switch` already use `Vec<String>` |
| All components | missing `origin`, `message_expiry_interval`, `visible_by_default` (all in `MQTT_ENTITY_COMMON_SCHEMA`) |
| `Qos` | `#[repr(u8)]` + derived `Serialize` emits `"AtMostOnce"` etc. instead of `0/1/2` |
| All enums | hand-derived serde, closed; must become `string_enum!` |
| `NumberMode` | closed enum, `#[default]` on an enum used inside `Option` (misleading) |
| `Device.connections: Vec<(String, String)>` | serializes as `[["mac","…"]]`, correct, but must also deserialize; `identifiers` may be a single string in HA |
| `expire_after`, `off_delay: NonZeroU32` | HA uses `cv.positive_int`; verify whether 0 is accepted — if so, `NonZeroU32` rejects valid payloads |

### Already in line with the architecture

- One struct per platform with public fields, `Option<T>` +
  `skip_serializing_if`, `derive_builder` attributes `default, setter(into, strip_option)`.
- Doc links to the per-platform MQTT docs on the component structs.
- `default_entity_id` and `group` are present, so the field lists were updated
  after HA 2025.10; `object_id` (removed in HA 2026.4) is already absent.

### How outdated is it? (spot checks against `dev`)

| Area | Crate | Current HA | Verdict |
|---|---|---|---|
| Sensor device classes | 49 | 62 | missing `absolute_humidity`, `area`, `blood_glucose_concentration`, `conductivity`, `energy_distance`, `ph`, `pm4`, `radon`, `reactive_energy`, `temperature_delta`, `uptime`, `volume_flow_rate`, `wind_direction` |
| Binary sensor device classes | uses the *sensor* list | 29 own values (`motion`, `door`, …) | **wrong**, not just outdated |
| Switch device classes | uses the *sensor* list | `outlet`, `switch` | **wrong** |
| Number device classes | uses the *sensor* list | sensor list minus `date`, `enum`, `timestamp`, `uptime` | wrong (accepts invalid values) |
| State classes | 3 | 4 (`measurement_angle`) | outdated |
| Entity category | `config`, `diagnostic` | same | OK |
| Availability mode | `all`, `any`, `latest` | same | OK |
| Units | ~130 | 29 `UnitOf*` classes + constants | many missing (`mW`, `MW`, `GW`, `TW`, `J`, `kJ`, `GWh`, `cal`…`thm`, `μA`, `μV`, most flow rates, `UnitOfDensity`, `UnitOfRadiationConcentration`, …) |
| Unit strings | `"ft³/m"` | `"ft³/min"` | **wrong value** |
| Micro sign | `µg`, `µS/cm`, `µg/m³` use U+00B5 | canonical U+03BC; U+00B5 rewritten as legacy (`sensor/const.py` `AMBIGUOUS_UNITS`) | serializes non-canonical form |
| Unit string collisions | `"m"` = Meters **and** Months; `"dB"` twice (one commented out) | HA itself uses `"m"` for both `UnitOfLength.METERS` and `UnitOfTime.MONTHS` | decided, see decision 2 |
| Common entity fields | – | `message_expiry_interval` (2026-05), `visible_by_default` (2026-06), `origin` | missing |

Conclusion: the struct field lists are about 6 months behind; the enums are one to
two years behind and partly wrong by design (shared `DeviceClass`). Every enum
and every field list must be re-derived from the sources, not patched.

Noted docs/source discrepancies (resolved in favour of the docs, decision 4):

- `number.mqtt` docs: `min` default **1**; `number/const.py`: **0.0**.
- MQTT docs: device `name` must be set in shared device info; source keeps it
  optional (logs INFO).
- `sensor.mqtt` docs: `expire_after` default 0; source: no default.
- `message_expiry_interval`, `group` and top-level `payload_(not_)available` are
  accepted by the source but missing from several platform docs.
- The abbreviation table in the MQTT docs is out of sync with `abbreviations.py`
  (use the source for `abbreviations.rs`).

---

## 2. Plan

### Phase A — make it build

- [x] **`Remove I/O code and rumqttc`**
  Delete `state.rs`, `config.rs`, `client.rs`, `mixins.rs`, top-level
  `availability.rs`, `announce()` and `DiscoveryPayload`, all `impl State` blocks,
  `OptionsItem`. Keep the `Discovery` trait (topic building only) for now.
  Replace `Cargo.toml` with `Cargo.toml.new` (drops `rumqttc`, `serde/rc`),
  delete `Cargo.toml.new`, update `Cargo.lock`.
  Done when `cargo build` and `cargo build --all-features` succeed without `rumqttc`
  in `Cargo.lock`. (`--no-default-features` still fails; next step.)
  Also switched `Arc<Device>` → `Device` here: without `serde/rc` it no longer
  serializes.
- [x] **`Gate derive_builder behind the builder feature`**
  `#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]`,
  `#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option)))]`,
  `impl X { fn builder() }` under `#[cfg(feature = "builder")]`.
  Done when both feature sets build and clippy is clean (fix remaining lints here).
- [x] **`Add CI and project files`**
  Move `ci.yml` to `.github/workflows/ci.yml`; commit `AGENTS.md`,
  `ARCHITECTURE.md`, `CLAUDE.md`, `README.md`, `CHANGELOG.md`, `.gitignore`,
  `docs/plan.md`. Mark the README example as `ignore` (or remove it from doctests)
  until the API exists. First point at which the full `AGENTS.md` command set passes.
  (README is not included in the crate docs, so it is not a doctest yet. Bare doc
  URLs wrapped in `<…>` so `cargo doc` passes with `-D warnings`.)

### Phase B — foundations

- [x] **`Move modules to common/ and component/`**
  Pure `git mv` + path fixes, no behaviour change: `availability`, `device`,
  `entity_category`, `qos`, `state_class`, `unit` → `src/common/`;
  `sensor`, `binary_sensor`, `switch`, `number`, `select`, `device_class` →
  `src/component/`. `lib.rs` with module docs and the re-exports from the README.
- [x] **`Add string_enum! macro`** (`src/string_enum.rs`)
  `macro_rules!` that generates a `#[non_exhaustive]` enum with `Other(String)`,
  `as_str()`, `Display`, `FromStr` (infallible), per-variant doc comments and
  optional input-only aliases (decision 1). Serde via plain derives: each variant
  `#[serde(rename = "…", alias = "…")]`, `Other` as `#[serde(untagged)]`
  (verified: known string or alias → variant, anything else → `Other`, non-strings
  rejected). `FromStr` must agree with serde, including aliases. Test-time guard
  against duplicate strings/aliases. Unit tests incl. `Other` round-trip, alias
  normalisation and the `Other("known")` case.
- [x] **`Add snapshot and fixture scaffolding`**
  `tests/` layout, `tests/fixtures/`, a small round-trip helper, one insta
  snapshot as example. No component tests yet.
- [x] **`Serialize Qos as integers`**
  Hand-written serde (integer enum, not a string enum): serialize `0/1/2`,
  deserialize only `0/1/2`. Tests for both directions and for rejecting `3`.

### Phase C — common types

One commit each. Every enum is regenerated from the source file named, with the
URL in the doc comment and the file in the commit message.

All string-valued enums use `string_enum!`, including closed sets (decision 11).

- [x] **`Convert EntityCategory to string_enum!`** (`homeassistant/const.py`)
- [x] **`Convert AvailabilityMode to string_enum!`** (MQTT docs, `schemas.py`)
- [x] **`Sync StateClass with HA`** (`sensor/const.py`; adds `measurement_angle`)
- [x] **`Sync Unit with HA`**
  Replace `UnitOfMeasurement` with the 29 `UnitOf*` classes (generated from
  `const.py`, `string_enum! { @closed … }`) and the `Unit` wrapper with the
  plain constants (`°`, `lx`, `UV index`, `rpm`, `dB`, `dBm`, currencies,
  deprecated `p/m³`) (decision 2). Fixes `ft³/m` → `ft³/min` and the micro
  sign. Legacy spellings from `AMBIGUOUS_UNITS` become input aliases
  (decision 1).
- [x] **`Make Availability deserializable and document it`**
  `Deserialize`, `PartialEq`, `#[serde(default)]`, docs with HA defaults
  (`online`/`offline`), fields verified against `schemas.py`.
- [x] **`Make Device deserializable and document it`**
  Verify the 12 fields; `connections` as `Vec<(String, String)>` (or a named
  type); `name` per decision 4; `identifiers:
  Vec<String>`, skipped when empty, deserializer also accepts a single string
  (decision 5; shared helper in `common/`, reused for `group`); no `Arc`.
- [x] **`Add Origin`** (`name` required, `sw_version`, `support_url`).
- [x] **`Add MessageExpiryInterval`** (decision 12)
  Untagged enum: `Seconds(u32)` or a time period struct. Derive the period keys
  from `cv.time_period_dict` in `helpers/config_validation.py`.
- [x] **`Replace Discovery trait with Platform`**
  `trait Platform { const PLATFORM: &'static str; }`, implemented for all
  existing components; `unique_id` becomes a plain `Option<String>` field.

### Phase D — existing components

One commit per component. Each commit covers: the per-platform device class
enum via `string_enum!` (where the platform has one), `availabilty` →
`availability`, a field list re-derived from
the docs + `mqtt/<platform>.py` + `MQTT_ENTITY_COMMON_SCHEMA` (incl. `origin`,
`group`, `message_expiry_interval`, `visible_by_default`; legacy top-level
availability fields removed, decision 10), alphabetical order, correct
required/optional fields (required fields have no builder default, decision 13),
docs with HA defaults as stated in the docs (decision 4), full + minimal
snapshot, round-trip test, one real-world fixture, CHANGELOG line.

- [x] **`Sync sensor with HA`** (`name` → `Option<EntityName>`; 62 device classes)
- [x] **`Sync binary_sensor with HA`** (`group` → `Vec<String>`; 29 device classes)
- [x] **`Sync switch with HA`** (`SwitchDeviceClass`: `outlet`, `switch`)
- [x] **`Sync number with HA`** (`min` default 1 per docs; `NumberMode` via `string_enum!`; 58 device classes)
- [x] **`Sync select with HA`** (`entity_category` typed; no device class in HA)
- [ ] **`Remove shared DeviceClass`** (should be empty of users by now)

### Phase E — new components (scope 0.1)

Same checklist as Phase D.

- [ ] **`Add button`**
- [ ] **`Add text`**
- [ ] **`Add cover`**
- [ ] **`Add light (default schema)`** (decision 7: deserializing `Light`
  rejects `schema` values other than `basic`)

### Phase F — discovery

- [ ] **`Add Component enum`** (tagged with `platform`, one variant per
  platform, plus `Removed { platform }` for removal entries (decision 6) and,
  with feature `json`, `Other { platform, config: serde_json::Map }` for unknown
  platforms and non-basic lights (decisions 3, 7). `Serialize` always;
  `Deserialize` only with `json` (buffers each entry as a `serde_json::Value`).
- [ ] **`Add DeviceDiscovery`** (`device`, `origin`, `components` serialized as
  `components`; shared options: `availability`, `availability_mode`,
  `command_topic`, `encoding`, `message_expiry_interval`, `qos`, `state_topic`
  (decision 8). `Deserialize` only with `json`.)
- [ ] **`Add topic helpers and HA status constants`**
  One helper for the documented format
  `<prefix>/<component>/[<node_id>/]<object_id>/config`, where `<component>` is
  a platform or `device` (decision 9), `homeassistant/status`,
  `online`/`offline`, default prefix `homeassistant`. Validate or document the
  `[a-zA-Z0-9_-]` restriction for `node_id`/`object_id`.
- [ ] **`Add abbreviation expansion`**
  Port `ABBREVIATIONS` (275), `DEVICE_ABBREVIATIONS` (11),
  `ORIGIN_ABBREVIATIONS` (3) from `abbreviations.py`; apply like
  `discovery.py` (root, each availability entry, each `cmps` entry; device/origin
  tables only to those blocks) and the `~` base topic substitution shown in
  the MQTT docs. Abbreviated fixtures must equal their expanded form. Consider a test that fails when the table size drifts.

### Phase G — polish

- [ ] **`Add crate docs and test README example`**
  `#![doc = include_str!("../README.md")]` or an equivalent tested example; enable
  `#![warn(missing_docs)]`.
- [ ] **`Audit all types against HA`**
  Re-check every enum and field list against `dev` on the day before the 0.1
  release; update the baseline date at the top of this file.

---

## 3. Decisions

Decided with the user on 2026-09-28. General rule: the Home Assistant MQTT docs
are the source of truth; the core source only fills gaps the docs leave open.

0. **Reading stays in scope.** The crate serializes *and* deserializes, for
   debugging and consuming payloads of other devices.
1. **Input aliases:** yes. `string_enum!` accepts input-only aliases (e.g. U+00B5
   `µ` spellings, `VAr`); serialization always emits the canonical string.
2. **Units mirror HA's classes** (revised the same day): one closed enum per
   `UnitOf*` class, wrapped by `Unit` (one variant per class, the plain constants,
   `Other(String)`), `From<UnitOf*> for Unit`. When reading, shared strings
   resolve by precedence: `m`, `mm`, `cm`, `in` → length; `dB` → sound pressure.
   A test pins the set of shared strings.
3. **Unknown platforms in `cmps`:** fallback `Component::Other { platform, config }`
   holding a `serde_json::Map`, feature `json`. Without `json`, `Component` and
   `DeviceDiscovery` are serialize-only.
4. **Docs vs source on defaults:** follow the docs: `number` `min` default 1;
   `sensor` `expire_after` default 0. Doc comments mention the differing source
   value. Device `name` (docs contradict each other; HA only logs when it is
   missing): `Option<String>`, but the builder requires it.
4a. **Builders enforce, reading tolerates.** What HA requires or the docs call
   mandatory is enforced by the builders (e.g. device `name`, at least one of
   `identifiers`/`connections`). Deserialization stays lenient so payloads HA
   would reject can still be inspected.
5. **String-or-list fields** (`identifiers`, `group`): plain `Vec<String>`,
   skipped when empty. Because HA accepts a single string, deserialization also
   accepts one. `connections` (list only, HA default `[]`) is also a plain
   `Vec`.
6. **Removal entries:** `Component::Removed { platform }` (always available; also
   useful when writing).
7. **Light schemas:** only the basic schema is modeled. Other schemas go to
   `Component::Other` inside `cmps`; a standalone `Light` rejects them.
8. **`DeviceDiscovery` shared options:** the documented list plus
   `message_expiry_interval` (source).
9. **Device topic:** the docs define `<component>` as a platform "or `device` in
   case of device discovery", so the general format with optional `node_id`
   applies to both.
10. **Legacy availability fields:** dropped (`availability_topic`,
    `availability_template`, `payload_available`, `payload_not_available` at the
    top level). Only the `availability` list is modeled.
11. **Closed enums** (`EntityCategory`, `AvailabilityMode`, `NumberMode`, light
    `schema`): also `string_enum!`.
12. **`message_expiry_interval`:** untagged enum of integer seconds or a time
    period map.
13. **Builders:** required fields have no builder default; `build()` fails when
    they are missing.
14. **Nullable `name`:** HA distinguishes an absent `name` (platform default)
    from `null` (only the device name). Entity `name` fields are
    `Option<EntityName>` (`Null` or `Name(String)`, `From<&str>`).
15. **Lists where absent equals empty** (`availability`, `group`, device
    `identifiers`/`connections`) are plain `Vec`s, skipped when empty.
