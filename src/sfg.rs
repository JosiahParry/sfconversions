//! Typed wrappers around sf's `sfg` R objects
//!
//! Each type wraps the R object without copying it, and implements the
//! [geo-traits](https://docs.rs/geo-traits) so it can be used directly with
//! any georust algorithm that accepts them. All of sf's dimensions (`XY`, `XYZ`,
//! `XYM`, and `XYZM`) are supported.
//!
//! `TryFrom<Robj>` checks that an R object is a valid sfg, and `From<Sfg*> for Robj`
//! sets the `c(<dim>, <geometry type>, "sfg")` classes on the R object it returns.
use extendr_api::{error::Error, prelude::*};
use geo_traits::{
    CoordTrait, Dimensions, GeometryCollectionTrait, GeometryTrait, GeometryType, LineStringTrait,
    MultiLineStringTrait, MultiPointTrait, MultiPolygonTrait, PointTrait, PolygonTrait,
    UnimplementedLine, UnimplementedRect, UnimplementedTriangle,
};

/// An sf `POINT`.
#[derive(Debug)]
pub struct SfgPoint(Doubles);

/// An sf `MULTIPOINT`.
#[derive(Debug)]
pub struct SfgMultiPoint(RMatrix<Rfloat>);

/// An sf `LINESTRING`.
#[derive(Debug)]
pub struct SfgLineString(RMatrix<Rfloat>);

/// An sf `MULTILINESTRING`.
#[derive(Debug)]
pub struct SfgMultiLineString(List);

/// An sf `POLYGON`.
#[derive(Debug, Clone)]
pub struct SfgPolygon(List);

/// An sf `MULTIPOLYGON`.
#[derive(Debug)]
pub struct SfgMultiPolygon(List);

/// An sf `GEOMETRYCOLLECTION`.
#[derive(Debug)]
pub struct SfgGeometryCollection(List);

/// Any supported sfg object.
#[derive(Debug)]
pub enum Sfg {
    Point(SfgPoint),
    MultiPoint(SfgMultiPoint),
    LineString(SfgLineString),
    MultiLineString(SfgMultiLineString),
    Polygon(SfgPolygon),
    MultiPolygon(SfgMultiPolygon),
    GeometryCollection(SfgGeometryCollection),
}

/// A coordinate that borrows from an sfg's R memory.
///
/// sf stores coordinates in column-major matrices, so the nth ordinate of a
/// row is found at `n * nrow + row`. A `POINT` is treated as a matrix with one row.
#[derive(Debug, Clone, Copy)]
pub struct SfgCoord<'a> {
    data: &'a [Rfloat],
    nrow: usize,
    row: usize,
    dim: Dimensions,
}

/// A single point inside an [`SfgMultiPoint`].
#[derive(Debug, Clone, Copy)]
pub struct SfgPointRef<'a> {
    coord: SfgCoord<'a>,
}

/// A polygon ring or a multilinestring line: a coordinate matrix with no class of
/// its own. The parent geometry gives it its dimension.
#[derive(Debug)]
pub struct SfgRing {
    data: RMatrix<Rfloat>,
    dim: Dimensions,
}

impl SfgRing {
    /// The dimensions of the geometry this ring belongs to.
    pub fn dim(&self) -> Dimensions {
        self.dim
    }
}

// Dimensions -----------------------------------------------------------------

impl SfgPoint {
    /// The sf dimensions of this geometry, such as `Dimensions::Xyz`.
    pub fn dim(&self) -> Dimensions {
        parse_dim(self.0.class(), "POINT").unwrap_or(Dimensions::Xy)
    }
}

impl SfgMultiPoint {
    /// The sf dimensions of this geometry, such as `Dimensions::Xyz`.
    pub fn dim(&self) -> Dimensions {
        parse_dim(self.0.class(), "MULTIPOINT").unwrap_or(Dimensions::Xy)
    }
}

impl SfgLineString {
    /// The sf dimensions of this geometry, such as `Dimensions::Xyz`.
    pub fn dim(&self) -> Dimensions {
        parse_dim(self.0.class(), "LINESTRING").unwrap_or(Dimensions::Xy)
    }
}

impl SfgMultiLineString {
    /// The sf dimensions of this geometry, such as `Dimensions::Xyz`.
    pub fn dim(&self) -> Dimensions {
        parse_dim(self.0.class(), "MULTILINESTRING").unwrap_or(Dimensions::Xy)
    }
}

impl SfgPolygon {
    /// The sf dimensions of this geometry, such as `Dimensions::Xyz`.
    pub fn dim(&self) -> Dimensions {
        parse_dim(self.0.class(), "POLYGON").unwrap_or(Dimensions::Xy)
    }
}

impl SfgMultiPolygon {
    /// The sf dimensions of this geometry, such as `Dimensions::Xyz`.
    pub fn dim(&self) -> Dimensions {
        parse_dim(self.0.class(), "MULTIPOLYGON").unwrap_or(Dimensions::Xy)
    }
}

impl SfgGeometryCollection {
    /// The sf dimensions of this geometry, such as `Dimensions::Xyz`.
    pub fn dim(&self) -> Dimensions {
        parse_dim(self.0.class(), "GEOMETRYCOLLECTION").unwrap_or(Dimensions::Xy)
    }
}

