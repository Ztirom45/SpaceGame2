/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use soloud::Soloud;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use soloud::audio;

use crate::config::*;
use crate::gun::*;

pub struct Player<'a>{
    pub rect:Rect,
    pub speed:i32,
    pub texture:&'a Texture<'a>,
    pub gun:Gun<'a>,
    pub lives:i8,
    pub sound_hit:&'a audio::Wav,
}
impl Player<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(
            &self.texture,
            None,
            self.rect
        ).map_err(|e| e.to_string())?;
        self.gun.draw(canvas).map_err(|e| e.to_string())?;
        //draw lives: TODO: pretier
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Rect::new(650,10, 100, 10)).map_err(|e| e.to_string())?; 
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(650,10, (self.lives*(100/PLAYER_LIVES))as u32, 10)).map_err(|e| e.to_string())?;

        Ok(())
    }
    pub fn update(&mut self,e: &sdl2::EventPump,enemy_shots:&mut Vec<Shot>,sl:&mut Soloud){
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
            self.gun.shot(self.rect.x+SHOT_SPWAN_OFFSET,self.rect.y,sl);
            self.gun.shot(
                self.rect.x+(PLAYER_W as i32)-(SHOT_W as i32)-SHOT_SPWAN_OFFSET,
                self.rect.y,
                sl
            );
        }            
        self.gun.update(); 

        //cheak for damage
        let shots_len = enemy_shots.len();
        enemy_shots.retain(|i| self.rect.contains_rect(i.rect)==false);
        let damage:i8 = (shots_len-enemy_shots.len()) as i8;
        if damage != 0{
            self.lives -= damage;
            sl.play(self.sound_hit);
        }
    }
}


