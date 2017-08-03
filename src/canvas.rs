use std::mem;
use super::geometry::Matrix;
use super::geometry::Point;

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
         transform: Matrix::identity(),
      }
   }


   pub fn rotate(mut self, angle: f32) -> Self {
      self.transform = self.transform * Matrix::from_rotation(angle);
      self
   }


   pub fn translate(mut self, x: f32, y: f32) -> Self {
      self.transform = self.transform * Matrix::from_translation(x, y);
      self
   }


   /// Draw a line to the provided points
   pub fn line_to(mut self, x: f32, y: f32) -> Self {
      self.points.push(Point::new(x, y));

      if self.path_in_progress.len() == 0 {
         self.path_in_progress.push((self.points.len() - 1, 1));
      } else {
         self.path_in_progress.last_mut().unwrap().1 += 1;
      }

      self
   }


   /// Move the virtual "pen" to new coordinates without connecting them with a line
   pub fn move_to(mut self, x: f32, y: f32) -> Self {
      self.points.push(Point::new(x, y));
      self.path_in_progress.push((self.points.len() - 1, 1));
      self
   }


   /// Draw a rectangle
   pub fn rectangle(self, x: f32, y: f32, width: f32, height: f32) -> Self {
      self
         .move_to(x, y)
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
      begin_x: f32, begin_y: f32,
      end_x: f32,   end_y: f32,
      begin_red: f32, begin_green: f32, begin_blue: f32, begin_alpha: f32,
      end_red: f32,   end_green: f32,   end_blue: f32,   end_alpha: f32,
   ) -> Self {
      let begin = Point::new(begin_x, begin_y);
      let end = Point::new(end_x, end_y);
      let begin_color = (begin_red, begin_green, begin_blue, begin_alpha);
      let end_color = (end_red, end_green, end_blue, end_alpha);

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