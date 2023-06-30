//! Construct geo-types geometry from R objects
//! 
//! These function are used to convert R objects into geo-types geometry.
//! These functions mimic the structure of sfg objects from the sf package. 
//! Additional quality of life constructors are made available in {rsgeo}.
use extendr_api::prelude::*;
use geo_types::{coord, Coord, LineString, Point, Polygon, point, MultiLineString, MultiPoint, MultiPolygon};
use crate::Geom;


/// Create a single `point` from an x and y value.
pub fn geom_point(x: f64, y: f64) -> Robj {
    Geom::from(Point::new(x, y))
        .into_robj()
        .set_class(["point", "Geom"])
        .unwrap()
}

/// Create a single `multipoint` from a 2 dimensional matrix.
pub fn geom_multipoint(x: RArray<f64, [usize; 2]>) -> Robj {
    let mpnt = MultiPoint::new(matrix_to_points(x));
    Geom::from(mpnt)
        .into_robj()
        .set_class(["multipoint", "Geom"])
        .unwrap()
}

/// Create a single `linestring` from a 2 dimensional matrix.
pub fn geom_linestring(x: RArray<f64, [usize; 2]>) -> Robj {
    let coords = matrix_to_coords(x);
    let lns = LineString::new(coords);
    Geom::from(lns)
        .into_robj()
        .set_class(["linestring", "Geom"])
        .unwrap()
}


/// Create a single `multilinestring` from a list of 2 dimensional matrices.
pub fn geom_multilinestring(x: List) -> Robj {
    let vec_lns = x
        .into_iter()
        .map(|(_, x)| LineString::new(
            matrix_to_coords(
                RMatrix::try_from(x).unwrap()
            ))
        )
        .collect::<Vec<LineString>>();

    Geom::from(MultiLineString::new(vec_lns))
        .into_robj()
        .set_class(["multilinestring", "Geom"])
        .unwrap()
}

/// Create a single `polygon` from a list of 2 dimensional matrices.
pub fn geom_polygon(x: List) -> Robj {
    let n = x.len();
    let mut linestrings: Vec<LineString> = Vec::with_capacity(n);

    let exterior = matrix_to_coords(x[0].as_matrix().unwrap());
    let exterior = LineString::new(exterior);

    if n > 1 {
        for i in 1..n {
            let xi: RMatrix<f64> = x[i].to_owned().try_into().unwrap();
            let coords = matrix_to_coords(xi);
            let line = LineString::new(coords);
            linestrings.push(line);
        }
    }

    let polygon = Polygon::new(exterior, linestrings);
    Geom::from(polygon)
        .into_robj()
        .set_class(["polygon", "Geom"])
        .unwrap()
}

/// Create a single `multipolygon` from a list of lists of 2 dimensional matrices.
pub fn geom_multipolygon(x: List) -> Robj {
    let res = MultiPolygon::new(
        x.into_iter()
            .map(|(_, x)| polygon_inner(List::try_from(x).unwrap()))
            .collect::<Vec<Polygon>>(),
    );

    Geom::from(res)
        .into_robj()
        .set_class(["multipolygon", "Geom"])
        .unwrap()
}

// First, I need to take a matrix and convert into coordinates
/// Convert an `RMatrix<f64>` into a vector of `Coords`.
pub fn matrix_to_coords(x: RMatrix<f64>) -> Vec<Coord> {
    let nrow = x.nrows();
    let ncol = x.ncols();

    if ncol != 2 {
        panic!("Matrix should have only 2 columns for x and y coordinates, respectively.")
    }

    //let n = nrow.clone();
    let mut coords: Vec<Coord> = Vec::with_capacity(nrow);

    for i in 0..nrow {
        let crd = coord! {x: x[[i, 0]], y: x[[i, 1]]};
        coords.push(crd);
    }
    coords
}


/// Convert an `RMatrix<f64>` into a vector of `Points`. Is
/// used internally to create `MultiPoint`s.
pub fn matrix_to_points(x: RMatrix<f64>) -> Vec<Point> {
    let nrow = x.nrows();
    let ncol = x.ncols();

    if ncol != 2 {
        panic!("Matrix should have only 2 columns for x and y coordinates, respectively.")
    }

    //let n = nrow.clone();
    let mut coords: Vec<Point> = Vec::with_capacity(nrow);

    for i in 0..nrow {
        let crd = point! {x: x[[i, 0]], y: x[[i, 1]]};
        coords.push(crd);
    }
    coords
}

// utility function to take a list and convert to a Polygon
// will be used to collect into `Vec<Polygon>` and thus into `MultiPolygon`
fn polygon_inner(x: List) -> Polygon {
    let n = x.len();
    let mut linestrings: Vec<LineString> = Vec::with_capacity(n);

    let exterior = matrix_to_coords(x[0].as_matrix().unwrap());
    let exterior = LineString::new(exterior);

    if n > 1 {
        for i in 1..n {
            let xi: RMatrix<f64> = x[i].to_owned().try_into().unwrap();
            let coords = matrix_to_coords(xi);
            let line = LineString::new(coords);
            linestrings.push(line);
        }
    }

    Polygon::new(exterior, linestrings)
}


