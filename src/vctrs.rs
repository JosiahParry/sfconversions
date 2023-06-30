use extendr_api::prelude::*;
use extendr_api::List;

pub fn as_rsgeo_vctr(x: List, class: [String; 3]) -> Robj {
    x
        .set_class(class)
        .unwrap()
}

pub fn geom_class(cls: &str) -> [String; 3] {
    let cls = cls.to_uppercase();
    let geom_class = "rs_".to_owned() + cls.as_str();

    [geom_class, String::from("vctrs_vctr"), String::from("list")]
}

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