/// Reads the dimensions from an sfg's class, which must be
/// `c(<dim>, <geometry_type>, "sfg")` where `<dim>` is XY, XYZ, XYM, or XYZM.
fn parse_dim(class: Option<StrIter>, geometry_type: &str) -> Result<Dimensions, Error> {
    let classes = class.map(|c| c.collect::<Vec<&str>>()).unwrap_or_default();

    match classes.as_slice() {
        ["XY", cls, "sfg"] if *cls == geometry_type => Ok(Dimensions::Xy),
        ["XYZ", cls, "sfg"] if *cls == geometry_type => Ok(Dimensions::Xyz),
        ["XYM", cls, "sfg"] if *cls == geometry_type => Ok(Dimensions::Xym),
        ["XYZM", cls, "sfg"] if *cls == geometry_type => Ok(Dimensions::Xyzm),
        _ => Err(Error::Other(format!("Expected an sfg {geometry_type}."))),
    }
}

// TryFrom<Robj> ------------------------------------------------------------

impl TryFrom<Robj> for SfgPoint {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        let dim = parse_dim(value.class(), "POINT")?;
        let point = SfgPoint(Doubles::try_from(value)?);

        if point.0.len() != dim.size() {
            return Err(Error::Other(format!(
                "Expected {} values for {:?}. Found {}.",
                dim.size(),
                dim,
                point.0.len()
            )));
        }

        Ok(point)
    }
}

impl TryFrom<Robj> for SfgMultiPoint {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        let dim = parse_dim(value.class(), "MULTIPOINT")?;
        let multipoint = SfgMultiPoint(RMatrix::try_from(value)?);

        if multipoint.0.ncols() != dim.size() {
            return Err(Error::Other(format!(
                "Expected {} columns for {:?}. Found {}.",
                dim.size(),
                dim,
                multipoint.0.ncols()
            )));
        }

        Ok(multipoint)
    }
}

impl TryFrom<Robj> for SfgLineString {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        let dim = parse_dim(value.class(), "LINESTRING")?;
        let linestring = SfgLineString(RMatrix::try_from(value)?);

        if linestring.0.ncols() != dim.size() {
            return Err(Error::Other(format!(
                "Expected {} columns for {:?}. Found {}.",
                dim.size(),
                dim,
                linestring.0.ncols()
            )));
        }

        Ok(linestring)
    }
}

impl TryFrom<Robj> for SfgMultiLineString {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        if !value.inherits("MULTILINESTRING") {
            return Err(Error::Other("Expected an sfg MULTILINESTRING.".to_string()));
        }
        Ok(SfgMultiLineString(List::try_from(value)?))
    }
}

impl TryFrom<Robj> for SfgPolygon {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        if !value.inherits("POLYGON") {
            return Err(Error::Other("Expected an sfg POLYGON.".to_string()));
        }
        Ok(SfgPolygon(List::try_from(value)?))
    }
}

impl TryFrom<Robj> for SfgMultiPolygon {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        if !value.inherits("MULTIPOLYGON") {
            return Err(Error::Other("Expected an sfg MULTIPOLYGON.".to_string()));
        }
        Ok(SfgMultiPolygon(List::try_from(value)?))
    }
}

impl TryFrom<Robj> for SfgGeometryCollection {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        if !value.inherits("GEOMETRYCOLLECTION") {
            return Err(Error::Other(
                "Expected an sfg GEOMETRYCOLLECTION.".to_string(),
            ));
        }
        let data = List::try_from(value)?;

        for geom in data.values() {
            Sfg::try_from(geom)?;
        }

        Ok(SfgGeometryCollection(data))
    }
}

impl TryFrom<Robj> for Sfg {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        let geometry_type = value.class().and_then(|mut cls| cls.nth(1)).unwrap_or("");

        match geometry_type {
            "POINT" => SfgPoint::try_from(value).map(Sfg::Point),
            "MULTIPOINT" => SfgMultiPoint::try_from(value).map(Sfg::MultiPoint),
            "LINESTRING" => SfgLineString::try_from(value).map(Sfg::LineString),
            "MULTILINESTRING" => SfgMultiLineString::try_from(value).map(Sfg::MultiLineString),
            "POLYGON" => SfgPolygon::try_from(value).map(Sfg::Polygon),
            "MULTIPOLYGON" => SfgMultiPolygon::try_from(value).map(Sfg::MultiPolygon),
            "GEOMETRYCOLLECTION" => {
                SfgGeometryCollection::try_from(value).map(Sfg::GeometryCollection)
            }
            "" => Err(Error::Other(
                "Expected an sfg object, such as one made by `sf::st_point()`.".to_string(),
            )),
            other => Err(Error::Other(format!(
                "Converting a {other} isn't supported yet."
            ))),
        }
    }
}

// From<Sfg*> for Robj -------------------------------------------------------

impl From<SfgPoint> for Robj {
    fn from(value: SfgPoint) -> Self {
        value.0.into()
    }
}

impl From<SfgMultiPoint> for Robj {
    fn from(value: SfgMultiPoint) -> Self {
        value.0.into()
    }
}

impl From<SfgLineString> for Robj {
    fn from(value: SfgLineString) -> Self {
        value.0.into()
    }
}

impl From<SfgMultiLineString> for Robj {
    fn from(value: SfgMultiLineString) -> Self {
        value.0.into()
    }
}

impl From<SfgPolygon> for Robj {
    fn from(value: SfgPolygon) -> Self {
        value.0.into()
    }
}

impl From<SfgMultiPolygon> for Robj {
    fn from(value: SfgMultiPolygon) -> Self {
        value.0.into()
    }
}

impl From<SfgGeometryCollection> for Robj {
    fn from(value: SfgGeometryCollection) -> Self {
        value.0.into()
    }
}

