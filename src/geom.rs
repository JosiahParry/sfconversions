use extendr_api::prelude::*;
use extendr_api::{ExternalPtr, List, Robj};
// create an enum of geo-types
use geo_types::{
    Geometry, Line, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon, Rect,
};

use std::fmt;

/// The `Geom` struct is the backbone of sfconversions. It provides
/// an itermediary between extendr and geo / geo_types as required
/// by the orphan rule.
#[derive(Debug, Clone)]
pub struct Geom {
    /// a single field containing a geo_types [Geometry](https://docs.rs/geo-types/latest/geo_types/geometry/enum.Geometry.html) enum
    pub geom: Geometry,
}

impl fmt::Display for Geom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.geom)
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

impl From<ExternalPtr<Geom>> for Geom {
    fn from(pntr: ExternalPtr<Geom>) -> Self {
        let geo = pntr.geom.clone();
        Geom { geom: geo }
    }
}
/// When converting from an Robj, we assume it is a _pointer_ to a 
/// Geom struct
impl From<Robj> for Geom {
    fn from(robj: Robj) -> Self {
        let robj: ExternalPtr<Geom> = robj.try_into().unwrap();
        let robj: Geom = robj.into();
        robj
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

/// utility function to extract a Vec of Geoms from a list
/// Assume you have an `sfc` object, this function will take the `sfc`
/// object (a `List`) and return a `Vec<Geom>`.
pub fn from_list(x: List) -> Vec<Geom> {
    x.into_iter()
        .map(|(_, robj)| Geom::try_from(robj).unwrap())
        .collect::<Vec<_>>()
}

// helpers to cast to the proper external pointer
/// Helper function to create pointers to `Geom` structs with classes
/// used by [`rsgeo`](https://rsgeo.josiahparry.com/)
pub fn to_pntr(x: Geom) -> Robj {
    let cls = match x.geom {
        Geometry::Point(ref _geom) => "point",
        Geometry::MultiPoint(ref _geom) => "multipoint",
        Geometry::LineString(ref _geom) => "linestring",
        Geometry::MultiLineString(ref _geom) => "multilinestring",
        Geometry::Polygon(ref _geom) => "polygon",
        Geometry::MultiPolygon(ref _geom) => "multipolygon",
        _ => "",
    };

    ExternalPtr::new(x)
        .as_robj()
        .set_attrib("class", cls)
        .unwrap()
}
