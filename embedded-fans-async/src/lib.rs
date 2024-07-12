//! Async Fan API
//!
//! This API provides generic methods for interfacing with fans.
//!
//! # For HAL authors
//!
//! Here is an example of an embedded-fans implementation of the Fan and RpmSense traits.
//!
//! ```
//! use embedded_fans_async::{self, Fan, RpmSense};
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
//! impl embedded_fans_async::Error for Error {
//!     fn kind(&self) -> ErrorKind {
//!         match *self {
//!             // ...
//!         }
//!     }
//! }
//!
//! impl embedded_fans_async::ErrorType for MyFan {
//!     type Error = Error;
//! }
//!
//! impl Fan for MyFan {
//!     async fn set_speed_rpm(&mut self, rpm: u16) -> Result<u16, Self::Error> {
//!         // ...
//!         Ok(rpm)
//!     }
//!
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
//! }
//!
//! impl RpmSense for MyFan {
//!     async fn rpm(&mut self) -> Result<u16, Self::Error> {
//!         // ...
//!         Ok(42)
//!     }
//! }
//! ```

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![no_std]
#![allow(async_fn_in_trait)]

pub use embedded_fans::{Error, ErrorKind, ErrorType};

/// Async fan methods
pub trait Fan: ErrorType {
    /// Sets the fan's speed in terms of absolute RPM.
    /// Returns the actual RPM set on success.
    async fn set_speed_rpm(&mut self, rpm: u16) -> Result<u16, Self::Error>;

    /// Returns the maximum RPM the fan is capable of running at.
    fn max_rpm(&self) -> u16;

    /// Returns the minimum RPM the fan is capable of running at.
    fn min_rpm(&self) -> u16;

    /// Returns the minimum RPM needed for the fan to begin running from a dead stop
    /// (which may be the same as the minimum running speed).
    fn min_start_rpm(&self) -> u16;

    /// Sets the fan's speed in terms of percent of maximum RPM.
    /// Returns the actual RPM set on success.
    #[inline]
    async fn set_speed_percent(&mut self, percent: u8) -> Result<u16, Self::Error> {
        debug_assert!((0..=100).contains(&percent));
        self.set_speed_rpm((self.max_rpm() * u16::from(percent)) / 100)
            .await
    }

    /// Sets the fan's speed to the maximum RPM it's capable of running at.
    #[inline]
    async fn set_speed_max(&mut self) -> Result<(), Self::Error> {
        self.set_speed_rpm(self.max_rpm()).await?;
        Ok(())
    }

    /// Stops the fan completely.
    #[inline]
    async fn stop(&mut self) -> Result<(), Self::Error> {
        self.set_speed_rpm(0).await?;
        Ok(())
    }
}

impl<T: Fan + ?Sized> Fan for &mut T {
    #[inline]
    async fn set_speed_rpm(&mut self, rpm: u16) -> Result<u16, Self::Error> {
        T::set_speed_rpm(self, rpm).await
    }

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
    async fn set_speed_percent(&mut self, percent: u8) -> Result<u16, Self::Error> {
        T::set_speed_percent(self, percent).await
    }

    #[inline]
    async fn set_speed_max(&mut self) -> Result<(), Self::Error> {
        T::set_speed_max(self).await
    }

    #[inline]
    async fn stop(&mut self) -> Result<(), Self::Error> {
        T::stop(self).await
    }
}

/// Async RPM sensing (tachometer) methods
pub trait RpmSense: ErrorType {
    /// Returns the fan's currently measured RPM.
    async fn rpm(&mut self) -> Result<u16, Self::Error>;
}

impl<T: RpmSense + ?Sized> RpmSense for &mut T {
    #[inline]
    async fn rpm(&mut self) -> Result<u16, Self::Error> {
        T::rpm(self).await
    }
}
