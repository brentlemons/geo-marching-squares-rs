# Java-to-Rust Marching Squares Port Reference

**STATUS**: This is the definitive reference for implementing marching squares in Rust to match Java behavior exactly.

**Source Analysis**: See [JAVA_ANALYSIS.md](./JAVA_ANALYSIS.md) for complete 1,460-line Java analysis.

---

## Critical Rules (NEVER VIOLATE)

1. **NO ROUNDING during interpolation or edge creation** - Only round at final GeoJSON output
2. **8-point array order MUST be**: clockwise from top-right area (Java Shape.java:229-236)
3. **HashMap keys are Point objects** - Must use exact Point coordinates (unrounded) for lookups
4. **Cosine interpolation formula EXACT**: `mu2 = (1 - cos(mu*PI))/2`, `newMu = 0.5 + (mu2-0.5)*0.999`
5. **Edge tracing uses Move directions**: RIGHT(col+1), DOWN(row+1), LEFT(col-1), UP(row-1), NONE(same cell)
6. **Deduplication is by Point.equals()**: Compare x, y, value, limit, side fields

---

## Implementation Checklist

### Phase 1: Core Data Structures

- [ ] `Point` struct with fields: `x: Option<f64>`, `y: Option<f64>`, `value: Option<f64>`, `limit: Option<f64>`, `side: Option<Side>`
- [ ] `Point::equals()` compares ALL fields (not just x,y)
- [ ] `Edge` struct with: `start: Point`, `end: Point`, `move_dir: Move`
- [ ] `Move` enum: `Right, Down, Left, Up, None`
- [ ] `Side` enum: `Top, Right, Bottom, Left`

### Phase 2: Config Calculation (Shape.java:45-56)

```rust
// EXACT formula from Java:
let config: u8 =
    (if tl < lower { 0 } else if tl >= upper { 128 } else { 64 }) |
    (if tr < lower { 0 } else if tr >= upper { 32 } else { 16 }) |
    (if br < lower { 0 } else if br >= upper { 8 } else { 4 }) |
    (if bl < lower { 0 } else if bl >= upper { 2 } else { 1 });
```

**Test**: Config 85 (1111 in ternary) = all corners in band = 64|16|4|1 = 85 ✓

### Phase 3: The 8-Point Array (Shape.java:226-246)

**CRITICAL**: This is the heart of the algorithm. Must match EXACTLY.

```rust
// Position 0: Top edge at TR corner
let p0 = if is_top_blank() {
    None
} else if tr >= upper {
    Some(Point::placeholder(tr, upper, Side::Top))  // x=null, y=null
} else if tr < lower {
    Some(Point::placeholder(tr, lower, Side::Top))  // x=null, y=null
} else {
    Some(top_right.clone())  // Actual corner Point with x,y set
};

// Position 1: Right edge at TR corner
let p1 = if is_right_blank() {
    None
} else if tr >= upper {
    Some(Point::placeholder(tr, upper, Side::Right))
} else if tr < lower {
    Some(Point::placeholder(tr, lower, Side::Right))
} else {
    Some(top_right.clone())  // SAME object as p0 if in band!
};

// ... positions 2-7 follow same pattern ...
```

**Blank Edge Rules** (Shape.java:135-142):
```rust
fn is_top_blank() -> bool {
    (tl >= upper && tr >= upper) || (tl < lower && tr < lower)
}
fn is_right_blank() -> bool {
    (tr >= upper && br >= upper) || (tr < lower && br < lower)
}
// etc...
```

**Test**: If TR is in band (val=15, lower=10, upper=20):
- Position 0: `Some(Point { x: Some(-99.0), y: Some(41.0), ... })`
- Position 1: Same Point object (reference equality)

### Phase 4: Point Deduplication (Shape.java:238)

```rust
// Java: List<Point> slim = eightPoints.stream().distinct().filter(x -> x!=null).collect(...);

let mut slim: Vec<Point> = Vec::new();
for point in eight_points.into_iter().flatten() {
    if !slim.contains(&point) {  // Uses Point::eq() - ALL fields
        slim.push(point);
    }
}
```

**Point::eq() implementation**:
```rust
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.value == other.value &&
        self.limit == other.limit &&
        self.side == other.side
    }
}
```

**Key insight**: When TR is in band:
- Positions 0 and 1 both reference the SAME top_right Point
- `.distinct()` removes duplicates by object equality
- Result: Only ONE Point in slim array for that corner

### Phase 5: Interpolation (Shape.java:492-511)

```rust
fn interpolate(level: f64, value0: f64, value1: f64, point0: &Point, point1: &Point) -> Point {
    let mu = (level - value0) / (value1 - value0);
    let mu2 = (1.0 - (mu * PI).cos()) / 2.0;
    let center_diff = (mu2 - 0.5) * 0.999;  // CRITICAL: 0.999 factor
    let new_mu = 0.5 + center_diff;

    let x = (1.0 - new_mu) * point0.x.unwrap() + new_mu * point1.x.unwrap();
    let y = (1.0 - new_mu) * point0.y.unwrap() + new_mu * point1.y.unwrap();

    // NO ROUNDING - return exact f64 values
    Point { x: Some(x), y: Some(y), value: None, limit: None, side: None }
}
```

**Interpolation trigger** (Shape.java:239-242):
```rust
for i in 0..slim.len() {
    if slim[i].x.is_none() && slim[i].y.is_none() {
        slim[i] = interpolate(slim[i].limit.unwrap(), slim[i].side.unwrap());
    }
}
```

