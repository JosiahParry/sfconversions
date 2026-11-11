//! Typed wrappers around sf's `sfc` R objects
//!
//! Each type holds only its R list. Iterating reads one sfg from the list at a time,
//! so nothing is copied. Collecting sfgs into one of these types builds a list and sets
//! its `sfc_<TYPE>` class. The other attributes sf uses, such as `bbox` and `crs`, are
//! not set.
use crate::sfg::{
    SfgGeometryCollection, SfgLineString, SfgMultiLineString, SfgMultiPoint, SfgMultiPolygon,
    SfgPoint, SfgPolygon,
};
use extendr_api::{error::Error, prelude::*};

/// An sf `sfc_POINT`.
#[derive(Debug)]
pub struct SfcPoint(List);

impl TryFrom<&Robj> for SfcPoint {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        if !value.inherits("sfc_POINT") {
            return Err(Error::Other("Expected an sfc_POINT.".to_string()));
        }
        Ok(SfcPoint(List::try_from(value.clone())?))
    }
}

impl TryFrom<Robj> for SfcPoint {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        SfcPoint::try_from(&value)
    }
}

impl From<SfcPoint> for Robj {
    fn from(value: SfcPoint) -> Self {
        value.0.into()
    }
}

/// Iterates over the points of an [`SfcPoint`], reading each one from the list when asked.
pub struct SfcPointIter {
    list: List,
    front: usize,
    back: usize,
}

impl Iterator for SfcPointIter {
    type Item = Result<SfgPoint, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let item = SfgPoint::try_from(self.list[self.front].clone());
        self.front += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for SfcPointIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(SfgPoint::try_from(self.list[self.back].clone()))
    }
}

impl ExactSizeIterator for SfcPointIter {}

impl SfcPoint {
    /// The number of points.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no points.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the points.
    pub fn iter(&self) -> SfcPointIter {
        SfcPointIter {
            list: self.0.clone(),
            front: 0,
            back: self.0.len(),
        }
    }
}

impl IntoIterator for SfcPoint {
    type Item = Result<SfgPoint, Error>;
    type IntoIter = SfcPointIter;

    fn into_iter(self) -> Self::IntoIter {
        let back = self.0.len();
        SfcPointIter {
            list: self.0,
            front: 0,
            back,
        }
    }
}

impl IntoIterator for &SfcPoint {
    type Item = Result<SfgPoint, Error>;
    type IntoIter = SfcPointIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<SfgPoint> for SfcPoint {
    fn from_iter<I: IntoIterator<Item = SfgPoint>>(iter: I) -> Self {
        let mut data = iter.into_iter().map(Robj::from).collect::<List>();
        let _ = data.set_class(["sfc_POINT", "sfc"]);
        SfcPoint(data)
    }
}

/// An sf `sfc_MULTIPOINT`.
#[derive(Debug)]
pub struct SfcMultiPoint(List);

impl TryFrom<&Robj> for SfcMultiPoint {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        if !value.inherits("sfc_MULTIPOINT") {
            return Err(Error::Other("Expected an sfc_MULTIPOINT.".to_string()));
        }
        Ok(SfcMultiPoint(List::try_from(value.clone())?))
    }
}

impl TryFrom<Robj> for SfcMultiPoint {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        SfcMultiPoint::try_from(&value)
    }
}

impl From<SfcMultiPoint> for Robj {
    fn from(value: SfcMultiPoint) -> Self {
        value.0.into()
    }
}

/// Iterates over the multipoints of an [`SfcMultiPoint`], reading each one from the list when asked.
pub struct SfcMultiPointIter {
    list: List,
    front: usize,
    back: usize,
}

impl Iterator for SfcMultiPointIter {
    type Item = Result<SfgMultiPoint, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let item = SfgMultiPoint::try_from(self.list[self.front].clone());
        self.front += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for SfcMultiPointIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(SfgMultiPoint::try_from(self.list[self.back].clone()))
    }
}

impl ExactSizeIterator for SfcMultiPointIter {}

impl SfcMultiPoint {
    /// The number of multipoints.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no multipoints.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the multipoints.
    pub fn iter(&self) -> SfcMultiPointIter {
        SfcMultiPointIter {
            list: self.0.clone(),
            front: 0,
            back: self.0.len(),
        }
    }
}

