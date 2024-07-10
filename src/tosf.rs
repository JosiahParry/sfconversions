//! Conversion from geo-types to {sf} type objects
//!
//! Provides simple conversion from `Geom` wrapper struct to an sfg class object.
//! Additionally provides the ability to convert from `Vec<Option<Geom>>` to a list
//! of sfg objects that can be easily converted into an sfc object by running `sf::st_sfc()`.
//!
use crate::Geom;
/// Takes a single Geom struct and creates the corresponding `sfg` object
use extendr_api::prelude::*;
use extendr_api::Robj;
use geo_types::*;

/// A general purpose function that matches on the `Geometry` enum to convert into the
/// appropriate sfg object type. If the Geom cannot be matched (e.g. Line or Triangle),
/// it will return a `NULL` Robj.
pub fn to_sfg(x: Geom) -> Robj {
    let x = x.geom;
    match x {
        Geometry::Point(x) => from_point(x),
        Geometry::MultiPoint(x) => from_multipoint(x),
        Geometry::LineString(x) => from_linestring(x),
        Geometry::MultiLineString(x) => from_multilinestring(x),
        Geometry::Polygon(x) => from_polygon(x),
        Geometry::MultiPolygon(x) => from_multipolygon(x),
        _ => Robj::from(NULL),
    }
}

/// Takes a `Vec<Option<Geom>>` such as the result of `sfc_to_geometry()`
/// and creates a list of sfg objects. This can be easily turned into an `sfc`
/// by passing the results to `sf::st_sfc()`. This cannot be converted into an
/// `sfc` object without first calculating the bounding box which would require
/// importing geo.
pub fn geoms_to_sfc(x: Vec<Option<Geom>>) -> List {
    //let cls = determine_sfc_class(&x).to_ascii_uppercase();
    // let cls_array = [format!("sfc_{cls}"), String::from("sfc")];

    x.into_iter()
        .map(|geom| match geom {
            Some(geo) => to_sfg(geo),
            None => Robj::from(NULL),
        })
        .collect::<List>()
}

/// Utility function to identify the class of an sfc object .
pub fn determine_sfc_class(x: &Vec<Option<Geom>>) -> String {
    let mut result = String::new();
    for geom in x {
        match geom {
            Some(geom) => {
                let fstr = format!("{:?}", geom.geom);
                let cls = fstr.splitn(2, '(').next().unwrap().to_string();
                if result.is_empty() {
                    result = cls;
                } else if result != cls {
                    result = "GEOMETRYCOLLECTION".to_string();
                    break;
                }
            }
            None => continue,
        }
    }
    result
}

fn from_coord(x: Coord) -> [f64; 2] {
    [x.x, x.y]
}

/// Convert a `Point` to a sfg
pub fn from_point(x: Point) -> Robj {
    let x = from_coord(x.0);
    Robj::try_from(x)
        .unwrap()
        .set_class(["XY", "POINT", "sfg"])
        .unwrap()
        .clone()
}

/// Convert a `MultiPoint` to an sfg
pub fn from_multipoint(x: MultiPoint) -> Robj {
    let x = x
        .into_iter()
        .map(|p| from_coord(p.into()))
        .collect::<Vec<[f64; 2]>>();

    let res = RMatrix::new_matrix(x.len(), 2, |r, c| x[r][c]);
    Robj::from(res)
        .set_class(["XY", "MULTIPOINT", "sfg"])
        .unwrap()
        .clone()
}

/// Convert a `LineString` to an sfg
pub fn from_linestring(x: LineString) -> Robj {
    let x = x.into_iter().map(from_coord).collect::<Vec<[f64; 2]>>();

    let res = RMatrix::new_matrix(x.len(), 2, |r, c| x[r][c]);
    Robj::from(res)
        .set_class(["XY", "LINESTRING", "sfg"])
        .unwrap()
        .clone()
}

/// Convert a `MultiLineString` to an sfg
pub fn from_multilinestring(x: MultiLineString) -> Robj {
    x.0.into_iter()
        .map(from_linestring)
        .collect::<List>()
        .into_robj()
        .set_class(["XY", "MULTILINESTRING", "sfg"])
        .unwrap()
        .clone()
}

/// Convert a `Polygon` to an sfg
pub fn from_polygon(x: Polygon) -> Robj {
    let exterior = x.exterior().to_owned();
    let interriors = x.interiors().to_owned();

    // combine the exterior ring and interrior rings into 1 vector first
    // then iterate through them.
    // no method to go from Polygon to multilinestring
    let mut res: Vec<LineString> = Vec::with_capacity(interriors.len() + 1);
    res.push(exterior);
    res.extend(interriors.into_iter());

    let res = res.into_iter().map(from_linestring).collect::<Vec<Robj>>();

    Robj::from(List::from_values(res))
        .set_class(["XY", "POLYGON", "sfg"])
        .unwrap()
        .clone()
}

/// Convert a `MultiPolygon` to an sfg
pub fn from_multipolygon(x: MultiPolygon) -> Robj {
    let res = x.into_iter().map(from_polygon).collect::<List>();

    Robj::from(res)
        .set_class(["XY", "MULTIPOLYGON", "sfg"])
        .unwrap()
        .clone()
}
