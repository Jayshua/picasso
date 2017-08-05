use imagefmt;
use gl;
use gl::types::*;

use std::path::Path;
use std::mem;
use std::rc::Rc;

#[derive(Debug)]
pub struct Image {
   pub(crate) texture_id: GLuint
}


impl Drop for Image {
   fn drop(&mut self) {
      unsafe {
         println!("Dropping Image");
         gl::DeleteTextures(1, &self.texture_id);
      }
   }
}


impl Image {
   pub fn new<P: AsRef<Path>>(path: P) -> Result<Rc<Image>, imagefmt::Error> {
      let image = imagefmt::read(path, imagefmt::ColFmt::RGBA)?;

      let texture_id = unsafe {
         let mut texture_id = 0;
         gl::GenTextures(1, &mut texture_id);
         gl::BindTexture(gl::TEXTURE_2D, texture_id);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
         gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.w as i32, image.h as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(&image.buf[0]));
         texture_id
      };

      Ok(Rc::new(Image {
         texture_id: texture_id
      }))
   }
}