impl From<Sfg> for Robj {
    fn from(value: Sfg) -> Self {
        match value {
            Sfg::Point(x) => x.into(),
            Sfg::MultiPoint(x) => x.into(),
            Sfg::LineString(x) => x.into(),
            Sfg::MultiLineString(x) => x.into(),
            Sfg::Polygon(x) => x.into(),
            Sfg::MultiPolygon(x) => x.into(),
            Sfg::GeometryCollection(x) => x.into(),
        }
    }
}

// From<geo-types> -----------------------------------------------------------

impl From<geo_types::Point<f64>> for SfgPoint {
    fn from(value: geo_types::Point<f64>) -> Self {
        let mut data = Doubles::from_values([value.x(), value.y()]);
        let _ = data.set_class(["XY", "POINT", "sfg"]);
        SfgPoint(data)
    }
}

impl From<geo_types::MultiPoint<f64>> for SfgMultiPoint {
    fn from(value: geo_types::MultiPoint<f64>) -> Self {
        let mut data = RMatrix::new_matrix(value.0.len(), 2, |r, c| {
            let pnt = value.0[r];
            Rfloat::from(if c == 0 { pnt.x() } else { pnt.y() })
        });
        let _ = data.set_class(["XY", "MULTIPOINT", "sfg"]);
        SfgMultiPoint(data)
    }
}

impl From<geo_types::LineString<f64>> for SfgLineString {
    fn from(value: geo_types::LineString<f64>) -> Self {
        let mut data = RMatrix::new_matrix(value.0.len(), 2, |r, c| {
            let crd = value.0[r];
            Rfloat::from(if c == 0 { crd.x } else { crd.y })
        });
        let _ = data.set_class(["XY", "LINESTRING", "sfg"]);
        SfgLineString(data)
    }
}

impl From<geo_types::MultiLineString<f64>> for SfgMultiLineString {
    fn from(value: geo_types::MultiLineString<f64>) -> Self {
        let mut data = value
            .0
            .into_iter()
            .map(|line| {
                Robj::from(RMatrix::new_matrix(line.0.len(), 2, |r, c| {
                    let crd = line.0[r];
                    Rfloat::from(if c == 0 { crd.x } else { crd.y })
                }))
            })
            .collect::<List>();
        let _ = data.set_class(["XY", "MULTILINESTRING", "sfg"]);
        SfgMultiLineString(data)
    }
}

impl From<geo_types::Polygon<f64>> for SfgPolygon {
    fn from(value: geo_types::Polygon<f64>) -> Self {
        let (exterior, interiors) = value.into_inner();
        let mut data = std::iter::once(exterior)
            .chain(interiors)
            .map(|ring| {
                Robj::from(RMatrix::new_matrix(ring.0.len(), 2, |r, c| {
                    let crd = ring.0[r];
                    Rfloat::from(if c == 0 { crd.x } else { crd.y })
                }))
            })
            .collect::<List>();
        let _ = data.set_class(["XY", "POLYGON", "sfg"]);
        SfgPolygon(data)
    }
}

impl From<geo_types::MultiPolygon<f64>> for SfgMultiPolygon {
    fn from(value: geo_types::MultiPolygon<f64>) -> Self {
        let mut data = value
            .0
            .into_iter()
            .map(|polygon| {
                let (exterior, interiors) = polygon.into_inner();
                std::iter::once(exterior)
                    .chain(interiors)
                    .map(|ring| {
                        Robj::from(RMatrix::new_matrix(ring.0.len(), 2, |r, c| {
                            let crd = ring.0[r];
                            Rfloat::from(if c == 0 { crd.x } else { crd.y })
                        }))
                    })
                    .collect::<List>()
            })
            .collect::<List>();
        let _ = data.set_class(["XY", "MULTIPOLYGON", "sfg"]);
        SfgMultiPolygon(data)
    }
}

// geo-traits ---------------------------------------------------------------

impl CoordTrait for SfgCoord<'_> {
    type T = f64;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn x(&self) -> f64 {
        self.nth_or_panic(0)
    }

    fn y(&self) -> f64 {
        self.nth_or_panic(1)
    }

    fn nth_or_panic(&self, n: usize) -> f64 {
        self.data[n * self.nrow + self.row].0
    }
}

/// Implements `GeometryTrait` for a type whose `as_type()` is the given variant.
/// Every sfg type shares the same associated types.
macro_rules! impl_geometry_trait {
    ($variant:ident, $ty:ty) => {
        impl GeometryTrait for $ty {
            impl_geometry_trait!(@body $variant);
        }
    };
    (@body $variant:ident) => {
            type T = f64;
            type PointType<'b> = SfgPoint where Self: 'b;
            type LineStringType<'b> = SfgLineString where Self: 'b;
            type PolygonType<'b> = SfgPolygon where Self: 'b;
            type MultiPointType<'b> = SfgMultiPoint where Self: 'b;
            type MultiLineStringType<'b> = SfgMultiLineString where Self: 'b;
            type MultiPolygonType<'b> = SfgMultiPolygon where Self: 'b;
            type GeometryCollectionType<'b> = SfgGeometryCollection where Self: 'b;
            type RectType<'b> = UnimplementedRect<f64> where Self: 'b;
            type TriangleType<'b> = UnimplementedTriangle<f64> where Self: 'b;
            type LineType<'b> = UnimplementedLine<f64> where Self: 'b;

            fn dim(&self) -> Dimensions {
                self.dim()
            }

            fn as_type(&self) -> SfgGeometryType<'_> {
                GeometryType::$variant(self)
            }
    };
}

