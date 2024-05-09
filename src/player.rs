/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/

use std::vec;

use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::render::Texture;

use crate::config::*;
use crate::gun::*;

pub struct Player<'a>{
    pub rect:Rect,
    pub speed:i32,
    pub texture:&'a Texture<'a>,
    pub gun:Gun<'a>,
    pub lives:i8,
}
impl Player<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(
            &self.texture,
            None,
            self.rect
        ).map_err(|e| e.to_string())?;
        self.gun.draw(canvas).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn update(&mut self,e: &sdl2::EventPump,enemy_shots:&mut Vec<Shot>){
        if e.keyboard_state().is_scancode_pressed(Scancode::A){
            self.rect.x -= self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::D){
            self.rect.x += self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::W){
            self.rect.y -= self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::S){
            self.rect.y += self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::Space){
            self.gun.shot(self.rect.x+SHOT_SPWAN_OFFSET,self.rect.y);
            self.gun.shot(
                self.rect.x+(PLAYER_W as i32)-(SHOT_W as i32)-SHOT_SPWAN_OFFSET,
                self.rect.y
            );
        }            
        self.gun.update(); 

    //cheak for damage
    let shots_len = enemy_shots.len();
    enemy_shots.retain(|i| self.rect.contains_rect(i.rect)==false);
    self.lives -= (shots_len-enemy_shots.len()) as i8;
    }
}


