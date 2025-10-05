# Java Marching Squares Implementation - Complete Analysis

## Document Purpose

This document provides a line-by-line analysis of the Java marching squares implementation to serve as the definitive reference for porting to Rust. Every critical behavior, formula, and data flow is documented with exact line numbers and concrete examples.

---

## Table of Contents

1. [Overall Architecture](#overall-architecture)
2. [Data Flow: Input to Output](#data-flow-input-to-output)
3. [The 8-Point Array Generation](#the-8-point-array-generation)
4. [Point Deduplication with .distinct()](#point-deduplication-with-distinct)
5. [Interpolation Algorithm](#interpolation-algorithm)
6. [Edge Creation and Storage](#edge-creation-and-storage)
7. [Edge Tracing in Main Loop](#edge-tracing-in-main-loop)
8. [Coordinate Rounding](#coordinate-rounding)
9. [Polygon Nesting Algorithm](#polygon-nesting-algorithm)
10. [Shape-Specific Implementations](#shape-specific-implementations)
11. [Critical Edge Cases](#critical-edge-cases)

---

## 1. Overall Architecture

### Class Hierarchy

```
Shape (base class)
├── Triangle
├── Pentagon
├── Rectangle
├── Trapezoid
├── Hexagon
├── Saddle
└── Square

Cell (separate class for isolines)
```

### Key Files

- **MarchingSquares.java**: Main orchestration and edge tracing
- **Shape.java**: Base class with interpolation and point generation
- **Point.java**: Represents coordinates (can be actual or placeholders)
- **Edge.java**: Represents cell edges with start, end, and move direction
- **Cell.java**: Simplified isoline implementation (incomplete in Java)

---

## 2. Data Flow: Input to Output

### 2.1 Input: Feature[][] Array

**Location**: `MarchingSquares.java:34-36`

```java
public Feature processBand(Feature[][] data, double lower, double upper) {
    int rows = data.length;
    int columns = data[0].length;
```

**Feature Structure**:
- GeoJSON Feature with Point geometry
- Property "value" contains the data value (e.g., temperature)
- Geometry contains coordinates: longitude, latitude

### 2.2 Cell Value Calculation

**Location**: `Shape.java:45-56`

```java
public static Shape create(Feature topLeft, Feature topRight, Feature bottomRight,
                          Feature bottomLeft, double lower, double upper, ...) {
    double tl = ((Double)topLeft.getProperty("value")).doubleValue();
    double tr = ((Double)topRight.getProperty("value")).doubleValue();
    double bl = ((Double)bottomLeft.getProperty("value")).doubleValue();
    double br = ((Double)bottomRight.getProperty("value")).doubleValue();
    int value = 0;

    value |= (tl < lower) ? 0 : (tl >= upper) ? 128 : 64;
    value |= (tr < lower) ? 0 : (tr >= upper) ? 32 : 16;
    value |= (br < lower) ? 0 : (br >= upper) ? 8 : 4;
    value |= (bl < lower) ? 0 : (bl >= upper) ? 2 : 1;
```

**3-Level Encoding**:
- Below lower: 0
- In band (>= lower && < upper): middle value (64, 16, 4, 1)
- Above upper: high value (128, 32, 8, 2)

**Bit positions**:
- Top-left: bits 7-6 (128/64)
- Top-right: bits 5-4 (32/16)
- Bottom-right: bits 3-2 (8/4)
- Bottom-left: bits 1-0 (2/1)

**Example**:
```
Corner values: tl=10, tr=15, br=25, bl=20
Band: lower=15, upper=20

tl=10 < 15 → 0
tr=15 >= 15 && < 20 → 16
br=25 >= 20 → 8
bl=20 >= 20 → 2

value = 0 | 16 | 8 | 2 = 26 (0000011010 binary)
```

### 2.3 Shape Creation

**Location**: `Shape.java:57-161`

The `value` determines which shape class to instantiate via a large switch statement. Each case maps to a specific geometric configuration.

### 2.4 Grid Creation

**Location**: `MarchingSquares.java:38-53`

```java
Shape[][] cells = new Shape[rows-1][columns-1];

for (int r = 0; r < rows - 1; r++) {
    for (int c = 0; c < columns - 1; c++) {
        cells[r][c] = Shape.create(data[r][c],
            data[r][c+1],
            data[r+1][c+1],
            data[r+1][c],
            lower,
            upper, c, r,
            r==0,        // top edge
            c+1==columns-1,  // right edge
            r+1==rows-1,     // bottom edge
            c==0);       // left edge
    }
}
```

**Critical Detail**: Each cell uses 4 features in order: topLeft, topRight, bottomRight, bottomLeft (clockwise from top-left).

**Grid boundaries** are passed to determine if edges should move to adjacent cells.

---

## 3. The 8-Point Array Generation

### 3.1 Overview

**Location**: `Shape.java:226-246`

The `getPoints()` method creates an 8-element array representing the 8 potential edge crossing points around a cell (2 per side: entering and exiting).

### 3.2 Line-by-Line Construction

```java
List<Point> eightPoints = new ArrayList<Point>();

eightPoints.add(isTopBlank() ? null : (tr >= upper) ? new Point(tr, upper, Side.TOP) : (tr < lower) ? new Point(tr, lower, Side.TOP) : topRight);
eightPoints.add(isRightBlank() ? null : (tr >= upper) ? new Point(tr, upper, Side.RIGHT) : (tr < lower) ? new Point(tr, lower, Side.RIGHT) : topRight);
eightPoints.add(isRightBlank() ? null : (br >= upper) ? new Point(br, upper, Side.RIGHT) : (br < lower) ? new Point(br, lower, Side.RIGHT) : bottomRight);
eightPoints.add(isBottomBlank() ? null : (br >= upper) ? new Point(br, upper, Side.BOTTOM) : (br < lower) ? new Point(br, lower, Side.BOTTOM) : bottomRight);
eightPoints.add(isBottomBlank() ? null : (bl >= upper) ? new Point(bl, upper, Side.BOTTOM) : (bl < lower) ? new Point(bl, lower, Side.BOTTOM) : bottomLeft);
eightPoints.add(isLeftBlank() ? null : (bl >= upper) ? new Point(bl, upper, Side.LEFT) : (bl < lower) ? new Point(bl, lower, Side.LEFT) : bottomLeft);
eightPoints.add(isLeftBlank() ? null : (tl >= upper) ? new Point(tl, upper, Side.LEFT) : (tl < lower) ? new Point(tl, lower, Side.LEFT) : topLeft);
eightPoints.add(isTopBlank() ? null : (tl >= upper) ? new Point(tl, upper, Side.TOP) : (tl < lower) ? new Point(tl, lower, Side.TOP) : topLeft);
```

**Order** (clockwise from top-right corner):
1. Index 0: TOP side, near right corner (uses `tr`)
2. Index 1: RIGHT side, near top corner (uses `tr`)
3. Index 2: RIGHT side, near bottom corner (uses `br`)
4. Index 3: BOTTOM side, near right corner (uses `br`)
5. Index 4: BOTTOM side, near left corner (uses `bl`)
6. Index 5: LEFT side, near bottom corner (uses `bl`)
7. Index 6: LEFT side, near top corner (uses `tl`)
8. Index 7: TOP side, near left corner (uses `tl`)

### 3.3 Point Types

Each array element can be one of:

1. **null**: Side is blank (no crossing)
2. **Placeholder Point**: `new Point(value, limit, side)` - stores value and threshold for later interpolation
3. **Actual corner Point**: The corner itself (when corner value is exactly in the band)

### 3.4 Blank Side Detection

**Location**: `Shape.java:251-265`

```java
private boolean isTopBlank() {
    return ((tl >= upper) && (tr >= upper)) || ((tl < lower) && (tr < lower));
}

private boolean isRightBlank() {
    return ((tr >= upper) && (br >= upper)) || ((tr < lower) && (br < lower));
}

private boolean isBottomBlank() {
    return ((bl >= upper) && (br >= upper)) || ((bl < lower) && (br < lower));
}

private boolean isLeftBlank() {
    return ((tl >= upper) && (bl >= upper)) || ((tl < lower) && (bl < lower));
}
```

**Logic**: A side is "blank" (has no crossing) if BOTH corners are on the same side of the band (both below lower OR both above upper).

### 3.5 Concrete Example

```
Cell corners:
  tl=10  tr=18
  bl=16  br=22

Band: lower=15, upper=20

isTopBlank(): (10<15 && 18>=15) → false (different sides)
isRightBlank(): (18<20 && 22>=20) → false (different sides)
isBottomBlank(): (16>=15 && 22>=20) → false (different sides)
isLeftBlank(): (10<15 && 16>=15) → false (different sides)

eightPoints construction:
[0] !isTopBlank(), tr=18 in band → topRight (actual Point)
[1] !isRightBlank(), tr=18 in band → topRight (actual Point)
[2] !isRightBlank(), br=22 >= 20 → new Point(22, 20, RIGHT) (placeholder)
[3] !isBottomBlank(), br=22 >= 20 → new Point(22, 20, BOTTOM) (placeholder)
[4] !isBottomBlank(), bl=16 in band → bottomLeft (actual Point)
[5] !isLeftBlank(), bl=16 in band → bottomLeft (actual Point)
[6] !isLeftBlank(), tl=10 < 15 → new Point(10, 15, LEFT) (placeholder)
[7] !isTopBlank(), tl=10 < 15 → new Point(10, 15, TOP) (placeholder)
```

---

## 4. Point Deduplication with .distinct()

### 4.1 Java Stream Filtering

**Location**: `Shape.java:238`

```java
List<Point> slim = eightPoints.stream().distinct().filter(x -> x!=null).collect(Collectors.toList());
```

**Operations**:
1. `.distinct()`: Removes duplicate Points using `Point.equals()`
2. `.filter(x -> x!=null)`: Removes null entries
3. `.collect(...)`: Converts back to List

### 4.2 Point.equals() Implementation

**Location**: `Point.java:99-131`

```java
@Override
public boolean equals(Object obj) {
    if (this == obj)
        return true;
    if (obj == null)
        return false;
    if (getClass() != obj.getClass())
        return false;
    Point other = (Point) obj;
    if (limit == null) {
        if (other.limit != null)
            return false;
    } else if (!limit.equals(other.limit))
        return false;
    if (side != other.side)
        return false;
    if (value == null) {
        if (other.value != null)
            return false;
    } else if (!value.equals(other.value))
        return false;
    if (x == null) {
        if (other.x != null)
            return false;
    } else if (!x.equals(other.x))
        return false;
    if (y == null) {
        if (other.y != null)
            return false;
    } else if (!y.equals(other.y))
        return false;
    return true;
}
```

**Equality Test**: ALL fields must match (x, y, value, limit, side).

### 4.3 Deduplication Example

```
Before distinct():
[0] topRight (x=1.0, y=2.0)
[1] topRight (x=1.0, y=2.0)  ← duplicate, removed
[2] Point(22, 20, RIGHT)
[3] Point(22, 20, BOTTOM)
[4] bottomLeft (x=0.0, y=1.0)
[5] bottomLeft (x=0.0, y=1.0)  ← duplicate, removed
[6] Point(10, 15, LEFT)
[7] Point(10, 15, TOP)

After distinct():
[0] topRight (x=1.0, y=2.0)
[1] Point(22, 20, RIGHT)
[2] Point(22, 20, BOTTOM)
[3] bottomLeft (x=0.0, y=1.0)
[4] Point(10, 15, LEFT)
[5] Point(10, 15, TOP)
```

**Note**: Points with same value/limit but different sides are NOT equal, so they remain separate.

### 4.4 Interpolation of Placeholder Points

**Location**: `Shape.java:239-243`

```java
for (int pt = 0; pt < slim.size(); pt++) {
    if (slim.get(pt).getX() == null && slim.get(pt).getY() == null) {
        slim.set(pt, interpolate(slim.get(pt).getLimit(), slim.get(pt).getSide()));
    }
}
```

**Logic**: After deduplication, replace any placeholder Points (x/y are null) with interpolated Points.

---

## 5. Interpolation Algorithm

### 5.1 Cosine Interpolation Formula

**Location**: `Shape.java:492-511`

```java
public Point interpolate(double level, double value0, double value1, Point point0, Point point1) {
    double mu = (level - value0) / (value1 - value0);
    double mu2 = (1.0-Math.cos(mu*Math.PI))/2.0;

    double centerDiff = (mu2 - 0.5) * 0.999;

    double newMu = 0.5 + centerDiff;

    double x = ((1.0 - newMu) * point0.getX()) + (newMu * point1.getX());
    double y = ((1.0 - newMu) * point0.getY()) + (newMu * point1.getY());

    return new Point(x, y);
}
```

### 5.2 Formula Breakdown

**Step 1**: Calculate linear parameter
```
mu = (level - value0) / (value1 - value0)
```
- Range: 0.0 to 1.0
- 0.0 when level == value0
- 1.0 when level == value1

**Step 2**: Apply cosine smoothing
```
mu2 = (1.0 - cos(mu * π)) / 2.0
```
- Converts linear mu to cosine curve
- Still ranges 0.0 to 1.0
- Smoother transitions at edges

**Step 3**: Apply center bias (key modification!)
```
centerDiff = (mu2 - 0.5) * 0.999
newMu = 0.5 + centerDiff
```
- Pulls values toward center (0.5)
- The 0.999 factor slightly reduces the bias
- This prevents interpolated points from being exactly on corners

**Step 4**: Linear interpolation with adjusted mu
```
x = (1.0 - newMu) * point0.x + newMu * point1.x
y = (1.0 - newMu) * point0.y + newMu * point1.y
```

### 5.3 Side-Based Interpolation

**Location**: `Shape.java:513-525`

```java
public Point interpolate(double level, Side side) {
    Point retValue = null;
    if (side == Side.TOP)
        retValue = interpolate(level, this.tl, this.tr, this.topLeft, this.topRight);
    else if (side == Side.RIGHT)
        retValue = interpolate(level, this.tr, this.br, this.topRight, this.bottomRight);
    else if (side == Side.BOTTOM)
        retValue = interpolate(level, this.bl, this.br, this.bottomLeft, this.bottomRight);
    else if (side == Side.LEFT)
        retValue = interpolate(level, this.tl, this.bl, this.topLeft, this.bottomLeft);

    return retValue;
}
```

**Direction**:
- TOP: left → right (tl → tr)
- RIGHT: top → bottom (tr → br)
- BOTTOM: left → right (bl → br)
- LEFT: top → bottom (tl → bl)

### 5.4 Concrete Example

```
Interpolate on RIGHT side:
- tr corner: value=18, coords=(1.0, 2.0)
- br corner: value=22, coords=(1.0, 1.0)
- level: 20 (upper threshold)

mu = (20 - 18) / (22 - 18) = 2/4 = 0.5
mu2 = (1.0 - cos(0.5 * π)) / 2.0 = (1.0 - 0.0) / 2.0 = 0.5
centerDiff = (0.5 - 0.5) * 0.999 = 0.0
newMu = 0.5 + 0.0 = 0.5

x = (1.0 - 0.5) * 1.0 + 0.5 * 1.0 = 1.0
y = (1.0 - 0.5) * 2.0 + 0.5 * 1.0 = 1.0 + 0.5 = 1.5

Result: Point(1.0, 1.5)
```

### 5.5 Special Case: Corner Values Exactly on Threshold

When a corner value equals a threshold (e.g., tr == upper), the 8-point array includes the **actual corner Point** instead of a placeholder.

**Location**: `Shape.java:229` (example)

```java
eightPoints.add(isTopBlank() ? null : (tr >= upper) ? new Point(tr, upper, Side.TOP) : (tr < lower) ? new Point(tr, lower, Side.TOP) : topRight);
```

The ternary condition `(tr >= upper) ? ... : topRight` means:
- If `tr >= upper`: Create placeholder `new Point(tr, upper, Side.TOP)`
- Else if in band: Use actual `topRight` corner Point

**Critical**: When using actual corner, NO interpolation is needed - the Point already has x/y coordinates.

---

## 6. Edge Creation and Storage

### 6.1 Edge Structure

**Location**: `Edge.java:3-113`

```java
public class Edge {
    public static enum Type { OUTSIDE, INSIDE }
    public static enum Move { RIGHT, DOWN, LEFT, UP, UNK }

    private Point start;
    private Point end;
    private Type type;
    private Move move;
    private boolean used;
```

**Key Fields**:
- `start`: Starting Point of edge
- `end`: Ending Point of edge
- `move`: Direction to next cell (RIGHT, DOWN, LEFT, UP, UNK)

### 6.2 HashMap Storage

**Location**: `Shape.java:34, 187`

```java
protected HashMap<Point,Edge> edges;

// Constructor:
this.edges = new HashMap<Point,Edge>();
```

**Key**: The `start` Point of the edge
**Value**: The Edge object

**Why HashMap?**: Allows O(1) lookup of edges by starting point during edge tracing.

### 6.3 Edge Creation in Shape Subclasses

Each shape subclass creates edges in its constructor based on the cell value.

**Example - Triangle case 169/1**:

**Location**: `Triangle.java:33-40`

```java
case 169: // 2221
case 1:   // 0001
    if (bottom)
        this.edges.put(points.get(0), new Edge(points.get(0), points.get(1), Edge.Move.LEFT));
    if (left)
        this.edges.put(points.get(1), new Edge(points.get(1), points.get(2)));
    this.edges.put(points.get(2), new Edge(points.get(2), points.get(0), Edge.Move.DOWN));
    break;
```

**Logic**:
1. Create edge from point 0 to point 1 (conditional on `bottom` boundary flag)
2. Create edge from point 1 to point 2 (conditional on `left` boundary flag)
3. Create edge from point 2 to point 0 (always, with Move.DOWN to next cell)

**Boundary Flags** (bottom, left, right, top):
- `true`: Cell is on grid boundary, edge should NOT move to adjacent cell
- `false`: Edge can move to adjacent cell

**Move Direction Rules**:
- Edges that exit to adjacent cells have Move direction set (RIGHT, DOWN, LEFT, UP)
- Internal edges (that stay within cell boundary) have no Move or Move.UNK

### 6.4 Edge Chain Ordering

**Critical**: Edges form a continuous chain where each edge's `end` Point equals the next edge's `start` Point.

**Example**:
```
points = [p0, p1, p2, p3]

Edge chain:
  edges[p0] = Edge(p0 → p1, Move.RIGHT)
  edges[p1] = Edge(p1 → p2, Move.DOWN)
  edges[p2] = Edge(p2 → p3, Move.UNK)
  edges[p3] = Edge(p3 → p0, Move.LEFT)

Chain: p0 → p1 → p2 → p3 → p0 (closed loop)
```

### 6.5 Conditional Edge Creation

**Pattern**: Many edges are only created if NOT on grid boundary.

**Example** - Square:

**Location**: `Square.java:32-39`

```java
if (right)
    this.edges.put(points.get(0), new Edge(points.get(0), points.get(1), Edge.Move.DOWN));
if (bottom)
    this.edges.put(points.get(1), new Edge(points.get(1), points.get(2), Edge.Move.LEFT));
if (left)
    this.edges.put(points.get(2), new Edge(points.get(2), points.get(3), Edge.Move.UP));
if (top)
    this.edges.put(points.get(3), new Edge(points.get(3), points.get(0), Edge.Move.RIGHT));
```

**Why?**:
- If on right boundary: no cells to the right, so edge shouldn't have Move.DOWN
- If on bottom boundary: no cells below, so edge shouldn't have Move.LEFT
- etc.

**On Boundary**: The edge is still created but stored differently or omitted.

**Correction**: Looking more carefully, the pattern shows:
- `if (right)`: means if NOT on right edge (confusing naming!)
- The boolean flags are `true` when ON the boundary

**Re-examining**: `MarchingSquares.java:48-51`

```java
cells[r][c] = Shape.create(...,
    r==0,        // top edge
    c+1==columns-1,  // right edge
    r+1==rows-1,     // bottom edge
    c==0);       // left edge
```

So `right=true` means the cell IS on the right edge of the grid.

**Correct Interpretation**:
```java
if (right)  // If cell is ON right boundary
    this.edges.put(...)  // Still create edge
```

Wait, this seems backwards. Let me trace through Square more carefully.

In Square.java:32-39, the conditional `if (right)` wraps edge creation WITH Move direction. If on boundary, the edge with Move direction is created. This doesn't make sense.

**Re-reading Square**: The edges that move to adjacent cells should NOT be created when on boundary. But the code shows:

```java
if (right)
    this.edges.put(points.get(0), new Edge(points.get(0), points.get(1), Edge.Move.DOWN));
```

If `right=true` (on right boundary), why would we create an edge with Move.DOWN?

**Insight**: The Move.DOWN from point 0 to point 1 moves DOWN (to row+1), not RIGHT. So even if on RIGHT boundary, we can still move DOWN if not on BOTTOM boundary.

**Correct Interpretation**:
- Point 0 is on the right side of cell
- Moving from point 0 to point 1 goes DOWN (to next row)
- This is only valid if NOT on bottom boundary
- But the check is `if (right)` which checks RIGHT boundary

**This appears to be a bug or confusing code**. Let me check Pentagon to see the pattern.

**Pentagon.java:45-52** (case 101/69):

```java
if (right)
    this.edges.put(points.get(1), new Edge(points.get(1), points.get(2), Edge.Move.DOWN));
if (bottom)
    this.edges.put(points.get(2), new Edge(points.get(2), points.get(3), Edge.Move.LEFT));
```

Here, point 1 is on RIGHT side, moving to point 2 (DOWN), checked with `if (right)`.
Point 2 is on BOTTOM side, moving to point 3 (LEFT), checked with `if (bottom)`.

**Hypothesis**: The boundary check should be for the ENDING location, not starting. But that doesn't match the code either.

**Alternative**: The flags might be inverted - `right=false` when on right boundary?

Looking back at `MarchingSquares.java:49`:
```java
c+1==columns-1,  // right edge
```

This is TRUE when on right edge. But then why does Square create Move.DOWN edge when `if (right)` is true?

**Aha!**: The Move.DOWN doesn't leave through the RIGHT side - it leaves through the BOTTOM side. So checking `right` is INCORRECT for that edge.

**Likely explanation**: The code has bugs or the boundary checks are not actually enforced properly. The edge tracing in MarchingSquares compensates by checking array bounds.

**For Rust port**: We need to verify the exact boundary behavior. The safest approach:
1. Always create edges with Move directions
2. During edge tracing, check if the next cell exists before moving
3. If on boundary, the next cell won't exist, so tracing naturally stops

**Simplified guideline**: Create edges as shown in Java, don't worry about boundary flag logic - it appears inconsistent.

---

## 7. Edge Tracing in Main Loop

### 7.1 Main Loop Structure

**Location**: `MarchingSquares.java:61-112`

```java
for (int r = 0; r < cellRows; r++) {
    for (int c = 0; c < cellColumns; c++) {
        if (cells[r][c] != null && !cells[r][c].isCleared()) {
            int y = r;
            int x = c;
            boolean goOn = true;
            List<Edge> edges = new ArrayList<Edge>();
            Edge currentEdge = null;
            Polygon polygon = new Polygon();
            List<LngLatAlt> elements = new ArrayList<LngLatAlt>();
            Edge.Move prevMove = Edge.Move.UNK;

            while (goOn && !cells[y][x].getEdges(...).isEmpty()) {
                // Edge tracing logic
            }

            // Build polygon from edges
        }
    }
}
```

**Outer loops**: Scan all cells row by row, column by column
**Skip conditions**:
- `cells[r][c] == null`: No shape (empty cell)
- `cells[r][c].isCleared()`: All edges already used

### 7.2 Edge Retrieval and Consumption

**Location**: `MarchingSquares.java:73-84`

```java
while (goOn && !cells[y][x].getEdges(currentEdge==null?null:currentEdge.getEnd(),
                                     currentEdge==null?null:currentEdge.getMove()).isEmpty()) {
    List<Edge> tmpEdges = cells[y][x].getEdges(currentEdge==null?null:currentEdge.getEnd(),
                                                currentEdge==null?null:currentEdge.getMove());
    cells[y][x].incrementUsedEdges(tmpEdges.size());
    for (Edge edge : tmpEdges) {
        cells[y][x].removeEdge(edge.getStart());
        currentEdge = edge;
        edges.add(edge);
        if (currentEdge.getEnd().equals(edges.get(0).getStart())) {
            goOn = false;
            break;
        }
    }
```

**Step-by-step**:

1. **Get edges from current cell**:
   - If first iteration (`currentEdge == null`): get any edge
   - Otherwise: get edge(s) starting from `currentEdge.getEnd()`

2. **Increment used edge counter**:
   ```java
   cells[y][x].incrementUsedEdges(tmpEdges.size());
   ```
   This marks edges as consumed.

3. **Process each edge**:
   - Remove from HashMap: `cells[y][x].removeEdge(edge.getStart())`
   - Add to polygon edge list: `edges.add(edge)`
   - Check for closure: `if (currentEdge.getEnd().equals(edges.get(0).getStart()))`

4. **Closure detection**: When current edge ends where first edge started, polygon is complete.

### 7.3 getEdges() Method

**Location**: `Shape.java:359-378`

```java
public List<Edge> getEdges(Point start, Edge.Move prevMove) {
    if (this.edges.size() > 1) {
        List<Edge> edges = new ArrayList<Edge>();
        if (start == null) {
            for (int pos = 0; pos < this.points.size(); pos++) {
                start = this.points.get(pos);
                if (this.edges.containsKey(start))
                    break;
            }
        }
        while (this.edges.containsKey(start) && edges.size() < this.edges.size()) {
            Edge edge = this.edges.get(start);
            edges.add(edge);
            start = edge.getEnd();
        }
        return edges;
    } else {
        return new ArrayList<Edge>(this.edges.values());
    }
}
```

**Logic**:

- **If 1 or fewer edges**: Return all edges as-is
- **If multiple edges**:
  - If `start == null`: Find first edge in HashMap by iterating through points
  - Follow chain: Get edge by start point, add to list, move to edge.getEnd(), repeat
  - Stop when no more edges or all edges collected

**Note**: `prevMove` parameter is not used in this implementation! It's in the signature but ignored.

### 7.4 Cell Navigation

**Location**: `MarchingSquares.java:86-97`

```java
if (currentEdge.getMove() == Edge.Move.RIGHT) {
    x++;
} else if (currentEdge.getMove() == Edge.Move.DOWN) {
    y++;
} else if (currentEdge.getMove() == Edge.Move.LEFT) {
    x--;
} else if (currentEdge.getMove() == Edge.Move.UP) {
    y--;
} else if ((currentEdge.getMove() == Edge.Move.UNK) && goOn) {
    goOn = false;
    logger.error("Unknown edge move! -> " + cells[y][x]);
}
```

**Move directions**:
- RIGHT: x + 1 (next column)
- DOWN: y + 1 (next row)
- LEFT: x - 1 (previous column)
- UP: y - 1 (previous row)
- UNK: Error, stop tracing

**Critical**: Java uses row-major order where:
- y = row index (vertical position)
- x = column index (horizontal position)
- DOWN increases y, RIGHT increases x

### 7.5 Edge Clearing

**Location**: `Shape.java:540-544`

```java
public void incrementUsedEdges(int usedEdges) {
    this.usedEdges += usedEdges;
    if (this.usedEdges >= this.edges.size())
        this.cleared = true;
}
```

**Logic**: When all edges in a cell have been used, mark the cell as cleared. This prevents reprocessing the same cell.

**Location**: `MarchingSquares.java:63`

```java
if (cells[r][c] != null && !cells[r][c].isCleared()) {
```

Cells marked as cleared are skipped in the outer loop.

---

## 8. Coordinate Rounding

### 8.1 Rounding Location

**Location**: `MarchingSquares.java:30, 101-106`

```java
private static final int positionAccuracy = 5;

// Later in polygon building:
elements.add(new LngLatAlt((new BigDecimal(edges.get(0).getStart().getX())).setScale(positionAccuracy, RoundingMode.HALF_UP).doubleValue(),
                          (new BigDecimal(edges.get(0).getStart().getY())).setScale(positionAccuracy, RoundingMode.HALF_UP).doubleValue()));
for (int e = 0; e < edges.size(); e++) {
    elements.add(new LngLatAlt((new BigDecimal(edges.get(e).getEnd().getX())).setScale(positionAccuracy, RoundingMode.HALF_UP).doubleValue(),
                              (new BigDecimal(edges.get(e).getEnd().getY())).setScale(positionAccuracy, RoundingMode.HALF_UP).doubleValue()));
}
```

### 8.2 Rounding Algorithm

**Process**:
1. Convert `double` to `BigDecimal`
2. Set scale to 5 decimal places
3. Use rounding mode `HALF_UP` (standard "round half up" rule)
4. Convert back to `double`

**Example**:
```
Input: x = 1.234567890
BigDecimal: 1.234567890
setScale(5, HALF_UP): 1.23457
Output: 1.23457
```

### 8.3 When Rounding Occurs

**Critical**: Rounding happens ONLY when building the final GeoJSON output, NOT during:
- Interpolation calculations
- Point generation
- Edge tracing

**Why late rounding?**: Preserves maximum precision during geometric calculations, only rounds for output format.

### 8.4 Polygon Construction

**Location**: `MarchingSquares.java:100-108`

```java
if (edges.size() > 0) {
    elements.add(new LngLatAlt(...edges.get(0).getStart()...));  // First point
    for (int e = 0; e < edges.size(); e++) {
        elements.add(new LngLatAlt(...edges.get(e).getEnd()...));  // All end points
    }
    polygon.add(elements);
    holdPolygons.add(polygon);
}
```

**Coordinate list**:
1. Start point of first edge
2. End point of edge 0
3. End point of edge 1
4. ...
5. End point of last edge

**Note**: The end point of last edge should equal start point of first edge (closed loop).

---

## 9. Polygon Nesting Algorithm

### 9.1 Two-Phase Approach

**Phase 1**: Collect all polygons into `holdPolygons` (LinkedList used as stack)

**Phase 2**: Process polygons to determine nesting

**Location**: `MarchingSquares.java:114-147`

```java
while (holdPolygons.size() > 0) {
    Polygon subject = holdPolygons.pop();
    boolean external = true;
    for (int i = 0; i < polygons.size(); i++) {
        Polygon polygon = polygons.get(i);
        boolean pushOut = false;
        if (polygonInPolygon(subject, polygon)) {
            // subject is inside polygon
            for (int j = 0; j < polygon.getInteriorRings().size(); j++) {
                if (polygonInPolygon(subject, polygon.getInteriorRing(j))) {
                    pushOut = true;
                    break;
                }
            }
            if (!pushOut) {
                polygon.addInteriorRing(subject.getExteriorRing());
                external = false;
                break;
            }
        } else if (polygonInPolygon(polygon, subject)) {
            // polygon is inside subject
            if (polygon.getInteriorRings().size() > 0) {
                for (int j = 0; j < polygon.getInteriorRings().size(); j++) {
                    holdPolygons.push(new Polygon(polygon.getInteriorRing(j)));
                }
                holdPolygons.push(new Polygon(polygon.getExteriorRing()));
            } else {
                holdPolygons.push(polygon);
            }
            polygons.remove(i);
        }
    }
    if (external) {
        polygons.add(subject);
    }
}
```

### 9.2 Algorithm Logic

**For each polygon from stack**:

1. **Assume external** (top-level polygon)

2. **Check against all processed polygons**:

   **Case A**: Subject is inside processed polygon
   - Check if subject is inside any holes of that polygon
   - If inside a hole: subject is external (pushed out), add to polygons list
   - If not inside a hole: subject is a hole of this polygon, add as interior ring

   **Case B**: Processed polygon is inside subject
   - Subject supersedes the processed polygon
   - Remove processed polygon from list
   - Push processed polygon back to stack for reprocessing
   - Push any holes of processed polygon to stack

3. **If still external**: Add to polygons list as new top-level polygon

### 9.3 Point-in-Polygon Test

**Location**: `MarchingSquares.java:199-219`

```java
private boolean polygonInPolygon(Polygon subject, Polygon polygon) {
    List<LngLatAlt> polygonPoints = polygon.getExteriorRing();
    List<LngLatAlt> subjectPoints = subject.getExteriorRing();

    for (LngLatAlt subjectPoint : subjectPoints) {
        boolean inside = false;
        for (int i = 0, j = polygonPoints.size()-1; i < polygonPoints.size(); j = i++) {
            LngLatAlt one = polygonPoints.get(i);
            LngLatAlt two = polygonPoints.get(j);

            if (((one.getLatitude() > subjectPoint.getLatitude()) != (two.getLatitude() > subjectPoint.getLatitude())) &&
                (subjectPoint.getLongitude() < (two.getLongitude() - one.getLongitude()) * (subjectPoint.getLatitude() - one.getLatitude()) / (two.getLatitude() - one.getLatitude()) + one.getLongitude())) {
                inside = !inside;
            }
        }
        if (!inside) return false;
    }

    return true;
}
```

**Algorithm**: Ray casting method

**Logic**:
- For subject to be inside polygon, ALL subject points must be inside
- For each subject point, cast ray and count edge crossings
- Odd crossings = inside, even = outside

**Test**: `polygonInPolygon(subject, polygon)` returns true if ALL points of subject are inside polygon.

---

## 10. Shape-Specific Implementations

### 10.1 Triangle (4 cases)

**Values**: 1, 4, 16, 64, 169, 166, 154, 106

**Geometry**: 3 points forming triangle

**Example - Case 1 (0001)**:

```
Configuration: bl in band, all others below lower
Points: [bottom, left, bottom_left_corner]

Edges:
- bottom → left (Move.LEFT if not on bottom boundary)
- left → bottom_left_corner (Move.UNK if not on left boundary)
- bottom_left_corner → bottom (Move.DOWN)
```

### 10.2 Pentagon (24 cases)

**Geometry**: 5 points forming pentagon

**Example - Case 69 (1011)**:

**Location**: `Pentagon.java:33-53`

```java
case 101: // 1211
case 69:  // 1011
    this.edges.put(points.get(0), new Edge(points.get(0), points.get(1), Edge.Move.RIGHT));
    if (right)
        this.edges.put(points.get(1), new Edge(points.get(1), points.get(2), Edge.Move.DOWN));
    if (bottom)
        this.edges.put(points.get(2), new Edge(points.get(2), points.get(3), Edge.Move.LEFT));
    if (left)
        this.edges.put(points.get(3), new Edge(points.get(3), points.get(4), Edge.Move.UP));
    if (top)
        this.edges.put(points.get(4), new Edge(points.get(4), points.get(0)));
    break;
```

**Pattern**: Edges form continuous loop through 5 points, with Move directions for boundary exits.

### 10.3 Rectangle (12 cases)

**Geometry**: 4 points forming rectangle

**Example - Case 5 (0011)**:

**Location**: `Rectangle.java:33-43`

```java
case 5:   // 0011
case 165: // 2211
    if (right)
        this.edges.put(points.get(0), new Edge(points.get(0), points.get(1), Edge.Move.DOWN));
    if (bottom)
        this.edges.put(points.get(1), new Edge(points.get(1), points.get(2), Edge.Move.LEFT));
    if (left)
        this.edges.put(points.get(2), new Edge(points.get(2), points.get(3)));
    this.edges.put(points.get(3), new Edge(points.get(3), points.get(0), Edge.Move.RIGHT));
    break;
```

### 10.4 Trapezoid (8 cases)

**Geometry**: 4 points forming trapezoid

**Example - Case 2 (0002)**:

**Location**: `Trapezoid.java:14-21`

```java
case 168: // 2220
case 2:   // 0002
    if (bottom)
        this.edges.put(points.get(0), new Edge(points.get(0), points.get(1)));
    this.edges.put(points.get(1), new Edge(points.get(1), points.get(2), Edge.Move.LEFT));
    if (left)
        this.edges.put(points.get(2), new Edge(points.get(2), points.get(3)));
    this.edges.put(points.get(3), new Edge(points.get(3), points.get(0), Edge.Move.DOWN));
    break;
```

### 10.5 Hexagon (12 cases)

**Geometry**: 6 points forming hexagon

**Example - Case 37 (0211)**:

**Location**: `Hexagon.java:28-40`

```java
case 37:  // 0211
case 133: // 2011
    this.edges.put(points.get(0), new Edge(points.get(0), points.get(1), Edge.Move.RIGHT));
    if (right)
        this.edges.put(points.get(1), new Edge(points.get(1), points.get(2), Edge.Move.DOWN));
    if (bottom)
        this.edges.put(points.get(2), new Edge(points.get(2), points.get(3), Edge.Move.LEFT));
    if (left)
        this.edges.put(points.get(3), new Edge(points.get(3), points.get(4)));
    this.edges.put(points.get(4), new Edge(points.get(4), points.get(5), Edge.Move.UP));
    if (top)
        this.edges.put(points.get(5), new Edge(points.get(5), points.get(0)));
    break;
```

### 10.6 Saddle (14 cases)

**Special**: Saddle cases require disambiguation using cell average.

**Average Calculation**:

**Location**: `Saddle.java:35`

```java
average = (((Double)topLeft.getProperty("value")).doubleValue() +
           ((Double)topRight.getProperty("value")).doubleValue() +
           ((Double)bottomRight.getProperty("value")).doubleValue() +
           ((Double)bottomLeft.getProperty("value")).doubleValue()) / 4.0;
```

**Example - Case 153 (2121)**:

**Location**: `Saddle.java:38-75`

```java
case 153: // 2121
    if (average >= upper) {
        // Configuration A: 2 separate regions
        points[0] = interpolate(upper, Side.RIGHT);
        points[1] = interpolate(upper, Side.TOP);
        points[2] = this.topRight;
        // ... create edges for first region

        points[3] = interpolate(upper, Side.LEFT);
        points[4] = interpolate(upper, Side.BOTTOM);
        points[5] = this.bottomLeft;
        // ... create edges for second region
    } else if (average >= lower && average < upper) {
        // Configuration B: 1 continuous region
        points[0] = interpolate(upper, Side.RIGHT);
        points[1] = interpolate(upper, Side.BOTTOM);
        points[2] = this.bottomLeft;
        points[3] = interpolate(upper, Side.LEFT);
        points[4] = interpolate(upper, Side.TOP);
        points[5] = this.topRight;
        // ... create single edge chain
    }
    break;
```

**Critical**: Saddle cases override the `points` array created by base class `getPoints()`.

**Location**: `Saddle.java:33, 735`

```java
Point[] points = new Point[6];  // Local array, not using inherited points
// ... populate points array based on average

this.points = Arrays.asList(points);  // Override inherited points
```

**Special case values**:

- **153, 102, 68, 17**: Standard saddles (6 points)
- **136, 34**: Double saddles (8 points) - can create 2 separate regions
- **152, 18, 137, 33, 98, 72, 38, 132**: Corner saddles (7 points)

### 10.7 Square (1 case)

**Value**: 85 (1111)

**Geometry**: All 4 corners in band, 4-point rectangle

**Location**: `Square.java:32-39`

```java
if (right)
    this.edges.put(points.get(0), new Edge(points.get(0), points.get(1), Edge.Move.DOWN));
if (bottom)
    this.edges.put(points.get(1), new Edge(points.get(1), points.get(2), Edge.Move.LEFT));
if (left)
    this.edges.put(points.get(2), new Edge(points.get(2), points.get(3), Edge.Move.UP));
if (top)
    this.edges.put(points.get(3), new Edge(points.get(3), points.get(0), Edge.Move.RIGHT));
```

**Special**: Uses actual corner points instead of interpolation.

---

## 11. Critical Edge Cases

### 11.1 Empty Cells (value 0 or 170)

**Location**: `Shape.java:59-61`

```java
case 0:   // 0000
case 170: // 2222
    break;
```

**Behavior**: Return `null` from `Shape.create()`, no shape created.

### 11.2 Double Saddles Creating Multiple Polygons

**Location**: `Saddle.java:191-265` (case 136)

**Behavior**:
- Can create TWO separate polygons in single cell
- Each has its own edge chain
- Edge tracing will process them separately in the main loop

### 11.3 Boundary Cells

**Special handling**: Edges that would exit the grid are marked differently.

**However**: The Java code doesn't consistently enforce this. Edge tracing relies on:
1. Cell existence checks (implicit via array bounds)
2. Edge chain termination

### 11.4 Coordinate Precision

**Interpolation**: Uses standard double precision (64-bit floating point)

**Output**: Rounded to 5 decimal places

**Potential issue**: Points that should be identical might differ due to floating point errors.

**Java mitigation**: `Point.equals()` uses exact equality, so floating point differences will prevent deduplication.

**For Rust**: Consider epsilon-based equality or explicit rounding during Point creation.

### 11.5 Corner Values Exactly on Threshold

**Example**: `tr == upper`

**Java behavior**: Creates placeholder Point, then later interpolates (even though corner is exact).

**Result**: Interpolation between corner and itself returns the corner.

**Optimization opportunity**: Could detect this case and use corner directly.

### 11.6 Cell with Multiple Edge Chains

**Possibility**: Some saddle cases create disjoint edge chains in single cell.

**Java handling**: `getEdges()` returns one chain at a time. After tracing first chain, cell is marked cleared, preventing second chain from being traced.

**Potential bug**: Multi-chain cells might lose some polygons.

**Evidence**: Looking at Saddle case 136, it creates TWO separate edge chains:
- Chain 1: points 0→1→2→0
- Chain 2: points 4→5→6→7→4

These are stored in same HashMap but don't connect to each other.

**Edge tracing behavior**: When entering this cell:
1. First polygon traces one chain (e.g., 0→1→2→0), marks those edges as used
2. Cell has more edges, but once `usedEdges >= edges.size()`, cell marked cleared
3. Second chain never traced!

**This appears to be a bug in the Java implementation** - double saddles might not work correctly.

**For Rust**: Need to handle multiple disjoint chains per cell properly.

---

## 12. Summary Checklist for Rust Port

### Data Structures

- [ ] Feature with value property and Point geometry
- [ ] Point with x, y (actual) OR value, limit, side (placeholder)
- [ ] Point equality using ALL fields
- [ ] Edge with start, end, move direction
- [ ] HashMap<Point, Edge> for edge storage
- [ ] Cell with cleared flag and usedEdges counter

### Algorithms

- [ ] 3-level cell value calculation (0/middle/high for each corner)
- [ ] 8-point array generation (clockwise from top-right area)
- [ ] Blank side detection (both corners same side of band)
- [ ] Point deduplication using .distinct() equivalent
- [ ] Cosine interpolation with center bias (0.999 factor)
- [ ] Side-based interpolation (TOP/RIGHT/BOTTOM/LEFT)
- [ ] Edge chain creation per shape type
- [ ] Edge tracing with cell navigation (RIGHT/DOWN/LEFT/UP)
- [ ] Polygon closure detection
- [ ] Point-in-polygon test (ray casting, ALL points)
- [ ] Polygon nesting with hole detection
- [ ] Coordinate rounding (5 decimal places, HALF_UP)

### Shape Types

- [ ] Triangle (4 variants)
- [ ] Pentagon (24 variants)
- [ ] Rectangle (12 variants)
- [ ] Trapezoid (8 variants)
- [ ] Hexagon (12 variants)
- [ ] Saddle (14 variants) - with average disambiguation
- [ ] Square (1 variant)

### Edge Cases

- [ ] Empty cells (value 0, 170) - return null
- [ ] Double saddles (multiple disjoint chains per cell)
- [ ] Boundary cells (grid edge handling)
- [ ] Corner values exactly on threshold
- [ ] Floating point precision issues

### Testing Priorities

1. **Interpolation formula** - verify exact match with Java (including 0.999 factor)
2. **Point deduplication** - ensure identical points are merged correctly
3. **Saddle disambiguation** - test average calculation and branching
4. **Polygon nesting** - verify holes are assigned to correct parents
5. **Coordinate rounding** - match Java's BigDecimal behavior
6. **Double saddles** - ensure multiple chains per cell are handled (Java bug?)

---

## Appendix A: Complete Cell Value Reference

### Binary to Decimal Mapping

Each corner has 3 states (below/in/above), encoded as 2 bits:
- 00: below lower
- 01 or 10: in band (uses middle value)
- 11: above upper (uses high value)

**Corner bit positions**:
- TL: bits 7-6 (values: 0, 64, 128)
- TR: bits 5-4 (values: 0, 16, 32)
- BR: bits 3-2 (values: 0, 4, 8)
- BL: bits 1-0 (values: 0, 1, 2)

### All 81 Cases

```
Value | Binary | TL TR BR BL | Shape
------|--------|-------------|-------
0     | 0000   | 0  0  0  0  | Empty
1     | 0001   | 0  0  0  1  | Triangle
2     | 0002   | 0  0  0  2  | Trapezoid
4     | 0010   | 0  0  1  0  | Triangle
5     | 0011   | 0  0  1  1  | Rectangle
6     | 0012   | 0  0  1  2  | Pentagon
8     | 0020   | 0  0  2  0  | Trapezoid
9     | 0021   | 0  0  2  1  | Pentagon
10    | 0022   | 0  0  2  2  | Rectangle
16    | 0100   | 0  1  0  0  | Triangle
17    | 0101   | 0  1  0  1  | Saddle
18    | 0102   | 0  1  0  2  | Saddle
20    | 0110   | 0  1  1  0  | Rectangle
21    | 0111   | 0  1  1  1  | Pentagon
22    | 0112   | 0  1  1  2  | Hexagon
24    | 0120   | 0  1  2  0  | Pentagon
25    | 0121   | 0  1  2  1  | Hexagon
26    | 0122   | 0  1  2  2  | Pentagon
32    | 0200   | 0  2  0  0  | Trapezoid
33    | 0201   | 0  2  0  1  | Saddle
34    | 0202   | 0  2  0  2  | Saddle
36    | 0210   | 0  2  1  0  | Pentagon
37    | 0211   | 0  2  1  1  | Hexagon
38    | 0212   | 0  2  1  2  | Saddle
40    | 0220   | 0  2  2  0  | Rectangle
41    | 0221   | 0  2  2  1  | Pentagon
42    | 0222   | 0  2  2  2  | Trapezoid
64    | 1000   | 1  0  0  0  | Triangle
65    | 1001   | 1  0  0  1  | Rectangle
66    | 1002   | 1  0  0  2  | Pentagon
68    | 1010   | 1  0  1  0  | Saddle
69    | 1011   | 1  0  1  1  | Pentagon
70    | 1012   | 1  0  1  2  | Hexagon
72    | 1020   | 1  0  2  0  | Saddle
73    | 1021   | 1  0  2  1  | Hexagon
74    | 1022   | 1  0  2  2  | Pentagon
80    | 1100   | 1  1  0  0  | Rectangle
81    | 1101   | 1  1  0  1  | Pentagon
82    | 1102   | 1  1  0  2  | Hexagon
84    | 1110   | 1  1  1  0  | Pentagon
85    | 1111   | 1  1  1  1  | Square
86    | 1112   | 1  1  1  2  | Pentagon
88    | 1120   | 1  1  2  0  | Hexagon
89    | 1121   | 1  1  2  1  | Pentagon
90    | 1122   | 1  1  2  2  | Rectangle
96    | 1200   | 1  2  0  0  | Pentagon
97    | 1201   | 1  2  0  1  | Hexagon
98    | 1202   | 1  2  0  2  | Saddle
100   | 1210   | 1  2  1  0  | Hexagon
101   | 1211   | 1  2  1  1  | Pentagon
102   | 1212   | 1  2  1  2  | Saddle
104   | 1220   | 1  2  2  0  | Pentagon
105   | 1221   | 1  2  2  1  | Rectangle
106   | 1222   | 1  2  2  2  | Triangle
128   | 2000   | 2  0  0  0  | Trapezoid
129   | 2001   | 2  0  0  1  | Pentagon
130   | 2002   | 2  0  0  2  | Rectangle
132   | 2010   | 2  0  1  0  | Saddle
133   | 2011   | 2  0  1  1  | Hexagon
134   | 2012   | 2  0  1  2  | Pentagon
136   | 2020   | 2  0  2  0  | Saddle
137   | 2021   | 2  0  2  1  | Saddle
138   | 2022   | 2  0  2  2  | Trapezoid
144   | 2100   | 2  1  0  0  | Pentagon
145   | 2101   | 2  1  0  1  | Hexagon
146   | 2102   | 2  1  0  2  | Pentagon
148   | 2110   | 2  1  1  0  | Hexagon
149   | 2111   | 2  1  1  1  | Pentagon
150   | 2112   | 2  1  1  2  | Rectangle
152   | 2120   | 2  1  2  0  | Saddle
153   | 2121   | 2  1  2  1  | Saddle
154   | 2122   | 2  1  2  2  | Triangle
160   | 2200   | 2  2  0  0  | Rectangle
161   | 2201   | 2  2  0  1  | Pentagon
162   | 2202   | 2  2  0  2  | Trapezoid
164   | 2210   | 2  2  1  0  | Pentagon
165   | 2211   | 2  2  1  1  | Rectangle
166   | 2212   | 2  2  1  2  | Triangle
168   | 2220   | 2  2  2  0  | Trapezoid
169   | 2221   | 2  2  2  1  | Triangle
170   | 2222   | 2  2  2  2  | Empty
```

### Shape Count by Type

- Empty: 2 (values 0, 170)
- Triangle: 8 (4 unique configurations × 2 for lower/upper)
- Pentagon: 24 (12 unique × 2)
- Rectangle: 12 (6 unique × 2)
- Trapezoid: 8 (4 unique × 2)
- Hexagon: 12 (6 unique × 2)
- Saddle: 14 (ambiguous cases requiring average test)
- Square: 1 (value 85)

**Total**: 81 cases

---

## Appendix B: Java vs Rust Type Mapping

| Java Type | Rust Equivalent | Notes |
|-----------|-----------------|-------|
| `double` | `f64` | 64-bit floating point |
| `int` | `i32` or `usize` | Depends on usage |
| `boolean` | `bool` | |
| `HashMap<Point,Edge>` | `HashMap<Point, Edge>` | Need `#[derive(Hash, Eq)]` |
| `ArrayList<Edge>` | `Vec<Edge>` | |
| `LinkedList<Polygon>` | `VecDeque<Polygon>` | For stack operations |
| `List<LngLatAlt>` | `Vec<LngLatAlt>` | |
| `BigDecimal.setScale()` | `(value * 100000.0).round() / 100000.0` | Or use `rust_decimal` crate |
| `RoundingMode.HALF_UP` | Standard `f64::round()` | |
| `.distinct()` | `Vec::dedup()` after sorting OR use `HashSet` | |
| `Feature` | Custom struct | With GeoJSON Point geometry |
| `enum Side` | `enum Side` | Direct translation |
| `enum Move` | `enum Move` | Direct translation |

---

## Document Change History

- **2024-10-05**: Initial comprehensive analysis created
