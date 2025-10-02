//! Phase 2 demonstration with edge tracing and polygon nesting
//!
//! This example showcases the advanced features of Phase 2:
//! - Edge tracing algorithm
//! - Polygon nesting with hole detection
//! - Parallel band processing (with 'parallel' feature)
//!
//! Run with: cargo run --example phase2_demo

use geo_marching_squares_rs::{GeoGrid, GridPoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Phase 2 Marching Squares Demo ===\n");

    // Create a more complex grid that will produce nested polygons
    println!("Creating a 10x10 grid with complex patterns...");

    let mut points = Vec::new();
    for row in 0..10 {
        let mut row_points = Vec::new();
        for col in 0..10 {
            let lon = -100.0 + (col as f64) * 0.5;
            let lat = 40.0 + (row as f64) * 0.5;

            // Create a pattern with a "hole" in the middle
            let dist_from_center = ((row as f64 - 4.5).powi(2) + (col as f64 - 4.5).powi(2)).sqrt();

            let value = if dist_from_center < 2.0 {
                // Center hole - low values
                10.0
            } else if dist_from_center < 4.0 {
                // Ring - high values
                30.0 + dist_from_center as f32 * 2.0
            } else {
                // Outer area - medium values
                15.0 + dist_from_center as f32
            };

            row_points.push(GridPoint::new(lon, lat, value));
        }
        points.push(row_points);
    }

    let grid = GeoGrid::from_points(points)?;

    println!("Grid bounds: {:?}", grid.bounds());
    println!("Value range: {:?}\n", grid.value_range());

    // Generate isobands with Phase 2
    println!("Generating isobands with Phase 2 algorithm...");
    println!("Features:");
    println!("  - Edge tracing for accurate boundaries");
    println!("  - Polygon nesting with hole detection");
    #[cfg(feature = "parallel")]
    println!("  - Parallel processing enabled (rayon)");
    #[cfg(not(feature = "parallel"))]
    println!("  - Sequential processing (enable 'parallel' feature for speedup)");

    let thresholds = vec![10.0, 15.0, 20.0, 25.0, 30.0, 35.0];
    let isobands = grid.isobands(&thresholds)?;

    println!("\nGenerated {} isobands:", isobands.len());
    for (i, feature) in isobands.iter().enumerate() {
        if let Some(ref props) = feature.properties {
            let lower = props.get("lower_level").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let upper = props.get("upper_level").and_then(|v| v.as_f64()).unwrap_or(0.0);

            // Check if this band has holes
            if let Some(ref geom) = feature.geometry {
                if let geojson::Value::MultiPolygon(ref mp) = geom.value {
                    let total_polygons = mp.len();
                    let polygons_with_holes: usize = mp
                        .iter()
                        .filter(|poly| poly.len() > 1)
                        .count();

                    println!(
                        "  Band {}: {:.1}°C to {:.1}°C - {} polygon(s), {} with hole(s)",
                        i + 1,
                        lower,
                        upper,
                        total_polygons,
                        polygons_with_holes
                    );
                }
            }
        }
    }

    // Generate isolines for comparison
    println!("\nGenerating isolines...");
    let levels = vec![15.0, 20.0, 25.0, 30.0];
    let isolines = grid.isolines(&levels)?;

    println!("Generated {} isolines:", isolines.len());
    for (i, feature) in isolines.iter().enumerate() {
        if let Some(ref props) = feature.properties {
            let isovalue = props.get("isovalue").and_then(|v| v.as_f64()).unwrap_or(0.0);
            println!("  Line {}: {:.1}°C", i + 1, isovalue);
        }
    }

    // Show a sample feature with details
    println!("\n=== Sample Feature (First Isoband) ===");
    if let Some(first_band) = isobands.first() {
        let json = serde_json::to_string_pretty(first_band)?;
        println!("{}", json);
    }

    println!("\n✓ Phase 2 demo completed successfully!");
    println!("\nKey Phase 2 Features Demonstrated:");
    println!("  ✓ Edge tracing - Complete polygon rings");
    println!("  ✓ Polygon nesting - Proper hole detection");
    println!("  ✓ GeoJSON output - RFC 7946 compliant");
    #[cfg(feature = "parallel")]
    println!("  ✓ Parallel processing - Concurrent band generation");

    Ok(())
}