impl IntoIterator for SfcMultiPoint {
    type Item = Result<SfgMultiPoint, Error>;
    type IntoIter = SfcMultiPointIter;

    fn into_iter(self) -> Self::IntoIter {
        let back = self.0.len();
        SfcMultiPointIter {
            list: self.0,
            front: 0,
            back,
        }
    }
}

impl IntoIterator for &SfcMultiPoint {
    type Item = Result<SfgMultiPoint, Error>;
    type IntoIter = SfcMultiPointIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<SfgMultiPoint> for SfcMultiPoint {
    fn from_iter<I: IntoIterator<Item = SfgMultiPoint>>(iter: I) -> Self {
        let mut data = iter.into_iter().map(Robj::from).collect::<List>();
        let _ = data.set_class(["sfc_MULTIPOINT", "sfc"]);
        SfcMultiPoint(data)
    }
}

/// An sf `sfc_LINESTRING`.
#[derive(Debug)]
pub struct SfcLineString(List);

impl TryFrom<&Robj> for SfcLineString {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        if !value.inherits("sfc_LINESTRING") {
            return Err(Error::Other("Expected an sfc_LINESTRING.".to_string()));
        }
        Ok(SfcLineString(List::try_from(value.clone())?))
    }
}

impl TryFrom<Robj> for SfcLineString {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        SfcLineString::try_from(&value)
    }
}

impl From<SfcLineString> for Robj {
    fn from(value: SfcLineString) -> Self {
        value.0.into()
    }
}

/// Iterates over the linestrings of an [`SfcLineString`], reading each one from the list when asked.
pub struct SfcLineStringIter {
    list: List,
    front: usize,
    back: usize,
}

impl Iterator for SfcLineStringIter {
    type Item = Result<SfgLineString, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let item = SfgLineString::try_from(self.list[self.front].clone());
        self.front += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for SfcLineStringIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(SfgLineString::try_from(self.list[self.back].clone()))
    }
}

impl ExactSizeIterator for SfcLineStringIter {}

impl SfcLineString {
    /// The number of linestrings.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no linestrings.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the linestrings.
    pub fn iter(&self) -> SfcLineStringIter {
        SfcLineStringIter {
            list: self.0.clone(),
            front: 0,
            back: self.0.len(),
        }
    }
}

impl IntoIterator for SfcLineString {
    type Item = Result<SfgLineString, Error>;
    type IntoIter = SfcLineStringIter;

    fn into_iter(self) -> Self::IntoIter {
        let back = self.0.len();
        SfcLineStringIter {
            list: self.0,
            front: 0,
            back,
        }
    }
}

impl IntoIterator for &SfcLineString {
    type Item = Result<SfgLineString, Error>;
    type IntoIter = SfcLineStringIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<SfgLineString> for SfcLineString {
    fn from_iter<I: IntoIterator<Item = SfgLineString>>(iter: I) -> Self {
        let mut data = iter.into_iter().map(Robj::from).collect::<List>();
        let _ = data.set_class(["sfc_LINESTRING", "sfc"]);
        SfcLineString(data)
    }
}

/// An sf `sfc_MULTILINESTRING`.
#[derive(Debug)]
pub struct SfcMultiLineString(List);

impl TryFrom<&Robj> for SfcMultiLineString {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        if !value.inherits("sfc_MULTILINESTRING") {
            return Err(Error::Other("Expected an sfc_MULTILINESTRING.".to_string()));
        }
        Ok(SfcMultiLineString(List::try_from(value.clone())?))
    }
}

impl TryFrom<Robj> for SfcMultiLineString {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        SfcMultiLineString::try_from(&value)
    }
}

impl From<SfcMultiLineString> for Robj {
    fn from(value: SfcMultiLineString) -> Self {
        value.0.into()
    }
}

/// Iterates over the multilinestrings of an [`SfcMultiLineString`], reading each one from the list when asked.
pub struct SfcMultiLineStringIter {
    list: List,
    front: usize,
    back: usize,
}

impl Iterator for SfcMultiLineStringIter {
    type Item = Result<SfgMultiLineString, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let item = SfgMultiLineString::try_from(self.list[self.front].clone());
        self.front += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for SfcMultiLineStringIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(SfgMultiLineString::try_from(self.list[self.back].clone()))
    }
}

