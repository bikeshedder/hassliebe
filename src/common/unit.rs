//! Units of measurement.
//!
//! Mirrors Home Assistant's `homeassistant/const.py`: one closed enum per
//! `UnitOf*` class, plus [`Unit`], which wraps them, adds the units Home
//! Assistant defines as plain constants, and keeps unknown units in
//! [`Unit::Other`].
//!
//! Every `UnitOf*` type converts into [`Unit`], so builders accept them
//! directly: `.unit_of_measurement(UnitOfSpeed::MetersPerSecond)`.
//!
//! <https://www.home-assistant.io/integrations/sensor.mqtt/#unit_of_measurement>

use std::{convert::Infallible, fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::string_enum::string_enum;

string_enum! {
    @closed
    /// Units of apparent power, like `UnitOfApparentPower` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfApparentPower {
        /// `mVA` (`MILLIVOLT_AMPERE`).
        MillivoltAmpere = "mVA",
        /// `VA` (`VOLT_AMPERE`).
        VoltAmpere = "VA",
        /// `kVA` (`KILO_VOLT_AMPERE`).
        KiloVoltAmpere = "kVA",
    }
}

string_enum! {
    @closed
    /// Units of area, like `UnitOfArea` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfArea {
        /// `m²` (`SQUARE_METERS`).
        SquareMeters = "m²",
        /// `cm²` (`SQUARE_CENTIMETERS`).
        SquareCentimeters = "cm²",
        /// `km²` (`SQUARE_KILOMETERS`).
        SquareKilometers = "km²",
        /// `mm²` (`SQUARE_MILLIMETERS`).
        SquareMillimeters = "mm²",
        /// `in²` (`SQUARE_INCHES`).
        SquareInches = "in²",
        /// `ft²` (`SQUARE_FEET`).
        SquareFeet = "ft²",
        /// `yd²` (`SQUARE_YARDS`).
        SquareYards = "yd²",
        /// `mi²` (`SQUARE_MILES`).
        SquareMiles = "mi²",
        /// `ac` (`ACRES`).
        Acres = "ac",
        /// `ha` (`HECTARES`).
        Hectares = "ha",
    }
}

string_enum! {
    @closed
    /// Units of blood glucose concentration, like `UnitOfBloodGlucoseConcentration` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfBloodGlucoseConcentration {
        /// `mg/dL` (`MILLIGRAMS_PER_DECILITER`).
        MilligramsPerDeciliter = "mg/dL",
        /// `mmol/L` (`MILLIMOLE_PER_LITER`).
        MillimolePerLiter = "mmol/L",
    }
}

string_enum! {
    @closed
    /// Units of conductivity, like `UnitOfConductivity` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfConductivity {
        /// `S/cm` (`SIEMENS_PER_CM`).
        SiemensPerCm = "S/cm",
        /// `μS/cm` (`MICROSIEMENS_PER_CM`). Also accepts `µS/cm` when reading.
        MicrosiemensPerCm = "μS/cm" | "µS/cm",
        /// `mS/cm` (`MILLISIEMENS_PER_CM`).
        MillisiemensPerCm = "mS/cm",
    }
}

string_enum! {
    @closed
    /// Units of data rate, like `UnitOfDataRate` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfDataRate {
        /// `bit/s` (`BITS_PER_SECOND`).
        BitsPerSecond = "bit/s",
        /// `kbit/s` (`KILOBITS_PER_SECOND`).
        KilobitsPerSecond = "kbit/s",
        /// `Mbit/s` (`MEGABITS_PER_SECOND`).
        MegabitsPerSecond = "Mbit/s",
        /// `Gbit/s` (`GIGABITS_PER_SECOND`).
        GigabitsPerSecond = "Gbit/s",
        /// `B/s` (`BYTES_PER_SECOND`).
        BytesPerSecond = "B/s",
        /// `kB/s` (`KILOBYTES_PER_SECOND`).
        KilobytesPerSecond = "kB/s",
        /// `MB/s` (`MEGABYTES_PER_SECOND`).
        MegabytesPerSecond = "MB/s",
        /// `GB/s` (`GIGABYTES_PER_SECOND`).
        GigabytesPerSecond = "GB/s",
        /// `KiB/s` (`KIBIBYTES_PER_SECOND`).
        KibibytesPerSecond = "KiB/s",
        /// `MiB/s` (`MEBIBYTES_PER_SECOND`).
        MebibytesPerSecond = "MiB/s",
        /// `GiB/s` (`GIBIBYTES_PER_SECOND`).
        GibibytesPerSecond = "GiB/s",
    }
}

