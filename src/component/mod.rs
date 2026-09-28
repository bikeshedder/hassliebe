//! One module per MQTT platform, each with its config struct.
//!
//! <https://www.home-assistant.io/integrations/#search/mqtt>

#[cfg(feature = "json")]
use serde::{Deserialize, Deserializer, ser::SerializeMap};
use serde::{Serialize, Serializer};

pub mod binary_sensor;
pub mod button;
pub mod cover;
pub mod light;
pub mod number;
pub mod select;
pub mod sensor;
pub mod switch;
pub mod text;

/// Implemented by every platform config struct.
pub trait Platform {
    /// The platform name Home Assistant uses in discovery topics and in the
    /// `platform` key of device discovery components, e.g. `"sensor"`.
    const PLATFORM: &'static str;
}

macro_rules! components {
    ($($variant:ident($module:ident::$ty:ident),)*) => {
        /// One entry of the `components` map in device discovery: a platform
        /// config tagged with its `platform`.
        ///
        /// Serializes as the platform struct with an added `"platform"` key.
        /// Deserializing needs the `json` feature: each entry is read as a JSON
        /// object first, so that removal entries and platforms this crate does
        /// not model can be kept.
        ///
        /// <https://www.home-assistant.io/integrations/mqtt/#device-discovery-payload>
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        #[expect(
            clippy::large_enum_variant,
            reason = "all variants are large config structs; boxing would only complicate matching"
        )]
        pub enum Component {
            $(
                #[doc = concat!("A [`", stringify!($ty), "`](", stringify!($module), "::", stringify!($ty), ").")]
                $variant($module::$ty),
            )*
            /// An entry with only a `platform`, which tells Home Assistant to
            /// remove this component from the device.
            Removed {
                /// The platform of the removed component.
                platform: String,
            },
            /// A platform this crate does not model, or a light with a schema
            /// other than `basic`. `config` holds all keys except `platform`.
            #[cfg(feature = "json")]
            Other {
                /// The platform, e.g. `"climate"`.
                platform: String,
                /// The component config without the `platform` key.
                config: serde_json::Map<String, serde_json::Value>,
            },
        }

        impl Component {
            /// Returns the platform name, e.g. `"sensor"`.
            pub fn platform(&self) -> &str {
                match self {
                    $(Self::$variant(_) => <$module::$ty as Platform>::PLATFORM,)*
                    Self::Removed { platform } => platform,
                    #[cfg(feature = "json")]
                    Self::Other { platform, .. } => platform,
                }
            }
        }

        $(
            impl From<$module::$ty> for Component {
                fn from(config: $module::$ty) -> Self {
                    Self::$variant(config)
                }
            }
        )*

        impl Serialize for Component {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                #[derive(Serialize)]
                struct Tagged<'a, T> {
                    platform: &'a str,
                    #[serde(flatten)]
                    config: &'a T,
                }

                let platform = self.platform();
                match self {
                    $(Self::$variant(config) => Tagged { platform, config }.serialize(serializer),)*
                    Self::Removed { .. } => Tagged { platform, config: &() }.serialize(serializer),
                    #[cfg(feature = "json")]
                    Self::Other { config, .. } => {
                        let mut map = serializer.serialize_map(Some(config.len() + 1))?;
                        map.serialize_entry("platform", platform)?;
                        for (key, value) in config {
                            if key != "platform" {
                                map.serialize_entry(key, value)?;
                            }
                        }
                        map.end()
                    }
                }
            }
        }

        #[cfg(feature = "json")]
        impl<'de> Deserialize<'de> for Component {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                use serde::de::Error;
                use serde_json::Value;

                let mut config = serde_json::Map::deserialize(deserializer)?;
                let platform = match config.remove("platform") {
                    Some(Value::String(platform)) => platform,
                    Some(other) => {
                        return Err(D::Error::custom(format!(
                            "`platform` must be a string, got {other}"
                        )));
                    }
                    None => return Err(D::Error::missing_field("platform")),
                };
                if config.is_empty() {
                    return Ok(Self::Removed { platform });
                }
                let non_basic_light = platform == <light::Light as Platform>::PLATFORM
                    && config
                        .get("schema")
                        .is_some_and(|schema| schema.as_str() != Some("basic"));
                if non_basic_light {
                    return Ok(Self::Other { platform, config });
                }
                match platform.as_str() {
                    $(
                        <$module::$ty as Platform>::PLATFORM => {
                            serde_json::from_value(Value::Object(config))
                                .map(Self::$variant)
                                .map_err(D::Error::custom)
                        }
                    )*
                    _ => Ok(Self::Other { platform, config }),
                }
            }
        }
    };
}

components! {
    BinarySensor(binary_sensor::BinarySensor),
    Button(button::Button),
    Cover(cover::Cover),
    Light(light::Light),
    Number(number::Number),
    Select(select::Select),
    Sensor(sensor::Sensor),
    Switch(switch::Switch),
    Text(text::Text),
}