impl ExactSizeIterator for SfcMultiLineStringIter {}

impl SfcMultiLineString {
    /// The number of multilinestrings.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no multilinestrings.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the multilinestrings.
    pub fn iter(&self) -> SfcMultiLineStringIter {
        SfcMultiLineStringIter {
            list: self.0.clone(),
            front: 0,
            back: self.0.len(),
        }
    }
}

impl IntoIterator for SfcMultiLineString {
    type Item = Result<SfgMultiLineString, Error>;
    type IntoIter = SfcMultiLineStringIter;

    fn into_iter(self) -> Self::IntoIter {
        let back = self.0.len();
        SfcMultiLineStringIter {
            list: self.0,
            front: 0,
            back,
        }
    }
}

impl IntoIterator for &SfcMultiLineString {
    type Item = Result<SfgMultiLineString, Error>;
    type IntoIter = SfcMultiLineStringIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<SfgMultiLineString> for SfcMultiLineString {
    fn from_iter<I: IntoIterator<Item = SfgMultiLineString>>(iter: I) -> Self {
        let mut data = iter.into_iter().map(Robj::from).collect::<List>();
        let _ = data.set_class(["sfc_MULTILINESTRING", "sfc"]);
        SfcMultiLineString(data)
    }
}

/// An sf `sfc_POLYGON`.
#[derive(Debug)]
pub struct SfcPolygon(List);

impl TryFrom<&Robj> for SfcPolygon {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        if !value.inherits("sfc_POLYGON") {
            return Err(Error::Other("Expected an sfc_POLYGON.".to_string()));
        }
        Ok(SfcPolygon(List::try_from(value.clone())?))
    }
}

impl TryFrom<Robj> for SfcPolygon {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        SfcPolygon::try_from(&value)
    }
}

impl From<SfcPolygon> for Robj {
    fn from(value: SfcPolygon) -> Self {
        value.0.into()
    }
}

/// Iterates over the polygons of an [`SfcPolygon`], reading each one from the list when asked.
pub struct SfcPolygonIter {
    list: List,
    front: usize,
    back: usize,
}

impl Iterator for SfcPolygonIter {
    type Item = Result<SfgPolygon, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let item = SfgPolygon::try_from(self.list[self.front].clone());
        self.front += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for SfcPolygonIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(SfgPolygon::try_from(self.list[self.back].clone()))
    }
}

impl ExactSizeIterator for SfcPolygonIter {}

impl SfcPolygon {
    /// The number of polygons.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no polygons.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the polygons.
    pub fn iter(&self) -> SfcPolygonIter {
        SfcPolygonIter {
            list: self.0.clone(),
            front: 0,
            back: self.0.len(),
        }
    }
}

impl IntoIterator for SfcPolygon {
    type Item = Result<SfgPolygon, Error>;
    type IntoIter = SfcPolygonIter;

    fn into_iter(self) -> Self::IntoIter {
        let back = self.0.len();
        SfcPolygonIter {
            list: self.0,
            front: 0,
            back,
        }
    }
}

impl IntoIterator for &SfcPolygon {
    type Item = Result<SfgPolygon, Error>;
    type IntoIter = SfcPolygonIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<SfgPolygon> for SfcPolygon {
    fn from_iter<I: IntoIterator<Item = SfgPolygon>>(iter: I) -> Self {
        let mut data = iter.into_iter().map(Robj::from).collect::<List>();
        let _ = data.set_class(["sfc_POLYGON", "sfc"]);
        SfcPolygon(data)
    }
}

/// An sf `sfc_MULTIPOLYGON`.
#[derive(Debug)]
pub struct SfcMultiPolygon(List);

impl TryFrom<&Robj> for SfcMultiPolygon {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        if !value.inherits("sfc_MULTIPOLYGON") {
            return Err(Error::Other("Expected an sfc_MULTIPOLYGON.".to_string()));
        }
        Ok(SfcMultiPolygon(List::try_from(value.clone())?))
    }
}

impl TryFrom<Robj> for SfcMultiPolygon {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        SfcMultiPolygon::try_from(&value)
    }
}

impl From<SfcMultiPolygon> for Robj {
    fn from(value: SfcMultiPolygon) -> Self {
        value.0.into()
    }
}