string_enum! {
    @closed
    /// Units of density, like `UnitOfDensity` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfDensity {
        /// `g/m³` (`GRAMS_PER_CUBIC_METER`).
        GramsPerCubicMeter = "g/m³",
        /// `mg/m³` (`MILLIGRAMS_PER_CUBIC_METER`).
        MilligramsPerCubicMeter = "mg/m³",
        /// `μg/m³` (`MICROGRAMS_PER_CUBIC_METER`). Also accepts `µg/m³` when reading.
        MicrogramsPerCubicMeter = "μg/m³" | "µg/m³",
        /// `μg/ft³` (`MICROGRAMS_PER_CUBIC_FOOT`). Also accepts `µg/ft³` when reading.
        MicrogramsPerCubicFoot = "μg/ft³" | "µg/ft³",
    }
}

string_enum! {
    @closed
    /// Units of electric current, like `UnitOfElectricCurrent` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfElectricCurrent {
        /// `μA` (`MICROAMPERE`).
        Microampere = "μA",
        /// `mA` (`MILLIAMPERE`).
        Milliampere = "mA",
        /// `A` (`AMPERE`).
        Ampere = "A",
    }
}

string_enum! {
    @closed
    /// Units of electric potential, like `UnitOfElectricPotential` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfElectricPotential {
        /// `μV` (`MICROVOLT`). Also accepts `µV` when reading.
        Microvolt = "μV" | "µV",
        /// `mV` (`MILLIVOLT`).
        Millivolt = "mV",
        /// `V` (`VOLT`).
        Volt = "V",
        /// `kV` (`KILOVOLT`).
        Kilovolt = "kV",
        /// `MV` (`MEGAVOLT`).
        Megavolt = "MV",
    }
}

string_enum! {
    @closed
    /// Units of energy, like `UnitOfEnergy` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfEnergy {
        /// `J` (`JOULE`).
        Joule = "J",
        /// `kJ` (`KILO_JOULE`).
        KiloJoule = "kJ",
        /// `MJ` (`MEGA_JOULE`).
        MegaJoule = "MJ",
        /// `GJ` (`GIGA_JOULE`).
        GigaJoule = "GJ",
        /// `mWh` (`MILLIWATT_HOUR`).
        MilliwattHour = "mWh",
        /// `Wh` (`WATT_HOUR`).
        WattHour = "Wh",
        /// `kWh` (`KILO_WATT_HOUR`).
        KiloWattHour = "kWh",
        /// `MWh` (`MEGA_WATT_HOUR`).
        MegaWattHour = "MWh",
        /// `GWh` (`GIGA_WATT_HOUR`).
        GigaWattHour = "GWh",
        /// `TWh` (`TERA_WATT_HOUR`).
        TeraWattHour = "TWh",
        /// `cal` (`CALORIE`).
        Calorie = "cal",
        /// `kcal` (`KILO_CALORIE`).
        KiloCalorie = "kcal",
        /// `Mcal` (`MEGA_CALORIE`).
        MegaCalorie = "Mcal",
        /// `Gcal` (`GIGA_CALORIE`).
        GigaCalorie = "Gcal",
        /// `thm` (`THERM`).
        Therm = "thm",
    }
}

