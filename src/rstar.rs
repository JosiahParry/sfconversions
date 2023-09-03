impl Geom {
        // Helper to create an AABB
        fn to_aabb(&self) -> AABB<[f64; 2]> {
            let rct = self.geom.bounding_rect().unwrap();
            let ll = rct.min();
            let ur = rct.max();
            AABB::from_corners(ll.into(), ur.into())
        }
}
