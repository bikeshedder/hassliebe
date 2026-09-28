//! Field specs of the platform structs and the `component` region generator.
//!
//! Every platform lists its own fields; entity platforms also get the common
//! fields of `SCHEMA_BASE` and `MQTT_ENTITY_COMMON_SCHEMA`. Field docs are
//! short paraphrases of the Home Assistant docs.

use std::{error::Error, fmt::Write as _};

/// How a field is declared.
#[derive(Clone, Copy)]
pub enum Kind {
    /// Required: `T`.
    Req,
    /// Optional: `Option<T>`.
    Opt,
    /// A list where absent and empty are alike: `Vec<T>`, skipped when empty.
    List,
    /// Like `Vec`, but also read from a single string.
    StringList,
    /// Absent, `null` or a value: `Option<Nullable<T>>`.
    Nullable,
}

use Kind::*;

/// A field of a platform struct.
#[derive(Clone, Copy)]
pub struct Field {
    pub name: &'static str,
    pub kind: Kind,
    /// The inner type, e.g. `String` for `Option<String>`.
    pub ty: &'static str,
    pub doc: &'static [&'static str],
}

const fn field(
    name: &'static str,
    kind: Kind,
    ty: &'static str,
    doc: &'static [&'static str],
) -> Field {
    Field {
        name,
        kind,
        ty,
        doc,
    }
}

/// A platform struct.
pub struct Platform {
    /// The platform name, e.g. `binary_sensor`.
    pub name: &'static str,
    pub struct_name: &'static str,
    /// The noun used in docs, e.g. `binary sensor`.
    pub noun: &'static str,
    pub doc: &'static [&'static str],
    pub url: &'static str,
    /// The `schema` this struct models, for platforms with several schemas.
    pub schema: Option<&'static str>,
    /// `None` for platforms that are not entities (no common entity fields).
    pub entity: Option<Entity>,
    pub fields: &'static [Field],
}

/// Settings for the common entity fields.
pub struct Entity {
    pub entity_id_example: &'static str,
    pub icon_example: &'static str,
    pub default_name: &'static str,
    pub encoding_doc: &'static [&'static str],
    pub qos_doc: &'static [&'static str],
    pub entity_category_doc: &'static [&'static str],
    /// Whether the platform docs list `message_expiry_interval`.
    pub expiry_documented: bool,
    /// Whether the platform docs list `origin`.
    pub origin_documented: bool,
}

const RO_ENCODING: &[&str] = &[
    "Encoding of received payloads; an empty string disables decoding.",
    "Default: `utf-8`.",
];
const RW_ENCODING: &[&str] = &[
    "Encoding of received and published payloads; an empty string disables",
    "decoding of received payloads. Default: `utf-8`.",
];
const RO_QOS: &[&str] = &["Maximum QoS level for subscriptions. Default: `0`."];
const RW_QOS: &[&str] = &["QoS level for subscribing and publishing. Default: `0`."];
const CATEGORY: &[&str] = &["Category of the entity."];
const SENSOR_CATEGORY: &[&str] =
    &["Category of the entity. The docs only allow `diagnostic` for sensors."];

const fn entity(
    entity_id_example: &'static str,
    icon_example: &'static str,
    default_name: &'static str,
    rw: bool,
) -> Entity {
    Entity {
        entity_id_example,
        icon_example,
        default_name,
        encoding_doc: if rw { RW_ENCODING } else { RO_ENCODING },
        qos_doc: if rw { RW_QOS } else { RO_QOS },
        entity_category_doc: CATEGORY,
        expiry_documented: true,
        origin_documented: false,
    }
}

const COMMAND_TEMPLATE: Field = field(
    "command_template",
    Opt,
    "String",
    &["Template to generate the payload sent to `command_topic`."],
);
const RETAIN: Field = field(
    "retain",
    Opt,
    "bool",
    &["Publish commands with the retain flag. Default: `false`."],
);

const fn optimistic(doc: &'static [&'static str]) -> Field {
    field("optimistic", Opt, "bool", doc)
}