type SfgGeometryType<'a> = GeometryType<
    'a,
    SfgPoint,
    SfgLineString,
    SfgPolygon,
    SfgMultiPoint,
    SfgMultiLineString,
    SfgMultiPolygon,
    SfgGeometryCollection,
    UnimplementedRect<f64>,
    UnimplementedTriangle<f64>,
    UnimplementedLine<f64>,
>;

impl_geometry_trait!(GeometryCollection, SfgGeometryCollection);
impl_geometry_trait!(Point, SfgPoint);
impl_geometry_trait!(MultiPoint, SfgMultiPoint);
impl_geometry_trait!(LineString, SfgLineString);
impl_geometry_trait!(MultiLineString, SfgMultiLineString);
impl_geometry_trait!(Polygon, SfgPolygon);
impl_geometry_trait!(MultiPolygon, SfgMultiPolygon);

impl PointTrait for SfgPoint {
    type CoordType<'a> = SfgCoord<'a>;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        let data: &[Rfloat] = &self.0;

        // sf represents an empty point as all NA values
        if data.iter().all(|v| v.0.is_nan()) {
            return None;
        }

        Some(SfgCoord {
            data,
            nrow: 1,
            row: 0,
            dim: self.dim(),
        })
    }
}

impl<'a> GeometryTrait for SfgPointRef<'a> {
    type T = f64;
    type PointType<'b>
        = SfgPointRef<'a>
    where
        Self: 'b;
    type LineStringType<'b>
        = SfgLineString
    where
        Self: 'b;
    type PolygonType<'b>
        = SfgPolygon
    where
        Self: 'b;
    type MultiPointType<'b>
        = SfgMultiPoint
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = SfgMultiLineString
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = SfgMultiPolygon
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = SfgGeometryCollection
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<f64>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<f64>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<f64>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        self.coord.dim
    }

    fn as_type(
        &self,
    ) -> GeometryType<
        '_,
        SfgPointRef<'a>,
        SfgLineString,
        SfgPolygon,
        SfgMultiPoint,
        SfgMultiLineString,
        SfgMultiPolygon,
        SfgGeometryCollection,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        GeometryType::Point(self)
    }
}

impl<'a> PointTrait for SfgPointRef<'a> {
    type CoordType<'b>
        = SfgCoord<'a>
    where
        Self: 'b;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        Some(self.coord)
    }
}

impl MultiPointTrait for SfgMultiPoint {
    type InnerPointType<'a> = SfgPointRef<'a>;

    fn num_points(&self) -> usize {
        self.0.nrows()
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::InnerPointType<'_> {
        SfgPointRef {
            coord: SfgCoord {
                data: self.0.data(),
                nrow: self.0.nrows(),
                row: i,
                dim: self.dim(),
            },
        }
    }
}

impl LineStringTrait for SfgLineString {
    type CoordType<'a> = SfgCoord<'a>;

    fn num_coords(&self) -> usize {
        self.0.nrows()
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        SfgCoord {
            data: self.0.data(),
            nrow: self.0.nrows(),
            row: i,
            dim: self.dim(),
        }
    }
}

impl GeometryTrait for SfgRing {
    type T = f64;
    type PointType<'b>
        = SfgPoint
    where
        Self: 'b;
    type LineStringType<'b>
        = SfgRing
    where
        Self: 'b;
    type PolygonType<'b>
        = SfgPolygon
    where
        Self: 'b;
    type MultiPointType<'b>
        = SfgMultiPoint
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = SfgMultiLineString
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = SfgMultiPolygon
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = SfgGeometryCollection
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<f64>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<f64>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<f64>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn as_type(
        &self,
    ) -> GeometryType<
        '_,
        SfgPoint,
        SfgRing,
        SfgPolygon,
        SfgMultiPoint,
        SfgMultiLineString,
        SfgMultiPolygon,
        SfgGeometryCollection,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        GeometryType::LineString(self)
    }
}

impl LineStringTrait for SfgRing {
    type CoordType<'a> = SfgCoord<'a>;

    fn num_coords(&self) -> usize {
        self.data.nrows()
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        SfgCoord {
            data: self.data.data(),
            nrow: self.data.nrows(),
            row: i,
            dim: self.dim,
        }
    }
}

impl MultiLineStringTrait for SfgMultiLineString {
    type InnerLineStringType<'a> = SfgRing;

    fn num_line_strings(&self) -> usize {
        self.0.len()
    }

    unsafe fn line_string_unchecked(&self, i: usize) -> Self::InnerLineStringType<'_> {
        SfgRing {
            data: RMatrix::try_from(&self.0[i]).unwrap(),
            dim: self.dim(),
        }
    }
}

impl PolygonTrait for SfgPolygon {
    type RingType<'a> = SfgRing;

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        self.0
            .first()
            .and_then(|ring| RMatrix::try_from(ring).ok())
            .map(|data| SfgRing {
                data,
                dim: self.dim(),
            })
    }

    fn num_interiors(&self) -> usize {
        self.0.len().saturating_sub(1)
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        SfgRing {
            data: RMatrix::try_from(&self.0[i + 1]).unwrap(),
            dim: self.dim(),
        }
    }
}

impl MultiPolygonTrait for SfgMultiPolygon {
    type InnerPolygonType<'a> = SfgPolygon;

    fn num_polygons(&self) -> usize {
        self.0.len()
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::InnerPolygonType<'_> {
        SfgPolygon(List::try_from(&self.0[i]).unwrap())
    }
}

