/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::config::*;

pub struct Shot<'a>{
    pub x:i32,
    pub y:i32,
    pub speed:i32,
    pub texture:&'a Texture<'a>,
}
impl Shot<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(&self.texture,None,Rect::new(self.x,self.y,6,28)).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn update(&mut self){
        self.y -= self.speed;
    }
}