/// Iterates over the multipolygons of an [`SfcMultiPolygon`], reading each one from the list when asked.
pub struct SfcMultiPolygonIter {
    list: List,
    front: usize,
    back: usize,
}

impl Iterator for SfcMultiPolygonIter {
    type Item = Result<SfgMultiPolygon, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let item = SfgMultiPolygon::try_from(self.list[self.front].clone());
        self.front += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for SfcMultiPolygonIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(SfgMultiPolygon::try_from(self.list[self.back].clone()))
    }
}

impl ExactSizeIterator for SfcMultiPolygonIter {}

impl SfcMultiPolygon {
    /// The number of multipolygons.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no multipolygons.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the multipolygons.
    pub fn iter(&self) -> SfcMultiPolygonIter {
        SfcMultiPolygonIter {
            list: self.0.clone(),
            front: 0,
            back: self.0.len(),
        }
    }
}

impl IntoIterator for SfcMultiPolygon {
    type Item = Result<SfgMultiPolygon, Error>;
    type IntoIter = SfcMultiPolygonIter;

    fn into_iter(self) -> Self::IntoIter {
        let back = self.0.len();
        SfcMultiPolygonIter {
            list: self.0,
            front: 0,
            back,
        }
    }
}

impl IntoIterator for &SfcMultiPolygon {
    type Item = Result<SfgMultiPolygon, Error>;
    type IntoIter = SfcMultiPolygonIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<SfgMultiPolygon> for SfcMultiPolygon {
    fn from_iter<I: IntoIterator<Item = SfgMultiPolygon>>(iter: I) -> Self {
        let mut data = iter.into_iter().map(Robj::from).collect::<List>();
        let _ = data.set_class(["sfc_MULTIPOLYGON", "sfc"]);
        SfcMultiPolygon(data)
    }
}

/// An sf `sfc_GEOMETRYCOLLECTION`.
#[derive(Debug)]
pub struct SfcGeometryCollection(List);

impl TryFrom<&Robj> for SfcGeometryCollection {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        if !value.inherits("sfc_GEOMETRYCOLLECTION") {
            return Err(Error::Other(
                "Expected an sfc_GEOMETRYCOLLECTION.".to_string(),
            ));
        }
        Ok(SfcGeometryCollection(List::try_from(value.clone())?))
    }
}

impl TryFrom<Robj> for SfcGeometryCollection {
    type Error = Error;

    fn try_from(value: Robj) -> Result<Self, Self::Error> {
        SfcGeometryCollection::try_from(&value)
    }
}

impl From<SfcGeometryCollection> for Robj {
    fn from(value: SfcGeometryCollection) -> Self {
        value.0.into()
    }
}

/// Iterates over the geometry collections of an [`SfcGeometryCollection`], reading each one from the list when asked.
pub struct SfcGeometryCollectionIter {
    list: List,
    front: usize,
    back: usize,
}

impl Iterator for SfcGeometryCollectionIter {
    type Item = Result<SfgGeometryCollection, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let item = SfgGeometryCollection::try_from(self.list[self.front].clone());
        self.front += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for SfcGeometryCollectionIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(SfgGeometryCollection::try_from(
            self.list[self.back].clone(),
        ))
    }
}

impl ExactSizeIterator for SfcGeometryCollectionIter {}

impl SfcGeometryCollection {
    /// The number of geometry collections.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no geometry collections.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the geometry collections.
    pub fn iter(&self) -> SfcGeometryCollectionIter {
        SfcGeometryCollectionIter {
            list: self.0.clone(),
            front: 0,
            back: self.0.len(),
        }
    }
}

impl IntoIterator for SfcGeometryCollection {
    type Item = Result<SfgGeometryCollection, Error>;
    type IntoIter = SfcGeometryCollectionIter;

    fn into_iter(self) -> Self::IntoIter {
        let back = self.0.len();
        SfcGeometryCollectionIter {
            list: self.0,
            front: 0,
            back,
        }
    }
}