impl GeometryCollectionTrait for SfgGeometryCollection {
    type GeometryType<'a> = Sfg;

    fn num_geometries(&self) -> usize {
        self.0.len()
    }

    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        Sfg::try_from(self.0[i].clone()).unwrap()
    }
}

/// Implements `GeometryTrait` for `Sfg` and `&Sfg` by matching on the variant.
macro_rules! impl_sfg_enum_geometry_trait {
    ($($ty:tt)+) => {
        impl<'a> GeometryTrait for $($ty)+ {
            type T = f64;
            type PointType<'b> = SfgPoint where Self: 'b;
            type LineStringType<'b> = SfgLineString where Self: 'b;
            type PolygonType<'b> = SfgPolygon where Self: 'b;
            type MultiPointType<'b> = SfgMultiPoint where Self: 'b;
            type MultiLineStringType<'b> = SfgMultiLineString where Self: 'b;
            type MultiPolygonType<'b> = SfgMultiPolygon where Self: 'b;
            type GeometryCollectionType<'b> = SfgGeometryCollection where Self: 'b;
            type RectType<'b> = UnimplementedRect<f64> where Self: 'b;
            type TriangleType<'b> = UnimplementedTriangle<f64> where Self: 'b;
            type LineType<'b> = UnimplementedLine<f64> where Self: 'b;

            fn dim(&self) -> Dimensions {
                match self {
                    Sfg::Point(x) => x.dim(),
                    Sfg::MultiPoint(x) => x.dim(),
                    Sfg::LineString(x) => x.dim(),
                    Sfg::MultiLineString(x) => x.dim(),
                    Sfg::Polygon(x) => x.dim(),
                    Sfg::MultiPolygon(x) => x.dim(),
                    Sfg::GeometryCollection(x) => x.dim(),
                }
            }

            fn as_type(&self) -> SfgGeometryType<'_> {
                match self {
                    Sfg::Point(x) => GeometryType::Point(x),
                    Sfg::MultiPoint(x) => GeometryType::MultiPoint(x),
                    Sfg::LineString(x) => GeometryType::LineString(x),
                    Sfg::MultiLineString(x) => GeometryType::MultiLineString(x),
                    Sfg::Polygon(x) => GeometryType::Polygon(x),
                    Sfg::MultiPolygon(x) => GeometryType::MultiPolygon(x),
                    Sfg::GeometryCollection(x) => GeometryType::GeometryCollection(x),
                }
            }
        }
    };
}

