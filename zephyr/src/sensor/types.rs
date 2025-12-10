//! Generic sensor types - reusable for all sensor drivers
//!
//! This module provides generic types that mirror Zephyr RTOS sensor subsystem structures,
//! enabling Rust code to interact with sensor hardware through safe abstractions.
//! These types are designed to be reusable across different sensor driver implementations.

/// Sensor channel types (matches Zephyr's `sensor_channel` enum)
///
/// Represents different types of sensor channels that can be sampled or configured.
/// Each channel corresponds to a specific measurement type (e.g., temperature, pressure,
/// color components). The numeric values match Zephyr's C enum definitions to ensure
/// correct FFI interoperability.
///
/// # Examples
///
/// ```no_run
/// use zephyr::sensor::types::SensorChannel;
///
/// // Request all channels
/// let channel = SensorChannel::All;
///
/// // Request specific temperature channel
/// let temp_channel = SensorChannel::AmbientTemp;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SensorChannel {
    /// All available sensor channels
    ///
    /// Used to fetch or configure all channels at once. This is commonly used
    /// when you want to sample all available measurements from a sensor.
    All = 0,

    /// Red color channel
    ///
    /// Used by color/light sensors to measure red light intensity.
    Red = 1,

    /// Green color channel
    ///
    /// Used by color/light sensors to measure green light intensity.
    Green = 2,

    /// Blue color channel
    ///
    /// Used by color/light sensors to measure blue light intensity.
    Blue = 3,

    /// Overall light intensity
    ///
    /// Measures total light intensity across the visible spectrum.
    /// Used by ambient light sensors.
    Intensity = 4,

    /// Infrared channel
    ///
    /// Measures infrared light intensity. Used by proximity sensors
    /// and IR light sensors.
    Ir = 5,

    /// Ambient temperature
    ///
    /// Measures the temperature of the surrounding environment.
    /// Common in environmental sensors like BME280 or TMP11x.
    AmbientTemp = 13,

    /// Die (chip) temperature
    ///
    /// Measures the temperature of the sensor chip itself.
    /// Useful for temperature compensation or thermal monitoring.
    DieTemp = 14,

    /// Accelerometer data (3-axis)
    ///
    /// Measures linear acceleration in X, Y, and Z axes.
    /// Used by accelerometer sensors like ADXL345 or MPU6050.
    Accel = 15,

    /// Gyroscope data (3-axis)
    ///
    /// Measures angular velocity around X, Y, and Z axes.
    /// Used by gyroscope sensors like MPU6050 or L3GD20.
    Gyro = 16,

    /// Magnetometer data (3-axis)
    ///
    /// Measures magnetic field strength in X, Y, and Z axes.
    /// Used by magnetometer sensors for compass applications.
    Magn = 17,

    /// Relative humidity
    ///
    /// Measures the percentage of water vapor in the air.
    /// Common in environmental sensors like BME280 or SHT3x.
    Humidity = 22,

    /// Atmospheric pressure
    ///
    /// Measures barometric pressure. Used by pressure sensors
    /// like BMP280 or BME280 for weather monitoring or altitude estimation.
    Pressure = 23,

    /// Proximity detection
    ///
    /// Measures the proximity of nearby objects. Used by proximity
    /// sensors for presence detection or touchless interfaces.
    Prox = 24,

    /// Distance measurement
    ///
    /// Measures absolute distance to objects. Used by time-of-flight
    /// or ultrasonic distance sensors.
    Distance = 25,
}

/// Converts a raw integer channel ID to a `SensorChannel` enum variant
///
/// This implementation enables conversion from Zephyr's C integer channel values
/// to the type-safe Rust enum. Unknown or invalid channel IDs default to `All`.
///
/// # Examples
///
/// ```
/// use zephyr::sensor::types::SensorChannel;
///
/// let channel = SensorChannel::from(13);
/// assert_eq!(channel, SensorChannel::AmbientTemp);
///
/// // Invalid channels default to All
/// let unknown = SensorChannel::from(999);
/// assert_eq!(unknown, SensorChannel::All);
/// ```
impl From<i32> for SensorChannel {
    fn from(chan: i32) -> Self {
        match chan {
            0 => SensorChannel::All,
            1 => SensorChannel::Red,
            2 => SensorChannel::Green,
            3 => SensorChannel::Blue,
            4 => SensorChannel::Intensity,
            5 => SensorChannel::Ir,
            13 => SensorChannel::AmbientTemp,
            14 => SensorChannel::DieTemp,
            15 => SensorChannel::Accel,
            16 => SensorChannel::Gyro,
            17 => SensorChannel::Magn,
            22 => SensorChannel::Humidity,
            23 => SensorChannel::Pressure,
            24 => SensorChannel::Prox,
            25 => SensorChannel::Distance,
            _ => SensorChannel::All, // Default fallback
        }
    }
}

