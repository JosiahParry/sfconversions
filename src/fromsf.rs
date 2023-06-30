//! Conversion from {sf} R objects to geo_types 
//! 
//! Provides simple conversions between sfg, sfc objects from sf, and 
//! geometry primitives from geo_types that can be used with other 
//! georust libraries powered by [extendr](https://extendr.github.io/extendr/extendr_api/).
//! 
//! Due to the [orphan rule](https://github.com/Ixrec/rust-orphan-rules) conversion
//! directly from extendr `Lists` to geo_types is not possible. For that reason
//! a simple struct `Geom` is implemented with a single field `geom` which contains
//! a geo_types `Geometry` enum. 
//! 
//! ## Example
//! 
//! ```
//! use sfconversions::{sfg_to_geom, geom::Geom};
//! 
//! #[extendr]
//! fn extract_sfg(x: Robj) -> String {
//!     sfg_to_geom(x).unwrap().print()
//! }
//! ```

use extendr_api::prelude::*;


use crate::Geom;
use geo_types::Geometry;

use std::{
    error::Error,
    result::Result
};

extendr_module! {
    mod fromsf;
    fn sfg_to_rsgeo;
    fn sfc_to_rsgeo;
}


// this function is for rsgeo
#[extendr]
pub fn sfc_to_rsgeo(x: List) -> Vec<Robj> {
    x
        .into_iter()
        .map(|(_, robj)| {
            let geo = sfg_to_geom(robj);
            match geo {
                Ok(g) => g.into(),
                Err(_) => Robj::from(NULL)
            }
        }).collect::<Vec<Robj>>()
}

// These functions are for people who do not want to use rsgeo

/// Given an sfc object, creates a vector of `Option<Geometry>`. NULL geometries are stored
/// as `None` and non-null are `Some(Geometry)`. Match on the result to get the underlying
/// geo-types geometry object or handle null geometry. 
pub fn sfc_to_geometry(x: List) -> Vec<Option<Geometry>> {
    x
        .into_iter()
        .map(|(_, robj)| {
            let geo = sfg_to_geom(robj);
            match geo {
                Ok(g) => Some(g.geom),
                Err(_) => None
            }
        }).collect::<Vec<Option<Geometry>>>()
}


pub fn sfc_to_geoms(x: List) -> Vec<Option<Geom>> {
    x
        .into_iter()
        .map(|(_, robj)| {
            let geo = sfg_to_geom(robj);
            match geo {
                Ok(g) => Some(g),
                Err(_) => None
            }
        }).collect::<Vec<Option<Geom>>>()
}




/// Falliably takes an extendr `Robj` and returns a `Geom` struct.
/// Supports conversion from `"POINT"`, `"MULTIPOINT"`, `"LINESTRING"`, `"MULTILINESTRING"`,
/// `"POLYGON"`, and `"MULTIPOLYGON"` to their corresponding geo_type primitive. 
// `GEOMETRYCOLLECTION` are not supported.
/// 
/// ```
/// use extendr_api::prelude::*;
/// use extendr_api::Doubles;
/// use sfconversions::sfg_to_geometry;
/// // Create an extendr doubles object and set the appropriate class
/// let dbls = Doubles::from_values([0.0, 10.0])
///     .into_robj()
///     .set_class(["XY", "POINT", "sfg"])
///     .unwrap();
/// 
/// // convert using `sfg_to_geometry()` and extract the underlyig
/// let geo_primitive = sfg_to_geometry(dbls).geom;
/// ```
/// 
pub fn sfg_to_geom(x: Robj) -> Result<Geom, Box<dyn Error>> {

    let cls2 = x.class().unwrap().map(|x| x).collect::<Vec<&str>>();
    let cls = cls2[1];

    match cls {
        "POINT" => {
            let x = Doubles::try_from(x).unwrap();
            Ok(geom_point(x[0].0, x[1].0).into())
        }
        "MULTIPOINT" => {
            let x = RMatrix::from_robj(&x).unwrap();
            Ok(geom_multipoint(x).into())
        }
        "LINESTRING" => {
            let x = RMatrix::from_robj(&x).unwrap();
            Ok(geom_linestring(x).into())
        }

        "MULTILINESTRING" => {
            let x = List::try_from(x).unwrap();
            Ok(geom_multilinestring(x).into())
        }
        "POLYGON" => {
            let x = List::try_from(x).unwrap();
            Ok(geom_polygon(x).into())
        }

        "MULTIPOLYGON" => {
            let x = List::try_from(x).unwrap();
            Ok(geom_multipolygon(x).into())
        }

        &_ => Err(format!("Null or unsupported geometry type").into()),
    }
}

use crate::constructors::*;

#[extendr]
pub fn sfg_to_rsgeo(x: Robj) -> Robj {

    let cls2 = x.class().unwrap().map(|x| x).collect::<Vec<&str>>();
    let cls = cls2[1];

    match cls {
        "POINT" => {
            let x = Doubles::try_from(x).unwrap();
            geom_point(x[0].0, x[1].0)
        }
        "MULTIPOINT" => {
            let x = RMatrix::from_robj(&x).unwrap();
            geom_multipoint(x)
        }
        "LINESTRING" => {
            let x = RMatrix::from_robj(&x).unwrap();
            geom_linestring(x)
        }

        "MULTILINESTRING" => {
            let x = List::try_from(x).unwrap();
            geom_multilinestring(x)
        }
        "POLYGON" => {
            let x = List::try_from(x).unwrap();
            geom_polygon(x)
        }

        "MULTIPOLYGON" => {
            let x = List::try_from(x).unwrap();
            geom_multipolygon(x)
        }

        &_ => Robj::from(NULL)
    }
}