impl_sfg_enum_geometry_trait!(Sfg);
impl_sfg_enum_geometry_trait!(&'a Sfg);

#[cfg(test)]
mod tests {
    use super::*;
    use geo_traits::to_geo::{
        ToGeoGeometry, ToGeoLineString, ToGeoMultiLineString, ToGeoMultiPoint, ToGeoMultiPolygon,
        ToGeoPoint, ToGeoPolygon,
    };
    use geo_types::{
        Geometry, LineString, MultiLineString, MultiPolygon, line_string, point, polygon,
    };

    const SQUARE_WITH_HOLE: &str = "
        structure(
            list(
                matrix(c(0, 10, 10, 0, 0, 0, 0, 10, 10, 0), ncol = 2),
                matrix(c(1, 2, 2, 1, 1, 1, 1, 2, 2, 1), ncol = 2)
            ),
            class = c('XY', 'POLYGON', 'sfg')
        )
    ";

    fn classes(x: Robj) -> Vec<String> {
        x.class().unwrap().map(String::from).collect()
    }

    #[test]
    fn parse_dim_reads_each_dimension() {
        test! {
            let cases = [
                ("XY", Dimensions::Xy),
                ("XYZ", Dimensions::Xyz),
                ("XYM", Dimensions::Xym),
                ("XYZM", Dimensions::Xyzm),
            ];

            for (dim, expected) in cases {
                let x = eval_string(&format!("structure(c(1, 2), class = c('{dim}', 'POINT', 'sfg'))"))?;
                assert_eq!(parse_dim(x.class(), "POINT")?, expected);
            }
        }
    }

    #[test]
    fn parse_dim_rejects_bad_classes() {
        test! {
            let cases = [
                "c(1, 2)",
                "structure(c(1, 2), class = 'sfg')",
                "structure(c(1, 2), class = c('XYQ', 'POINT', 'sfg'))",
                "structure(c(1, 2), class = c('XY', 'LINESTRING', 'sfg'))",
                "structure(c(1, 2), class = c('XY', 'POINT'))",
                "structure(c(1, 2), class = c('POINT', 'XY', 'sfg'))",
            ];

            for code in cases {
                let x = eval_string(code)?;
                let err = parse_dim(x.class(), "POINT").unwrap_err().to_string();
                assert!(err.contains("Expected an sfg POINT."), "{code}: {err}");
            }
        }
    }

    #[test]
    fn dim_of_each_type_is_read_from_its_class() {
        test! {
            let mpnt = SfgMultiPoint::try_from(R!("
                structure(matrix(c(0, 1, 2, 3, 4, 5), ncol = 3), class = c('XYM', 'MULTIPOINT', 'sfg'))
            ")?)?;
            assert_eq!(mpnt.dim(), Dimensions::Xym);

            let mlns = SfgMultiLineString::try_from(R!("
                structure(list(matrix(c(0, 1, 0, 1), ncol = 2)), class = c('XY', 'MULTILINESTRING', 'sfg'))
            ")?)?;
            assert_eq!(mlns.dim(), Dimensions::Xy);

            let gc = SfgGeometryCollection::try_from(R!("
                structure(list(), class = c('XYZ', 'GEOMETRYCOLLECTION', 'sfg'))
            ")?)?;
            assert_eq!(gc.dim(), Dimensions::Xyz);
        }
    }

    #[test]
    fn rings_take_the_dimension_of_their_parent() {
        test! {
            let poly = SfgPolygon::try_from(R!("
                structure(
                    list(matrix(c(0, 1, 1, 0, 0, 0, 1, 0, 5, 6, 7, 5), ncol = 3)),
                    class = c('XYM', 'POLYGON', 'sfg')
                )
            ")?)?;
            let exterior = poly.exterior().unwrap();
            assert_eq!(exterior.dim(), Dimensions::Xym);
            assert_eq!(exterior.coord(2).unwrap().dim(), Dimensions::Xym);
            assert_eq!(exterior.coord(2).unwrap().nth(2), Some(7.0));

            let mlns = SfgMultiLineString::try_from(R!("
                structure(
                    list(matrix(c(0, 1, 0, 1, 2, 3), ncol = 3)),
                    class = c('XYZ', 'MULTILINESTRING', 'sfg')
                )
            ")?)?;
            assert_eq!(mlns.line_string(0).unwrap().dim(), Dimensions::Xyz);
        }
    }

    #[test]
    fn point_xy() {
        test! {
            let x = SfgPoint::try_from(R!("structure(c(1, 2), class = c('XY', 'POINT', 'sfg'))")?)?;
            assert_eq!(x.dim(), Dimensions::Xy);
            assert_eq!(x.to_point(), point! { x: 1.0, y: 2.0 });
        }
    }

    #[test]
    fn point_xyzm() {
        test! {
            let x = SfgPoint::try_from(R!("structure(c(1, 2, 3, 4), class = c('XYZM', 'POINT', 'sfg'))")?)?;
            let coord = x.coord().unwrap();
            assert_eq!(coord.dim(), Dimensions::Xyzm);
            assert_eq!((coord.nth(2), coord.nth(3), coord.nth(4)), (Some(3.0), Some(4.0), None));
        }
    }

    #[test]
    fn empty_point_has_no_coord() {
        test! {
            let x = SfgPoint::try_from(R!("structure(c(NA_real_, NA_real_), class = c('XY', 'POINT', 'sfg'))")?)?;
            assert!(x.coord().is_none());
            assert!(x.try_to_point().is_none());
        }
    }

    #[test]
    fn invalid_points_are_errors() {
        test! {
            let cases = [
                "c(1, 2)",
                "structure(c(1, 2), class = c('XY', 'LINESTRING', 'sfg'))",
                "structure(c(1, 2), class = c('XYQ', 'POINT', 'sfg'))",
                "structure(c(1, 2), class = c('XYZ', 'POINT', 'sfg'))",
                "structure(c('a', 'b'), class = c('XY', 'POINT', 'sfg'))",
            ];

            for code in cases {
                assert!(SfgPoint::try_from(eval_string(code)?).is_err(), "{code} should be an error");
            }
        }
    }

    #[test]
    fn multipoint() {
        test! {
            let x = SfgMultiPoint::try_from(R!("
                structure(matrix(c(0, 1, 2, 3), ncol = 2), class = c('XY', 'MULTIPOINT', 'sfg'))
            ")?)?;
            assert_eq!(x.num_points(), 2);
            assert_eq!(
                x.to_multi_point(),
                vec![point! { x: 0.0, y: 2.0 }, point! { x: 1.0, y: 3.0 }].into()
            );
        }
    }

    #[test]
    fn linestring_xyz() {
        test! {
            let x = SfgLineString::try_from(R!("
                structure(matrix(c(0, 1, 2, 3, 4, 5), ncol = 3), class = c('XYZ', 'LINESTRING', 'sfg'))
            ")?)?;
            assert_eq!(x.dim(), Dimensions::Xyz);
            assert_eq!(x.num_coords(), 2);
            assert_eq!(x.coord(1).unwrap().nth(2), Some(5.0));
            assert_eq!(x.to_line_string(), line_string![(x: 0.0, y: 2.0), (x: 1.0, y: 3.0)]);
        }
    }

    #[test]
    fn linestring_with_wrong_number_of_columns_is_an_error() {
        test! {
            let x = R!("structure(matrix(c(0, 1, 2, 3, 4, 5), ncol = 3), class = c('XY', 'LINESTRING', 'sfg'))")?;
            assert!(SfgLineString::try_from(x).is_err());
        }
    }

    #[test]
    fn multilinestring() {
        test! {
            let x = SfgMultiLineString::try_from(R!("
                structure(
                    list(matrix(c(0, 1, 0, 1), ncol = 2), matrix(c(5, 6, 5, 6), ncol = 2)),
                    class = c('XY', 'MULTILINESTRING', 'sfg')
                )
            ")?)?;
            assert_eq!(x.num_line_strings(), 2);
            assert_eq!(
                x.to_multi_line_string(),
                MultiLineString::new(vec![
                    line_string![(x: 0.0, y: 0.0), (x: 1.0, y: 1.0)],
                    line_string![(x: 5.0, y: 5.0), (x: 6.0, y: 6.0)],
                ])
            );
        }
    }

    #[test]
    fn polygon_with_hole() {
        test! {
            let x = SfgPolygon::try_from(eval_string(SQUARE_WITH_HOLE)?)?;
            assert_eq!(x.num_interiors(), 1);

            let expected = geo_types::Polygon::new(
                line_string![(x: 0.0, y: 0.0), (x: 10.0, y: 0.0), (x: 10.0, y: 10.0), (x: 0.0, y: 10.0), (x: 0.0, y: 0.0)],
                vec![line_string![(x: 1.0, y: 1.0), (x: 2.0, y: 1.0), (x: 2.0, y: 2.0), (x: 1.0, y: 2.0), (x: 1.0, y: 1.0)]],
            );
            assert_eq!(x.to_polygon(), expected);
        }
    }

    #[test]
    fn empty_polygon_has_no_exterior() {
        test! {
            let x = SfgPolygon::try_from(R!("structure(list(), class = c('XY', 'POLYGON', 'sfg'))")?)?;
            assert!(x.exterior().is_none());
            assert_eq!(x.num_interiors(), 0);
            assert_eq!(x.to_polygon(), geo_types::Polygon::new(LineString::new(vec![]), vec![]));
        }
    }

    #[test]
    fn multipolygon() {
        test! {
            let x = SfgMultiPolygon::try_from(R!("
                structure(
                    list(
                        list(matrix(c(0, 1, 1, 0, 0, 0, 1, 0), ncol = 2)),
                        list(matrix(c(5, 6, 6, 5, 5, 5, 6, 5), ncol = 2))
                    ),
                    class = c('XY', 'MULTIPOLYGON', 'sfg')
                )
            ")?)?;
            assert_eq!(x.num_polygons(), 2);
            assert_eq!(
                x.to_multi_polygon(),
                MultiPolygon::new(vec![
                    polygon![(x: 0.0, y: 0.0), (x: 1.0, y: 0.0), (x: 1.0, y: 1.0)],
                    polygon![(x: 5.0, y: 5.0), (x: 6.0, y: 5.0), (x: 6.0, y: 6.0)],
                ])
            );
        }
    }

    #[test]
    fn sfg_dispatches_on_class() {
        test! {
            let cases = [
                ("structure(c(1, 2), class = c('XY', 'POINT', 'sfg'))", "POINT"),
                ("structure(matrix(c(0, 1, 0, 1), ncol = 2), class = c('XY', 'MULTIPOINT', 'sfg'))", "MULTIPOINT"),
                ("structure(matrix(c(0, 1, 0, 1), ncol = 2), class = c('XY', 'LINESTRING', 'sfg'))", "LINESTRING"),
                ("structure(list(matrix(c(0, 1, 0, 1), ncol = 2)), class = c('XY', 'MULTILINESTRING', 'sfg'))", "MULTILINESTRING"),
                (SQUARE_WITH_HOLE, "POLYGON"),
                ("structure(list(list(matrix(c(0, 1, 1, 0, 0, 0, 1, 0), ncol = 2))), class = c('XY', 'MULTIPOLYGON', 'sfg'))", "MULTIPOLYGON"),
                ("structure(list(structure(c(1, 2), class = c('XY', 'POINT', 'sfg'))), class = c('XY', 'GEOMETRYCOLLECTION', 'sfg'))", "GEOMETRYCOLLECTION"),
            ];

            for (code, expected) in cases {
                let variant = match Sfg::try_from(eval_string(code)?)? {
                    Sfg::Point(_) => "POINT",
                    Sfg::MultiPoint(_) => "MULTIPOINT",
                    Sfg::LineString(_) => "LINESTRING",
                    Sfg::MultiLineString(_) => "MULTILINESTRING",
                    Sfg::Polygon(_) => "POLYGON",
                    Sfg::MultiPolygon(_) => "MULTIPOLYGON",
                    Sfg::GeometryCollection(_) => "GEOMETRYCOLLECTION",
                };
                assert_eq!(variant, expected);
            }
        }
    }

    #[test]
    fn sfg_to_geometry() {
        test! {
            let x = Sfg::try_from(R!("structure(c(1, 2), class = c('XY', 'POINT', 'sfg'))")?)?;
            assert_eq!(x.to_geometry(), Geometry::Point(point! { x: 1.0, y: 2.0 }));
        }
    }

    #[test]
    fn unsupported_sfg_is_an_error() {
        test! {
            let unsupported = R!("structure(matrix(c(0, 1, 2, 0, 1, 0), ncol = 2), class = c('XY', 'CIRCULARSTRING', 'sfg'))")?;
            assert!(Sfg::try_from(unsupported).is_err());
            assert!(Sfg::try_from(Robj::from(NULL)).is_err());
        }
    }

    #[test]
    fn dim_reads_the_class() {
        test! {
            let pnt = SfgPoint::try_from(R!("structure(c(1, 2, 3), class = c('XYZ', 'POINT', 'sfg'))")?)?;
            assert_eq!(pnt.dim(), Dimensions::Xyz);

            let lns = SfgLineString::try_from(R!("
                structure(matrix(c(0, 1, 0, 1), ncol = 2), class = c('XY', 'LINESTRING', 'sfg'))
            ")?)?;
            assert_eq!(lns.dim(), Dimensions::Xy);

            let poly = SfgPolygon::try_from(R!("
                structure(
                    list(matrix(c(0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0), ncol = 3)),
                    class = c('XYM', 'POLYGON', 'sfg')
                )
            ")?)?;
            assert_eq!(poly.dim(), Dimensions::Xym);

            let sfg = Sfg::try_from(R!("structure(c(1, 2, 3, 4), class = c('XYZM', 'POINT', 'sfg'))")?)?;
            assert_eq!(sfg.dim(), Dimensions::Xyzm);
        }
    }

    #[test]
    fn round_trip_returns_the_same_r_object() {
        test! {
            let original = eval_string(SQUARE_WITH_HOLE)?;
            let back = Robj::from(SfgPolygon::try_from(original.clone())?);
            assert_eq!(back, original);
            assert_eq!(classes(back), ["XY", "POLYGON", "sfg"]);

            let pnt = R!("structure(c(1, 2, 3), class = c('XYM', 'POINT', 'sfg'))")?;
            assert_eq!(Robj::from(Sfg::try_from(pnt.clone())?), pnt);
        }
    }
}

#[cfg(test)]
mod from_geo_types_tests {
    use super::*;
    use geo_types::{line_string, point, polygon};

    #[test]
    fn geo_types_convert_to_the_matching_sfg() {
        test! {
            let pnt = Robj::from(SfgPoint::from(point! { x: 1.0, y: 2.0 }));
            assert_eq!(pnt, eval_string("structure(c(1, 2), class = c('XY', 'POINT', 'sfg'))")?);

            let mpnt = geo_types::MultiPoint::new(vec![point! { x: 0.0, y: 2.0 }, point! { x: 1.0, y: 3.0 }]);
            assert_eq!(
                Robj::from(SfgMultiPoint::from(mpnt)),
                eval_string("structure(matrix(c(0, 1, 2, 3), ncol = 2), class = c('XY', 'MULTIPOINT', 'sfg'))")?
            );

            let lns = line_string![(x: 0.0, y: 2.0), (x: 1.0, y: 3.0)];
            assert_eq!(
                Robj::from(SfgLineString::from(lns)),
                eval_string("structure(matrix(c(0, 1, 2, 3), ncol = 2), class = c('XY', 'LINESTRING', 'sfg'))")?
            );

            let mlns = geo_types::MultiLineString::new(vec![
                line_string![(x: 0.0, y: 0.0), (x: 1.0, y: 1.0)],
                line_string![(x: 5.0, y: 5.0), (x: 6.0, y: 6.0)],
            ]);
            assert_eq!(
                Robj::from(SfgMultiLineString::from(mlns)),
                eval_string("
                    structure(
                        list(matrix(c(0, 1, 0, 1), ncol = 2), matrix(c(5, 6, 5, 6), ncol = 2)),
                        class = c('XY', 'MULTILINESTRING', 'sfg')
                    )
                ")?
            );

            let poly = geo_types::Polygon::new(
                line_string![(x: 0.0, y: 0.0), (x: 10.0, y: 0.0), (x: 10.0, y: 10.0), (x: 0.0, y: 10.0), (x: 0.0, y: 0.0)],
                vec![line_string![(x: 1.0, y: 1.0), (x: 2.0, y: 1.0), (x: 2.0, y: 2.0), (x: 1.0, y: 2.0), (x: 1.0, y: 1.0)]],
            );
            assert_eq!(
                Robj::from(SfgPolygon::from(poly)),
                eval_string("
                    structure(
                        list(
                            matrix(c(0, 10, 10, 0, 0, 0, 0, 10, 10, 0), ncol = 2),
                            matrix(c(1, 2, 2, 1, 1, 1, 1, 2, 2, 1), ncol = 2)
                        ),
                        class = c('XY', 'POLYGON', 'sfg')
                    )
                ")?
            );

            let mpoly = geo_types::MultiPolygon::new(vec![
                polygon![(x: 0.0, y: 0.0), (x: 1.0, y: 0.0), (x: 1.0, y: 1.0)],
                polygon![(x: 5.0, y: 5.0), (x: 6.0, y: 5.0), (x: 6.0, y: 6.0)],
            ]);
            assert_eq!(
                Robj::from(SfgMultiPolygon::from(mpoly)),
                eval_string("
                    structure(
                        list(
                            list(matrix(c(0, 1, 1, 0, 0, 0, 1, 0), ncol = 2)),
                            list(matrix(c(5, 6, 6, 5, 5, 5, 6, 5), ncol = 2))
                        ),
                        class = c('XY', 'MULTIPOLYGON', 'sfg')
                    )
                ")?
            );
        }
    }
}

#[cfg(test)]
mod geometry_collection_tests {
    use super::*;
    use geo_traits::to_geo::ToGeoGeometry;
    use geo_types::{Geometry, GeometryCollection, line_string, point};

    #[test]
    fn geometry_collection_to_geometry() {
        test! {
            let x = SfgGeometryCollection::try_from(R!("
                structure(
                    list(
                        structure(c(1, 2), class = c('XY', 'POINT', 'sfg')),
                        structure(matrix(c(0, 1, 0, 1), ncol = 2), class = c('XY', 'LINESTRING', 'sfg'))
                    ),
                    class = c('XY', 'GEOMETRYCOLLECTION', 'sfg')
                )
            ")?)?;
            assert_eq!(x.num_geometries(), 2);

            let expected = GeometryCollection::new_from(vec![
                Geometry::Point(point! { x: 1.0, y: 2.0 }),
                Geometry::LineString(line_string![(x: 0.0, y: 0.0), (x: 1.0, y: 1.0)]),
            ]);
            assert_eq!(x.to_geometry(), Geometry::GeometryCollection(expected));
        }
    }

    #[test]
    fn geometry_collection_with_invalid_member_is_an_error() {
        test! {
            let x = R!("
                structure(list(structure(1, class = c('XY', 'POINT', 'sfg'))), class = c('XY', 'GEOMETRYCOLLECTION', 'sfg'))
            ")?;
            assert!(SfgGeometryCollection::try_from(x).is_err());
        }
    }
}
