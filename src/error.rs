//! Error types for the geo-marching-squares-rs crate

use thiserror::Error;

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during marching squares operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid grid dimensions: {message}")]
    InvalidDimensions { message: String },

    #[error("Invalid threshold values: {message}")]
    InvalidThresholds { message: String },

    #[error("Invalid coordinates: lat={lat}, lon={lon}")]
    InvalidCoordinates { lat: f64, lon: f64 },

    #[error("Empty grid provided")]
    EmptyGrid,

    #[error("Interpolation failed: {message}")]
    InterpolationError { message: String },

    #[error("GeoJSON conversion failed: {source}")]
    GeoJsonError {
        #[from]
        source: geojson::Error,
    },

    #[error("Geometric operation failed: {message}")]
    GeometryError { message: String },
}

impl Error {
    pub fn invalid_dimensions(message: impl Into<String>) -> Self {
        Self::InvalidDimensions {
            message: message.into(),
        }
    }

    pub fn invalid_thresholds(message: impl Into<String>) -> Self {
        Self::InvalidThresholds {
            message: message.into(),
        }
    }

    pub fn invalid_coordinates(lat: f64, lon: f64) -> Self {
        Self::InvalidCoordinates { lat, lon }
    }

    pub fn interpolation_error(message: impl Into<String>) -> Self {
        Self::InterpolationError {
            message: message.into(),
        }
    }

    pub fn geometry_error(message: impl Into<String>) -> Self {
        Self::GeometryError {
            message: message.into(),
        }
    }
}