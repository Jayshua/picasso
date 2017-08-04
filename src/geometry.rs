// We use simple type aliases for easy compatibility with other systems.
// A side effect of this is that all methods must be functions rather
// than methods or implementations of std::ops traits.
pub type Point = (f32, f32);

// Matrix layout is row-major
// 0 1 2
// 3 4 5
// 6 7 8
pub type Matrix = [f32; 9];


// Get the identity matrix
// This could have been a constant, but I went with symmetry with the other matrix construction methods instead.
pub fn identity() -> Matrix {
   [
      1.0, 0.0, 0.0,
      0.0, 1.0, 0.0,
      0.0, 0.0, 1.0
   ]
}


// Get a matrix representing the given angle in radians
pub fn rotation(angle: f32) -> Matrix {
   [
      angle.cos(), -angle.sin(), 0.0,
      angle.sin(),  angle.cos(), 0.0,
      0.0,          0.0,         1.0,
   ]
}


// Get a matrix representing the provided translation
pub fn translation((x, y): Point) -> Matrix {
   [
      1.0, 0.0, x,
      0.0, 1.0, y,
      0.0, 0.0, 1.0,
   ]
}


// Apply the given matrix transform to the given point
pub fn mul_point(left: Matrix, (x, y): Point) -> Point {
   (
      left[0] * x + left[1] * y + left[2],
      left[3] * x + left[4] * y + left[5],
   )
}


// Compose to matrices via multiplication
pub fn mul_matrix(left: Matrix, right: Matrix) -> Matrix {
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