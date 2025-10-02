//! Simple example demonstrating how to generate contours from a geographic grid
//!
//! Run with: cargo run --example simple_contours

use geo_marching_squares_rs::{GeoGrid, GridPoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Geo Marching Squares Example ===\n");

    // Create a simple grid representing temperature data
    // Imagine this is from a weather model like HRRR
    println!("Creating a 5x5 grid with temperature values...");

    let mut points = Vec::new();
    for row in 0..5 {
        let mut row_points = Vec::new();
        for col in 0..5 {
            let lon = -100.0 + (col as f64) * 0.5;
            let lat = 40.0 + (row as f64) * 0.5;
            // Create a gradient pattern
            let value = 15.0 + (row as f32) * 3.0 + (col as f32) * 2.0;
            row_points.push(GridPoint::new(lon, lat, value));
        }
        points.push(row_points);
    }

    let grid = GeoGrid::from_points(points)?;

    println!("Grid bounds: {:?}", grid.bounds());
    println!("Value range: {:?}\n", grid.value_range());

    // Generate temperature isobands
    println!("Generating isobands (filled contours)...");
    let thresholds = vec![15.0, 20.0, 25.0, 30.0, 35.0];
    let isobands = grid.isobands(&thresholds)?;

    println!("Generated {} isobands:", isobands.len());
    for feature in &isobands {
        if let Some(ref props) = feature.properties {
            let lower = props.get("lower_level").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let upper = props.get("upper_level").and_then(|v| v.as_f64()).unwrap_or(0.0);
            println!("  - Band: {}°C to {}°C", lower, upper);
        }
    }

    // Generate temperature isolines
    println!("\nGenerating isolines (contour lines)...");
    let levels = vec![18.0, 22.0, 26.0, 30.0];
    let isolines = grid.isolines(&levels)?;

    println!("Generated {} isolines:", isolines.len());
    for feature in &isolines {
        if let Some(ref props) = feature.properties {
            let isovalue = props.get("isovalue").and_then(|v| v.as_f64()).unwrap_or(0.0);
            println!("  - Line at: {}°C", isovalue);
        }
    }

    // Demonstrate GeoJSON output
    println!("\n=== GeoJSON Output Example ===");
    if let Some(first_band) = isobands.first() {
        println!("First isoband as GeoJSON:");
        let json = serde_json::to_string_pretty(first_band)?;
        println!("{}", json);
    }

    println!("\n✓ Example completed successfully!");

    Ok(())
}
