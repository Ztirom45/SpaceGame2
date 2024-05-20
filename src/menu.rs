/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/

use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::font_parse::*;
use crate::config::*;


pub struct Menu<'a>{
    pub image_mode_background:&'a Texture<'a>,
    pub modes:Vec<Text<'a>>,
    pub selcted_mode:usize,
}
impl Menu<'_>{
    pub fn init(&mut self){
        let mut ypos:i32 = 150;
        for i in self.modes.iter_mut(){
            i.rect.x = 200;
            i.rect.y = ypos;
            ypos += FONT_Y_DISTANCE as i32;
        }
    }
    pub fn main(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        e: &EventPump
    ) -> Result<(), String>{ 
        //update cursor 
        if e.keyboard_state().is_scancode_pressed(Scancode::S) && self.selcted_mode+1<self.modes.len(){
            self.selcted_mode += 1;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::W) && self.selcted_mode > 0{
            self.selcted_mode -= 1;
        }
        //draw stuff
        canvas.copy(
            &self.image_mode_background,
            None,
            Rect::new(144, 144, 512, 512),
        ).map_err(|e|e.to_string())?;
        
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.fill_rect(
            Rect::new(
                200,
                165+(self.selcted_mode as i32)*FONT_Y_DISTANCE as i32,
                400,
                FONT_Y_DISTANCE
            )
        ).map_err(|e| e.to_string())?;
        
        for mode in self.modes.iter(){ 
           canvas.copy(
                &mode.texture,
                None,
                mode.rect
            ).map_err(|e|e.to_string())?;
       }
        Ok(())
    }
}
