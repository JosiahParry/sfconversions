use extendr_api::prelude::*;
use extendr_api::List;
use extendr_api::RMatrix;
use geo_types::*;

pub mod geom;
use crate::geom::*;

pub fn sfg_to_geometry(x: Robj) -> Geom {
    let cls = x.class().unwrap().next().unwrap();
    let cls2 = x.class().unwrap().map(|x| x).collect::<Vec<&str>>();

    rprintln!("{}", cls);
    rprintln!("{:?}", cls2);
    let cls = cls2[1];
    
    match cls {
        "POINT" => {
            let x = Doubles::try_from(x).unwrap();
            Geom::from(Point::new(x[0].0, x[1].0))
        }
        "MULTIPOINT" => {
            let x = RMatrix::from_robj(&x).unwrap();
            let mpnt = MultiPoint::new(matrix_to_points(x));

            Geom::from(mpnt)
        }
        "LINESTRING" => {
            let x = RMatrix::from_robj(&x).unwrap();
            let coords = matrix_to_coords(x);
            let lns = LineString::new(coords);
            Geom::from(lns)
        }

        "MULTILINESTRING" => {
            let x = List::try_from(x).unwrap();
            let vec_lns = x
                .into_iter()
                .map(|(_, x)| LineString::new(matrix_to_coords(RMatrix::try_from(x).unwrap())))
                .collect::<Vec<LineString>>();

            Geom::from(MultiLineString::new(vec_lns))
        }
        "POLYGON" => {
            let x = List::try_from(x).unwrap();
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
            polygon.into()
        }

        "MULTIPOLYGON" => {
            let x = List::try_from(x).unwrap();
            let res = MultiPolygon::new(
                x.into_iter()
                    .map(|(_, x)| polygon_inner(List::try_from(x).unwrap()))
                    .collect::<Vec<Polygon>>(),
            );

            res.into()
        }

        &_ => Geom::from(Point::new(0.0, 0.0)),
    }
}

// First, I need to take a matrix and convert into coordinates
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