use extendr_api::prelude::*;

// TODO impl TryFrom<Sfg> for Robj
// TODO impl IntoRobj for Sfg
pub struct SfgPoint(Doubles);
pub struct SfgMultiPoint(RMatrix<f64>);

// LineString 
pub struct SfgLineString(RMatrix<f64>);
pub struct SfgMultiLineString(List);

pub struct SfgPolygon(List);
pub struct SfgMultiPolygon(List);