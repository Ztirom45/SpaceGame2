/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/

use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::config::*;

pub struct Menu<'a>{
    pub image_buttons:&'a Texture<'a>,
    pub image_mode1:&'a Texture<'a>,
    pub image_mode_background:&'a Texture<'a>,
}

impl Menu<'_>{
    pub fn main(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{ 
        canvas.copy(
            &self.image_buttons,
            Rect::new(0, 40, 40, 40),
            Rect::new(330, 700, 40, 40),
        ).map_err(|e| e.to_string())?;

        canvas.copy(
            &self.image_buttons,
            Rect::new(0, 80, 40, 40),
            Rect::new(380, 700, 40, 40),
        ).map_err(|e| e.to_string())?;
        
        canvas.copy(
            &self.image_buttons,
            Rect::new(0, 0, 40, 40),
            Rect::new(430, 700, 40, 40),
        ).map_err(|e| e.to_string())?;

        Ok(())
    }
}
