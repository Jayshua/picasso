use std::mem;
use super::geometry;
use super::geometry::{Point, Matrix, Color};

/// Used to create shapes by calling `line_to` and `move_to`.
/// Pass this to a Window to render
#[derive(Debug)]
pub struct Canvas {
  points: Vec<Point>,
  figures: Vec<Figure>,
  path_in_progress: Vec<(usize, usize)>,
  transform: Matrix,
}



impl Canvas {
   pub fn new() -> Canvas {
      Canvas {
         points: vec![],
         figures: vec![],
         path_in_progress: vec![],
         transform: geometry::identity(),
      }
   }


   pub fn rotate(mut self, angle: f32) -> Self {
      self.transform = geometry::mul_matrix(self.transform, geometry::rotation(angle));
      self
   }


   pub fn translate(mut self, point: Point) -> Self {
      self.transform = geometry::mul_matrix(self.transform, geometry::translation(point));
      self
   }


   /// Draw a line to the provided points
   pub fn line_to(mut self, point: Point) -> Self {
      self.points.push(point);

      if self.path_in_progress.len() == 0 {
         self.path_in_progress.push((self.points.len() - 1, 1));
      } else {
         self.path_in_progress.last_mut().unwrap().1 += 1;
      }

      self
   }


   /// Move the virtual "pen" to new coordinates without connecting them with a line
   pub fn move_to(mut self, point: Point) -> Self {
      self.points.push(point);
      self.path_in_progress.push((self.points.len() - 1, 1));
      self
   }


   /// Draw a rectangle
   pub fn rectangle(self, (x, y): Point, width: f32, height: f32) -> Self {
      self
         .move_to((x, y))
         .line_to(x + width, y)
         .line_to(x + width, y + height)
         .line_to(x, y + height)
   }


   /// Complete the current shape by giving it a fill
   pub fn fill(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
      self.figures.push(Figure {fill: Fill::SolidColor((red, green, blue, alpha)), paths: self.path_in_progress});
      self.path_in_progress = vec![];
      self
   }


   pub fn fill_linear_gradient(
      mut self,
      begin: Point,
      end: Point,
      begin_color: Color,
      end_color: Color,
   ) -> Self {
      self.figures.push(Figure {
         fill: Fill::LinearGradient(begin, end, begin_color, end_color),
         paths: self.path_in_progress
      });

      self.path_in_progress = vec![];

      self
   }


   /// Copy another canvas to this one
   pub fn attach(mut self, other: &Canvas) -> Self {
      let offset = self.points.len();

      self.points.extend(other.points.iter().map(|point| *point));
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




type Color = (f32, f32, f32, f32);

#[derive(Debug, Copy, Clone)]
pub(crate) enum Fill {
   SolidColor(Color),
   LinearGradient(Point, Point, Color, Color),
}

#[derive(Debug)]
pub(crate) struct Figure {
  pub fill: Fill,
  pub paths: Vec<(usize, usize)> // (index, length)
}