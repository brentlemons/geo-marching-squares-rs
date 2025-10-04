//! Core marching squares algorithm implementation
//!
//! This module implements the marching squares algorithm for generating isobands (filled contours)
//! and isolines (contour lines) from geographic grid data.
//!
//! Two implementations are provided:
//! - Phase 1: Simple isoline-based (fast, basic)
//! - Phase 2: Full edge tracing with polygon nesting (accurate, complex)

use crate::cell_shapes::CellShape;
use crate::edge_tracing::{trace_all_rings, CellWithEdges};
use crate::error::Result;
use crate::grid::GeoGrid;
use crate::interpolation::interpolate_side;
use crate::polygon_util::organize_polygons;
use crate::types::{GridPoint, Point, Side};
use geojson::{Feature, Geometry, Value as GeoValue};

/// Generate isobands (filled contour polygons) for the given thresholds
///
/// For n thresholds, generates n-1 isobands, where each isoband represents
/// the area where values fall between consecutive thresholds.
///
/// Uses Phase 2 algorithm with edge tracing and polygon nesting.
/// If the 'parallel' feature is enabled, processes bands concurrently.
pub fn generate_isobands(grid: &GeoGrid, thresholds: &[f64]) -> Result<Vec<Feature>> {
    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;

        // Process bands in parallel
        let features: Result<Vec<Option<Feature>>> = (0..thresholds.len() - 1)
            .into_par_iter()
            .map(|i| {
                let lower = thresholds[i];
                let upper = thresholds[i + 1];
                generate_isobands_phase2(grid, lower, upper)
            })
            .collect();

        Ok(features?.into_iter().flatten().collect())
    }

    #[cfg(not(feature = "parallel"))]
    {
        let mut features = Vec::new();

        // Process each band sequentially
        for i in 0..thresholds.len() - 1 {
            let lower = thresholds[i];
            let upper = thresholds[i + 1];

            let band = generate_isobands_phase2(grid, lower, upper)?;
            if let Some(feature) = band {
                features.push(feature);
            }
        }

        Ok(features)
    }
}

/// Generate isolines (contour lines) for the given levels
pub fn generate_isolines(grid: &GeoGrid, levels: &[f64]) -> Result<Vec<Feature>> {
    let mut features = Vec::new();

    for &level in levels {
        let line = process_isoline(grid, level)?;
        if let Some(feature) = line {
            features.push(feature);
        }
    }

    Ok(features)
}

/// Process a single isoband between lower and upper thresholds
fn process_band(grid: &GeoGrid, lower: f64, upper: f64) -> Result<Option<Feature>> {
    let rows = grid.rows();
    let cols = grid.cols();

    // Storage for all polygon rings
    let mut polygons: Vec<Vec<Vec<f64>>> = Vec::new();

    // Process each cell in the grid
    for row in 0..rows - 1 {
        for col in 0..cols - 1 {
            // Get the four corners of the cell
            let tl = grid.get(row, col).unwrap();
            let tr = grid.get(row, col + 1).unwrap();
            let br = grid.get(row + 1, col + 1).unwrap();
            let bl = grid.get(row + 1, col).unwrap();

            // Calculate the cell configuration value
            let config = calculate_cell_config(tl, tr, br, bl, lower, upper);

            // Skip empty cells
            if config == 0 || config == 0b10101010 {
                continue;
            }

            // Get the edges for this cell configuration
            if let Some(edges) = get_cell_edges(
                config,
                tl,
                tr,
                br,
                bl,
                lower,
                upper,
                grid.config().smoothing_factor.into(),
            ) {
                // Convert edges to polygon format
                for edge_list in edges {
                    let ring: Vec<Vec<f64>> = edge_list
                        .iter()
                        .map(|p| vec![
                            crate::types::round_coordinate(p.x),
                            crate::types::round_coordinate(p.y)
                        ])
                        .collect();
                    if ring.len() >= 3 {
                        polygons.push(ring);
                    }
                }
            }
        }
    }

    // If no polygons found, return None
    if polygons.is_empty() {
        // Return empty feature instead of None to maintain expected count
        let geometry = Geometry::new(GeoValue::MultiPolygon(vec![]));
        let mut feature = Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: Some(serde_json::Map::new()),
            foreign_members: None,
        };
        if let Some(ref mut props) = feature.properties {
            props.insert("lower_level".to_string(), serde_json::json!(lower));
            props.insert("upper_level".to_string(), serde_json::json!(upper));
        }
        return Ok(Some(feature));
    }

    // Create MultiPolygon geometry
    // For now, treat each ring as a separate polygon
    let multi_polygon: Vec<Vec<Vec<Vec<f64>>>> = polygons
        .into_iter()
        .map(|ring| vec![ring])
        .collect();

    let geometry = Geometry::new(GeoValue::MultiPolygon(multi_polygon));

    let mut feature = Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: Some(serde_json::Map::new()),
        foreign_members: None,
    };

    // Add properties
    if let Some(ref mut props) = feature.properties {
        props.insert("lower_level".to_string(), serde_json::json!(lower));
        props.insert("upper_level".to_string(), serde_json::json!(upper));
    }

    Ok(Some(feature))
}

