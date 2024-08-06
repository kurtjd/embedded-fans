//! Blocking Fan API
//!
//! This API provides generic methods for interfacing with fans.
//!
//! # For HAL authors
//!
//! Here is an example of an embedded-fans implementation of the Fan and RpmSense traits.
//!
//! ```
//! use embedded_fans::{self, Fan, RpmSense};
//!
//! // A struct representing a fan device.
//! pub struct MyFan {
//!     // ...
//! }
//!
//! #[derive(Clone, Copy, Debug)]
//! pub enum Error {
//!     // ...
//! }
//!
//! impl embedded_fans::Error for Error {
//!     fn kind(&self) -> embedded_fans::ErrorKind {
//!         match *self {
//!             // ...
//!         }
//!     }
//! }
//!
//! impl embedded_fans::ErrorType for MyFan {
//!     type Error = Error;
//! }
//!
//! impl Fan for MyFan {
//!     fn max_rpm(&self) -> u16 {
//!         3150
//!     }
//!
//!     fn min_rpm(&self) -> u16 {
//!         0
//!     }
//!
//!     fn min_start_rpm(&self) -> u16 {
//!         1120
//!     }
//!
//!     fn set_speed_rpm(&mut self, rpm: u16) -> Result<u16, Self::Error> {
//!         // ...
//!         Ok(rpm)
//!     }
//! }
//!
//! impl RpmSense for MyFan {
//!     fn rpm(&mut self) -> Result<u16, Self::Error> {
//!         // ...
//!         Ok(42)
//!     }
//! }
//! ```

#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

/// Fan error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic Fan error kind.
    ///
    /// By using this method, Fan errors freely defined by HAL implementations
    /// can be converted to a set of generic Fan errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

/// Fan error kind.
///
/// This represents a common set of Fan operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common Fan errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// An error occurred on the underlying peripheral supporting the fan.
    /// e.g. A PWM error occured for a PWM-controlled fan or a DAC error occured for a voltage-controlled fan.
    Peripheral,
    /// The fan is not capable of operating at the requested speed.
    InvalidSpeed,
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Peripheral => {
                write!(f, "An error occured on the underlying peripheral")
            }
            Self::InvalidSpeed => {
                write!(f, "Fan is not capable of operating at the requested speed")
            }
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Fan error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Blocking fan methods
pub trait Fan: ErrorType {
    /// Returns the maximum RPM the fan is capable of running at.
    fn max_rpm(&self) -> u16;

    /// Returns the minimum RPM the fan is capable of running at.
    fn min_rpm(&self) -> u16;

    /// Returns the minimum RPM needed for the fan to begin running from a dead stop
    /// (which may be the same as the minimum running speed).
    fn min_start_rpm(&self) -> u16;

    /// Sets the fan's speed in terms of absolute RPM.
    /// Returns the actual RPM set on success.
    fn set_speed_rpm(&mut self, rpm: u16) -> Result<u16, Self::Error>;

    /// Sets the fan's speed in terms of percent of maximum RPM.
    /// Returns the actual RPM set on success.
    #[inline]
    fn set_speed_percent(&mut self, percent: u8) -> Result<u16, Self::Error> {
        debug_assert!((0..=100).contains(&percent));

        // Cast operands to u32 to prevent overflow during multiplication
        self.set_speed_rpm(((u32::from(self.max_rpm()) * u32::from(percent)) / 100) as u16)
    }

    /// Sets the fan's speed to the maximum RPM it's capable of running at.
    #[inline]
    fn set_speed_max(&mut self) -> Result<(), Self::Error> {
        self.set_speed_rpm(self.max_rpm())?;
        Ok(())
    }

    /// Stops the fan completely.
    #[inline]
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.set_speed_rpm(0)?;
        Ok(())
    }
}

impl<T: Fan + ?Sized> Fan for &mut T {
    #[inline]
    fn max_rpm(&self) -> u16 {
        T::max_rpm(self)
    }

    #[inline]
    fn min_rpm(&self) -> u16 {
        T::min_rpm(self)
    }

    #[inline]
    fn min_start_rpm(&self) -> u16 {
        T::min_start_rpm(self)
    }

    #[inline]
    fn set_speed_rpm(&mut self, rpm: u16) -> Result<u16, Self::Error> {
        T::set_speed_rpm(self, rpm)
    }

    #[inline]
    fn set_speed_percent(&mut self, percent: u8) -> Result<u16, Self::Error> {
        T::set_speed_percent(self, percent)
    }

    #[inline]
    fn set_speed_max(&mut self) -> Result<(), Self::Error> {
        T::set_speed_max(self)
    }

    #[inline]
    fn stop(&mut self) -> Result<(), Self::Error> {
        T::stop(self)
    }
}

/// Blocking RPM sensing (tachometer) methods
pub trait RpmSense: ErrorType {
    /// Returns the fan's currently measured RPM.
    fn rpm(&mut self) -> Result<u16, Self::Error>;
}

impl<T: RpmSense + ?Sized> RpmSense for &mut T {
    #[inline]
    fn rpm(&mut self) -> Result<u16, Self::Error> {
        T::rpm(self)
    }
}
