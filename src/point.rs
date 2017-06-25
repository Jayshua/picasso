pub struct Color {
   red: f32,
   green: f32,
   blue: f32,
   alpha: f32
}

pub impl Color {
   pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
      Color {red: red, green: green, blue: blue, alpha: alpha}
   }
}

pub struct Point {
   x: f32,
   y: f32
}

pub impl Point {
   pub fn new(x: f32, y: f32) -> Point {
      Point {x: x, y: y}
   }
}
