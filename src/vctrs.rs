//! Given a `List` of `Geom` pointers, create a `vctrs_vctr`
//!
//! rsgeo uses a list of pointers to `Geom` structs as a representation of a
//! vector. These functions help to create those vctrs with the appropriate class
//! attributes. Note that Geom pointers themselves are not a vector but must be
//! contained in a list with the appropriate `rs_{GEOM}` class to be treated as such.
//! {rsgeo} has `c()` methods so that `c(geom_struct, geom_struct)` will result
//! in a vctrs_vctr of the appropriate type.
//!
//! Missing geometries are stored as an `extendr_api::NULL` object. Be sure to handle
//! them accordingly.
use extendr_api::prelude::*;
use extendr_api::List;

/// Converts a List of Geom pointers to a {vctrs} vctr
pub fn as_rsgeo_vctr(mut x: List, class: &str) -> Robj {
    x.set_class(geom_class(class)).unwrap().clone().into()
}

/// Create a `String` array of the vctrs class
pub fn geom_class(cls: &str) -> [String; 4] {
    let cls = cls.to_uppercase();
    let geom_class = "rs_".to_owned() + cls.as_str();

    [
        geom_class,
        String::from("rsgeo"),
        String::from("vctrs_vctr"),
        String::from("list"),
    ]
}

/// From a List, determine the {vctrs} class of the pointer list
pub fn determine_geoms_class(x: &List) -> [String; 4] {
    let class = x[0].class().unwrap().nth(0).unwrap();

    let all_identical = x
        .iter()
        .all(|(_, robj)| robj.class().unwrap().nth(0).unwrap() == class);

    let class = if all_identical {
        x[0].class().unwrap().nth(0).unwrap()
    } else {
        "geometrycollection"
    };

    geom_class(class)
}

/// Check if an object is an rsgeo vector
pub fn is_rsgeo(x: &List) -> Rbool {
    if x.is_null() {
        return Rbool::na();
    } else {
        let cls = x.class().unwrap().next().unwrap();
        return cls.starts_with("rs_").into();
    }
}

/// Panics if x is not an rsgeo vector
pub fn verify_rsgeo(x: &List) {
    let cls = x.class().unwrap().next().unwrap();
    if !cls.starts_with("rs_") {
        panic!("`x` must be a Rust geometry type")
    }
}

/// Returns the rsgeo vector type such as "point", "linestring", etc
pub fn rsgeo_type(x: &List) -> String {
    if !x.inherits("rsgeo") {
        panic!("object is not an `rsgeo` vector")
    }

    let cls = x.class().unwrap().next().unwrap();

    if !cls.starts_with("rs_") {
        panic!("Object is not an `rsgeo` vector with `rs_` prefix")
    }

    let mut cls = cls.to_string();
    cls.split_off(3).to_lowercase()
}
