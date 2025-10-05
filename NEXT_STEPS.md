# Next Steps for Marching Squares Implementation

**Last Updated**: After creating reference documents, before Step 2 implementation

## Current State

✅ **Step 1 Complete**: Reference documents created and committed
- `JAVA_ANALYSIS.md` - 1,460 line deep analysis
- `JAVA_PORT_REFERENCE.md` - 336 line implementation guide
- `CLAUDE.md` - Updated with critical context

⏳ **Step 2 In Progress**: Implement Rust version following reference
⏳ **Step 3 Pending**: Verify with tests

## Immediate Next Steps

### 1. Rewrite Point Structure (types.rs)

**Current problem**: Point only has `x: f64, y: f64` - no placeholder support

**Required change** (from JAVA_PORT_REFERENCE.md Phase 1):
```rust
#[derive(Clone, Debug)]
pub struct Point {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub value: Option<f64>,
    pub limit: Option<f64>,
    pub side: Option<Side>,
}

impl Point {
    // Actual point (has coordinates)
    pub fn actual(x: f64, y: f64) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            value: None,
            limit: None,
            side: None
        }
    }

    // Placeholder point (to be interpolated)
    pub fn placeholder(value: f64, limit: f64, side: Side) -> Self {
        Self {
            x: None,
            y: None,
            value: Some(value),
            limit: Some(limit),
            side: Some(side)
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.value == other.value &&
        self.limit == other.limit &&
        self.side == other.side
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash all fields for HashMap
        self.x.map(|v| v.to_bits()).hash(state);
        self.y.map(|v| v.to_bits()).hash(state);
        self.value.map(|v| v.to_bits()).hash(state);
        self.limit.map(|v| v.to_bits()).hash(state);
        self.side.hash(state);
    }
}
```

### 2. Fix 8-Point Generation (cell_shapes/mod.rs)

**Follow JAVA_PORT_REFERENCE.md Phase 3 exactly**:
- Must create placeholder Points for out-of-band corners
- Must use actual corner Points for in-band corners
- Exact clockwise order from Java Shape.java:229-236

### 3. Fix Deduplication (cell_shapes/mod.rs)

**Current (WRONG)**:
```rust
if !points.iter().any(|existing| {
    const EPSILON: f64 = 1e-9;
    (existing.x - pt.x).abs() < EPSILON && (existing.y - pt.y).abs() < EPSILON
})
```

**Correct (from JAVA_PORT_REFERENCE.md Phase 4)**:
```rust
let mut slim: Vec<Point> = Vec::new();
for point in eight_points.into_iter().flatten() {
    if !slim.contains(&point) {  // Uses Point::eq() - ALL fields
        slim.push(point);
    }
}
```

### 4. Add Interpolation Trigger (cell_shapes/mod.rs)

After deduplication, interpolate placeholder points:
```rust
for i in 0..slim.len() {
    if slim[i].x.is_none() && slim[i].y.is_none() {
        slim[i] = interpolate(
            slim[i].limit.unwrap(),
            slim[i].side.unwrap(),
            // ... pass corner values and points
        );
    }
}
```

### 5. Verify HashMap Usage (edge_tracing.rs)

With new Point::eq(), HashMap lookups should work correctly:
- Line 86: `self.shape.edges.get(&current_start)`
- Line 242: `if edge.end == all_edges[0].start`

Both use bitwise equality on Option<f64> - should work!

## Testing Priority (from JAVA_PORT_REFERENCE.md)

1. Test config calculation (verify 3-level encoding)
2. Test 8-point array (placeholder vs actual)
3. Test deduplication (ALL fields compared)
4. Test interpolation (0.999 factor, unrounded)
5. Test edge tracing (HashMap lookups work)

## Critical Rules (NEVER VIOLATE)

1. NO ROUNDING during interpolation - only at GeoJSON output
2. 8-point array EXACT order: clockwise from top-right
3. HashMap keys use Point with ALL fields
4. Cosine formula: `newMu = 0.5 + (mu2-0.5)*0.999`
5. Deduplication compares ALL Point fields

## Recovery After Context Compact

1. Read `JAVA_PORT_REFERENCE.md` first (implementation guide)
2. Read `CLAUDE.md` for project context
3. Read `JAVA_ANALYSIS.md` for deep details
4. Check this file for immediate next steps
5. **DO NOT GUESS** - reference Java at `/tmp/marching-squares-java/`

## File Locations

- Java source: `/tmp/marching-squares-java/src/main/java/com/fltck/data/wx/marchingsquares/`
- Reference docs: `JAVA_PORT_REFERENCE.md`, `JAVA_ANALYSIS.md`, `CLAUDE.md`
- Rust source: `src/`
- Current branch: `main`
- Last commit: `865155b` (reference docs added)
