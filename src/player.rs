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
use crate::font_parse::Gamemode;
use crate::gun::*;

pub struct Player<'a>{
    pub rect:Rect,
    pub speed:i32,
    pub texture:&'a Texture<'a>,
    pub texture_heart:&'a Texture<'a>,
    pub texture_heart_off:&'a Texture<'a>,
    pub gun:Gun<'a>,
    pub lives:i8,
    pub max_lives:i8,
    pub sound_hit:&'a audio::Wav,
}
impl Player<'_>{
    pub fn init(&mut self,mode:&Gamemode){
        self.lives = match mode{
            Gamemode::Normal => PLAYER_LIVES,
            Gamemode::Hard => PLAYER_LIVES_HARD,
        };
        self.max_lives = self.lives;
    }
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(
            &self.texture,
            None,
            self.rect
        ).map_err(|e| e.to_string())?;
        self.gun.draw(canvas).map_err(|e| e.to_string())?;
        //draw lives: TODO: pretier
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.set_draw_color(Color::RGB(0,255, 0));
        let texture = |condition| if condition{
            &self.texture_heart
        }else{
            &self.texture_heart_off
        };
        for i in 1..self.max_lives+1{
            canvas.copy(
                texture(i<=self.lives),
                None,
                Rect::new(800-(i as i32)*24,0,20,20)
            ).map_err(|e| e.to_string())?; 

        }

        Ok(())
    }
    pub fn update(&mut self,e: &sdl2::EventPump,enemy_shots:&mut Vec<Shot>,sl:&mut Soloud){
        if (e.keyboard_state().is_scancode_pressed(Scancode::A)
        ||e.keyboard_state().is_scancode_pressed(Scancode::H))
        && self.rect.x >0{
            self.rect.x -= self.speed;
        }
        if (e.keyboard_state().is_scancode_pressed(Scancode::D)
        ||e.keyboard_state().is_scancode_pressed(Scancode::L))
        && self.rect.x < (SCREEN_SIZE as i32-self.rect.w as i32){
            self.rect.x += self.speed;
        }
        if (e.keyboard_state().is_scancode_pressed(Scancode::W)
        ||e.keyboard_state().is_scancode_pressed(Scancode::K))
        && self.rect.y >0{
            self.rect.y -= self.speed;
        }
        if (e.keyboard_state().is_scancode_pressed(Scancode::S)
        ||e.keyboard_state().is_scancode_pressed(Scancode::J))
        && self.rect.y < (SCREEN_SIZE as i32-self.rect.h as i32){
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


