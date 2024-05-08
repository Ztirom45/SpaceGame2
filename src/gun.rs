/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::config::*;

pub struct Shot<'a>{ 
    pub rect:Rect,
    pub speed:i32,
    pub texture:&'a Texture<'a>,
}
impl Shot<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(
            &self.texture,
            None,
            self.rect,
        ).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn update(&mut self){
        self.rect.y -= self.speed;
    }
}

pub struct Gun<'a>{
    pub shots:Vec<Shot<'a>>,
    pub texture:&'a Texture<'a>,
    pub last_time_shot:u8,
}

impl Gun<'_>{
     pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        for shot in self.shots.iter(){
            shot.draw(canvas).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn update(&mut self){
        self.shots.iter_mut().for_each(|shot| shot.update());
        //remove star witout the screen
        self.shots.retain(|i| (i.rect.y as u32) < SCREEN_SIZE);
        self.last_time_shot += 1;
    }

    pub fn shot(&mut self, x:i32,y:i32){
        if self.last_time_shot > SHOT_SPAWN_DELAY{
            self.shots.push(Shot{
                rect:Rect::new(x+SHOT_SPWAN_OFFSET,y,SHOT_W,SHOT_H),
                speed:SHOT_START_SPEED,texture:self.texture});
            self.shots.push(Shot{
                rect:Rect::new(x,y,SHOT_W,SHOT_H,),
                speed:SHOT_START_SPEED,
                texture:self.texture,
            });
            self.last_time_shot = 0;
        }   
    }
   
}
