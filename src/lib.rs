use extendr_api::prelude::*;

pub mod fromsf;
pub mod tosf;
pub mod constructors;
pub mod vctrs;

use geo_types::{
    Geometry, Line, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon, Rect,
};

// #[extendr]
// fn round_trip(x: List) -> Robj {
//     let y = sfc_to_geoms(x);

//     y.iter()
//         .filter_map(|xi| {
//             match xi {
//                 Some(xi) => xi.geom.
//             }
//         })
//     geoms_to_sfc(y)

// }

/// The `Geom` struct is the backbone of sfconversions. It provides
/// an itermediary between extendr and geo / geo_types as required
/// by the orphan rule.
#[derive(Debug, Clone)]
pub struct Geom {
    /// a single field containing a geo_types [Geometry](https://docs.rs/geo-types/latest/geo_types/geometry/enum.Geometry.html) enum
    pub geom: Geometry,
}

#[extendr]
impl Geom {
    fn print(&self) -> String {
        let fstr = format!("{:?}", self.geom);
        fstr.splitn(2, '(')
            .nth(1)
            .unwrap_or("")
            .to_string()
    }
}

// FROM geo-types to Geom
/// Convert a Geometry enum to a Geom struct
impl From<Geometry> for Geom {
    fn from(geo: Geometry) -> Self {
        Geom { geom: geo }
    }
}

impl From<Point> for Geom {
    fn from(pnt: Point) -> Self {
        let x: Geometry = pnt.into();
        Geom { geom: x }
    }
}

impl From<MultiPoint> for Geom {
    fn from(pnt: MultiPoint) -> Self {
        let x: Geometry = pnt.into();
        Geom { geom: x }
    }
}

impl From<Polygon> for Geom {
    fn from(poly: Polygon) -> Self {
        let x: Geometry = poly.into();
        Geom { geom: x }
    }
}

impl From<MultiPolygon> for Geom {
    fn from(poly: MultiPolygon) -> Self {
        let x: Geometry = poly.into();
        Geom { geom: x }
    }
}

impl From<LineString> for Geom {
    fn from(lns: LineString) -> Self {
        let x: Geometry = lns.into();
        Geom { geom: x }
    }
}

impl From<MultiLineString> for Geom {
    fn from(lns: MultiLineString) -> Self {
        let x: Geometry = lns.into();
        Geom { geom: x }
    }
}

impl From<Rect> for Geom {
    fn from(r: Rect) -> Self {
        let x: Geometry = r.into();
        Geom { geom: x }
    }
}

impl From<Line> for Geom {
    fn from(l: Line) -> Self {
        let x: Geometry = l.into();
        Geom { geom: x }
    }
}

// TO geo-types from Geom
impl From<Geom> for Polygon {
    fn from(geom: Geom) -> Self {
        let x = geom.geom;
        let x: Polygon = x.try_into().unwrap();
        x
    }
}

impl From<Geom> for LineString {
    fn from(geom: Geom) -> Self {
        let x = geom.geom;
        let x: LineString = x.try_into().unwrap();
        x
    }
}

impl From<Geom> for Point {
    fn from(geom: Geom) -> Self {
        let x = geom.geom;
        let x: Point = x.try_into().unwrap();
        x
    }
}

/// extendr does not permit taking ownership of the pointers it creates
/// for structs. This impl clones the struct to create an owned struct.
impl From<Robj> for Geom {
    fn from(robj: Robj) -> Self {
        <&Geom>::from_robj(&robj)
            .unwrap()
            .clone()
    }
}

// This is infallible. It requires that there are no missing geometries.
// In the case that there are missing geometries, they must be handled
// independently. This implementation clones the pointers
// Missing geometries are recorded as a NULL (extendr_api::NULL)
pub fn geoms_from_list(x: List) -> Vec<Option<Geom>> {
    x
        .into_iter()
        .map(|(_, robj)| {

            if robj.is_null() {
                None
            } else {
                Some(Geom::from(robj))
            }
        })
        .collect::<Vec<Option<Geom>>>()
}

pub fn geoms_ref_from_list(x: List) -> Vec<Option<&'static Geom>> {
    x
    .into_iter()
    .map(|(_, robj)| {

        if robj.is_null() {
            None
        } else {
            Some(<&Geom>::from_robj(&robj).unwrap())
        }
    })
    .collect::<Vec<Option<&Geom>>>()

}