const OPTIMISTIC_STATE: Field = optimistic(&[
    "Assume the state changes as commanded. Default: `true` without",
    "`state_topic`, otherwise `false`.",
]);
const OPTIMISTIC_VALUE: Field = optimistic(&[
    "Assume the value changes as commanded. Default: `true` without",
    "`state_topic`, otherwise `false`.",
]);
const OPTIMISTIC_OPTION: Field = optimistic(&[
    "Assume the option changes as commanded. Default: `true` without",
    "`state_topic`, otherwise `false`.",
]);

/// Every platform struct with a `component` region.
#[rustfmt::skip]
pub const PLATFORMS: &[Platform] = &[
    Platform {
        name: "sensor",
        struct_name: "Sensor",
        noun: "sensor",
        doc: &[
            "An MQTT sensor: a read-only entity whose state is received on",
            "`state_topic`.",
        ],
        url: "https://www.home-assistant.io/integrations/sensor.mqtt/",
        schema: None,
        entity: Some(Entity {
            entity_category_doc: SENSOR_CATEGORY,
            expiry_documented: false,
            ..entity("sensor.kitchen_temperature", "mdi:thermometer", "MQTT Sensor", false)
        }),
        fields: &[
            field("device_class", Opt, "SensorDeviceClass", &[
                "Type of the sensor, which sets its icon and how its state is displayed.",
            ]),
            field("expire_after", Opt, "u32", &[
                "Seconds after the last update until the state becomes unavailable.",
                "Default: `0` (never expires).",
            ]),
            field("force_update", Opt, "bool", &[
                "Send update events even if the value has not changed. Default: `false`.",
            ]),
            field("last_reset_value_template", Opt, "String", &[
                "Template to extract the `last_reset` timestamp. Requires `state_class`",
                "`total`.",
            ]),
            field("options", Opt, "Vec<String>", &[
                "The possible states of an `enum` sensor. Requires `device_class` `enum`",
                "and must not be empty.",
            ]),
            field("state_class", Opt, "StateClass", &[
                "State class of the sensor, used for long-term statistics.",
            ]),
            field("state_topic", Req, "String", &["Topic to receive sensor values from. Required."]),
            field("suggested_display_precision", Opt, "u32", &[
                "Number of decimals to round the state to when displaying it.",
            ]),
            field("unit_of_measurement", Opt, "Unit", &["Unit of measurement of the sensor's state."]),
            field("value_template", Opt, "String", &[
                "Template to extract the value from the payload. If it fails, the",
                "previous state is kept.",
            ]),
        ],
    },
    Platform {
        name: "binary_sensor",
        struct_name: "BinarySensor",
        noun: "binary sensor",
        doc: &[
            "An MQTT binary sensor: a read-only entity that is `on` or `off`,",
            "received on `state_topic`.",
        ],
        url: "https://www.home-assistant.io/integrations/binary_sensor.mqtt/",
        schema: None,
        entity: Some(Entity {
            entity_category_doc: SENSOR_CATEGORY,
            expiry_documented: false,
            ..entity("binary_sensor.front_door", "mdi:door", "MQTT binary sensor", false)
        }),
        fields: &[
            field("device_class", Opt, "BinarySensorDeviceClass", &[
                "Type of the binary sensor, which sets its icon and how its state is",
                "displayed.",
            ]),
            field("expire_after", Opt, "u32", &[
                "Seconds after the last update until the state becomes unavailable.",
            ]),
            field("force_update", Opt, "bool", &[
                "Send update events even if the value has not changed. Default: `false`.",
            ]),
            field("off_delay", Opt, "u32", &["Seconds after which the sensor resets to `off` on its own."]),
            field("payload_off", Opt, "String", &["Payload that means `off`. Default: `OFF`."]),
            field("payload_on", Opt, "String", &["Payload that means `on`. Default: `ON`."]),
            field("state_topic", Req, "String", &["Topic to receive sensor states from. Required."]),
            field("value_template", Opt, "String", &[
                "Template to extract a value from the payload, which is compared to",
                "`payload_on` and `payload_off`.",
            ]),
        ],
    },
    Platform {
        name: "switch",
        struct_name: "Switch",
        noun: "switch",
        doc: &[
            "An MQTT switch: an entity that is switched on and off by publishing to",
            "`command_topic`.",
        ],
        url: "https://www.home-assistant.io/integrations/switch.mqtt/",
        schema: None,
        entity: Some(entity("switch.garden_irrigation", "mdi:sprinkler", "MQTT Switch", true)),
        fields: &[
            COMMAND_TEMPLATE,
            field("command_topic", Req, "String", &["Topic to publish commands to. Required."]),
            field("device_class", Opt, "SwitchDeviceClass", &["Type of the switch, which sets its icon."]),
            OPTIMISTIC_STATE,
            field("payload_off", Opt, "String", &["Payload sent to turn the switch off. Default: `OFF`."]),
            field("payload_on", Opt, "String", &["Payload sent to turn the switch on. Default: `ON`."]),
            RETAIN,
            field("state_off", Opt, "String", &[
                "Payload on `state_topic` that means off. Default: `payload_off` if set,",
                "otherwise `OFF`.",
            ]),
            field("state_on", Opt, "String", &[
                "Payload on `state_topic` that means on. Default: `payload_on` if set,",
                "otherwise `ON`.",
            ]),
            field("state_topic", Opt, "String", &["Topic to receive state updates from."]),
            field("value_template", Opt, "String", &["Template to extract the state from the payload."]),
        ],
    },
    Platform {
        name: "number",
        struct_name: "Number",
        noun: "number",
        doc: &[
            "An MQTT number: a numeric value that is set by publishing to",
            "`command_topic`.",
        ],
        url: "https://www.home-assistant.io/integrations/number.mqtt/",
        schema: None,
        entity: Some(entity("number.heating_setpoint", "mdi:thermostat", "MQTT Number", true)),
        fields: &[
            COMMAND_TEMPLATE,
            field("command_topic", Req, "String", &["Topic to publish new values to. Required."]),
            field("device_class", Opt, "NumberDeviceClass", &[
                "Type of the number, which sets its icon and how its value is displayed.",
            ]),
            field("max", Opt, "f64", &["Maximum value. Default: `100`."]),
            field("min", Opt, "f64", &[
                "Minimum value. Default: `1` per the docs (`0` in Home Assistant's source).",
            ]),
            field("mode", Opt, "NumberMode", &["How the number is shown in the UI. Default: `auto`."]),
            OPTIMISTIC_VALUE,
            field("payload_reset", Opt, "String", &[
                "Payload on `state_topic` that resets the value to unknown. Default: the string `None`.",
            ]),
            RETAIN,
            field("state_topic", Opt, "String", &["Topic to receive the current value from."]),
            field("step", Opt, "f64", &["Step size; at least `0.001`. Default: `1`."]),
            field("unit_of_measurement", Opt, "Unit", &["Unit of measurement of the value."]),
            field("value_template", Opt, "String", &["Template to extract the value from the payload."]),
        ],
    },
    Platform {
        name: "select",
        struct_name: "Select",
        noun: "select",
        doc: &[
            "An MQTT select: one option out of a fixed list, chosen by publishing to",
            "`command_topic`.",
        ],
        url: "https://www.home-assistant.io/integrations/select.mqtt/",
        schema: None,
        entity: Some(entity("select.fan_mode", "mdi:fan", "MQTT Select", true)),
        fields: &[
            COMMAND_TEMPLATE,
            field("command_topic", Req, "String", &["Topic to publish the selected option to. Required."]),
            OPTIMISTIC_OPTION,
            field("options", Req, "Vec<String>", &["The options that can be selected. Required."]),
            RETAIN,
            field("state_topic", Opt, "String", &["Topic to receive the selected option from."]),
            field("value_template", Opt, "String", &[
                "Template to extract the selected option from the payload.",
            ]),
        ],
    },
    Platform {
        name: "button",
        struct_name: "Button",
        noun: "button",
        doc: &[
            "An MQTT button: pressing it publishes `payload_press` to",
            "`command_topic`.",
        ],
        url: "https://www.home-assistant.io/integrations/button.mqtt/",
        schema: None,
        entity: Some(Entity {
            encoding_doc: &["Encoding of published payloads. Default: `utf-8`."],
            qos_doc: &["QoS level for publishing. Default: `0`."],
            ..entity("button.restart_bedroom_switch", "mdi:restart", "MQTT Button", true)
        }),
        fields: &[
            COMMAND_TEMPLATE,
            field("command_topic", Req, "String", &["Topic to publish button presses to. Required."]),
            field("device_class", Opt, "ButtonDeviceClass", &["Type of the button, which sets its icon."]),
            field("payload_press", Opt, "String", &[
                "Payload sent when the button is pressed. Default: `PRESS`.",
            ]),
            field("retain", Opt, "bool", &["Publish presses with the retain flag. Default: `false`."]),
        ],
    },
    Platform {
        name: "text",
        struct_name: "Text",
        noun: "text",
        doc: &["An MQTT text: a text value that is set by publishing to `command_topic`."],
        url: "https://www.home-assistant.io/integrations/text.mqtt/",
        schema: None,
        entity: Some(entity("text.remote_lcd_screen", "mdi:ab-testing", "MQTT Text", true)),
        fields: &[
            COMMAND_TEMPLATE,
            field("command_topic", Req, "String", &["Topic to publish the text to. Required."]),
            field("max", Opt, "u32", &["Maximum length of the text; at most `255`. Default: `255`."]),
            field("min", Opt, "u32", &["Minimum length of the text. Default: `0`."]),
            field("mode", Opt, "TextMode", &["How the text is shown in the UI. Default: `text`."]),
            field("optimistic", Opt, "bool", &[
                "Assume the text changes as commanded. Not in the text docs; accepted",
                "by Home Assistant. Default: `true` without `state_topic`, otherwise",
                "`false`.",
            ]),
            field("pattern", Opt, "String", &["Regular expression the text must match."]),
            RETAIN,
            field("state_topic", Opt, "String", &["Topic to receive the text from."]),
            field("value_template", Opt, "String", &["Template to extract the text from the payload."]),
        ],
    },
    Platform {
        name: "cover",
        struct_name: "Cover",
        noun: "cover",
        doc: &["An MQTT cover, such as a blind, shutter or garage door."],
        url: "https://www.home-assistant.io/integrations/cover.mqtt/",
        schema: None,
        entity: Some(entity("cover.living_room", "mdi:blinds", "MQTT Cover", true)),
        fields: &[
            field("command_topic", Opt, "String", &["Topic to publish open, close and stop commands to."]),
            field("device_class", Opt, "CoverDeviceClass", &[
                "Type of the cover, which sets its icon and the text shown for its",
                "states.",
            ]),
            field("optimistic", Opt, "bool", &[
                "Assume the state changes as commanded. Default: `false`, or `true`",
                "without `state_topic` and `position_topic`.",
            ]),
            field("payload_close", Nullable, "String", &[
                "Payload sent to close the cover; [`Nullable::Null`] disables the command.",
                "Default: `CLOSE`.",
            ]),
            field("payload_open", Nullable, "String", &[
                "Payload sent to open the cover; [`Nullable::Null`] disables the command.",
                "Default: `OPEN`.",
            ]),
            field("payload_stop", Nullable, "String", &[
                "Payload sent to stop the cover; [`Nullable::Null`] disables the command.",
                "Default: `STOP`.",
            ]),
            field("payload_stop_tilt", Nullable, "String", &[
                "Payload sent to stop tilting; [`Nullable::Null`] disables the",
                "command. Default: `stop` per the docs (`STOP` in Home Assistant's",
                "source).",
            ]),
            field("position_closed", Opt, "i32", &["Position that means fully closed. Default: `0`."]),
            field("position_open", Opt, "i32", &["Position that means fully open. Default: `100`."]),
            field("position_template", Opt, "String", &[
                "Template to extract the position from messages on `position_topic`.",
            ]),
            field("position_topic", Opt, "String", &["Topic to receive the position from."]),
            RETAIN,
            field("set_position_template", Opt, "String", &[
                "Template to generate the payload sent to `set_position_topic`.",
            ]),
            field("set_position_topic", Opt, "String", &[
                "Topic to publish position commands to. Requires `position_topic`.",
            ]),
            field("state_closed", Opt, "String", &["Payload on `state_topic` that means closed. Default: `closed`."]),
            field("state_closing", Opt, "String", &["Payload on `state_topic` that means closing. Default: `closing`."]),
            field("state_open", Opt, "String", &["Payload on `state_topic` that means open. Default: `open`."]),
            field("state_opening", Opt, "String", &["Payload on `state_topic` that means opening. Default: `opening`."]),
            field("state_stopped", Opt, "String", &["Payload on `state_topic` that means stopped. Default: `stopped`."]),
            field("state_topic", Opt, "String", &["Topic to receive the state from."]),
            field("tilt_closed_value", Opt, "i32", &["Tilt value sent to close the tilt. Default: `0`."]),
            field("tilt_command_template", Opt, "String", &[
                "Template to generate the payload sent to `tilt_command_topic`.",
            ]),
            field("tilt_command_topic", Opt, "String", &["Topic to publish tilt commands to."]),
            field("tilt_max", Opt, "i32", &["Maximum tilt value. Default: `100`."]),
            field("tilt_min", Opt, "i32", &["Minimum tilt value. Default: `0`."]),
            field("tilt_opened_value", Opt, "i32", &["Tilt value sent to open the tilt. Default: `100`."]),
            field("tilt_optimistic", Opt, "bool", &[
                "Assume the tilt changes as commanded. Default: `true` without",
                "`tilt_status_topic`, otherwise `false`.",
            ]),
            field("tilt_status_template", Opt, "String", &[
                "Template to extract the tilt from messages on `tilt_status_topic`.",
            ]),
            field("tilt_status_topic", Opt, "String", &["Topic to receive the tilt from."]),
            field("value_template", Opt, "String", &[
                "Template to extract the state from messages on `state_topic`.",
            ]),
        ],
    },
    Platform {
        name: "light",
        struct_name: "Light",
        noun: "light",
        doc: &["An MQTT light with the default (`basic`) schema."],
        url: "https://www.home-assistant.io/integrations/light.mqtt/#default-schema",
        schema: Some("basic"),
        entity: Some(entity("light.office", "mdi:ceiling-light", "MQTT Light", true)),
        fields: LIGHT_BASIC,
    },
    Platform {
        name: "date",
        struct_name: "Date",
        noun: "date",
        doc: &["An MQTT date: a date value that is set by publishing to `command_topic`."],
        url: "https://www.home-assistant.io/integrations/date.mqtt/",
        schema: None,
        entity: Some(entity("date.scheduled_task", "mdi:calendar", "MQTT Date", true)),
        fields: &[
            COMMAND_TEMPLATE,
            field("command_topic", Req, "String", &[
                "Topic to publish the date to, in ISO format. Required.",
            ]),
            field("optimistic", Opt, "bool", &[
                "Assume the date changes as commanded. Not in the date docs; accepted",
                "by Home Assistant. Default: `true` without `state_topic`, otherwise",
                "`false`.",
            ]),
            RETAIN,
            field("state_topic", Opt, "String", &[
                "Topic to receive the date from, as a parsable date string.",
            ]),
            field("value_template", Opt, "String", &["Template to extract the date from the payload."]),
        ],
    },
];

