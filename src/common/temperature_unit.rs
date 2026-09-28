//! The temperature unit of climate and water heater devices.

use crate::string_enum::string_enum;

string_enum! {
    /// Temperature unit of a device, `C` or `F`. Home Assistant also accepts
    /// lowercase letters; they are read as aliases.
    ///
    /// Not the same as [`UnitOfTemperature`](crate::unit::UnitOfTemperature),
    /// whose values (`°C`, `°F`) Home Assistant rejects here.
    ///
    /// <https://www.home-assistant.io/integrations/climate.mqtt/#temperature_unit>
    ///
    /// Source: `temperature_unit` in `homeassistant/helpers/config_validation.py`
    pub enum TemperatureUnit {
        /// Degrees Celsius.
        Celsius = "C" | "c",
        /// Degrees Fahrenheit.
        Fahrenheit = "F" | "f",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn serde() {
        assert_eq!(
            serde_json::to_string(&TemperatureUnit::Celsius).unwrap(),
            r#""C""#
        );
        assert_eq!(
            serde_json::from_str::<TemperatureUnit>(r#""f""#).unwrap(),
            TemperatureUnit::Fahrenheit
        );
    }
}
