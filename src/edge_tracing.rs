//! Edge tracing algorithm for marching squares
//!
//! This module implements the cell-to-cell edge following algorithm that traces
//! complete polygon rings from individual cell edges.

use crate::cell_shapes::CellShape;
use crate::types::{Edge, Point};
use smallvec::SmallVec;

/// A cell in the grid with its edges
#[derive(Debug, Clone)]
pub struct CellWithEdges {
    /// The shape configuration for this cell
    pub shape: CellShape,
    /// Number of edges that have been used
    pub used_edges: usize,
    /// Whether this cell has been fully processed
    pub cleared: bool,
}

impl CellWithEdges {
    /// Create a new cell with edges
    pub fn new(shape: CellShape) -> Self {
        Self {
            shape,
            used_edges: 0,
            cleared: false,
        }
    }

    /// Get the first available edge, if any
    pub fn get_first_edge(&self) -> Option<&Edge> {
        if self.cleared || self.used_edges >= self.shape.edges.len() {
            return None;
        }
        self.shape.edges.get(self.used_edges)
    }

    /// Get edges that start from a given point
    /// Uses SmallVec to avoid heap allocation for typical case (1-2 edges)
    pub fn get_edges_from(&self, start_point: &Point) -> SmallVec<[&Edge; 4]> {
        if self.cleared {
            return SmallVec::new();
        }

        self.shape
            .edges
            .iter()
            .skip(self.used_edges)
            .filter(|edge| points_equal(&edge.start, start_point))
            .collect()
    }

    /// Mark edges as used
    pub fn mark_edges_used(&mut self, count: usize) {
        self.used_edges += count;
        if self.used_edges >= self.shape.edges.len() {
            self.cleared = true;
        }
    }

    /// Check if this cell is cleared
    pub fn is_cleared(&self) -> bool {
        self.cleared
    }
}

/// Compare two points with floating point tolerance
fn points_equal(p1: &Point, p2: &Point) -> bool {
    const EPSILON: f64 = 1e-9;
    (p1.x - p2.x).abs() < EPSILON && (p1.y - p2.y).abs() < EPSILON
}

/// Trace a single polygon ring starting from a cell
///
/// Returns the list of points forming a closed ring, or None if tracing fails
pub fn trace_ring(
    cells: &mut Vec<Vec<Option<CellWithEdges>>>,
    start_row: usize,
    start_col: usize,
) -> Option<Vec<Point>> {
    let rows = cells.len();
    let cols = if rows > 0 { cells[0].len() } else { 0 };

    // Check if starting cell is valid
    let start_cell = cells.get(start_row)?.get(start_col)?.as_ref()?;
    if start_cell.is_cleared() {
        return None;
    }

    // Get the first available edge
    let first_edge = start_cell.get_first_edge()?.clone();
    let start_point = first_edge.start.clone();

    // Pre-allocate with estimated capacity (typical rings have 20-50 points)
    let mut points = Vec::with_capacity(32);
    points.push(start_point.clone());

    let mut current_row = start_row;
    let mut current_col = start_col;
    let mut current_edge = first_edge;

    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 10000; // Safety limit

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            // Safety check to prevent infinite loops
            return None;
        }

        // Check if we've closed the loop BEFORE adding the point
        // to avoid duplicate start/end points
        if points_equal(&current_edge.end, &start_point) {
            // Mark this edge as used before returning
            if let Some(Some(cell)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
                cell.mark_edges_used(1);
            }
            // Successfully closed the ring
            return Some(points);
        }

        // Add the end point only if it's not a duplicate of the last point
        // (This can happen with Move::None edges in the same cell)
        if points.is_empty() || !points_equal(points.last().unwrap(), &current_edge.end) {
            points.push(current_edge.end.clone());
        }

        // Mark this edge as used
        if let Some(Some(cell)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
            cell.mark_edges_used(1);
        }

        // Move to the next cell based on the edge direction
        let next_pos = current_edge.move_dir.apply(current_row, current_col);

        match next_pos {
            Some((next_row, next_col)) => {
                // Validate bounds
                if next_row >= rows || next_col >= cols {
                    // Reached grid boundary without closing
                    return None;
                }

                current_row = next_row;
                current_col = next_col;
            }
            None => {
                // Edge doesn't move to another cell (Move::None)
                // Look for another edge in the same cell that continues from this point
                if let Some(Some(cell)) = cells.get(current_row).and_then(|r| r.get(current_col)) {
                    let next_edges = cell.get_edges_from(&current_edge.end);
                    if let Some(&next_edge) = next_edges.first() {
                        current_edge = next_edge.clone();
                        continue;
                    }
                }
                // No continuation found
                return None;
            }
        }

        // Get the next cell
        let next_cell = match cells
            .get(current_row)
            .and_then(|r| r.get(current_col))
            .and_then(|c| c.as_ref())
        {
            Some(cell) => cell,
            None => return None, // No cell at this position
        };

        if next_cell.is_cleared() {
            // Cell has no more edges
            return None;
        }

        // Find an edge that starts where we ended
        let matching_edges = next_cell.get_edges_from(&current_edge.end);

        if matching_edges.is_empty() {
            // No matching edge found
            return None;
        }

        // Take the first matching edge
        current_edge = matching_edges[0].clone();
    }
}

