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
pub fn as_rsgeo_vctr(x: List, class: [String; 3]) -> Robj {
    x
        .set_class(class)
        .unwrap()
}

/// Create a `String` array of the vctrs class
pub fn geom_class(cls: &str) -> [String; 3] {
    let cls = cls.to_uppercase();
    let geom_class = "rs_".to_owned() + cls.as_str();

    [geom_class, String::from("vctrs_vctr"), String::from("list")]
}


/// From a List, determine the {vctrs} class of the pointer list
pub fn determine_geoms_class(x: &List) -> [String; 3] {

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
