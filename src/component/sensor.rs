//! MQTT sensor: <https://www.home-assistant.io/integrations/sensor.mqtt/>

// @generated begin component sensor
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
        state_class::StateClass,
        unit::Unit,
    },
    string_enum::string_enum,
};

/// An MQTT sensor: a read-only entity whose state is received on
/// `state_topic`.
///
/// <https://www.home-assistant.io/integrations/sensor.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Sensor {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `sensor.kitchen_temperature`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this sensor belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Type of the sensor, which sets its icon and how its state is displayed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device_class: Option<SensorDeviceClass>,
    /// Whether the entity is enabled when first added. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub enabled_by_default: Option<bool>,
    /// Encoding of received payloads; an empty string disables decoding.
    /// Default: `utf-8`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub encoding: Option<String>,
    /// Category of the entity. The docs only allow `diagnostic` for sensors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_category: Option<EntityCategory>,
    /// URL of a picture for the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_picture: Option<String>,
    /// Seconds after the last update until the state becomes unavailable.
    /// Default: `0` (never expires).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub expire_after: Option<u32>,
    /// Send update events even if the value has not changed. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub force_update: Option<bool>,
    /// Unique IDs of member entities, making this a group sensor. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:thermometer`.
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
    /// Template to extract the `last_reset` timestamp. Requires `state_class`
    /// `total`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub last_reset_value_template: Option<String>,
    /// Message expiry interval for command payloads.
    /// Not in the sensor docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Name of the sensor; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Sensor`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// The possible states of an `enum` sensor. Requires `device_class` `enum`
    /// and must not be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub options: Option<Vec<String>>,
    /// The application that sent the discovery message.
    /// Not in the sensor docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Maximum QoS level for subscriptions. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// State class of the sensor, used for long-term statistics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_class: Option<StateClass>,
    /// Topic to receive sensor values from. Required.
    pub state_topic: String,
    /// Number of decimals to round the state to when displaying it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub suggested_display_precision: Option<u32>,
    /// ID that uniquely identifies this sensor. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Unit of measurement of the sensor's state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unit_of_measurement: Option<Unit>,
    /// Template to extract the value from the payload. If it fails, the
    /// previous state is kept.
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
impl Sensor {
    /// Returns a builder for [`Sensor`].
    pub fn builder() -> SensorBuilder {
        SensorBuilder::default()
    }
}

impl Platform for Sensor {
    const PLATFORM: &'static str = "sensor";
}
// @generated end

string_enum! {
    /// Device class of a sensor, which sets its icon and how its state is displayed.
    ///
    /// <https://www.home-assistant.io/integrations/sensor/#device-class>
    ///
    /// Source: `homeassistant/components/sensor/const.py`
    pub enum SensorDeviceClass {
        // @generated begin device-class homeassistant/components/sensor/const.py SensorDeviceClass
        /// Date.
        Date = "date",
        /// Enumeration.
        Enum = "enum",
        /// Timestamp.
        Timestamp = "timestamp",
        /// Uptime.
        Uptime = "uptime",
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
        /// Unit of measurement: `A`, `mA`, `μA`
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
        /// Unit of measurement: `SPEED_*` units or `UnitOfVolumetricFlux`; SI /metric: `mm/d`, `mm/h`, `m/s`, `km/h`, `mm/s`; USCS / imperial: `in/d`, `in/h`, `in/s`, `ft/s`, `mph`; Nautical: `kn`; Beaufort: `Beaufort`
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
        /// Unit of measurement: `SPEED_*` units; SI /metric: `m/s`, `km/h`; USCS / imperial: `ft/s`, `mph`; Nautical: `kn`; Beaufort: `Beaufort`
        WindSpeed = "wind_speed",
        // @generated end
    }
}