**Test**:
- Input: level=10.0, value0=5.0, value1=15.0, point0=(-100,41), point1=(-100,40)
- mu = 0.5, mu2 = 0.5, center_diff = 0, new_mu = 0.5
- Result: Point(-100.0, 40.5) - UNROUNDED

### Phase 6: Edge Creation (Triangle.java:33-38 example)

```rust
// Triangle case 1 (config 1 = 0001)
if config == 1 {
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];

    if is_bottom {
        edges.insert(p0.clone(), Edge::new(p0.clone(), p1.clone(), Move::Left));
    }
    if is_left {
        edges.insert(p1.clone(), Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.insert(p2.clone(), Edge::new(p2.clone(), p0.clone(), Move::Down));
}
```

**HashMap structure**:
- Key: Start Point (exact coordinates, unrounded)
- Value: Edge object
- Lookup: `edges.get(&point)` uses Point bitwise equality for f64

### Phase 7: Edge Tracing (MarchingSquares.java:63-109)

```rust
let mut current_row = start_row;
let mut current_col = start_col;
let mut current_edge: Option<Edge> = None;
let mut all_edges = Vec::new();

loop {
    let cell = &mut cells[current_row][current_col];

    // Get chained edges from this cell
    let tmp_edges = if let Some(ref edge) = current_edge {
        cell.get_edges(Some(&edge.end))  // Continue from edge.end
    } else {
        cell.get_edges(None)  // Start fresh
    };

    if tmp_edges.is_empty() { break; }

    cell.increment_used_edges(tmp_edges.len());

    for edge in tmp_edges {
        cell.remove_edge(&edge.start);
        current_edge = Some(edge.clone());
        all_edges.push(edge.clone());

        // Check if ring closed (end == first start)
        if edge.end == all_edges[0].start {
            goto_on = false;
            break;
        }
    }

    if !goto_on { break; }

    // Move to next cell based on last edge's move direction
    match current_edge.unwrap().move_dir {
        Move::Right => current_col += 1,
        Move::Down => current_row += 1,
        Move::Left => current_col -= 1,
        Move::Up => current_row -= 1,
        Move::None => {} // Stay in same cell
    }
}
```

**Critical**: `edge.end == all_edges[0].start` uses **bitwise f64 equality**

### Phase 8: Coordinate Rounding (MarchingSquares.java:101-105)

```rust
// ONLY at final GeoJSON output:
fn round_coordinate(coord: f64) -> f64 {
    use bigdecimal::BigDecimal;
    use bigdecimal::RoundingMode;

    BigDecimal::from_f64(coord)
        .unwrap()
        .round(5)  // 5 decimal places
        .to_f64()
        .unwrap()
}

// Apply when building GeoJSON:
let lon = round_coordinate(edge.start.x.unwrap());
let lat = round_coordinate(edge.start.y.unwrap());
```

**Test**:
- Input: -99.1234567
- Output: -99.12346 (HALF_UP rounding)

---

## Testing Priorities

### 1. Config Calculation
- [ ] All corners below: config = 0
- [ ] All corners in band: config = 85
- [ ] All corners above: config = 170
- [ ] Mixed: TL=below, TR=in, BR=above, BL=below → config = 0 | 16 | 8 | 0 = 24

### 2. 8-Point Array Generation
- [ ] Blank edges produce None
- [ ] In-band corners produce actual Point (x,y set)
- [ ] Out-of-band corners produce placeholder Point (x=null,y=null)
- [ ] Same corner in multiple positions → same Point reference

### 3. Deduplication
- [ ] Duplicate Points removed (by ALL fields, not just x,y)
- [ ] Placeholder Points with different sides NOT deduplicated

### 4. Interpolation
- [ ] Exact formula match (0.999 factor critical)
- [ ] Returns unrounded f64
- [ ] Division by zero → NaN (filtered later)

### 5. Edge Tracing
- [ ] HashMap lookup finds edges by exact Point
- [ ] Ring closes when end == first start
- [ ] Move directions navigate correctly

### 6. Coordinate Rounding
- [ ] Only at GeoJSON output
- [ ] 5 decimal places, HALF_UP

---

## Common Pitfalls to Avoid

1. ❌ **Rounding during interpolation** → breaks HashMap lookups
2. ❌ **Wrong 8-point array order** → wrong edge connections
3. ❌ **Deduplication by x,y only** → doesn't match Java .distinct()
4. ❌ **Using f64 epsilon comparison** → should use exact equality
5. ❌ **Missing the 0.999 factor** → points land on corners incorrectly

---

## File-by-File Implementation Order

1. `types.rs`: Point, Edge, Move, Side enums
2. `config.rs`: Config calculation (exact formula)
3. `point_generation.rs`: 8-point array with blank edge logic
4. `deduplication.rs`: Point.equals() and .distinct() equivalent
5. `interpolation.rs`: Exact cosine formula with 0.999
6. `shapes/`: All 7 shape types (Triangle, Pentagon, etc.)
7. `edge_tracing.rs`: Main loop with cell navigation
8. `output.rs`: GeoJSON with coordinate rounding

---

## Verification Commands

```bash
# Test config calculation
cargo test test_config_calculation

# Test 8-point generation
cargo test test_eight_point_array

# Test deduplication
cargo test test_point_deduplication

# Test interpolation
cargo test test_interpolation_exact

# Full integration
cargo test test_triangle_case_1
```

---

## When Context Compresses

If you lose context and need to resume:

1. Read this file first
2. Read JAVA_ANALYSIS.md for deep details
3. Check inline comments with `// Java: Shape.java:XXX` line references
4. Run tests to verify current state
5. **DO NOT guess** - always reference the Java source