string_enum! {
    @closed
    /// Units of energy distance, like `UnitOfEnergyDistance` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfEnergyDistance {
        /// `kWh/100km` (`KILO_WATT_HOUR_PER_100_KM`).
        KiloWattHourPer100Km = "kWh/100km",
        /// `Wh/km` (`WATT_HOUR_PER_KM`).
        WattHourPerKm = "Wh/km",
        /// `mi/kWh` (`MILES_PER_KILO_WATT_HOUR`).
        MilesPerKiloWattHour = "mi/kWh",
        /// `km/kWh` (`KM_PER_KILO_WATT_HOUR`).
        KmPerKiloWattHour = "km/kWh",
    }
}

string_enum! {
    @closed
    /// Units of frequency, like `UnitOfFrequency` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfFrequency {
        /// `mHz` (`MILLIHERTZ`).
        Millihertz = "mHz",
        /// `Hz` (`HERTZ`).
        Hertz = "Hz",
        /// `kHz` (`KILOHERTZ`).
        Kilohertz = "kHz",
        /// `MHz` (`MEGAHERTZ`).
        Megahertz = "MHz",
        /// `GHz` (`GIGAHERTZ`).
        Gigahertz = "GHz",
    }
}

string_enum! {
    @closed
    /// Units of information, like `UnitOfInformation` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfInformation {
        /// `bit` (`BITS`).
        Bits = "bit",
        /// `kbit` (`KILOBITS`).
        Kilobits = "kbit",
        /// `Mbit` (`MEGABITS`).
        Megabits = "Mbit",
        /// `Gbit` (`GIGABITS`).
        Gigabits = "Gbit",
        /// `B` (`BYTES`).
        Bytes = "B",
        /// `kB` (`KILOBYTES`).
        Kilobytes = "kB",
        /// `MB` (`MEGABYTES`).
        Megabytes = "MB",
        /// `GB` (`GIGABYTES`).
        Gigabytes = "GB",
        /// `TB` (`TERABYTES`).
        Terabytes = "TB",
        /// `PB` (`PETABYTES`).
        Petabytes = "PB",
        /// `EB` (`EXABYTES`).
        Exabytes = "EB",
        /// `ZB` (`ZETTABYTES`).
        Zettabytes = "ZB",
        /// `YB` (`YOTTABYTES`).
        Yottabytes = "YB",
        /// `KiB` (`KIBIBYTES`).
        Kibibytes = "KiB",
        /// `MiB` (`MEBIBYTES`).
        Mebibytes = "MiB",
        /// `GiB` (`GIBIBYTES`).
        Gibibytes = "GiB",
        /// `TiB` (`TEBIBYTES`).
        Tebibytes = "TiB",
        /// `PiB` (`PEBIBYTES`).
        Pebibytes = "PiB",
        /// `EiB` (`EXBIBYTES`).
        Exbibytes = "EiB",
        /// `ZiB` (`ZEBIBYTES`).
        Zebibytes = "ZiB",
        /// `YiB` (`YOBIBYTES`).
        Yobibytes = "YiB",
    }
}

string_enum! {
    @closed
    /// Units of irradiance, like `UnitOfIrradiance` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfIrradiance {
        /// `W/m²` (`WATTS_PER_SQUARE_METER`).
        WattsPerSquareMeter = "W/m²",
        /// `BTU/(h⋅ft²)` (`BTUS_PER_HOUR_SQUARE_FOOT`).
        BtusPerHourSquareFoot = "BTU/(h⋅ft²)",
    }
}

string_enum! {
    @closed
    /// Units of length, like `UnitOfLength` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfLength {
        /// `mm` (`MILLIMETERS`).
        Millimeters = "mm",
        /// `cm` (`CENTIMETERS`).
        Centimeters = "cm",
        /// `m` (`METERS`).
        Meters = "m",
        /// `km` (`KILOMETERS`).
        Kilometers = "km",
        /// `in` (`INCHES`).
        Inches = "in",
        /// `ft` (`FEET`).
        Feet = "ft",
        /// `yd` (`YARDS`).
        Yards = "yd",
        /// `mi` (`MILES`).
        Miles = "mi",
        /// `nmi` (`NAUTICAL_MILES`).
        NauticalMiles = "nmi",
    }
}

