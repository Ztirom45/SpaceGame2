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
        canvas.copy(
            &self.texture,
            None,
            Rect::new(self.x,self.y,SHOT_W,SHOT_H)
        ).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn update(&mut self){
        self.y -= self.speed;
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
        for shot in self.shots.iter_mut(){
            shot.update();
        }
        //remove star witout the screen
        self.shots.retain(|i| (i.y as u32) < SCREEN_SIZE);
        self.last_time_shot += 1;
    }

    pub fn shot(&mut self, x:i32,y:i32){
        if self.last_time_shot > SHOT_SPAWN_DELAY{
            self.shots.push(Shot{x:x+SHOT_SPWAN_OFFSET,y,speed:SHOT_START_SPEED,texture:self.texture});
            self.shots.push(Shot{
                x,
                y,
                speed:SHOT_START_SPEED,
                texture:self.texture,
            });
            self.last_time_shot = 0;
        }   
    }
   
}
