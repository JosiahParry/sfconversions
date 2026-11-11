## sfconversions

Strongly typed Rust objects to interface with the {sf} R package (todo hyperlink).

- Implements `geo-traits` easy intergration with the geo-rust ecosystem
- Implement conversions for `geo-types`
- Rudimentary `sfc` support

This crate provides scalar types for each `Sfg` variant that can be used directly with `extendr` (todo hyperlink).
Most importantly, this crate implements [`geo-traits`](TODO) which provides direct access to the `geo` library and generic geometry accessor functions.

## Example:

For example, we can take an `sfc_POINT` object and calculate its centroid as a `POINT` sfg object.

```rust
use extendr_api::prelude::*;
use sfconversions::{sfc::SfcPoint, sfg::SfgPoint};

#[extendr]
fn calc_centroid(x: SfcPoint) -> SfgPoint {
    let (sx, sy, n) = x.into_iter().fold((0.0, 0.0, 0usize), |(sx, sy, n), pnt| {
        match pnt.ok().and_then(|p| p.coord().map(|c| c.x_y())) {
            Some((x, y)) => (sx + x, sy + y, n + 1),
            None => (sx, sy, n),
        }
    });

    let p = point!(x: sx / n as f64, y :sy / n as f64);

    SfgPoint::from(p)
}
```
