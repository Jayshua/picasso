#![allow(dead_code)]
use std::mem;

/// Used to create shapes by calling `line_to` and `move_to`.
/// Pass this to a Window to render
#[derive(Debug)]
pub struct Canvas {
  points: Vec<Point>,
  figures: Vec<Figure>,
  path_in_progress: Vec<(usize, usize)>,
  transform: Matrix,
}


fn matrix_multiply(left: Matrix, right: Matrix) -> Matrix {
   // a b c   0 1 2
   // d e f   3 4 5
   // g h i   6 7 8

   let a = left[0] * right[0] + left[1] * right[3] + left[2] * right[6];
   let b = left[0] * right[1] + left[1] * right[4] + left[2] * right[7];
   let c = left[0] * right[2] + left[1] * right[5] + left[2] * right[8];

   let d = left[3] * right[0] + left[4] * right[3] + left[5] * right[6];
   let e = left[3] * right[1] + left[4] * right[4] + left[5] * right[7];
   let f = left[3] * right[2] + left[4] * right[5] + left[5] * right[8];

   let g = left[6] * right[0] + left[7] * right[3] + left[8] * right[6];
   let h = left[6] * right[1] + left[7] * right[4] + left[8] * right[7];
   let i = left[6] * right[2] + left[7] * right[5] + left[8] * right[8];

   [
      a, b, c,
      d, e, f,
      g, h, i,
   ]
}

fn matrix_point_mul(matrix: Matrix, point: Point) -> Point {
   (
      matrix[0] * point.0 + matrix[1] * point.1 + matrix[2],
      matrix[3] * point.0 + matrix[4] * point.1 + matrix[5],
   )
}


impl Canvas {
   pub fn new() -> Canvas {
      Canvas {
         points: vec![],
         figures: vec![],
         path_in_progress: vec![],
         transform: [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
         ],
      }
   }


   pub fn rotate(mut self, angle: f32) -> Self {
      let rotation_matrix = [
         angle.cos(), -angle.sin(), 0.0,
         angle.sin(),  angle.cos(), 0.0,
         0.0,          0.0,         1.0,
      ];

      self.transform = matrix_multiply(self.transform, rotation_matrix);

      self
   }


   pub fn translate(mut self, x: f32, y: f32) -> Self {
      let translation_matrix = [
         1.0, 0.0, x,
         0.0, 1.0, y,
         0.0, 0.0, 1.0,
      ];

      self.transform = matrix_multiply(self.transform, translation_matrix);

      self
   }


   /// Draw a line to the provided points
   pub fn line_to(mut self, x: f32, y: f32) -> Self {
      self.points.push(matrix_point_mul(self.transform, (x, y)));

      if self.path_in_progress.len() == 0 {
         self.path_in_progress.push((self.points.len() - 1, 1));
      } else {
         self.path_in_progress.last_mut().unwrap().1 += 1;
      }

      self
   }


   /// Move the virtual "pen" to new coordinates without connecting them with a line
   pub fn move_to(mut self, x: f32, y: f32) -> Self {
      self.points.push(matrix_point_mul(self.transform, (x, y)));
      self.path_in_progress.push((self.points.len() - 1, 1));
      self
   }


   /// Draw a rectangle
   pub fn rectangle(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
      self
         .move_to(x, y)
         .line_to(x + width, y)
         .line_to(x + width, y + height)
         .line_to(x, y + height)
   }


   /// Complete the current shape by giving it a fill
   pub fn fill(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
      self.figures.push(Figure {fill: (red, green, blue, alpha), paths: self.path_in_progress});
      self.path_in_progress = vec![];
      self
   }


   /// Copy another canvas to this one
   pub fn attach(mut self, other: &Canvas) -> Self {
      let offset = self.points.len();

      self.points.extend(other.points.iter());
      self.figures.extend(other.figures.iter().map(|figure| {
         Figure {
            fill: figure.fill,
            paths: figure.paths.iter().map(|&(index, length)| (index + offset, length)).collect()
         }
      }));

      self
   }
}



impl Canvas {
   pub(crate) fn figures_iter<'a>(&'a self) -> Box<::std::iter::Iterator<Item = &Figure> + 'a> {
      Box::new(self.figures.iter())
   }


   // Return an unsafe pointer to the point data for this figure
   // (buffer_length, buffer_pointer)
   pub(crate) unsafe fn get_points_buffer(&self) -> (*const Point, usize) {
      (
         mem::transmute(&self.points[0]),
         (mem::size_of::<Point>() * self.points.len()) as usize
      )
   }
}



type Point = (f32, f32);
type Matrix = [f32; 9];
// 0 1 2
// 3 4 5
// 6 7 8

#[derive(Debug)]
pub struct Figure {
  pub fill: (f32, f32, f32, f32),
  pub paths: Vec<(usize, usize)> // (index, length)
}


