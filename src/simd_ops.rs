//! SIMD-optimized operations for marching squares
//!
//! This module provides vectorized implementations of hot-path operations
//! using portable SIMD when available.

use crate::types::Point;

/// Batch interpolate multiple points using SIMD when available
///
/// This processes 4 interpolations at once using SIMD instructions
#[cfg(target_feature = "avx2")]
pub fn batch_interpolate_4(
    levels: &[f64; 4],
    values0: &[f64; 4],
    values1: &[f64; 4],
    points0: &[&Point; 4],
    points1: &[&Point; 4],
    smoothing_factor: f64,
) -> [Point; 4] {
    use std::arch::x86_64::*;

    unsafe {
        // Load values into SIMD registers
        let level_vec = _mm256_loadu_pd(levels.as_ptr());
        let value0_vec = _mm256_loadu_pd(values0.as_ptr());
        let value1_vec = _mm256_loadu_pd(values1.as_ptr());

        // Calculate mu = (level - value0) / (value1 - value0)
        let numerator = _mm256_sub_pd(level_vec, value0_vec);
        let denominator = _mm256_sub_pd(value1_vec, value0_vec);
        let mu = _mm256_div_pd(numerator, denominator);

        // Apply cosine smoothing: mu2 = (1.0 - cos(mu * PI)) / 2.0
        // Note: No vector cos in AVX, so we fall back to scalar for this part
        let mut mu_array = [0.0; 4];
        _mm256_storeu_pd(mu_array.as_mut_ptr(), mu);

        let mut mu2_array = [0.0; 4];
        for i in 0..4 {
            let mu_pi = mu_array[i] * std::f64::consts::PI;
            mu2_array[i] = (1.0 - mu_pi.cos()) / 2.0;
        }

        let mu2_vec = _mm256_loadu_pd(mu2_array.as_ptr());

        // Apply center bias: centerDiff = (mu2 - 0.5) * smoothing_factor
        let half_vec = _mm256_set1_pd(0.5);
        let smooth_vec = _mm256_set1_pd(smoothing_factor);
        let center_diff = _mm256_mul_pd(
            _mm256_sub_pd(mu2_vec, half_vec),
            smooth_vec
        );

        // newMu = 0.5 + centerDiff
        let new_mu_vec = _mm256_add_pd(half_vec, center_diff);

        // Store new_mu for point interpolation
        let mut new_mu_array = [0.0; 4];
        _mm256_storeu_pd(new_mu_array.as_mut_ptr(), new_mu_vec);

        // Interpolate points (still need scalar for now due to Point structure)
        let mut results = [Point::new(0.0, 0.0); 4];
        for i in 0..4 {
            let new_mu = new_mu_array[i];
            let x = (1.0 - new_mu) * points0[i].x + new_mu * points1[i].x;
            let y = (1.0 - new_mu) * points0[i].y + new_mu * points1[i].y;
            results[i] = Point::new(x, y);
        }

        results
    }
}

/// Fallback non-SIMD batch interpolation
#[cfg(not(target_feature = "avx2"))]
pub fn batch_interpolate_4(
    levels: &[f64; 4],
    values0: &[f64; 4],
    values1: &[f64; 4],
    points0: &[&Point; 4],
    points1: &[&Point; 4],
    smoothing_factor: f64,
) -> [Point; 4] {
    use crate::interpolation::interpolate_point;

    [
        interpolate_point(levels[0], values0[0], values1[0], points0[0], points1[0], smoothing_factor),
        interpolate_point(levels[1], values0[1], values1[1], points0[1], points1[1], smoothing_factor),
        interpolate_point(levels[2], values0[2], values1[2], points0[2], points1[2], smoothing_factor),
        interpolate_point(levels[3], values0[3], values1[3], points0[3], points1[3], smoothing_factor),
    ]
}

/// Vectorized threshold comparison for cell configuration
///
/// Compares 4 cell corner values against lower/upper thresholds simultaneously
#[inline]
pub fn vectorized_cell_config(
    tl: f64, tr: f64, br: f64, bl: f64,
    lower: f64, upper: f64
) -> u8 {
    let mut config = 0u8;

    // These comparisons could be vectorized, but they're already very fast
    // and the bit manipulation afterwards isn't easily vectorizable
    config |= if tl < lower { 0 } else if tl >= upper { 128 } else { 64 };
    config |= if tr < lower { 0 } else if tr >= upper { 32 } else { 16 };
    config |= if br < lower { 0 } else if br >= upper { 8 } else { 4 };
    config |= if bl < lower { 0 } else if bl >= upper { 2 } else { 1 };

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_interpolate() {
        let p0 = Point::new(-100.0, 40.0);
        let p1 = Point::new(-99.0, 40.0);
        let p2 = Point::new(-100.0, 41.0);
        let p3 = Point::new(-99.0, 41.0);

        let levels = [15.0, 15.0, 15.0, 15.0];
        let values0 = [10.0, 10.0, 10.0, 10.0];
        let values1 = [20.0, 20.0, 20.0, 20.0];
        let points0 = [&p0, &p1, &p2, &p3];
        let points1 = [&p1, &p0, &p3, &p2];

        let results = batch_interpolate_4(&levels, &values0, &values1, &points0, &points1, 0.999);

        // All should interpolate to approximately midpoint
        for result in &results {
            assert!(result.x > -100.0 && result.x < -99.0);
        }
    }

    #[test]
    fn test_vectorized_config() {
        let config = vectorized_cell_config(5.0, 15.0, 25.0, 35.0, 10.0, 20.0);

        // tl=5 (< 10): 0
        // tr=15 (10-20): 16
        // br=25 (>= 20): 8
        // bl=35 (>= 20): 2
        assert_eq!(config, 0 + 16 + 8 + 2);
    }
}
