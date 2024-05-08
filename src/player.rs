/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/

use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Player<'a>{
    pub x:i32,
    pub y:i32,
    pub speed:i32,
    pub texture:&'a Texture<'a>,
}
impl Player<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(&self.texture,None,Rect::new(self.x,self.y,52,56)).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn update(&mut self,e: &sdl2::EventPump){
        if e.keyboard_state().is_scancode_pressed(Scancode::A){
            self.x -= self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::D){
            self.x += self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::W){
            self.y -= self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::S){
            self.y += self.speed;
        }
    }
}


