## sfconversions

A minimal Rust library to convert geometry objects from the R package [{sf}](https://r-spatial.github.io/sf/) into [geo-types](https://docs.rs/geo-types/latest/geo_types/) geometry primitives using [extendr](https://extendr.github.io/extendr/extendr_api/).

Provides simple conversions between sfg, sfc objects from sf, and geometry primitives from geo_types that can be used with other georust libraries powered by [extendr](https://extendr.github.io/extendr/extendr_api/).

Due to the [orphan rule](https://github.com/Ixrec/rust-orphan-rules) conversion directly from extendr `Lists` to geo_types is not possible. For that reason a simple struct `Geom` is implemented with a single field `geom` which contains a geo_types `Geometry` enum. 

## Example
 
Basic conversion from sfg objects is done with `sfg_to_geo()`.

```rust
use sfconversions::{sfg_to_geometry, geom::Geom};

#[extendr]
fn sfg_to_geo(x: Robj) -> Geom {
    // takes a single `Robj` and converts it to `Geom`.
    // if the appropriate Robj class isn't found
    // a single point with coords (0, 0) is returned
    let geo = sfg_to_geometry(x);
    // extract the Geometry
    geo.geom
}
```

The `Geom` struct implement `From<Geom> for Robj` so converting from a Geom to the corresponding sfg object is fairly straight forward. Further, the `From` trait is also implemented for `Geometry` enums. 

sfconversions acts similarly to [sfheaders](https://github.com/dcooley/sfheaders) in that it returns the correct R objects with the appropriate classes without dependence upon sf. If sf is not available the conversions still will work but the print methods and other functions from sf will not be available. 

`From<Geom>` also applies the appropriate `sfg` classes making direct conversion to sf fairly straight forward. Note that there is no concept of a CRS in geo_type so that information will be dropped. 

Additionally, there is no conversion _to_ `sfc` objects at this moment. To create an `sfc` object return a `List` of `sfg` objects and in R use
`sf::st_sfc()` to complete the conversion. 

See examples in [h3o](https://github.com/JosiahParry/h3o/blob/e50144c57e1f6b997b0784bab12b9c9d4627c630/R/h3-constructors.R#L87) for conversion. 