macro_rules! color_fields {
    ($prefix:literal, $what:literal) => {
        [
            field(
                concat!($prefix, "_command_template"),
                Opt,
                "String",
                &[concat!(
                    "Template to generate the payload sent to `",
                    $prefix,
                    "_command_topic`."
                )],
            ),
            field(
                concat!($prefix, "_command_topic"),
                Opt,
                "String",
                &[concat!("Topic to publish ", $what, " commands to.")],
            ),
            field(
                concat!($prefix, "_state_topic"),
                Opt,
                "String",
                &[concat!("Topic to receive ", $what, " updates from.")],
            ),
            field(
                concat!($prefix, "_value_template"),
                Opt,
                "String",
                &[concat!(
                    "Template to extract the ",
                    $what,
                    " value from `",
                    $prefix,
                    "_state_topic`."
                )],
            ),
        ]
    };
}

const BRIGHTNESS: [Field; 4] = color_fields!("brightness", "brightness");
const COLOR_TEMP: [Field; 4] = color_fields!("color_temp", "color temperature");
const EFFECT: [Field; 4] = color_fields!("effect", "effect");
const HS: [Field; 4] = color_fields!("hs", "hue and saturation");
const RGB: [Field; 4] = color_fields!("rgb", "RGB color");
const RGBW: [Field; 4] = color_fields!("rgbw", "RGBW color");
const RGBWW: [Field; 4] = color_fields!("rgbww", "RGBWW color");
const XY: [Field; 4] = color_fields!("xy", "XY color");

