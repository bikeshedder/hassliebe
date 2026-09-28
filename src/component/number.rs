//! MQTT number: <https://www.home-assistant.io/integrations/number.mqtt/>

use serde::{Deserialize, Serialize};

use super::Platform;
use crate::{
    common::{
        availability::{Availability, AvailabilityMode},
        device::Device,
        entity_category::EntityCategory,
        message_expiry_interval::MessageExpiryInterval,
        nullable::Nullable,
        origin::Origin,
        qos::Qos,
        unit::Unit,
    },
    string_enum::string_enum,
};

/// An MQTT number: a numeric value that is set by publishing to
/// `command_topic`.
///
/// <https://www.home-assistant.io/integrations/number.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Number {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Template to generate the payload sent to `command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub command_template: Option<String>,
    /// Topic to publish new values to. Required.
    pub command_topic: String,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `number.heating_setpoint`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this number belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Type of the number, which sets its icon and how its value is displayed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device_class: Option<NumberDeviceClass>,
    /// Whether the entity is enabled when first added. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub enabled_by_default: Option<bool>,
    /// Encoding of received and published payloads; an empty string disables
    /// decoding of received payloads. Default: `utf-8`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub encoding: Option<String>,
    /// Category of the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_category: Option<EntityCategory>,
    /// URL of a picture for the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_picture: Option<String>,
    /// Unique IDs of member entities, making this a group number. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::string_list::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:thermostat`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub icon: Option<String>,
    /// Template to extract a JSON dictionary of attributes from messages on
    /// `json_attributes_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub json_attributes_template: Option<String>,
    /// Topic whose JSON dictionary payload is added as entity attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub json_attributes_topic: Option<String>,
    /// Maximum value. Default: `100`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub max: Option<f64>,
    /// Message expiry interval for command payloads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Minimum value. Default: `1` per the docs (`0` in Home Assistant's source).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub min: Option<f64>,
    /// How the number is shown in the UI. Default: `auto`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode: Option<NumberMode>,
    /// Name of the number; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Number`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// Assume the value changes as commanded. Default: `true` without
    /// `state_topic`, otherwise `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub optimistic: Option<bool>,
    /// The application that sent the discovery message. Not in the number
    /// docs; accepted by Home Assistant for all MQTT entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload on `state_topic` that resets the value to unknown. Default: the string `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_reset: Option<String>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Topic to receive the current value from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_topic: Option<String>,
    /// Step size; at least `0.001`. Default: `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub step: Option<f64>,
    /// ID that uniquely identifies this number. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Unit of measurement of the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unit_of_measurement: Option<Unit>,
    /// Template to extract the value from the payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub value_template: Option<String>,
    /// Whether the entity is visible on dashboards when first added. Default:
    /// `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub visible_by_default: Option<bool>,
}

#[cfg(feature = "builder")]
impl Number {
    /// Returns a builder for [`Number`].
    pub fn builder() -> NumberBuilder {
        NumberBuilder::default()
    }
}

impl Platform for Number {
    const PLATFORM: &'static str = "number";
}

string_enum! {
    /// How a number is shown in the UI.
    ///
    /// <https://www.home-assistant.io/integrations/number.mqtt/#mode>
    ///
    /// Source: `homeassistant/components/number/const.py`
    pub enum NumberMode {
        /// Chosen by Home Assistant.
        Auto = "auto",
        /// An input box.
        Box = "box",
        /// A slider.
        Slider = "slider",
    }
}