string_enum! {
    @closed
    /// Units of mass, like `UnitOfMass` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfMass {
        /// `g` (`GRAMS`).
        Grams = "g",
        /// `kg` (`KILOGRAMS`).
        Kilograms = "kg",
        /// `mg` (`MILLIGRAMS`).
        Milligrams = "mg",
        /// `μg` (`MICROGRAMS`). Also accepts `µg` when reading.
        Micrograms = "μg" | "µg",
        /// `oz` (`OUNCES`).
        Ounces = "oz",
        /// `lb` (`POUNDS`).
        Pounds = "lb",
        /// `st` (`STONES`).
        Stones = "st",
    }
}

string_enum! {
    @closed
    /// Units of power, like `UnitOfPower` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfPower {
        /// `mW` (`MILLIWATT`).
        Milliwatt = "mW",
        /// `W` (`WATT`).
        Watt = "W",
        /// `kW` (`KILO_WATT`).
        KiloWatt = "kW",
        /// `MW` (`MEGA_WATT`).
        MegaWatt = "MW",
        /// `GW` (`GIGA_WATT`).
        GigaWatt = "GW",
        /// `TW` (`TERA_WATT`).
        TeraWatt = "TW",
        /// `BTU/h` (`BTU_PER_HOUR`).
        BtuPerHour = "BTU/h",
    }
}

string_enum! {
    @closed
    /// Units of precipitation depth, like `UnitOfPrecipitationDepth` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfPrecipitationDepth {
        /// `in` (`INCHES`).
        Inches = "in",
        /// `mm` (`MILLIMETERS`).
        Millimeters = "mm",
        /// `cm` (`CENTIMETERS`).
        Centimeters = "cm",
    }
}

string_enum! {
    @closed
    /// Units of pressure, like `UnitOfPressure` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfPressure {
        /// `mPa` (`MILLIPASCAL`).
        Millipascal = "mPa",
        /// `Pa` (`PA`).
        Pa = "Pa",
        /// `hPa` (`HPA`).
        Hpa = "hPa",
        /// `kPa` (`KPA`).
        Kpa = "kPa",
        /// `bar` (`BAR`).
        Bar = "bar",
        /// `cbar` (`CBAR`).
        Cbar = "cbar",
        /// `mbar` (`MBAR`).
        Mbar = "mbar",
        /// `mmHg` (`MMHG`).
        Mmhg = "mmHg",
        /// `inHg` (`INHG`).
        Inhg = "inHg",
        /// `inH₂O` (`INH2O`).
        Inh2o = "inH₂O",
        /// `psi` (`PSI`).
        Psi = "psi",
        /// `atm` (`ATM`).
        Atm = "atm",
    }
}

string_enum! {
    @closed
    /// Units of radiation concentration, like `UnitOfRadiationConcentration` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfRadiationConcentration {
        /// `Bq/m³` (`BECQUEREL_PER_CUBIC_METER`).
        BecquerelPerCubicMeter = "Bq/m³",
        /// `pCi/L` (`PICOCURIES_PER_LITER`).
        PicocuriesPerLiter = "pCi/L",
    }
}

string_enum! {
    @closed
    /// Units of ratio, like `UnitOfRatio` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfRatio {
        /// `ppm` (`PARTS_PER_MILLION`).
        PartsPerMillion = "ppm",
        /// `ppb` (`PARTS_PER_BILLION`).
        PartsPerBillion = "ppb",
        /// `%` (`PERCENTAGE`).
        Percentage = "%",
    }
}

string_enum! {
    @closed
    /// Units of reactive energy, like `UnitOfReactiveEnergy` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfReactiveEnergy {
        /// `varh` (`VOLT_AMPERE_REACTIVE_HOUR`). Also accepts `VArh` when reading.
        VoltAmpereReactiveHour = "varh" | "VArh",
        /// `kvarh` (`KILO_VOLT_AMPERE_REACTIVE_HOUR`). Also accepts `kVArh` when reading.
        KiloVoltAmpereReactiveHour = "kvarh" | "kVArh",
    }
}

