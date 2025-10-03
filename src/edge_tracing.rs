//! Edge tracing algorithm for marching squares
//!
//! This module implements the cell-to-cell edge following algorithm that traces
//! complete polygon rings from individual cell edges.

use crate::cell_shapes::CellShape;
use crate::types::{Edge, Point};

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

    /// Get chained edges starting from a given point (Java-style)
    ///
    /// Matches Java's getEdges(Point start, Edge.Move prevMove) behavior:
    /// 1. If start is None, find first available edge
    /// 2. Follow chain: edge.end becomes next search point
    /// 3. Return all edges in the chain
    pub fn get_chained_edges_from(&self, start_point: Option<&Point>) -> Vec<Edge> {
        if self.cleared || self.shape.edges.is_empty() {
            return Vec::new();
        }

        // Find starting point
        let mut current_start = if let Some(pt) = start_point {
            *pt
        } else {
            // No start point - use first available edge's start
            // Java iterates through points list to find first edge in HashMap
            match self.shape.edges.keys().next() {
                Some(pt) => *pt,
                None => return Vec::new(),
            }
        };

        let mut result = Vec::new();
        let max_edges = self.shape.edges.len();

        // Follow the chain of edges within this cell
        // Java: while (this.edges.containsKey(start) && edges.size() < this.edges.size())
        while result.len() < max_edges {
            if let Some(edge) = self.shape.edges.get(&current_start) {
                result.push(edge.clone());
                current_start = edge.end;
            } else {
                break; // No more edges in chain
            }
        }

        result
    }

    /// Remove an edge by its start point (matches Java removeEdge)
    pub fn remove_edge(&mut self, start_point: &Point) {
        self.shape.edges.remove(start_point);
    }

    /// Increment used edge counter and check if cleared
    pub fn increment_used_edges(&mut self, count: usize) {
        self.used_edges += count;
        if self.shape.edges.is_empty() {
            self.cleared = true;
        }
    }

    /// Check if this cell is cleared
    pub fn is_cleared(&self) -> bool {
        self.cleared || self.shape.edges.is_empty()
    }
}

/// Compare two points with floating point tolerance
/// Using 1e-6 (about 0.1 meters at equator) for geographic coordinate comparisons
/// after PROJ transformations which can introduce small numerical differences
fn points_equal(p1: &Point, p2: &Point) -> bool {
    const EPSILON: f64 = 1e-6;
    (p1.x - p2.x).abs() < EPSILON && (p1.y - p2.y).abs() < EPSILON
}

/// Trace a single polygon ring starting from a cell
///
/// Returns the list of points forming a closed ring, or None if tracing fails
///
/// This follows the Java algorithm:
/// 1. Get chained edges from current cell (may be multiple edges)
/// 2. Add all edge points to the ring
/// 3. Use the last edge's Move direction to go to next cell
/// 4. Repeat until ring closes
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

    // Get the chained edges from the starting cell (Java: getEdges(null, null))
    let first_edges = start_cell.get_chained_edges_from(None);
    if first_edges.is_empty() {
        return None;
    }

    let start_point = first_edges[0].start.clone();
    let mut all_edges = Vec::new();
    all_edges.extend(first_edges.clone());

    // Pre-allocate with estimated capacity
    let mut points = Vec::with_capacity(32);

    let mut current_row = start_row;
    let mut current_col = start_col;
    let mut current_edges = first_edges;

    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 10000;

    // Mark initial edges as used AND REMOVE THEM (matches Java lines 75-78)
    if let Some(Some(cell)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
        cell.increment_used_edges(current_edges.len());
        for edge in &current_edges {
            cell.remove_edge(&edge.start); // NEW: Actual removal
        }
    }

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return None;
        }

        // Get the last edge from the current batch (determines next move)
        let last_edge = match current_edges.last() {
            Some(edge) => edge,
            None => return None,
        };

        // Check if we've closed the loop
        if points_equal(&last_edge.end, &start_point) {
            break;
        }

        // Move to the next cell based on the last edge's direction
        let next_pos = last_edge.move_dir.apply(current_row, current_col);

        match next_pos {
            Some((next_row, next_col)) => {
                if next_row >= rows || next_col >= cols {
                    return None;
                }
                current_row = next_row;
                current_col = next_col;
            }
            None => {
                // Move::None means edge stays in same cell
                // This should only happen if loop closes in same cell
                // The check at the top of the loop will catch this on next iteration
                continue;
            }
        }

        // Get the next cell
        let next_cell = match cells
            .get(current_row)
            .and_then(|r| r.get(current_col))
            .and_then(|c| c.as_ref())
        {
            Some(cell) => cell,
            None => return None,
        };

        if next_cell.is_cleared() {
            return None;
        }

        // Get chained edges starting from where last edge ended
        let next_edges = next_cell.get_chained_edges_from(Some(&last_edge.end));

        if next_edges.is_empty() {
            // No matching edge - this ring cannot continue
            // This is normal when edges have been consumed by other rings
            return None;
        }

        // Mark edges as used AND REMOVE THEM (matches Java lines 75-78)
        if let Some(Some(cell)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
            cell.increment_used_edges(next_edges.len());
            for edge in &next_edges {
                cell.remove_edge(&edge.start); // NEW: Actual removal
            }
        }

        all_edges.extend(next_edges.clone());
        current_edges = next_edges;
    }

    // Build the points list from all edges (Java style: add start, then all ends)
    if all_edges.is_empty() {
        return None;
    }

    points.push(all_edges[0].start.clone());
    for edge in &all_edges {
        points.push(edge.end.clone());
    }

    if points.len() >= 3 {
        Some(points)
    } else {
        None
    }
}

