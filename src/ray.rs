use super::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    fn point_at_parameter(&self, t: f64 ) -> Vec3 {
        self.origin + t * self.direction
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let r1 = Ray {
            origin: Vec3::new(1.0, 1.0, 1.0,),
            direction: Vec3::new(1.0, 1.0, 1.0,),
        };
        let r2 = Ray::new (
            Vec3::new(1.0, 1.0, 1.0,),
            Vec3::new(1.0, 1.0, 1.0,),
        );

        assert_eq!(r1, r2);
    }
}