/// Process a single isoline at the given level
fn process_isoline(grid: &GeoGrid, level: f64) -> Result<Option<Feature>> {
    let rows = grid.rows();
    let cols = grid.cols();

    let mut line_strings: Vec<Vec<Vec<f64>>> = Vec::new();

    // Process each cell in the grid
    for row in 0..rows - 1 {
        for col in 0..cols - 1 {
            let tl = grid.get(row, col).unwrap();
            let tr = grid.get(row, col + 1).unwrap();
            let br = grid.get(row + 1, col + 1).unwrap();
            let bl = grid.get(row + 1, col).unwrap();

            // Calculate cell configuration for isoline
            let config = calculate_isoline_config(tl, tr, br, bl, level);

            if config == 0 || config == 15 {
                continue;
            }

            // Get the line segments for this cell
            if let Some(segments) = get_isoline_segments(
                config,
                tl,
                tr,
                br,
                bl,
                level,
                grid.config().smoothing_factor.into(),
            ) {
                for segment in segments {
                    let line: Vec<Vec<f64>> = segment
                        .iter()
                        .map(|p| vec![
                            crate::types::round_coordinate(p.x),
                            crate::types::round_coordinate(p.y)
                        ])
                        .collect();
                    if line.len() >= 2 {
                        line_strings.push(line);
                    }
                }
            }
        }
    }

    if line_strings.is_empty() {
        return Ok(None);
    }

    let geometry = Geometry::new(GeoValue::MultiLineString(line_strings));

    let mut feature = Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: Some(serde_json::Map::new()),
        foreign_members: None,
    };

    if let Some(ref mut props) = feature.properties {
        props.insert("isovalue".to_string(), serde_json::json!(level));
    }

    Ok(Some(feature))
}

/// Calculate the configuration value for an isoband cell (3-level comparison)
///
/// Returns a value where each pair of bits represents a corner:
/// - 00 = below lower threshold
/// - 01 = between lower and upper
/// - 10 = above upper threshold
///
/// Bit pattern: [tl_hi][tl_lo][tr_hi][tr_lo][br_hi][br_lo][bl_hi][bl_lo]
fn calculate_cell_config(
    tl: &GridPoint,
    tr: &GridPoint,
    br: &GridPoint,
    bl: &GridPoint,
    lower: f64,
    upper: f64,
) -> u8 {
    let mut config = 0u8;

    // Top left (bits 7-6)
    if tl.value as f64 >= upper {
        config |= 0b10000000;
    } else if tl.value as f64 >= lower {
        config |= 0b01000000;
    }

    // Top right (bits 5-4)
    if tr.value as f64 >= upper {
        config |= 0b00100000;
    } else if tr.value as f64 >= lower {
        config |= 0b00010000;
    }

    // Bottom right (bits 3-2)
    if br.value as f64 >= upper {
        config |= 0b00001000;
    } else if br.value as f64 >= lower {
        config |= 0b00000100;
    }

    // Bottom left (bits 1-0)
    if bl.value as f64 >= upper {
        config |= 0b00000010;
    } else if bl.value as f64 >= lower {
        config |= 0b00000001;
    }

    config
}

/// Calculate the configuration value for an isoline cell (single level comparison)
///
/// Returns a 4-bit value where each bit represents whether a corner is above the threshold:
/// bit 3 = top-left, bit 2 = top-right, bit 1 = bottom-right, bit 0 = bottom-left
fn calculate_isoline_config(
    tl: &GridPoint,
    tr: &GridPoint,
    br: &GridPoint,
    bl: &GridPoint,
    level: f64,
) -> u8 {
    let mut config = 0u8;

    if tl.value as f64 >= level {
        config |= 0b1000;
    }
    if tr.value as f64 >= level {
        config |= 0b0100;
    }
    if br.value as f64 >= level {
        config |= 0b0010;
    }
    if bl.value as f64 >= level {
        config |= 0b0001;
    }

    config
}

/// Get the edges for a given isoband cell configuration
///
/// Returns None for empty cells, Some(Vec) for cells with edges
fn get_cell_edges(
    _config: u8,
    tl: &GridPoint,
    tr: &GridPoint,
    br: &GridPoint,
    bl: &GridPoint,
    lower: f64,
    _upper: f64,
    smoothing: f64,
) -> Option<Vec<Vec<Point>>> {
    // For simplicity, convert to basic marching squares using the lower threshold
    // TODO: Implement full 3-level isoband algorithm from Java implementation
    let simple_config = calculate_isoline_config(tl, tr, br, bl, lower);

    get_isoline_segments(simple_config, tl, tr, br, bl, lower, smoothing)
}

