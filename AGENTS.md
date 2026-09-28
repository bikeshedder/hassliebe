# AGENTS.md

Instructions for coding agents working on this repository. Read `ARCHITECTURE.md`
before making structural changes.

## What this crate is

`hassliebe` provides typed Rust representations of the **Home Assistant MQTT
discovery payloads**: plain structs and enums with `serde` derives, plus a few pure
helper functions (topic construction, abbreviation expansion). That is all.

The crate is used by applications that announce devices to Home Assistant and by
tools that read discovery payloads. It must be useful with *any* MQTT client.

## Environment

You are usually running inside the devcontainer defined in
`.devcontainer/devcontainer.json` (Debian trixie, `mcr.microsoft.com/devcontainers/rust`).

- The repository is mounted at `/workspace`; nothing else from the host is available.
- You run as `vscode` with passwordless `sudo`. The container is disposable: anything
  installed or changed outside `/workspace` and `/home/vscode/.claude` is lost on
  rebuild. Tooling that should persist goes into `.devcontainer/devcontainer.json`.
- `cargo-insta` is not preinstalled. Install it with `cargo install cargo-insta --locked`
  when you need to review snapshots.

## Hard rules

- **No I/O, no async, no MQTT client.** Never add `tokio`, `rumqttc`, `paho-mqtt`,
  `reqwest` or similar. If a change seems to need them, it belongs in another crate.
- **Minimal dependencies.** Allowed: `serde` (required), `serde_json` (optional,
  feature `json`), `derive_builder` (optional, feature `builder`). Dev-dependencies
  are fine (`serde_json`, `insta`, `pretty_assertions`). Ask before adding anything else.
- **Never invent fields, enum values or defaults.** Every field must exist in the
  Home Assistant documentation or source (see "Source of truth"). If the docs
  contradict the source, follow the docs and mention the source value in the doc
  comment. If the docs are ambiguous, stop and ask instead of guessing.
- **No `#[serde(deny_unknown_fields)]`.** Home Assistant adds fields constantly;
  deserialization must tolerate unknown keys.
- **Serialize full key names, never abbreviations.** Abbreviations are only
  accepted on input (see `ARCHITECTURE.md`).
- Do not bump the crate version or publish. Do not edit `CHANGELOG.md` release
  sections, only `## [Unreleased]`.

## Source of truth

In this order:

1. The per-platform MQTT docs, e.g. `https://www.home-assistant.io/integrations/sensor.mqtt/`
   and the general MQTT discovery docs at `https://www.home-assistant.io/integrations/mqtt/`.
2. Home Assistant core source on GitHub (`home-assistant/core`, branch `dev`):
   - `homeassistant/components/mqtt/abbreviations.py` (abbreviation tables)
   - `homeassistant/components/mqtt/<platform>.py` (config schemas, defaults)
   - `homeassistant/const.py` (units of measurement)
   - `homeassistant/components/<platform>/const.py` or `__init__.py` (device classes)

When you add or change a type, put the docs URL in its doc comment. When you rely on
the core source, mention the file in the commit message body as a single line
(e.g. `Source: homeassistant/components/sensor/const.py`).

## Commands

Run all of these before declaring a task done. All must pass.

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo doc --no-deps --all-features   # must build without warnings
```

Snapshot tests use `insta`. New or changed snapshots are written as `*.snap.new`.
Inspect every new snapshot and check it against the Home Assistant docs before
accepting it with `cargo insta accept`. Never accept snapshots blindly.

## Code conventions

- Rust edition 2024, MSRV as in `Cargo.toml` (`rust-version`). Do not use features
  newer than that. Until the first release the MSRV may be raised to the latest
  stable; after that, raising it needs the user's approval.
- One module per platform in `src/component/` (e.g. `sensor.rs`), containing the
  config struct and the platform-specific device class enum.
- Struct fields are ordered **alphabetically**, like the Home Assistant docs.
- All optional fields: `Option<T>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`.
  Exception: fields HA accepts as a string or a list (`identifiers`, `group`) are
  `Vec<String>`, skipped when empty, and also deserialize from a single string.
- Derive at least `Clone, Debug, Default, PartialEq, Serialize, Deserialize` on
  structs; add `#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]`
  with `#[builder(default, setter(into, strip_option))]`. Required fields get no
  builder default, so `build()` fails when they are missing.
- String-valued enums (device classes, units, state classes, …) are generated with
  the crate's `string_enum!` macro so they get an `Other(String)` variant and stable
  serde behaviour. This includes closed sets like `EntityCategory`. Legacy
  spellings HA still accepts become input-only aliases. Do not hand-write their
  serde impls.
- Every public item has a doc comment. Field docs are short paraphrases of the
  Home Assistant docs (do not copy long passages), noting the HA default if any.
- No `unwrap()`/`expect()` in library code. No `unsafe`.

## Tests

- Every component: at least one snapshot test of a realistic, fully populated
  payload and one of a minimal payload.
- Every component: a serialize → deserialize round-trip test.
- Deserialization fixtures live in `tests/fixtures/*.json`. Prefer real-world
  payloads (examples from the HA docs, Zigbee2MQTT, Tasmota, ESPHome) and note
  their origin in a comment in the test.
- Abbreviated fixtures must deserialize to the same value as their expanded form.

## Workflow

- Work in small, reviewable steps: one component or one concern per commit.
- Commit messages: one short line, imperative mood, capitalized, no prefix
  (e.g. `Add Origin`, `Serialize Qos as integers`). Add a body only when really
  necessary. No `Co-Authored-By` or other attribution trailers.
- Add a line to `## [Unreleased]` in `CHANGELOG.md` for user-visible changes.
- The current work plan lives in `docs/plan.md`. Tick off items as you finish them
  and keep it up to date when scope changes.