string_enum! {
    @closed
    /// Units of reactive power, like `UnitOfReactivePower` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfReactivePower {
        /// `mvar` (`MILLIVOLT_AMPERE_REACTIVE`). Also accepts `mVAr` when reading.
        MillivoltAmpereReactive = "mvar" | "mVAr",
        /// `var` (`VOLT_AMPERE_REACTIVE`). Also accepts `VAr` when reading.
        VoltAmpereReactive = "var" | "VAr",
        /// `kvar` (`KILO_VOLT_AMPERE_REACTIVE`). Also accepts `kVAr` when reading.
        KiloVoltAmpereReactive = "kvar" | "kVAr",
    }
}

string_enum! {
    @closed
    /// Units of sound pressure, like `UnitOfSoundPressure` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfSoundPressure {
        /// `dB` (`DECIBEL`).
        Decibel = "dB",
        /// `dBA` (`WEIGHTED_DECIBEL_A`).
        WeightedDecibelA = "dBA",
    }
}

string_enum! {
    @closed
    /// Units of speed, like `UnitOfSpeed` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfSpeed {
        /// `Beaufort` (`BEAUFORT`).
        Beaufort = "Beaufort",
        /// `ft/s` (`FEET_PER_SECOND`).
        FeetPerSecond = "ft/s",
        /// `in/s` (`INCHES_PER_SECOND`).
        InchesPerSecond = "in/s",
        /// `m/min` (`METERS_PER_MINUTE`).
        MetersPerMinute = "m/min",
        /// `m/s` (`METERS_PER_SECOND`).
        MetersPerSecond = "m/s",
        /// `km/h` (`KILOMETERS_PER_HOUR`).
        KilometersPerHour = "km/h",
        /// `kn` (`KNOTS`).
        Knots = "kn",
        /// `mph` (`MILES_PER_HOUR`).
        MilesPerHour = "mph",
        /// `mm/s` (`MILLIMETERS_PER_SECOND`).
        MillimetersPerSecond = "mm/s",
    }
}

string_enum! {
    @closed
    /// Units of temperature, like `UnitOfTemperature` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfTemperature {
        /// `°C` (`CELSIUS`).
        Celsius = "°C",
        /// `°F` (`FAHRENHEIT`).
        Fahrenheit = "°F",
        /// `K` (`KELVIN`).
        Kelvin = "K",
    }
}

string_enum! {
    @closed
    /// Units of time, like `UnitOfTime` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfTime {
        /// `μs` (`MICROSECONDS`). Also accepts `µs` when reading.
        Microseconds = "μs" | "µs",
        /// `ms` (`MILLISECONDS`).
        Milliseconds = "ms",
        /// `s` (`SECONDS`).
        Seconds = "s",
        /// `min` (`MINUTES`).
        Minutes = "min",
        /// `h` (`HOURS`).
        Hours = "h",
        /// `d` (`DAYS`).
        Days = "d",
        /// `w` (`WEEKS`).
        Weeks = "w",
        /// `m` (`MONTHS`).
        Months = "m",
        /// `y` (`YEARS`).
        Years = "y",
    }
}

string_enum! {
    @closed
    /// Units of volume, like `UnitOfVolume` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfVolume {
        /// `ft³` (`CUBIC_FEET`).
        CubicFeet = "ft³",
        /// `CCF` (`CENTUM_CUBIC_FEET`).
        CentumCubicFeet = "CCF",
        /// `MCF` (`MILLE_CUBIC_FEET`).
        MilleCubicFeet = "MCF",
        /// `m³` (`CUBIC_METERS`).
        CubicMeters = "m³",
        /// `L` (`LITERS`).
        Liters = "L",
        /// `mL` (`MILLILITERS`).
        Milliliters = "mL",
        /// `gal` (`GALLONS`).
        Gallons = "gal",
        /// `fl. oz.` (`FLUID_OUNCES`).
        FluidOunces = "fl. oz.",
    }
}

