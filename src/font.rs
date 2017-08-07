use rusttype;
use gl;
use super::image::Image;
use std;

pub struct Font<'a> {
   font: rusttype::Font<'a>,
   cache: rusttype::gpu_cache::Cache,
   texture: Image,
}


impl<'a> Font<'a> {
   fn new(font_data: &[u8]) -> Font {
      let font = rusttype::FontCollection::from_bytes(font_data as &[u8]).into_font().unwrap();
      let mut font_cache = rusttype::gpu_cache::Cache::new(512, 512, 0.1, 0.1);


      let texture_id = unsafe {
         let mut texture_id = 0;
         gl::GenTextures(1, &mut texture_id);
         gl::BindTexture(gl::TEXTURE_2D, texture_id);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
         gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 512, 512, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
         texture_id
      };




      let glyph = font.glyph('A').unwrap().scaled(rusttype::Scale::uniform(24.0)).positioned(rusttype::point(0.0, 0.0));
      println!("beginqueu");
      font_cache.queue_glyph(0, glyph);
      println!("queued");
      font_cache.cache_queued(|rect, data| {
         println!("caching");
         gl::TexSubImage2D(
            texture_id,
            0,
            rect.min.x as i32,
            rect.min.y as i32,
            rect.width() as i32,
            rect.height() as i32,
            gl::RGB,
            gl::BYTE,
            std::ptr::null()
         );
      });
      println!("Done caching");

      println!("{:?}", font_cache.rect_for(0, 'A'));

      Font {
         cache: font_cache,
         font: font,
      }
   }
      // let font_data = include_bytes!("NotoSerif-Regular.ttf");

}