impl IntoIterator for &SfcGeometryCollection {
    type Item = Result<SfgGeometryCollection, Error>;
    type IntoIter = SfcGeometryCollectionIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<SfgGeometryCollection> for SfcGeometryCollection {
    fn from_iter<I: IntoIterator<Item = SfgGeometryCollection>>(iter: I) -> Self {
        let mut data = iter.into_iter().map(Robj::from).collect::<List>();
        let _ = data.set_class(["sfc_GEOMETRYCOLLECTION", "sfc"]);
        SfcGeometryCollection(data)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use geo_traits::to_geo::ToGeoPoint;
    use geo_traits::{CoordTrait, PointTrait};
    use geo_types::point;

    #[extendr]
    fn calc_centroid(x: SfcPoint) -> SfgPoint {
        let (sx, sy, n) = x.into_iter().fold((0.0, 0.0, 0usize), |(sx, sy, n), pnt| {
            match pnt.ok().and_then(|p| p.coord().map(|c| c.x_y())) {
                Some((x, y)) => (sx + x, sy + y, n + 1),
                None => (sx, sy, n),
            }
        });

        let p = point!(x: sx / n as f64, y :sy / n as f64);

        SfgPoint::from(p)
    }

    fn sfg_points() -> Result<Vec<SfgPoint>, Error> {
        ["c(1, 2)", "c(3, 4)", "c(NA_real_, NA_real_)"]
            .into_iter()
            .map(|xy| {
                SfgPoint::try_from(eval_string(&format!(
                    "structure({xy}, class = c('XY', 'POINT', 'sfg'))"
                ))?)
            })
            .collect()
    }

    #[test]
    fn collect_points_sets_the_class() {
        test! {
            let sfc: SfcPoint = sfg_points()?.into_iter().collect();
            assert_eq!(sfc.len(), 3);

            let expected = eval_string("
                structure(
                    list(
                        structure(c(1, 2), class = c('XY', 'POINT', 'sfg')),
                        structure(c(3, 4), class = c('XY', 'POINT', 'sfg')),
                        structure(c(NA_real_, NA_real_), class = c('XY', 'POINT', 'sfg'))
                    ),
                    class = c('sfc_POINT', 'sfc')
                )
            ")?;
            assert_eq!(Robj::from(sfc), expected);
        }
    }

    #[test]
    fn collect_each_type_sets_its_class() {
        test! {
            let classes = |x: Robj| x.class().unwrap().map(String::from).collect::<Vec<_>>();

            let mpnt = SfgMultiPoint::try_from(R!("
                structure(matrix(c(0, 1, 0, 1), ncol = 2), class = c('XY', 'MULTIPOINT', 'sfg'))
            ")?)?;
            assert_eq!(classes(SfcMultiPoint::from_iter([mpnt]).into()), ["sfc_MULTIPOINT", "sfc"]);

            let lns = SfgLineString::try_from(R!("
                structure(matrix(c(0, 1, 0, 1), ncol = 2), class = c('XY', 'LINESTRING', 'sfg'))
            ")?)?;
            assert_eq!(classes(SfcLineString::from_iter([lns]).into()), ["sfc_LINESTRING", "sfc"]);

            let mlns = SfgMultiLineString::try_from(R!("
                structure(list(matrix(c(0, 1, 0, 1), ncol = 2)), class = c('XY', 'MULTILINESTRING', 'sfg'))
            ")?)?;
            assert_eq!(classes(SfcMultiLineString::from_iter([mlns]).into()), ["sfc_MULTILINESTRING", "sfc"]);

            let poly = SfgPolygon::try_from(R!("
                structure(list(matrix(c(0, 1, 1, 0, 0, 0, 1, 0), ncol = 2)), class = c('XY', 'POLYGON', 'sfg'))
            ")?)?;
            assert_eq!(classes(SfcPolygon::from_iter([poly]).into()), ["sfc_POLYGON", "sfc"]);

            let mpoly = SfgMultiPolygon::try_from(R!("
                structure(
                    list(list(matrix(c(0, 1, 1, 0, 0, 0, 1, 0), ncol = 2))),
                    class = c('XY', 'MULTIPOLYGON', 'sfg')
                )
            ")?)?;
            assert_eq!(classes(SfcMultiPolygon::from_iter([mpoly]).into()), ["sfc_MULTIPOLYGON", "sfc"]);

            let gc = SfgGeometryCollection::try_from(R!("
                structure(
                    list(structure(c(1, 2), class = c('XY', 'POINT', 'sfg'))),
                    class = c('XY', 'GEOMETRYCOLLECTION', 'sfg')
                )
            ")?)?;
            assert_eq!(classes(SfcGeometryCollection::from_iter([gc]).into()), ["sfc_GEOMETRYCOLLECTION", "sfc"]);
        }
    }

    #[test]
    fn collect_nothing_gives_an_empty_sfc() {
        test! {
            let sfc: SfcPolygon = Vec::<SfgPolygon>::new().into_iter().collect();
            assert!(sfc.is_empty());

            let expected = eval_string("structure(list(), class = c('sfc_POLYGON', 'sfc'))")?;
            assert_eq!(Robj::from(sfc), expected);
        }
    }

    #[test]
    fn collect_results_stops_at_the_first_error() {
        test! {
            let x = List::try_from(R!("
                list(structure(c(1, 2), class = c('XY', 'POINT', 'sfg')), structure(1, class = c('XY', 'POINT', 'sfg')))
            ")?)?;
            let res = x.iter().map(|(_, g)| SfgPoint::try_from(g)).collect::<Result<SfcPoint, Error>>();
            assert!(res.is_err());
        }
    }

    #[test]
    fn try_from_sfc_and_iterate() {
        test! {
            let sfc = SfcPoint::try_from(R!("
                structure(
                    list(structure(c(1, 2), class = c('XY', 'POINT', 'sfg')), structure(c(3, 4), class = c('XY', 'POINT', 'sfg'))),
                    class = c('sfc_POINT', 'sfc')
                )
            ")?)?;

            let points = (&sfc)
                .into_iter()
                .map(|p| p.map(|p| p.to_point()))
                .collect::<Result<Vec<_>, Error>>()?;
            assert_eq!(points, [point! { x: 1.0, y: 2.0 }, point! { x: 3.0, y: 4.0 }]);

            let mut iter = sfc.iter();
            assert_eq!(iter.len(), 2);
            assert_eq!(iter.next_back().unwrap()?.to_point(), point! { x: 3.0, y: 4.0 });
            assert_eq!(iter.len(), 1);
            assert_eq!(iter.next().unwrap()?.to_point(), point! { x: 1.0, y: 2.0 });
            assert!(iter.next().is_none());
            assert!(iter.next_back().is_none());
        }
    }

    #[test]
    fn invalid_sfcs_are_errors() {
        test! {
            let wrong_class = R!("structure(list(), class = c('sfc_LINESTRING', 'sfc'))")?;
            assert!(SfcPoint::try_from(wrong_class).is_err());

            assert!(SfcPoint::try_from(R!("list()")?).is_err());

            let bad_geometry = R!("
                structure(
                    list(structure(c(1, 2), class = c('XY', 'POINT', 'sfg')), structure(1, class = c('XY', 'POINT', 'sfg'))),
                    class = c('sfc_POINT', 'sfc')
                )
            ")?;
            let items = SfcPoint::try_from(bad_geometry)?.iter().collect::<Vec<_>>();
            assert!(items[0].is_ok());
            assert!(items[1].is_err());
        }
    }

    #[test]
    fn round_trip_keeps_the_attributes() {
        test! {
            let original = R!("
                structure(
                    list(structure(c(1, 2), class = c('XY', 'POINT', 'sfg'))),
                    class = c('sfc_POINT', 'sfc'),
                    precision = 0,
                    bbox = structure(c(xmin = 1, ymin = 2, xmax = 1, ymax = 2), class = 'bbox'),
                    crs = structure(list(input = 'EPSG:4326', wkt = 'GEOGCRS[...]'), class = 'crs'),
                    n_empty = 0L
                )
            ")?;

            let back = Robj::from(SfcPoint::try_from(original.clone())?);
            assert_eq!(back, original);
        }
    }

    #[test]
    fn into_iter_then_collect_rebuilds_the_sfc() {
        test! {
            let sfc: SfcPoint = sfg_points()?.into_iter().collect();
            let expected = Robj::from(sfg_points()?.into_iter().collect::<SfcPoint>());

            let rebuilt = sfc.into_iter().collect::<Result<SfcPoint, Error>>()?;
            assert_eq!(Robj::from(rebuilt), expected);
        }
    }
}