#[rustfmt::skip]
const LIGHT_BASIC: &[Field] = &[
    BRIGHTNESS[0], BRIGHTNESS[1], BRIGHTNESS[2], BRIGHTNESS[3],
    field("brightness_scale", Opt, "u32", &[
        "Brightness value that means 100 %; at least `1`. Default: `255`.",
    ]),
    field("color_mode_state_topic", Opt, "String", &["Topic to receive color mode updates from."]),
    field("color_mode_value_template", Opt, "String", &[
        "Template to extract the color mode from `color_mode_state_topic`.",
    ]),
    COLOR_TEMP[0], COLOR_TEMP[1], COLOR_TEMP[2], COLOR_TEMP[3],
    field("color_temp_kelvin", Opt, "bool", &[
        "Use Kelvin instead of mireds for color temperatures. Default: `false`.",
    ]),
    field("command_topic", Req, "String", &["Topic to publish on/off commands to. Required."]),
    EFFECT[0], EFFECT[1], EFFECT[2], EFFECT[3],
    field("effect_list", StringList, "String", &[
        "Supported effects. Home Assistant also accepts a single string.",
    ]),
    HS[0], HS[1], HS[2], HS[3],
    field("max_kelvin", Opt, "u32", &["Maximum color temperature in Kelvin. Default: `6535`."]),
    field("max_mireds", Opt, "u32", &["Maximum color temperature in mireds."]),
    field("min_kelvin", Opt, "u32", &["Minimum color temperature in Kelvin. Default: `2000`."]),
    field("min_mireds", Opt, "u32", &["Minimum color temperature in mireds."]),
    field("on_command_type", Opt, "OnCommandType", &[
        "When to send `payload_on` along with other commands. Default: `last`.",
    ]),
    OPTIMISTIC_STATE,
    field("payload_off", Opt, "String", &["Payload sent to turn the light off. Default: `OFF`."]),
    field("payload_on", Opt, "String", &["Payload sent to turn the light on. Default: `ON`."]),
    RETAIN,
    RGB[0], RGB[1], RGB[2], RGB[3],
    RGBW[0], RGBW[1], RGBW[2], RGBW[3],
    RGBWW[0], RGBWW[1], RGBWW[2], RGBWW[3],
    field("schema", Opt, "LightSchema", &[
        "Schema of the light config. Only `basic` is modeled. Default: `basic`.",
    ]),
    field("state_topic", Opt, "String", &["Topic to receive on/off state updates from."]),
    field("state_value_template", Opt, "String", &[
        "Template to extract the on/off state from `state_topic`.",
    ]),
    field("white_command_topic", Opt, "String", &[
        "Topic to publish commands to switch to white mode with a brightness.",
    ]),
    field("white_scale", Opt, "u32", &["White value that means 100 %; at least `1`. Default: `255`."]),
    XY[0], XY[1], XY[2], XY[3],
];

