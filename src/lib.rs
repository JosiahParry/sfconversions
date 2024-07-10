use extendr_api::prelude::*;

pub mod constructors;
pub mod esri;
pub mod fromsf;
pub mod sfg;
pub mod tosf;
pub mod vctrs;

use geo_types::{
    Geometry, Line, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon, Rect,
};

use geo::BoundingRect;
use rstar::primitives::CachedEnvelope;

extendr_module! {
    mod sfconversions;
    impl Geom;
}

/// Implement RTreeObject for Geom
impl rstar::RTreeObject for Geom {
    type Envelope = rstar::AABB<[f64; 2]>;
    fn envelope(&self) -> Self::Envelope {
        let bbox = self.geom.bounding_rect().unwrap();
        let ll = bbox.min(); //lower left x coord
        let ur = bbox.max(); // upper right y
        rstar::AABB::from_corners(ll.into(), ur.into())
    }
}

// impl rstar::PointDistance for Geom {
//     fn distance_2(
//             &self,
//             point: &<Self::Envelope as rstar::Envelope>::Point,
//         ) -> <<Self::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar {
//             let pnt = geo_types::coord!{x: point[0], y: point[1]};
//             let pnt = geo_types::point!(pnt);
//             let d = &self.geom.euclidean_distance(&pnt);
//             d.powi(2)
//     }
// }

/// The `Geom` struct is the backbone of sfconversions. It provides
/// an itermediary between extendr and geo / geo_types as required
/// by the orphan rule.
#[derive(Debug, Clone)]
pub struct Geom {
    /// a single field containing a geo_types [Geometry](https://docs.rs/geo-types/latest/geo_types/geometry/enum.Geometry.html) enum
    pub geom: Geometry,
}

/// Trait to convert objects to Geom structs
pub trait IntoGeom {
    fn into_geom(self) -> Geom;
    fn cached_envelope(self) -> CachedEnvelope<Geom>;
}

/// Implement IntoGeom for any structs that have `From<T> for Geom`
impl<T> IntoGeom for T
where
    Geom: From<T>,
{
    fn into_geom(self) -> Geom {
        Geom::from(self)
    }

    fn cached_envelope(self) -> CachedEnvelope<Geom> {
        CachedEnvelope::new(self.into())
    }
}

#[extendr]
impl Geom {
    pub fn print(&self) -> String {
        let fstr = format!("{:?}", self.geom);
        fstr.splitn(2, '(').nth(1).unwrap_or("").to_string()
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

// impl From<Geom> for MultiPolygon {
//     fn from(geom: Geom) -> Self {
//         let x = geom.geom;
//         let x: MultiPolygon = x.try_into().unwrap();
//         x
//     }
// }

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
        <&Geom>::try_from(&robj).unwrap().clone()
    }
}

// This is infallible. It requires that there are no missing geometries.
// In the case that there are missing geometries, they must be handled
// independently. This implementation clones the pointers
// Missing geometries are recorded as a NULL (extendr_api::NULL)
pub fn geoms_from_list(x: List) -> Vec<Option<Geom>> {
    x.into_iter()
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
    x.into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                None
            } else {
                Some(<&Geom>::try_from(&robj).unwrap())
            }
        })
        .collect::<Vec<Option<&Geom>>>()
}

pub fn geometry_from_list(x: List) -> Vec<Option<Geometry>> {
    x.into_iter()
        .map(|(_, xi)| match <&Geom>::try_from(&xi) {
            Ok(g) => Some(g.geom.clone()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Geometry>>>()
}