/// Sensor value (mirrors Zephyr's `struct sensor_value`)
///
/// Represents a sensor reading as two 32-bit integers: an integer part (`val1`)
/// and a fractional part (`val2`). This structure matches Zephyr's sensor value
/// representation for FFI compatibility.
///
/// The interpretation of `val1` and `val2` depends on the sensor type:
/// - Temperature: `val1` = degrees, `val2` = micro-degrees
/// - Pressure: `val1` = pascals, `val2` = micro-pascals
/// - Humidity: `val1` = percentage, `val2` = fractional percentage
///
/// # Examples
///
/// ```
/// use zephyr::sensor::types::SensorValue;
///
/// // Represent 25.5°C
/// let temp = SensorValue::new(25, 500_000);
///
/// // Or use convenience methods
/// let temp = SensorValue::from_millicelsius(25_500);
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct SensorValue {
    /// Integer part of the value
    pub val1: i32,
    /// Fractional part of the value (typically in micro-units)
    pub val2: i32,
}

impl SensorValue {
    /// Creates a new `SensorValue` with the given integer and fractional parts
    ///
    /// # Arguments
    ///
    /// * `val1` - The integer part of the value
    /// * `val2` - The fractional part of the value
    ///
    /// # Examples
    ///
    /// ```
    /// use zephyr::sensor::types::SensorValue;
    ///
    /// let value = SensorValue::new(42, 123_456);
    /// ```
    pub const fn new(val1: i32, val2: i32) -> Self {
        Self { val1, val2 }
    }

    /// Creates a `SensorValue` from a temperature in millicelsius
    ///
    /// Converts a temperature value in millicelsius (1/1000th of a degree)
    /// to the two-part representation. For example, 25_500 millicelsius
    /// represents 25.5°C.
    ///
    /// # Arguments
    ///
    /// * `mc` - Temperature in millicelsius
    ///
    /// # Examples
    ///
    /// ```
    /// use zephyr::sensor::types::SensorValue;
    ///
    /// // 25.5°C = 25,500 millicelsius
    /// let temp = SensorValue::from_millicelsius(25_500);
    /// assert_eq!(temp.val1, 25);
    /// assert_eq!(temp.val2, 500_000);
    /// ```
    pub fn from_millicelsius(mc: i32) -> Self {
        Self {
            val1: mc / 1000,
            val2: (mc % 1000) * 1000,
        }
    }
}

/// Driver error type
///
/// Represents errors that can occur during sensor operations. Each variant
/// corresponds to a specific failure condition and maps to a standard
/// Zephyr errno value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorError {
    /// Sensor is not ready or not responding
    ///
    /// Indicates the sensor device is not available, possibly due to:
    /// - Device not being initialized
    /// - Hardware failure or disconnection
    /// - Device in sleep or low-power mode
    ///
    /// Maps to errno `-ENODEV` (-19).
    NotReady,

    /// Invalid or unsupported channel requested
    ///
    /// The requested sensor channel is not valid for this sensor or
    /// not supported by the driver.
    ///
    /// Maps to errno `-EINVAL` (-22).
    InvalidChannel,

    /// I/O error during communication
    ///
    /// A hardware I/O error occurred during sensor communication,
    /// such as I²C or SPI bus failures.
    ///
    /// Maps to errno `-EIO` (-5).
    IoError,

    /// Invalid argument provided to function
    ///
    /// An invalid parameter was passed to a sensor function.
    ///
    /// Maps to errno `-EINVAL` (-22).
    InvalidArgument,

    /// Operation not supported by this sensor
    ///
    /// The requested operation is not implemented or supported
    /// by this particular sensor driver.
    ///
    /// Maps to errno `-ENOTSUP` (-95).
    NotSupported,
}

impl SensorError {
    /// Converts the error to its corresponding Zephyr errno value
    ///
    /// Returns the negative errno value that corresponds to this error type,
    /// matching Zephyr's error code conventions.
    ///
    /// # Examples
    ///
    /// ```
    /// use zephyr::sensor::types::SensorError;
    ///
    /// let err = SensorError::IoError;
    /// assert_eq!(err.to_errno(), -5); // -EIO
    /// ```
    pub const fn to_errno(self) -> i32 {
        match self {
            SensorError::NotReady => -19,       // -ENODEV
            SensorError::InvalidChannel => -22, // -EINVAL
            SensorError::IoError => -5,         // -EIO
            SensorError::InvalidArgument => -22,
            SensorError::NotSupported => -95, // -ENOTSUP
        }
    }
}

/// Result type for sensor operations
///
/// A type alias for `Result<T, SensorError>`, providing a convenient way to
/// return results from sensor driver functions that may fail.
///
/// # Examples
///
/// ```
/// use zephyr::sensor::types::{SensorResult, SensorError, SensorValue};
///
/// fn read_temperature() -> SensorResult<SensorValue> {
///     // ... sensor reading logic ...
///     Ok(SensorValue::new(25, 500_000))
/// }
/// ```
pub type SensorResult<T> = Result<T, SensorError>;
