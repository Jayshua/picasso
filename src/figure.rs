use point::{Color, Point};

pub enum FillType {
    Solid(Color),
    LinearGradient(Point, Point, Color, Color)
}


pub struct Figure {
    pub points: Vec<Point>,
    pub paths: Vec<usize>,
    pub fill: FillType
}

impl Figure {
    pub fn new() -> Figure {
        Figure {points: vec![], paths: vec![], fill: FillType::Solid((1.0, 0.0, 0.0, 1.0))}
    }



    /* Primitive Functions */
    pub fn line_to(mut self, x: f32, y: f32) -> Figure {
        self.points.push(Point::new(x, y));
        self
    }

    pub fn move_to(mut self, x: f32, y: f32) -> Figure {
        self.points.push(Point::new(x, y));
        self.paths.push(self.points.len() - 1);
        self
    }



    /* Fill Functions */
    pub fn fill_solid(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Figure {
        self.fill = FillType::Solid((red, green, blue, alpha));
        self
    }


    pub fn fill_linear_gradient(
        mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        from_red: f32,
        from_green: f32,
        from_blue: f32,
        from_alpha: f32,
        to_red: f32,
        to_green: f32,
        to_blue: f32,
        to_alpha: f32
    ) -> Figure {
        self.fill = FillType::LinearGradient(
            Point::new(x1, y1),
            Point::new(x2, y2),
            Color::new(from_red, from_green, from_blue, from_alpha),
            Color::new(to_red, to_green, to_blue, to_alpha)
        );
        self
    }



    /* Convenience Functions */
    pub fn rectangle(mut self, x: f32, y: f32, width: f32, height: f32) -> Figure {
        self.move_to(x, y)
            .line_to(x + width, y)
            .line_to(x + width, y + height)
            .line_to(x, y + height)
    }



    pub fn bezier_to(mut self, in_handle_x: f32, in_handle_y: f32, out_handle_x: f32, out_handle_y: f32, point_x: f32, point_y: f32) -> Figure {
        assert!(self.points.len() > 0);

        let in_point = self.points.last().unwrap().clone();
        let in_handle  = Point::new(in_handle_x, in_handle_y);
        let out_handle = Point::new(out_handle_x, out_handle_y);
        let out_point  = Point::new(point_x, point_y);

        fn interpolate(amount: f32, from: Point, to: Point) -> Point {
            Point::new(from.x + (to.x - from.x) * amount, from.y + (to.y - from.y) * amount)
        }

        for iteration in 0..11 {
            let amount = (iteration as f32) / 10.0;
            let in_value  = interpolate(amount, in_point, in_handle);
            let mid_value = interpolate(amount, in_handle, out_handle);
            let out_value = interpolate(amount, out_handle, out_point);
            let abbc = interpolate(amount, in_value, mid_value);
            let bccd = interpolate(amount, mid_value, out_value);
            self.points.push(interpolate(amount, abbc, bccd));
        }

        self
    }


}