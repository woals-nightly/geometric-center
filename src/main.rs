use crate::{locate::Locate, polygon::Polygon};

mod locate {
    #[derive(Clone, PartialEq, Debug)]
    pub struct Locate {
        pub x: f64,
        pub y: f64,
    }
}

mod polygon {
    use std::vec;

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
                sum_x += locate.x;
                sum_y += locate.y;
            }

            let divisor = self.locates.len() as f64;

            Locate {
                x: sum_x / divisor,
                y: sum_y / divisor,
            }
        }
        pub fn quadrilateral_centroid(&self) -> Locate{
            let mut line: Vec<Locate> = Vec::new();
            for i in 0..2 {
                let mut clone_locates = self.locates.clone();
                clone_locates.rotate_left(i);
                let (first, elements) = clone_locates.split_first().unwrap();
                for loc in elements.windows(2) {
                    let triangle = Polygon {
                        locates: vec![first.clone(), loc[0].clone(), loc[1].clone()],
                    };
                    
                    let center = triangle.center_location();
                    line.push(center);
                }   
            };

            line_line_intersecton_point((&line[0], &line[1]), (&line[2], &line[3]))
        }
        // the centroid of a polygon using triangles
        pub fn centroid_by_triangulation(&self) -> Locate {
            match self.locates.len() {
                ..=3 => self.center_location(),
                4 => self.quadrilateral_centroid(),
                _ => {
                    let half_point = self.locates.len() / 2;
                    // [1,2,3,4,5,6]
                    // [1,2,3,4,1,4,5,6]
                    let mut loc = self.locates.clone();
                    loc.splice(half_point..half_point, [self.locates[half_point].clone(), self.locates[0].clone()]);
                    let (left, right) = loc.split_at(half_point + 1);
                    // 왼쪽 다각형과 오른쪽 다각형으로 나눔
                    let left_polygon = Polygon {
                        locates: left.to_vec()
                    };
                    let right_polygon = Polygon {
                        locates: right.to_vec()
                    };
                    // 위와 동일하지만 포지션만 다름
                    // [2,3,4,5,6,1]
                    // [2,3,4,5,2,5,6,1]
                    let mut loc2 = self.locates.clone();
                    loc2.rotate_left(1);
                    loc2.splice(half_point..half_point, [loc2[half_point].clone(), loc2[0].clone()]);
                    // 왼쪽 다각형과 오른쪽 다각형으로 나눔
                    let (left, right) = loc2.split_at(half_point + 1);
                    let left_polygon1 = Polygon {
                        locates: left.to_vec()
                    };
                    let right_polygon1 = Polygon {
                        locates: right.to_vec()
                    };

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
    let dx1 = line1.0.x - line1.1.x;
    let dy1 = line1.0.y - line1.1.y;
    let dx2  = line2.0.x - line2.1.x;
    let dy2 = line2.0.y - line2.1.y;

    let c1 = line1.0.x * line1.1.y - line1.0.y * line1.1.x;
    let c2 = line2.0.x * line2.1.y - line2.0.y * line2.1.x;
    let numerator1 = c1 * dx2 - dx1 * c2;
    let numerator2 = c1 * dy2 - dy1 * c2;
    let denominator = dx1 * dy2 - dy1 * dx2;

    Locate {
        x: numerator1 / denominator,
        y: numerator2 / denominator,
    }
}

fn main() {
    let rectangle = Polygon {
        locates: vec![
            Locate { x: -3.0, y: 1.0 },
            Locate { x: 3.0, y: 1.0 },
            Locate { x: 3.0, y: 4.0 },
            Locate { x: -3.0, y: 4.0 },
        ],
    };

    let pentagon = Polygon {
        locates: vec![
            Locate { x: 2.0, y: 5.0 },
            Locate { x: 5.0, y: 3.0 },
            Locate { x: 4.0, y: 0.0 },
            Locate { x: 0.0, y: 0.0 },
            Locate { x: -1.0, y: 2.0 },
        ],
    };

    let hexagon = Polygon {
        locates: vec![
            Locate { x: 1.0, y: 2.0 },
            Locate { x: 2.0, y: 0.0 },
            Locate { x: 1.0, y: -2.0 },
            Locate { x: -1.0, y: -2.0 },
            Locate { x: -2.0, y: 0.0 },
            Locate { x: -1.0, y: 2.0 },
        ],
    };

    let octagon = Polygon {
        locates: vec![
            Locate { x: 3.0, y: 0.0 },
            Locate { x: 5.0, y: 2.0 },
            Locate { x: 5.0, y: 5.0 },
            Locate { x: 3.0, y: 7.0 },
            Locate { x: 0.0, y: 7.0 },
            Locate { x: -2.0, y: 5.0 },
            Locate { x: -2.0, y: 2.0 },
            Locate { x: 0.0, y: 0.0 },
        ],
    };

    println!(
        "centroid: {:?}",
        octagon.center_location()
    );

    println!(
        "centroid: {:?}",
        octagon.centroid_by_triangulation()
    );
}
