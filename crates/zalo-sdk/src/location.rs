// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::error::{SdkError, SdkResult};

/// Geographic coordinates returned by the platform.
///
/// Latitude ranges from -90.0 to 90.0, longitude from -180.0 to 180.0. Values
/// outside these ranges are rejected at construction time.
///
/// # Examples
///
/// ```
/// use zalo_sdk::location::Coordinates;
///
/// let coords = Coordinates::new(10.7769, 106.7009)?;
/// assert_eq!(coords.latitude(), 10.7769);
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Coordinates {
    latitude:  f64,
    longitude: f64
}

impl Coordinates {
    const LAT_MIN: f64 = -90.0;
    const LAT_MAX: f64 = 90.0;
    const LNG_MIN: f64 = -180.0;
    const LNG_MAX: f64 = 180.0;

    /// Creates validated geographic coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidCoordinates`] when either value is outside
    /// the valid WGS-84 range or is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::location::Coordinates;
    ///
    /// let coords = Coordinates::new(21.0278, 105.8342)?;
    /// assert_eq!(coords.longitude(), 105.8342);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(latitude: f64, longitude: f64) -> SdkResult<Self> {
        if latitude.is_nan()
            || longitude.is_nan()
            || !(Self::LAT_MIN..=Self::LAT_MAX).contains(&latitude)
            || !(Self::LNG_MIN..=Self::LNG_MAX).contains(&longitude)
        {
            return Err(SdkError::InvalidCoordinates {
                latitude,
                longitude
            });
        }
        Ok(Self {
            latitude,
            longitude
        })
    }

    /// Returns the latitude component.
    #[must_use]
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Returns the longitude component.
    #[must_use]
    pub fn longitude(&self) -> f64 {
        self.longitude
    }
}

/// Precision level hint sent with a location request.
#[derive(Default, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationAccuracy {
    /// Best accuracy the device can provide.
    High,
    /// Balanced trade-off between accuracy and battery use.
    #[default]
    Balanced,
    /// Coarse accuracy, minimal battery impact.
    Low
}

/// Request parameters for the `getLocation` API.
///
/// # Examples
///
/// ```
/// use zalo_sdk::location::{GetLocationRequest, LocationAccuracy};
///
/// let req = GetLocationRequest::default();
/// assert_eq!(req.accuracy, LocationAccuracy::Balanced);
/// ```
#[derive(Default, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetLocationRequest {
    /// Desired accuracy level.
    pub accuracy: LocationAccuracy
}

/// Location data returned by the `getLocation` API.
///
/// # Examples
///
/// ```
/// use zalo_sdk::location::LocationResponse;
///
/// let json = r#"{"latitude":10.7769,"longitude":106.7009,"accuracy":15.0}"#;
/// let resp: LocationResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.latitude, 10.7769);
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LocationResponse {
    /// Latitude in decimal degrees.
    pub latitude:  f64,
    /// Longitude in decimal degrees.
    pub longitude: f64,
    /// Estimated accuracy radius in metres.
    pub accuracy:  f64
}

impl LocationResponse {
    /// Parses the raw response into validated [`Coordinates`].
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidCoordinates`] when the values are out of
    /// range.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::location::LocationResponse;
    ///
    /// let resp = LocationResponse {
    ///     latitude:  10.7769,
    ///     longitude: 106.7009,
    ///     accuracy:  10.0
    /// };
    /// let coords = resp.coordinates()?;
    /// assert_eq!(coords.latitude(), 10.7769);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn coordinates(&self) -> SdkResult<Coordinates> {
        Coordinates::new(self.latitude, self.longitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinates_rejects_nan_latitude() {
        let err = Coordinates::new(f64::NAN, 106.0).expect_err("nan lat");
        assert!(matches!(err, SdkError::InvalidCoordinates { .. }));
    }

    #[test]
    fn coordinates_rejects_out_of_range_latitude() {
        let err = Coordinates::new(91.0, 0.0).expect_err("lat > 90");
        assert!(matches!(err, SdkError::InvalidCoordinates { .. }));
    }

    #[test]
    fn coordinates_rejects_out_of_range_longitude() {
        let err = Coordinates::new(0.0, -181.0).expect_err("lng < -180");
        assert!(matches!(err, SdkError::InvalidCoordinates { .. }));
    }

    #[test]
    fn coordinates_accepts_boundary_values() {
        Coordinates::new(90.0, 180.0).expect("max boundary");
        Coordinates::new(-90.0, -180.0).expect("min boundary");
    }

    #[test]
    fn coordinates_accepts_valid_hcmc() {
        let c = Coordinates::new(10.7769, 106.7009).expect("hcmc");
        assert!((c.latitude() - 10.7769).abs() < 1e-10);
        assert!((c.longitude() - 106.7009).abs() < 1e-10);
    }

    #[test]
    fn location_accuracy_default_is_balanced() {
        assert_eq!(LocationAccuracy::default(), LocationAccuracy::Balanced);
    }

    #[test]
    fn location_response_parses_coordinates() {
        let resp = LocationResponse {
            latitude:  21.0278,
            longitude: 105.8342,
            accuracy:  5.0
        };
        let coords = resp.coordinates().expect("parse");
        assert!((coords.latitude() - 21.0278).abs() < 1e-10);
    }

    #[test]
    fn location_response_rejects_invalid_coordinates() {
        let resp = LocationResponse {
            latitude:  999.0,
            longitude: 0.0,
            accuracy:  0.0
        };
        let err = resp.coordinates().expect_err("out of range");
        assert!(matches!(err, SdkError::InvalidCoordinates { .. }));
    }

    #[test]
    fn get_location_request_default_has_balanced_accuracy() {
        let req = GetLocationRequest::default();
        assert_eq!(req.accuracy, LocationAccuracy::Balanced);
    }
}