/// Types imported from `crate::common`, keyed by the outer type name.
const IMPORTS: &[(&str, &str)] = &[
    (
        "Availability",
        "availability::{Availability, AvailabilityMode}",
    ),
    ("Device", "device::Device"),
    ("EntityCategory", "entity_category::EntityCategory"),
    (
        "MessageExpiryInterval",
        "message_expiry_interval::MessageExpiryInterval",
    ),
    ("Nullable", "nullable::Nullable"),
    ("Origin", "origin::Origin"),
    ("Qos", "qos::Qos"),
    ("StateClass", "state_class::StateClass"),
    ("Unit", "unit::Unit"),
];

const BUILTINS: &[&str] = &["String", "Vec", "bool", "u32", "i32", "f64"];

/// A field with an owned doc, for the common fields built at runtime.
struct Owned {
    name: &'static str,
    kind: Kind,
    ty: &'static str,
    doc: Vec<String>,
}

fn owned(field: &Field) -> Owned {
    Owned {
        name: field.name,
        kind: field.kind,
        ty: field.ty,
        doc: field.doc.iter().map(|line| (*line).to_owned()).collect(),
    }
}

fn lines(doc: &[&str]) -> Vec<String> {
    doc.iter().map(|line| (*line).to_owned()).collect()
}

