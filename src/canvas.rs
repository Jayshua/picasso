#![allow(dead_code)]
use std::mem;

/// Used to create shapes by calling `line_to` and `move_to`.
/// Pass this to a Window to render
#[derive(Debug)]
pub struct Canvas {
  points: Vec<Point>,
  figures: Vec<Figure>,
  path_in_progress: Vec<(usize, usize)>
}


impl Canvas {
   pub fn new() -> Canvas {
      Canvas {
         points: vec![],
         figures: vec![],
         path_in_progress: vec![]
      }
   }


   /// Draw a line to the provided points
   pub fn line_to(mut self, x: f32, y: f32) -> Self {
      self.points.push((x, y));

      if self.path_in_progress.len() == 0 {
         self.path_in_progress.push((self.points.len() - 1, 1));
      } else {
         self.path_in_progress.last_mut().unwrap().1 += 1;
      }

      self
   }


   /// Move the virtual "pen" to new coordinates without connecting them with a line
   pub fn move_to(mut self, x: f32, y: f32) -> Self {
      self.points.push((x, y));
      self.path_in_progress.push((self.points.len() - 1, 1));
      self
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

#[derive(Debug)]
pub struct Figure {
  pub fill: (f32, f32, f32, f32),
  pub paths: Vec<(usize, usize)> // (index, length)
}


