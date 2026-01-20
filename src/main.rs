use crate::{locate::Locate, polygon::Polygon};

mod locate {
    #[derive(Clone, PartialEq, Debug)]
    pub struct Locate(pub f64, pub f64);
}

mod polygon {
    use crate::{line_line_intersecton_point, locate::Locate};

    pub struct Polygon {
        pub locates: Vec<Locate>,
    }

    impl Polygon {
        // Formula for centroid of a polygon
        pub fn center_location(&self) -> Locate {
            let mut sum_x: f64 = 0.0;
            let mut sum_y: f64 = 0.0;
            for locate in &self.locates {
                sum_x += locate.0;
                sum_y += locate.1;
            }

            let divisor = self.locates.len() as f64;

            Locate(sum_x / divisor, sum_y / divisor)
        }
        // the centroid of a polygon using triangles
        pub fn centroid_by_triangulation(&self) -> Locate {
            match self.locates.len() {
                ..=3 => self.center_location(),
                _ => {
                    let half_point = self.locates.len() / 2;

                    let split = |position: usize| -> (Polygon, Polygon) {
                        // [1,2,3,4,5,6]
                        // [1,2,3,4,1,4,5,6]
                        let mut loc = self.locates.clone();
                        // if position is 1
                        // [2,3,4,5,6,1]
                        // [2,3,4,5,2,5,6,1]
                        loc.rotate_left(position);
                        loc.splice(half_point..half_point, [self.locates[half_point].clone(), self.locates[0].clone()]);
                        let (left, right) = loc.split_at(half_point + 1);
                        // 왼쪽 다각형과 오른쪽 다각형으로 나눔
                        let left = Polygon {
                            locates: left.to_vec()
                        };
                        let right = Polygon {
                            locates: right.to_vec()
                        };
                        (left, right)
                    };

                    let (left_polygon, right_polygon) = split(0);
                    let (left_polygon1, right_polygon1) = split(1);

                    // 삼각형의 중심과 도형의 중심을 잇는 두 선분의 교차점을 구함
                    line_line_intersecton_point(
                        (&left_polygon.centroid_by_triangulation(), &right_polygon.centroid_by_triangulation()), 
                        (&left_polygon1.centroid_by_triangulation(), &right_polygon1.centroid_by_triangulation()))
                }
            }
        }
    }
}

fn line_line_intersecton_point(line1: (&Locate,&Locate), line2: (&Locate, &Locate)) -> Locate {
    let dx1 = line1.0.0 - line1.1.0;
    let dy1 = line1.0.1 - line1.1.1;
    let dx2  = line2.0.0 - line2.1.0;
    let dy2 = line2.0.1 - line2.1.1;

    let c1 = line1.0.0 * line1.1.1 - line1.0.1 * line1.1.0;
    let c2 = line2.0.0 * line2.1.1 - line2.0.1 * line2.1.0;
    let numerator1 = c1 * dx2 - dx1 * c2;
    let numerator2 = c1 * dy2 - dy1 * c2;
    let denominator = dx1 * dy2 - dy1 * dx2;

    Locate(
        numerator1 / denominator,
        numerator2 / denominator,
    )
}

fn main() {
    let rectangle = Polygon {
        locates: vec![
            Locate(-3.0, 1.0),
            Locate(-3.0, 4.0),
            Locate(3.0, 4.0),
            Locate(3.0, 1.0)
        ],
    };

    println!(
        "Centroid of rectangle: {:?}",
        rectangle.centroid_by_triangulation()
    );
}

#[cfg(test)]
mod tests {
    use crate::{locate::Locate, polygon::Polygon};

    #[test]
    fn rectangle() {
        let rectangle = Polygon {
            locates: vec![
                Locate(-3.0, 1.0),
                Locate(-3.0, 4.0),
                Locate(3.0, 4.0),
                Locate(3.0, 1.0)
            ],
        };

        let rectangle1 = Polygon {
            locates: vec![
                Locate(-3.5, 1.2),  
                Locate(-3.5, 4.7),
                Locate(2.8, 4.7),
                Locate(2.8, 1.2), 
            ],
        };

        assert_eq!(
            rectangle.centroid_by_triangulation(),
            rectangle.center_location()
        );
        assert_eq!(
            rectangle1.centroid_by_triangulation(),
            rectangle1.center_location()
        )
    }

    #[test]
    fn pentagon() {
        let pentagon = Polygon {
            locates: vec![
                Locate(2.0, 5.0),
                Locate(-1.0, 2.0),
                Locate(0.0, 0.0),
                Locate(4.0, 0.0),
                Locate(5.0, 3.0),
            ],
        };

        assert_eq!(
            pentagon.centroid_by_triangulation(),
            pentagon.center_location()
        );
    }

    #[test]
    fn hexagon() {
        let hexagon = Polygon {
            locates: vec![
                Locate(1.0, 2.0),
                Locate(-1.0, 2.0),
                Locate(-2.0, 0.0),
                Locate(-1.0, -2.0),
                Locate(1.0, -2.0),
                Locate(2.0, 0.0),
            ],
        };

        assert_eq!(
            hexagon.centroid_by_triangulation(),
            hexagon.center_location()
        );
    }

    #[test]
    fn octagon() {
        let octagon = Polygon {
            locates: vec![
                Locate(3.0, 0.0),
                Locate(0.0, 0.0),
                Locate(-2.0, 2.0),
                Locate(-2.0, 5.0),
                Locate(0.0, 7.0),
                Locate(3.0, 7.0),
                Locate(5.0, 5.0),
                Locate(5.0, 2.0),
            ],
        };

        assert_eq!(
            octagon.centroid_by_triangulation(),
            octagon.center_location()
        );
    }
}