string_enum! {
    @closed
    /// Units of volume flow rate, like `UnitOfVolumeFlowRate` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfVolumeFlowRate {
        /// `m³/h` (`CUBIC_METERS_PER_HOUR`).
        CubicMetersPerHour = "m³/h",
        /// `m³/min` (`CUBIC_METERS_PER_MINUTE`).
        CubicMetersPerMinute = "m³/min",
        /// `m³/s` (`CUBIC_METERS_PER_SECOND`).
        CubicMetersPerSecond = "m³/s",
        /// `ft³/min` (`CUBIC_FEET_PER_MINUTE`).
        CubicFeetPerMinute = "ft³/min",
        /// `L/h` (`LITERS_PER_HOUR`).
        LitersPerHour = "L/h",
        /// `L/min` (`LITERS_PER_MINUTE`).
        LitersPerMinute = "L/min",
        /// `L/s` (`LITERS_PER_SECOND`).
        LitersPerSecond = "L/s",
        /// `gal/h` (`GALLONS_PER_HOUR`).
        GallonsPerHour = "gal/h",
        /// `gal/min` (`GALLONS_PER_MINUTE`).
        GallonsPerMinute = "gal/min",
        /// `gal/d` (`GALLONS_PER_DAY`).
        GallonsPerDay = "gal/d",
        /// `mL/s` (`MILLILITERS_PER_SECOND`).
        MillilitersPerSecond = "mL/s",
    }
}

string_enum! {
    @closed
    /// Units of volumetric flux, like `UnitOfVolumetricFlux` in Home Assistant.
    ///
    /// Source: `homeassistant/const.py`
    pub enum UnitOfVolumetricFlux {
        /// `in/d` (`INCHES_PER_DAY`).
        InchesPerDay = "in/d",
        /// `in/h` (`INCHES_PER_HOUR`).
        InchesPerHour = "in/h",
        /// `mm/d` (`MILLIMETERS_PER_DAY`).
        MillimetersPerDay = "mm/d",
        /// `mm/h` (`MILLIMETERS_PER_HOUR`).
        MillimetersPerHour = "mm/h",
    }
}

macro_rules! unit {
    (
        classes {
            $($class_variant:ident($class:ident),)*
        }
        constants {
            $(
                $(#[$cmeta:meta])*
                $const_variant:ident = $value:literal,
            )*
        }
    ) => {
        /// A unit of measurement.
        ///
        /// Wraps the `UnitOf*` classes and adds the units Home Assistant
        /// defines as plain constants in `homeassistant/const.py`. Unknown units
        /// are kept in [`Unit::Other`].
        ///
        /// Serializes as the unit string. When reading, some strings belong to
        /// more than one class; the classes are tried in the order of the
        /// variants below (alphabetical), before the plain constants. So `"m"`,
        /// `"mm"`, `"cm"` and `"in"` read as [`Unit::Length`] (not
        /// [`Unit::Time`] or [`Unit::PrecipitationDepth`]), and `"dB"` reads as
        /// [`Unit::SoundPressure`] (not [`Unit::SignalStrengthDecibels`]).
        /// Such values do not compare equal after a round trip.
        ///
        /// <https://www.home-assistant.io/integrations/sensor.mqtt/#unit_of_measurement>
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        #[non_exhaustive]
        pub enum Unit {
            $(
                #[doc = concat!("A unit from [`", stringify!($class), "`].")]
                $class_variant($class),
            )*
            $(
                $(#[$cmeta])*
                $const_variant,
            )*
            /// A unit this crate does not know (yet).
            ///
            /// Use [`FromStr`] to construct values from strings: it only yields
            /// `Other` for unknown units.
            Other(String),
        }

        const _: () = $crate::string_enum::assert_unique(&[$($value,)*]);

        impl Unit {
            /// Returns the string Home Assistant uses for this unit.
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$class_variant(unit) => unit.as_str(),)*
                    $(Self::$const_variant => $value,)*
                    Self::Other(unit) => unit,
                }
            }
        }

        #[cfg(test)]
        impl Unit {
            /// Every known unit, class by class, then the plain constants.
            fn all_known() -> Vec<Self> {
                let mut all = Vec::new();
                $(all.extend($class::ALL.iter().copied().map(Self::$class_variant));)*
                $(all.push(Self::$const_variant);)*
                all
            }
        }

        impl FromStr for Unit {
            type Err = Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $(
                    if let Ok(unit) = s.parse::<$class>() {
                        return Ok(Self::$class_variant(unit));
                    }
                )*
                Ok(match s {
                    $($value => Self::$const_variant,)*
                    other => Self::Other(other.into()),
                })
            }
        }

        $(
            impl From<$class> for Unit {
                fn from(unit: $class) -> Self {
                    Self::$class_variant(unit)
                }
            }
        )*
    };
}

