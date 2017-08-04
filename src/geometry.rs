#[derive(Debug, Copy, Clone)]
pub(crate) struct Point {
   pub x: f32,
   pub y: f32,
}

impl Point {
   pub fn new(x: f32, y: f32) -> Point {
      Point {x: x, y: y}
   }
}


// 0 1 2
// 3 4 5
// 6 7 8
#[derive(Debug, Copy, Clone)]
pub(crate) struct Matrix ([f32; 9]);


impl Matrix {
   pub fn identity() -> Matrix {
      Matrix ([
         1.0, 0.0, 0.0,
         0.0, 1.0, 0.0,
         0.0, 0.0, 1.0
      ])
   }

   pub fn from_rotation(angle: f32) -> Matrix {
      Matrix ([
         angle.cos(), -angle.sin(), 0.0,
         angle.sin(),  angle.cos(), 0.0,
         0.0,          0.0,         1.0,
      ])
   }

   pub fn from_translation(x: f32, y: f32) -> Matrix {
      Matrix ([
         1.0, 0.0, x,
         0.0, 1.0, y,
         0.0, 0.0, 1.0,
      ])
   }
}


impl ::std::ops::Mul for Matrix {
   type Output = Matrix;

   fn mul(self, right_matrix: Matrix) -> Self::Output {
      // a b c   0 1 2
      // d e f   3 4 5
      // g h i   6 7 8

      let left = self.0;
      let right = right_matrix.0;

      let a = left[0] * right[0] + left[1] * right[3] + left[2] * right[6];
      let b = left[0] * right[1] + left[1] * right[4] + left[2] * right[7];
      let c = left[0] * right[2] + left[1] * right[5] + left[2] * right[8];

      let d = left[3] * right[0] + left[4] * right[3] + left[5] * right[6];
      let e = left[3] * right[1] + left[4] * right[4] + left[5] * right[7];
      let f = left[3] * right[2] + left[4] * right[5] + left[5] * right[8];

      let g = left[6] * right[0] + left[7] * right[3] + left[8] * right[6];
      let h = left[6] * right[1] + left[7] * right[4] + left[8] * right[7];
      let i = left[6] * right[2] + left[7] * right[5] + left[8] * right[8];

      Matrix ([
         a, b, c,
         d, e, f,
         g, h, i,
      ])
   }
}


impl ::std::ops::Mul<Point> for Matrix {
   type Output = Point;

   fn mul(self, right: Point) -> Self::Output {
      let left = self.0;

      Point {
         x: left[0] * right.x + left[1] * right.y + left[2],
         y: left[3] * right.x + left[4] * right.y + left[5],
      }
   }
}