/// Get the line segments for a given isoline cell configuration
///
/// This implements the standard marching squares lookup table
fn get_isoline_segments(
    config: u8,
    tl: &GridPoint,
    tr: &GridPoint,
    br: &GridPoint,
    bl: &GridPoint,
    level: f64,
    smoothing: f64,
) -> Option<Vec<Vec<Point>>> {
    let tl_pt = Point::from_lon_lat(tl.lon, tl.lat);
    let tr_pt = Point::from_lon_lat(tr.lon, tr.lat);
    let br_pt = Point::from_lon_lat(br.lon, br.lat);
    let bl_pt = Point::from_lon_lat(bl.lon, bl.lat);

    let tl_val = tl.value as f64;
    let tr_val = tr.value as f64;
    let br_val = br.value as f64;
    let bl_val = bl.value as f64;

    // Marching squares lookup table
    let segments = match config {
        0 | 15 => return None, // All below or all above

        1 | 14 => {
            // Bottom-left corner
            let left = interpolate_side(
                level,
                Side::Left,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            let bottom = interpolate_side(
                level,
                Side::Bottom,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            vec![vec![left, bottom]]
        }

        2 | 13 => {
            // Bottom-right corner
            let bottom = interpolate_side(
                level,
                Side::Bottom,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            let right = interpolate_side(
                level,
                Side::Right,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            vec![vec![bottom, right]]
        }

        3 | 12 => {
            // Bottom edge
            let left = interpolate_side(
                level,
                Side::Left,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            let right = interpolate_side(
                level,
                Side::Right,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            vec![vec![left, right]]
        }

        4 | 11 => {
            // Top-right corner
            let right = interpolate_side(
                level,
                Side::Right,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            let top = interpolate_side(
                level,
                Side::Top,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            vec![vec![right, top]]
        }

        5 => {
            // Saddle case: top-right and bottom-left (ambiguous)
            // Use average to determine which way to connect
            let avg = (tl_val + tr_val + br_val + bl_val) / 4.0;
            if avg >= level {
                // Connect top-left to bottom-left, top-right to bottom-right
                let left = interpolate_side(
                    level,
                    Side::Left,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let bottom = interpolate_side(
                    level,
                    Side::Bottom,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let right = interpolate_side(
                    level,
                    Side::Right,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let top = interpolate_side(
                    level,
                    Side::Top,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                vec![vec![left, bottom], vec![right, top]]
            } else {
                let left = interpolate_side(
                    level,
                    Side::Left,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let top = interpolate_side(
                    level,
                    Side::Top,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let bottom = interpolate_side(
                    level,
                    Side::Bottom,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let right = interpolate_side(
                    level,
                    Side::Right,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                vec![vec![left, top], vec![bottom, right]]
            }
        }

        6 | 9 => {
            // Right edge
            let top = interpolate_side(
                level,
                Side::Top,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            let bottom = interpolate_side(
                level,
                Side::Bottom,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            vec![vec![top, bottom]]
        }

        7 | 8 => {
            // Top-left corner
            let top = interpolate_side(
                level,
                Side::Top,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            let left = interpolate_side(
                level,
                Side::Left,
                (&tl_pt, tl_val),
                (&tr_pt, tr_val),
                (&br_pt, br_val),
                (&bl_pt, bl_val),
                smoothing,
            );
            vec![vec![top, left]]
        }

        10 => {
            // Saddle case: top-left and bottom-right (ambiguous)
            let avg = (tl_val + tr_val + br_val + bl_val) / 4.0;
            if avg >= level {
                let top = interpolate_side(
                    level,
                    Side::Top,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let left = interpolate_side(
                    level,
                    Side::Left,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let bottom = interpolate_side(
                    level,
                    Side::Bottom,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let right = interpolate_side(
                    level,
                    Side::Right,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                vec![vec![top, left], vec![bottom, right]]
            } else {
                let top = interpolate_side(
                    level,
                    Side::Top,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let right = interpolate_side(
                    level,
                    Side::Right,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let bottom = interpolate_side(
                    level,
                    Side::Bottom,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                let left = interpolate_side(
                    level,
                    Side::Left,
                    (&tl_pt, tl_val),
                    (&tr_pt, tr_val),
                    (&br_pt, br_val),
                    (&bl_pt, bl_val),
                    smoothing,
                );
                vec![vec![top, right], vec![bottom, left]]
            }
        }

        _ => return None,
    };

    Some(segments)
}

/// Phase 2: Generate isobands using full edge tracing and polygon nesting
///
/// This is a more accurate implementation that:
/// - Creates cell shapes for each grid cell
/// - Traces complete polygon rings using edge-following
/// - Organizes polygons with proper hole detection
/// - Returns MultiPolygons with interior rings
pub fn generate_isobands_phase2(grid: &GeoGrid, lower: f64, upper: f64) -> Result<Option<Feature>> {
    let rows = grid.rows();
    let cols = grid.cols();

    // Create a 2D array of cells with their shapes
    let mut cells: Vec<Vec<Option<CellWithEdges>>> = Vec::with_capacity(rows - 1);

    for row in 0..rows - 1 {
        let mut cell_row = Vec::with_capacity(cols - 1);

        for col in 0..cols - 1 {
            let tl = grid.get(row, col).unwrap();
            let tr = grid.get(row, col + 1).unwrap();
            let br = grid.get(row + 1, col + 1).unwrap();
            let bl = grid.get(row + 1, col).unwrap();

            // Calculate cell configuration
            let config = calculate_cell_config(tl, tr, br, bl, lower, upper);

            // Create cell shape
            let is_top = row == 0;
            let is_right = col + 1 == cols - 1;
            let is_bottom = row + 1 == rows - 1;
            let is_left = col == 0;

            let shape_opt = CellShape::from_config(
                config,
                tl,
                tr,
                br,
                bl,
                lower,
                upper,
                grid.config().smoothing_factor.into(),
                grid.config().interpolation_method,
                is_top,
                is_right,
                is_bottom,
                is_left,
            );

            if let Some(shape) = shape_opt {
                // Debug TOP boundary cells only, and only first 20 columns
                if is_top && col < 20 {
                    eprintln!("🔍 TOP BOUNDARY ({},{}) config={} tl={:.2} tr={:.2} br={:.2} bl={:.2} edges={}",
                        row, col, config, tl.value, tr.value, br.value, bl.value, shape.edges.len());
                    for (start, edge) in &shape.edges {
                        eprintln!("   Edge: ({:.3},{:.3}) -> ({:.3},{:.3}) move={:?}",
                            start.x, start.y, edge.end.x, edge.end.y, edge.move_dir);
                    }
                }
                cell_row.push(Some(CellWithEdges::new(shape)));
            } else {
                cell_row.push(None);
            }
        }

        cells.push(cell_row);
    }

    // Trace all polygon rings
    let rings = trace_all_rings(&mut cells);

    // CRITICAL FIX: Match Java behavior - return None for empty results
    // Java filters out empty features (MarchingSquares.java:245)
    if rings.is_empty() {
        return Ok(None);
    }

    // Organize polygons with hole detection
    let organized = organize_polygons(rings);

    // Convert to GeoJSON MultiPolygon
    let multi_polygon: Vec<Vec<Vec<Vec<f64>>>> = organized
        .into_iter()
        .enumerate()
        .map(|(poly_idx, (exterior, holes))| {
            let mut polygon_rings = Vec::new();

            // CRITICAL FIX: Close the ring BEFORE rounding to ensure first == last after rounding
            // trace_ring returns rings where first and last are bitwise identical.
            // If we round first, they may round to different values, creating diagonal artifacts.
            // Solution: Duplicate the first point BEFORE rounding, then round all points together.
            let mut exterior_for_rounding = exterior.clone();
            if let Some(first) = exterior_for_rounding.first().cloned() {
                exterior_for_rounding.push(first); // Now guaranteed: first == last (bitwise)
            }

            // Now round all coordinates (including the duplicated closing point)
            let exterior_coords: Vec<Vec<f64>> = exterior_for_rounding
                .iter()
                .map(|p| vec![
                    crate::types::round_coordinate(p.x),
                    crate::types::round_coordinate(p.y)
                ])
                .collect();
            polygon_rings.push(exterior_coords);

            // Add interior rings (holes) - also must be closed BEFORE rounding
            for hole in holes {
                // Close the ring BEFORE rounding
                let mut hole_for_rounding = hole.clone();
                if let Some(first) = hole_for_rounding.first().cloned() {
                    hole_for_rounding.push(first);
                }

                // Now round all coordinates
                let hole_coords: Vec<Vec<f64>> = hole_for_rounding
                    .iter()
                    .map(|p| vec![
                        crate::types::round_coordinate(p.x),
                        crate::types::round_coordinate(p.y)
                    ])
                    .collect();
                polygon_rings.push(hole_coords);
            }

            polygon_rings
        })
        .collect();

    let geometry = Geometry::new(GeoValue::MultiPolygon(multi_polygon));

    let mut feature = Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: Some(serde_json::Map::new()),
        foreign_members: None,
    };

    if let Some(ref mut props) = feature.properties {
        props.insert("lower_level".to_string(), serde_json::json!(lower));
        props.insert("upper_level".to_string(), serde_json::json!(upper));
    }

    Ok(Some(feature))
}
