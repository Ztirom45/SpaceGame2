/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::config::*;
use crate::gun::*;

pub struct Enemy<'a>{
    pub rect:Rect,
    pub speed:i32,
    pub lives:i8,
    pub texture:&'a Texture<'a>,
    pub motion_counter:u8,
}
impl Enemy<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(
            &self.texture,
            None,
            self.rect,
        ).map_err(|e| e.to_string())?;
        println!("{}",self.lives);
        Ok(())
    }
    pub fn update(&mut self,shots_referece:&mut Vec<Shot>){
        /*
        if self.motion_counter < 10{
            self.rect.x -= self.speed;
        }else if self.motion_counter > 15 && self.motion_counter < 25{
            self.rect.x += self.speed;
        }*/

        let shots_len = shots_referece.len();
        shots_referece.retain(|i| self.rect.contains_rect(i.rect)==false);
        self.lives -= (shots_len-shots_referece.len()) as i8;
        self.motion_counter += 1;
        if self.motion_counter > 30{
            self.motion_counter = 0;
        }
    }
}


pub struct Formation<'a>{
    pub enemys:Vec<Enemy<'a>>,
}

impl Formation<'_>{
     pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        for enemy in self.enemys.iter(){
            enemy.draw(canvas).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn update(&mut self,shots: &mut Vec<Shot>){
        self.enemys.iter_mut().for_each(
            |enemy| enemy.update(shots)
        );
        //remove star witout the screen
        self.enemys.retain(|i| (i.lives) > 0);
    }
   
}