/// The fields of `SCHEMA_BASE` and `MQTT_ENTITY_COMMON_SCHEMA`.
fn common(platform: &Platform, entity: &Entity) -> Vec<Owned> {
    let noun = platform.noun;
    let docs = platform.noun;
    let not_in_docs = |what: &str| {
        vec![
            format!("{what} Not in the {docs}"),
            "docs; accepted by Home Assistant for all MQTT entities.".to_owned(),
        ]
    };
    let common = |name, kind, ty, doc| Owned {
        name,
        kind,
        ty,
        doc,
    };
    vec![
        common(
            "availability",
            List,
            "Availability",
            lines(&["Topics that report the availability of the entity."]),
        ),
        common(
            "availability_mode",
            Opt,
            "AvailabilityMode",
            lines(&["How the `availability` topics are combined. Default: `latest`."]),
        ),
        common(
            "default_entity_id",
            Opt,
            "String",
            vec![
                "Entity ID to use instead of one generated from the name, e.g.".to_owned(),
                format!("`{}`.", entity.entity_id_example),
            ],
        ),
        common(
            "device",
            Opt,
            "Device",
            vec![format!(
                "The device this {noun} belongs to. Only works when `unique_id` is set."
            )],
        ),
        common(
            "enabled_by_default",
            Opt,
            "bool",
            lines(&["Whether the entity is enabled when first added. Default: `true`."]),
        ),
        common("encoding", Opt, "String", lines(entity.encoding_doc)),
        common(
            "entity_category",
            Opt,
            "EntityCategory",
            lines(entity.entity_category_doc),
        ),
        common(
            "entity_picture",
            Opt,
            "String",
            lines(&["URL of a picture for the entity."]),
        ),
        common(
            "group",
            StringList,
            "String",
            vec![
                format!("Unique IDs of member entities, making this a group {noun}. Home"),
                "Assistant also accepts a single string.".to_owned(),
            ],
        ),
        common(
            "icon",
            Opt,
            "String",
            vec![format!(
                "Icon for the entity, e.g. `{}`.",
                entity.icon_example
            )],
        ),
        common(
            "json_attributes_template",
            Opt,
            "String",
            lines(&[
                "Template to extract a JSON dictionary of attributes from messages on",
                "`json_attributes_topic`.",
            ]),
        ),
        common(
            "json_attributes_topic",
            Opt,
            "String",
            lines(&["Topic whose JSON dictionary payload is added as entity attributes."]),
        ),
        common(
            "message_expiry_interval",
            Opt,
            "MessageExpiryInterval",
            if entity.expiry_documented {
                lines(&["Message expiry interval for command payloads."])
            } else {
                not_in_docs("Message expiry interval for command payloads.")
            },
        ),
        common(
            "name",
            Nullable,
            "String",
            vec![
                format!("Name of the {noun}; [`Nullable::Null`] if only the device name is"),
                format!("relevant. Default: `{}`.", entity.default_name),
            ],
        ),
        common(
            "origin",
            Opt,
            "Origin",
            if entity.origin_documented {
                lines(&["The application that sent the discovery message."])
            } else {
                not_in_docs("The application that sent the discovery message.")
            },
        ),
        common("qos", Opt, "Qos", lines(entity.qos_doc)),
        common(
            "unique_id",
            Opt,
            "String",
            vec![
                format!("ID that uniquely identifies this {noun}. Required for device discovery"),
                "and for `device`.".to_owned(),
            ],
        ),
        common(
            "visible_by_default",
            Opt,
            "bool",
            lines(&[
                "Whether the entity is visible on dashboards when first added. Default:",
                "`true`.",
            ]),
        ),
    ]
}