/// Trace all polygon rings from a grid of cells
///
/// Returns a list of polygon rings (each ring is a Vec<Point>)
/// Only returns rings with at least 3 points (valid polygons per GeoJSON spec)
pub fn trace_all_rings(cells: &mut Vec<Vec<Option<CellWithEdges>>>) -> Vec<Vec<Point>> {
    let mut rings = Vec::new();

    let rows = cells.len();
    if rows == 0 {
        return rings;
    }
    let cols = cells[0].len();

    // Iterate through all cells looking for unprocessed edges
    for row in 0..rows {
        for col in 0..cols {
            // Keep tracing from this cell until all its edges are used
            while let Some(ring) = trace_ring(cells, row, col) {
                // Only include rings with at least 3 points
                // (GeoJSON requires at least 4 coordinates for a valid polygon ring,
                // with the first and last being identical. Since we don't duplicate
                // the closing point, we need at least 3 distinct points)
                if ring.len() >= 3 {
                    rings.push(ring);
                }
            }
        }
    }

    rings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Move;

    #[test]
    fn test_points_equal() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(1.0, 2.0);
        let p3 = Point::new(1.0, 2.1);

        assert!(points_equal(&p1, &p2));
        assert!(!points_equal(&p1, &p3));
    }

    #[test]
    fn test_cell_with_edges() {
        let edge1 = Edge::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), Move::Right);
        let edge2 = Edge::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Move::None);

        let shape = CellShape::new(vec![edge1.clone(), edge2.clone()]);
        let mut cell = CellWithEdges::new(shape);

        assert!(!cell.is_cleared());
        assert_eq!(cell.used_edges, 0);

        // Get first edge
        let first = cell.get_first_edge().unwrap();
        assert_eq!(first.start.x, 0.0);

        // Mark as used
        cell.mark_edges_used(1);
        assert_eq!(cell.used_edges, 1);
        assert!(!cell.is_cleared());

        // Mark second edge as used
        cell.mark_edges_used(1);
        assert_eq!(cell.used_edges, 2);
        assert!(cell.is_cleared());
    }

    #[test]
    fn test_get_edges_from() {
        let edge1 = Edge::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), Move::Right);
        let edge2 = Edge::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Move::None);
        let edge3 = Edge::new(Point::new(1.0, 0.0), Point::new(2.0, 0.0), Move::None);

        let shape = CellShape::new(vec![edge1, edge2, edge3]);
        let cell = CellWithEdges::new(shape);

        // Find edges starting from (1.0, 0.0)
        let edges = cell.get_edges_from(&Point::new(1.0, 0.0));
        assert_eq!(edges.len(), 2);

        // Find edges starting from (0.0, 0.0)
        let edges = cell.get_edges_from(&Point::new(0.0, 0.0));
        assert_eq!(edges.len(), 1);

        // Find edges starting from non-existent point
        let edges = cell.get_edges_from(&Point::new(5.0, 5.0));
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn test_simple_ring_trace() {
        // Create a simple single-cell ring
        // One cell with edges that form a closed loop
        let edge1 = Edge::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), Move::None);
        let edge2 = Edge::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Move::None);
        let edge3 = Edge::new(Point::new(1.0, 1.0), Point::new(0.0, 1.0), Move::None);
        let edge4 = Edge::new(Point::new(0.0, 1.0), Point::new(0.0, 0.0), Move::None);

        let cell = CellWithEdges::new(CellShape::new(vec![edge1, edge2, edge3, edge4]));

        let mut cells = vec![vec![Some(cell)]];

        let ring = trace_ring(&mut cells, 0, 0);

        assert!(ring.is_some(), "Ring tracing should succeed");
        let points = ring.unwrap();
        assert!(points.len() >= 4, "Should have at least 4 points");
        // First and last should be the same (closed loop)
        assert!(points_equal(&points[0], points.last().unwrap()));
    }
}
