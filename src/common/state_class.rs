use crate::string_enum::string_enum;

string_enum! {
    /// The state class of a sensor, used for long-term statistics.
    ///
    /// <https://www.home-assistant.io/integrations/sensor.mqtt/#state_class>
    /// and <https://developers.home-assistant.io/docs/core/entity/sensor/#available-state-classes>
    pub enum StateClass {
        /// A measurement in present time, e.g. current temperature.
        Measurement = "measurement",
        /// An angle measurement in present time, e.g. wind direction. Only
        /// degrees are supported.
        MeasurementAngle = "measurement_angle",
        /// A total amount that can increase and decrease, e.g. net energy
        /// consumption.
        Total = "total",
        /// A monotonically increasing total that periodically restarts from 0,
        /// e.g. an amount of consumed gas.
        TotalIncreasing = "total_increasing",
    }
}