string_enum! {
    /// Device class of a number, which sets its icon and how its value is
    /// displayed.
    ///
    /// <https://www.home-assistant.io/integrations/number/#device-class>
    ///
    /// Source: `homeassistant/components/number/const.py`
    pub enum NumberDeviceClass {
        /// Absolute humidity.
        ///
        /// Unit of measurement: `g/m³`, `mg/m³`
        AbsoluteHumidity = "absolute_humidity",
        /// Apparent power.
        ///
        /// Unit of measurement: `mVA`, `VA`, `kVA`
        ApparentPower = "apparent_power",
        /// Air Quality Index.
        Aqi = "aqi",
        /// Area.
        ///
        /// Unit of measurement: `UnitOfArea` units
        Area = "area",
        /// Atmospheric pressure.
        ///
        /// Unit of measurement: `UnitOfPressure` units
        AtmosphericPressure = "atmospheric_pressure",
        /// Percentage of battery that is left.
        ///
        /// Unit of measurement: `%`
        Battery = "battery",
        /// Blood glucose concentration.
        ///
        /// Unit of measurement: `mg/dL`, `mmol/L`
        BloodGlucoseConcentration = "blood_glucose_concentration",
        /// Carbon Monoxide gas concentration.
        ///
        /// Unit of measurement: `ppb` (parts per billion), `ppm` (parts per million), `mg/m³`, `μg/m³`
        Co = "carbon_monoxide",
        /// Carbon Dioxide gas concentration.
        ///
        /// Unit of measurement: `ppm` (parts per million)
        Co2 = "carbon_dioxide",
        /// Conductivity.
        ///
        /// Unit of measurement: `S/cm`, `mS/cm`, `μS/cm`
        Conductivity = "conductivity",
        /// Current.
        ///
        /// Unit of measurement: `A`,  `mA`, `μA`
        Current = "current",
        /// Data rate.
        ///
        /// Unit of measurement: UnitOfDataRate
        DataRate = "data_rate",
        /// Data size.
        ///
        /// Unit of measurement: UnitOfInformation
        DataSize = "data_size",
        /// Generic distance.
        ///
        /// Unit of measurement: `LENGTH_*` units; SI /metric: `mm`, `cm`, `m`, `km`; USCS / imperial: `in`, `ft`, `yd`, `mi`
        Distance = "distance",
        /// Fixed duration.
        ///
        /// Unit of measurement: `d`, `h`, `min`, `s`, `ms`, `μs`
        Duration = "duration",
        /// Energy.
        ///
        /// Unit of measurement: `J`, `kJ`, `MJ`, `GJ`, `mWh`, `Wh`, `kWh`, `MWh`, `GWh`, `TWh`, `cal`, `kcal`, `Mcal`, `Gcal`, `thm`
        Energy = "energy",
        /// Energy distance.
        ///
        /// Unit of measurement: `kWh/100km`, `Wh/km`, `mi/kWh`, `km/kWh`
        EnergyDistance = "energy_distance",
        /// Stored energy.
        ///
        /// Unit of measurement: `J`, `kJ`, `MJ`, `GJ`, `mWh`, `Wh`, `kWh`, `MWh`, `GWh`, `TWh`, `cal`, `kcal`, `Mcal`, `Gcal`, `thm`
        EnergyStorage = "energy_storage",
        /// Frequency.
        ///
        /// Unit of measurement: `mHz`, `Hz`, `kHz`, `MHz`, `GHz`
        Frequency = "frequency",
        /// Gas.
        ///
        /// Unit of measurement: SI / metric: `L`, `m³`; USCS / imperial: `ft³`, `CCF`, `MCF`
        Gas = "gas",
        /// Relative humidity.
        ///
        /// Unit of measurement: `%`
        Humidity = "humidity",
        /// Illuminance.
        ///
        /// Unit of measurement: `lx`
        Illuminance = "illuminance",
        /// Irradiance.
        ///
        /// Unit of measurement: SI / metric: `W/m²`; USCS / imperial: `BTU/(h⋅ft²)`
        Irradiance = "irradiance",
        /// Moisture.
        ///
        /// Unit of measurement: `%`
        Moisture = "moisture",
        /// Amount of money.
        ///
        /// Unit of measurement: ISO4217 currency code
        Monetary = "monetary",
        /// Amount of NO2.
        ///
        /// Unit of measurement: `ppb` (parts per billion), `ppm` (parts per million), `μg/m³`
        NitrogenDioxide = "nitrogen_dioxide",
        /// Amount of NO.
        ///
        /// Unit of measurement: `ppb` (parts per billion), `μg/m³`
        NitrogenMonoxide = "nitrogen_monoxide",
        /// Amount of N2O.
        ///
        /// Unit of measurement: `μg/m³`
        NitrousOxide = "nitrous_oxide",
        /// Amount of O3.
        ///
        /// Unit of measurement: `ppb` (parts per billion), `ppm` (parts per million), `μg/m³`
        Ozone = "ozone",
        /// Potential hydrogen (acidity/alkalinity).
        ///
        /// Unit of measurement: Unitless
        Ph = "ph",
        /// Particulate matter <= 1 μm.
        ///
        /// Unit of measurement: `μg/m³`
        Pm1 = "pm1",
        /// Particulate matter <= 10 μm.
        ///
        /// Unit of measurement: `μg/m³`
        Pm10 = "pm10",
        /// Particulate matter <= 2.5 μm.
        ///
        /// Unit of measurement: `μg/m³`
        Pm25 = "pm25",
        /// Particulate matter <= 4 μm.
        ///
        /// Unit of measurement: `μg/m³`
        Pm4 = "pm4",
        /// Power factor.
        ///
        /// Unit of measurement: `%`, `None`
        PowerFactor = "power_factor",
        /// Power.
        ///
        /// Unit of measurement: `mW`, `W`, `kW`, `MW`, `GW`, `TW`, `BTU/h`
        Power = "power",
        /// Accumulated precipitation.
        ///
        /// Unit of measurement: UnitOfPrecipitationDepth; SI / metric: `cm`, `mm`; USCS / imperial: `in`
        Precipitation = "precipitation",
        /// Precipitation intensity.
        ///
        /// Unit of measurement: UnitOfVolumetricFlux; SI /metric: `mm/d`, `mm/h`; USCS / imperial: `in/d`, `in/h`
        PrecipitationIntensity = "precipitation_intensity",
        /// Pressure.
        ///
        /// Unit of measurement: `mbar`, `cbar`, `bar`; `mPa`, `Pa`, `hPa`, `kPa`; `inHg`; `psi`; `inH₂O`
        Pressure = "pressure",
        /// Radon.
        ///
        /// Unit of measurement: UnitOfRadiationConcentration; SI / metric: `Bq/m³`; USCS / imperial: `pCi/L`
        Radon = "radon",
        /// Reactive energy.
        ///
        /// Unit of measurement: `varh`, `kvarh`
        ReactiveEnergy = "reactive_energy",
        /// Reactive power.
        ///
        /// Unit of measurement: `mvar`, `var`, `kvar`
        ReactivePower = "reactive_power",
        /// Signal strength.
        ///
        /// Unit of measurement: `dB`, `dBm`
        SignalStrength = "signal_strength",
        /// Sound pressure.
        ///
        /// Unit of measurement: `dB`, `dBA`
        SoundPressure = "sound_pressure",
        /// Generic speed.
        ///
        /// Unit of measurement: `SPEED_*` units or `UnitOfVolumetricFlux`; SI /metric: `mm/d`, `mm/h`, `m/s`, `km/h`; USCS / imperial: `in/d`, `in/h`, `ft/s`, `mph`; Nautical: `kn`
        Speed = "speed",
        /// Amount of SO2.
        ///
        /// Unit of measurement: `ppb` (parts per billion), `μg/m³`
        SulphurDioxide = "sulphur_dioxide",
        /// Temperature.
        ///
        /// Unit of measurement: `°C`, `°F`, `K`
        Temperature = "temperature",
        /// Difference of temperatures - Temperature range.
        ///
        /// Unit of measurement: `°C`, `°F`, `K`
        TemperatureDelta = "temperature_delta",
        /// Amount of VOC.
        ///
        /// Unit of measurement: `μg/m³`, `mg/m³`
        VolatileOrganicCompounds = "volatile_organic_compounds",
        /// Ratio of VOC.
        ///
        /// Unit of measurement: `ppm`, `ppb`
        VolatileOrganicCompoundsParts = "volatile_organic_compounds_parts",
        /// Voltage.
        ///
        /// Unit of measurement: `V`, `mV`, `μV`, `kV`, `MV`
        Voltage = "voltage",
        /// Generic volume.
        ///
        /// Unit of measurement: `VOLUME_*` units; SI / metric: `mL`, `L`, `m³`; USCS / imperial: `ft³`, `CCF`, `MCF`, `fl. oz.`, `gal` (warning: volumes expressed in USCS/imperial units are currently assumed to be US volumes)
        Volume = "volume",
        /// Generic stored volume.
        ///
        /// Unit of measurement: `VOLUME_*` units; SI / metric: `mL`, `L`, `m³`; USCS / imperial: `ft³`, `CCF`, `MCF`, `fl. oz.`, `gal` (warning: volumes expressed in USCS/imperial units are currently assumed to be US volumes)
        VolumeStorage = "volume_storage",
        /// Generic flow rate.
        ///
        /// Unit of measurement: UnitOfVolumeFlowRate; SI / metric: `m³/h`, `m³/min`, `m³/s`, `L/h`, `L/min`, `L/s`, `mL/s`; USCS / imperial: `ft³/min`, `gal/min`, `gal/d`
        VolumeFlowRate = "volume_flow_rate",
        /// Water.
        ///
        /// Unit of measurement: SI / metric: `m³`, `L`; USCS / imperial: `ft³`, `CCF`, `MCF`, `gal` (warning: volumes expressed in USCS/imperial units are currently assumed to be US volumes)
        Water = "water",
        /// Generic weight, represents a measurement of an object's mass.
        ///
        /// Unit of measurement: `MASS_*` units; SI / metric: `μg`, `mg`, `g`, `kg`; USCS / imperial: `oz`, `lb`
        Weight = "weight",
        /// Wind direction.
        ///
        /// Unit of measurement: `°`
        WindDirection = "wind_direction",
        /// Wind speed.
        ///
        /// Unit of measurement: `SPEED_*` units; SI /metric: `m/s`, `km/h`; USCS / imperial: `ft/s`, `mph`; Nautical: `kn`
        WindSpeed = "wind_speed",
    }
}
