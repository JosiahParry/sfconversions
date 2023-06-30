## sfconversions

A minimal Rust library to convert geometry objects from the R package [{sf}](https://r-spatial.github.io/sf/) into [geo-types](https://docs.rs/geo-types/latest/geo_types/) geometry primitives using [extendr](https://extendr.github.io/extendr/extendr_api/).

Provides simple conversions between sfg, sfc objects from sf, and geometry primitives from geo_types that can be used with other georust libraries powered by [extendr](https://extendr.github.io/extendr/extendr_api/).

Due to the [orphan rule](https://github.com/Ixrec/rust-orphan-rules) conversion directly from extendr `Lists` to geo_types is not possible. For that reason a simple struct `Geom` is implemented with a single field `geom` which contains a geo_types `Geometry` enum. 

## Example
 
Basic conversion from sfg objects is done with `sfg_to_geom()`.

```rust
use sfconversions::{sfg_to_geom, geom::Geom};
use extendr_api::prelude::*;

#[extendr]
fn extract_sfg(x: Robj) -> String {
  sfg_to_geom(x).unwrap().print()
}
```

The `Geom` struct is an extendr compatible struct with a single method which prints the string.

sfconversions acts similarly to [sfheaders](https://github.com/dcooley/sfheaders) in that it returns the correct R objects with the appropriate classes without dependence upon sf. If sf is not available the conversions still will work but the print methods and other functions from sf will not be available. 

It is important to note that sfconversions will _only_ create sfg objects and will not make `sfc` class object. This is because `sfc` objects require a bounding box attribute which can only be calculated using `geo` which is a larger dependency. To create an `sfc` object return a `List` of `sfg` objects and in R use
`sf::st_sfc()` to complete the conversion. Use `geos_to_sfc()` to aid in this process.