/// Trace all polygon rings from a grid of cells
///
/// Returns a list of polygon rings (each ring is a Vec<Point>)
/// Only returns rings with at least 3 points (valid polygons per GeoJSON spec)
pub fn trace_all_rings(cells: &mut Vec<Vec<Option<CellWithEdges>>>) -> Vec<Vec<Point>> {
    let mut rings = Vec::new();
    let mut failed_traces = 0;
    let mut total_attempts = 0;

    let rows = cells.len();
    if rows == 0 {
        return rings;
    }
    let cols = cells[0].len();

    // Iterate through all cells looking for unprocessed edges
    for row in 0..rows {
        for col in 0..cols {
            // Keep tracing from this cell until all its edges are used
            loop {
                total_attempts += 1;
                match trace_ring(cells, row, col) {
                    Some(ring) => {
                        // Only include rings with at least 3 points
                        // (GeoJSON requires at least 4 coordinates for a valid polygon ring,
                        // with the first and last being identical. Since we don't duplicate
                        // the closing point, we need at least 3 distinct points)
                        if ring.len() >= 3 {
                            rings.push(ring);
                        }
                    }
                    None => {
                        // Check if there are still edges in this cell
                        if let Some(Some(cell)) = cells.get(row).and_then(|r| r.get(col)) {
                            if !cell.is_cleared() && !cell.shape.edges.is_empty() {
                                failed_traces += 1;
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    // Optional: Uncomment for debugging
    // eprintln!("\n📊 EDGE TRACING SUMMARY:");
    // eprintln!("   Total rings traced: {}", rings.len());
    // eprintln!("   Total trace attempts: {}", total_attempts);
    // eprintln!("   Failed traces: {}", failed_traces);

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
        let first = cell.get_chained_edges_from(None);
        assert!(!first.is_empty());

        // Remove the edge
        cell.remove_edge(&first[0].start);
        cell.increment_used_edges(1);
        assert_eq!(cell.used_edges, 1);
        assert!(!cell.is_cleared());

        // Remove second edge
        let second = cell.get_chained_edges_from(None);
        if !second.is_empty() {
            cell.remove_edge(&second[0].start);
            cell.increment_used_edges(1);
        }
        assert_eq!(cell.used_edges, 2);
        assert!(cell.is_cleared());
    }

    #[test]
    fn test_edge_chaining() {
        // Create a chain of edges: (0,0) -> (1,0) -> (1,1) -> (0,1)
        let edge1 = Edge::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), Move::Right);
        let edge2 = Edge::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Move::None);
        let edge3 = Edge::new(Point::new(1.0, 1.0), Point::new(0.0, 1.0), Move::None);

        let shape = CellShape::new(vec![edge1, edge2, edge3]);
        let cell = CellWithEdges::new(shape);

        // Start from (0,0), should follow chain through all 3 edges
        let edges = cell.get_chained_edges_from(Some(&Point::new(0.0, 0.0)));
        assert_eq!(edges.len(), 3);

        // Verify chain order
        assert_eq!(edges[0].start, Point::new(0.0, 0.0));
        assert_eq!(edges[0].end, Point::new(1.0, 0.0));
        assert_eq!(edges[1].end, Point::new(1.0, 1.0));
        assert_eq!(edges[2].end, Point::new(0.0, 1.0));

        // Find edges starting from non-existent point
        let edges = cell.get_chained_edges_from(Some(&Point::new(5.0, 5.0)));
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
