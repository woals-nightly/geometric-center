use crate::{locate::Locate, polygon::Polygon};

mod locate {
    #[derive(Clone, PartialEq, Debug)]
    pub struct Locate {
        pub x: f64,
        pub y: f64,
    }
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
                    let (first, elements) = self.locates.split_first().unwrap();
                    let mut center_polygon_location: Vec<Locate> = Vec::new();
                    for loc in elements.windows(2) {
                        let triangle = Polygon {
                            locates: vec![first.clone(), loc[0].clone(), loc[1].clone()],
                        };
                        let center = triangle.center_location();
                        center_polygon_location.push(center);
                    }
                    let center_polygon = Polygon {
                        locates: center_polygon_location,
                    };
                    center_polygon.centroid_by_triangulation()
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
            Locate { x: 0.0, y: 0.0 },
            Locate { x: 4.0, y: 0.0 },
            Locate { x: 5.0, y: 3.0 },
            Locate { x: 2.0, y: 5.0 },
            Locate { x: -1.0, y: 2.0 },
        ],
    };

    assert_eq!(pentagon.center_location(), pentagon.centroid_by_triangulation());
    assert_eq!(rectangle.center_location(), rectangle.centroid_by_triangulation());
}
