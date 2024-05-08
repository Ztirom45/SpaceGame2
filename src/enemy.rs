/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::config::*;

pub struct Enemy<'a>{
    pub x:i32,
    pub y:i32,
    pub speed:i32,
    pub lives:u8,
    pub texture:&'a Texture<'a>,
    pub motion_counter:u8,
}
impl Enemy<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(
            &self.texture,
            None,
            Rect::new(self.x,self.y,ENEMY_W,ENEMY_H)
        ).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn update(&mut self){
        if self.motion_counter < 10{
            self.x -= self.speed;
        }else if self.motion_counter > 15 && self.motion_counter < 25{
            self.x += self.speed;
        }
        self.motion_counter += 1;
        if self.motion_counter > 30{
            self.motion_counter = 0;
        }
    }
}

/*
pub struct Formation<'a>{
    pub enemys:Vec<Enemy<'a>>,
    pub texture:&'a Texture<'a>,
    pub last_time_enemy:u8,
}

impl Formation<'_>{
     pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        for enemy in self.enemys.iter(){
            enemy.draw(canvas).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn update(&mut self){
        for enemy in self.enemys.iter_mut(){
            enemy.update();
        }
        //remove star witout the screen
        self.enemys.retain(|i| (i.y as u32) < SCREEN_SIZE);
        self.last_time_enemy += 1;
    }

    pub fn enemy(&mut self, x:i32,y:i32){
        if self.last_time_enemy > enemy_SPAWN_DELAY{
            self.enemys.push(Enemy{x:x+enemy_SPWAN_OFFSET,y,speed:enemy_START_SPEED,texture:self.texture});
            self.enemys.push(Enemy{
                x,
                y,
                speed:enemy_START_SPEED,
                texture:self.texture,
            });
            self.last_time_enemy = 0;
        }   
    }
   
}
*/