unit! {
    classes {
        ApparentPower(UnitOfApparentPower),
        Area(UnitOfArea),
        BloodGlucoseConcentration(UnitOfBloodGlucoseConcentration),
        Conductivity(UnitOfConductivity),
        DataRate(UnitOfDataRate),
        Density(UnitOfDensity),
        ElectricCurrent(UnitOfElectricCurrent),
        ElectricPotential(UnitOfElectricPotential),
        Energy(UnitOfEnergy),
        EnergyDistance(UnitOfEnergyDistance),
        Frequency(UnitOfFrequency),
        Information(UnitOfInformation),
        Irradiance(UnitOfIrradiance),
        Length(UnitOfLength),
        Mass(UnitOfMass),
        Power(UnitOfPower),
        PrecipitationDepth(UnitOfPrecipitationDepth),
        Pressure(UnitOfPressure),
        RadiationConcentration(UnitOfRadiationConcentration),
        Ratio(UnitOfRatio),
        ReactiveEnergy(UnitOfReactiveEnergy),
        ReactivePower(UnitOfReactivePower),
        SoundPressure(UnitOfSoundPressure),
        Speed(UnitOfSpeed),
        Temperature(UnitOfTemperature),
        Time(UnitOfTime),
        Volume(UnitOfVolume),
        VolumeFlowRate(UnitOfVolumeFlowRate),
        VolumetricFlux(UnitOfVolumetricFlux),
    }
    constants {
        /// `°` (`DEGREE`).
        Degree = "°",
        /// `€` (`CURRENCY_EURO`).
        CurrencyEuro = "€",
        /// `$` (`CURRENCY_DOLLAR`).
        CurrencyDollar = "$",
        /// `¢` (`CURRENCY_CENT`).
        CurrencyCent = "¢",
        /// `lx` (`LIGHT_LUX`).
        LightLux = "lx",
        /// `UV index` (`UV_INDEX`).
        UvIndex = "UV index",
        /// `rpm` (`REVOLUTIONS_PER_MINUTE`).
        RevolutionsPerMinute = "rpm",
        /// `dB` (`SIGNAL_STRENGTH_DECIBELS`). Never produced when reading: `"dB"`
        /// reads as [`UnitOfSoundPressure::Decibel`].
        SignalStrengthDecibels = "dB",
        /// `dBm` (`SIGNAL_STRENGTH_DECIBELS_MILLIWATT`).
        SignalStrengthDecibelsMilliwatt = "dBm",
        /// `p/m³` (`CONCENTRATION_PARTS_PER_CUBIC_METER`). Deprecated in Home
        /// Assistant, to be removed in 2027.8.
        ConcentrationPartsPerCubicMeter = "p/m³",
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for Unit {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl de::Visitor<'_> for Visitor {
            type Value = Unit;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a unit of measurement")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Unit, E> {
                let Ok(unit) = value.parse();
                Ok(unit)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use pretty_assertions::assert_eq;

    use super::*;

    fn parse(s: &str) -> Unit {
        let Ok(unit) = s.parse();
        unit
    }

    /// Strings that belong to more than one unit. If this fails after syncing
    /// with Home Assistant, decide the precedence and document it on [`Unit`].
    #[test]
    fn shared_strings_are_known() {
        let all = Unit::all_known();
        let mut by_string = BTreeMap::<&str, Vec<&Unit>>::new();
        for unit in &all {
            by_string.entry(unit.as_str()).or_default().push(unit);
        }
        by_string.retain(|_, units| units.len() > 1);
        assert_eq!(
            by_string.keys().copied().collect::<Vec<_>>(),
            ["cm", "dB", "in", "m", "mm"]
        );
    }

    #[test]
    fn shared_strings_resolve_by_precedence() {
        assert_eq!(parse("m"), Unit::Length(UnitOfLength::Meters));
        assert_eq!(parse("mm"), Unit::Length(UnitOfLength::Millimeters));
        assert_eq!(parse("cm"), Unit::Length(UnitOfLength::Centimeters));
        assert_eq!(parse("in"), Unit::Length(UnitOfLength::Inches));
        assert_eq!(
            parse("dB"),
            Unit::SoundPressure(UnitOfSoundPressure::Decibel)
        );
    }

    #[test]
    fn every_unique_unit_round_trips() {
        let all = Unit::all_known();
        for unit in &all {
            let json = serde_json::to_string(unit).unwrap();
            let back: Unit = serde_json::from_str(&json).unwrap();
            let shared = all.iter().filter(|u| u.as_str() == unit.as_str()).count() > 1;
            if !shared {
                assert_eq!(&back, unit, "{json}");
            }
            assert_eq!(back.as_str(), unit.as_str());
        }
    }

    #[test]
    fn legacy_spellings_are_aliases() {
        let cases = [
            (
                "\u{b5}S/cm",
                Unit::Conductivity(UnitOfConductivity::MicrosiemensPerCm),
            ),
            (
                "\u{b5}V",
                Unit::ElectricPotential(UnitOfElectricPotential::Microvolt),
            ),
            (
                "\u{b5}g/ft³",
                Unit::Density(UnitOfDensity::MicrogramsPerCubicFoot),
            ),
            (
                "\u{b5}g/m³",
                Unit::Density(UnitOfDensity::MicrogramsPerCubicMeter),
            ),
            ("\u{b5}g", Unit::Mass(UnitOfMass::Micrograms)),
            ("\u{b5}s", Unit::Time(UnitOfTime::Microseconds)),
            (
                "mVAr",
                Unit::ReactivePower(UnitOfReactivePower::MillivoltAmpereReactive),
            ),
            (
                "VAr",
                Unit::ReactivePower(UnitOfReactivePower::VoltAmpereReactive),
            ),
            (
                "kVAr",
                Unit::ReactivePower(UnitOfReactivePower::KiloVoltAmpereReactive),
            ),
            (
                "VArh",
                Unit::ReactiveEnergy(UnitOfReactiveEnergy::VoltAmpereReactiveHour),
            ),
            (
                "kVArh",
                Unit::ReactiveEnergy(UnitOfReactiveEnergy::KiloVoltAmpereReactiveHour),
            ),
        ];
        for (legacy, unit) in cases {
            assert_eq!(parse(legacy), unit, "{legacy}");
            assert_ne!(unit.as_str(), legacy);
        }
        assert_eq!(
            serde_json::to_string(&parse("\u{b5}g/m³")).unwrap(),
            "\"\u{3bc}g/m³\""
        );
    }

    #[test]
    fn unknown_units_are_other() {
        assert_eq!(parse("μSv/h"), Unit::Other("μSv/h".into()));
        assert_eq!(serde_json::to_string(&parse("μSv/h")).unwrap(), "\"μSv/h\"");
        assert!(serde_json::from_str::<Unit>("1").is_err());
    }

    #[test]
    fn constants() {
        assert_eq!(parse("%"), Unit::Ratio(UnitOfRatio::Percentage));
        assert_eq!(parse("°"), Unit::Degree);
        assert_eq!(parse("dBm"), Unit::SignalStrengthDecibelsMilliwatt);
        assert_eq!(Unit::SignalStrengthDecibels.as_str(), "dB");
    }

    #[test]
    fn classes_convert_into_unit() {
        let unit: Unit = UnitOfVolumeFlowRate::CubicFeetPerMinute.into();
        assert_eq!(unit.to_string(), "ft³/min");
    }
}