/// Renders the imports, the struct, its builder impl and its `Platform` impl.
pub fn render(name: &str) -> Result<String, Box<dyn Error>> {
    let platform = PLATFORMS
        .iter()
        .find(|platform| platform.name == name)
        .ok_or_else(|| format!("unknown platform {name}"))?;
    let mut fields: Vec<Owned> = platform.fields.iter().map(owned).collect();
    if let Some(entity) = &platform.entity {
        for field in common(platform, entity) {
            if fields.iter().any(|own| own.name == field.name) {
                return Err(format!("{name}: field {} is also a common field", field.name).into());
            }
            fields.push(field);
        }
    }
    fields.sort_by_key(|field| field.name);

    // Every type name used by a field, e.g. `Vec<Unit>` → `Vec`, `Unit`.
    let mut names: Vec<&str> = fields
        .iter()
        .flat_map(|field| field.ty.split(|c: char| !c.is_alphanumeric() && c != '_'))
        .filter(|name| !name.is_empty())
        .collect();
    if fields.iter().any(|field| matches!(field.kind, Nullable)) {
        names.push("Nullable");
    }
    let mut imports: Vec<&str> = IMPORTS
        .iter()
        .filter(|(ty, _)| names.contains(ty))
        .map(|(_, path)| *path)
        .collect();
    imports.sort_unstable();
    imports.dedup();
    let local_enums = names.iter().any(|name| {
        !BUILTINS.contains(name)
            && !IMPORTS.iter().any(|(import, _)| name == import)
            && *name != "AvailabilityMode"
    });

    let mut out = String::new();
    writeln!(out, "use serde::{{Deserialize, Serialize}};\n")?;
    writeln!(out, "use super::Platform;")?;
    writeln!(out, "use crate::{{\n    common::{{")?;
    for import in imports {
        writeln!(out, "        {import},")?;
    }
    writeln!(out, "    }},")?;
    if local_enums {
        writeln!(out, "    string_enum::string_enum,")?;
    }
    writeln!(out, "}};\n")?;
    for line in platform.doc {
        writeln!(out, "/// {line}")?;
    }
    writeln!(out, "///\n/// <{}>", platform.url)?;
    writeln!(
        out,
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]"
    )?;
    writeln!(
        out,
        "#[cfg_attr(feature = \"builder\", derive(derive_builder::Builder))]"
    )?;
    writeln!(
        out,
        "#[cfg_attr(feature = \"builder\", builder(setter(into, strip_option)))]"
    )?;
    let struct_name = platform.struct_name;
    writeln!(out, "pub struct {struct_name} {{")?;
    for field in &fields {
        for line in &field.doc {
            writeln!(out, "    /// {line}")?;
        }
        let ty = field.ty;
        let (serde, rust_ty) = match field.kind {
            Req => {
                writeln!(out, "    pub {}: {ty},", field.name)?;
                continue;
            }
            Opt => (
                "default, skip_serializing_if = \"Option::is_none\"",
                format!("Option<{ty}>"),
            ),
            List => (
                "default, skip_serializing_if = \"Vec::is_empty\"",
                format!("Vec<{ty}>"),
            ),
            StringList => (
                "default, skip_serializing_if = \"Vec::is_empty\", deserialize_with = \"crate::common::string_list::deserialize\"",
                format!("Vec<{ty}>"),
            ),
            Nullable => (
                "default, skip_serializing_if = \"Option::is_none\", deserialize_with = \"crate::common::nullable::deserialize\"",
                format!("Option<Nullable<{ty}>>"),
            ),
        };
        writeln!(out, "    #[serde({serde})]")?;
        writeln!(
            out,
            "    #[cfg_attr(feature = \"builder\", builder(default))]"
        )?;
        writeln!(out, "    pub {}: {rust_ty},", field.name)?;
    }
    writeln!(out, "}}\n")?;
    writeln!(out, "#[cfg(feature = \"builder\")]\nimpl {struct_name} {{")?;
    writeln!(out, "    /// Returns a builder for [`{struct_name}`].")?;
    writeln!(
        out,
        "    pub fn builder() -> {struct_name}Builder {{\n        {struct_name}Builder::default()\n    }}\n}}\n"
    )?;
    writeln!(out, "impl Platform for {struct_name} {{")?;
    writeln!(out, "    const PLATFORM: &'static str = \"{name}\";")?;
    if let Some(schema) = platform.schema {
        writeln!(
            out,
            "    const SCHEMA: Option<&'static str> = Some(\"{schema}\");"
        )?;
    }
    writeln!(out, "}}")?;
    Ok(out)
}
