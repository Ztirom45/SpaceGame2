/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::config::*;
use crate::paths::*;
use crate::gun::*;

pub struct Enemy<'a>{
    pub rect:Rect,
    pub speed:i32,
    pub lives:i8,
    pub texture:&'a Texture<'a>,
    pub enemy_path:EnemyPath,
    pub motion_counter:u16,
    pub actions:usize,
    pub last_time_shot:u8,
}
impl Enemy<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(
            &self.texture,
            None,
            self.rect,
        ).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn update(&mut self,player_shots:&mut Vec<Shot>,own_shots:&mut EnemyShots){
        //move
        match self.enemy_path.data[self.actions].direction{
               Direction::Up=>self.rect.y -= self.speed,
               Direction::Down=>self.rect.y += self.speed,
               Direction::Right=>self.rect.x += self.speed,
               Direction::Left=>self.rect.x -= self.speed,
        }
        //shot

        if self.last_time_shot > SHOT_SPAWN_DELAY_ENEMY{
            own_shots.shot(self.rect.x,self.rect.y);
            self.last_time_shot = 0;
        }

        if self.last_time_shot <= SHOT_SPAWN_DELAY_ENEMY{
            self.last_time_shot += 1;
        }
        //colision with player shots and dying
        let shots_len = player_shots.len();
        player_shots.retain(|i| self.rect.contains_rect(i.rect)==false);
        self.lives -= (shots_len-player_shots.len()) as i8;
        self.motion_counter += 1;
        if self.motion_counter > self.enemy_path.data[self.actions].time{
            self.motion_counter = 0;
            self.actions += 1;
            if self.actions >= self.enemy_path.data.len(){
                self.actions = 0;
            }
        }
        
    }
}


pub struct Formation<'a>{
    pub enemys:Vec<Enemy<'a>>,
    pub texture:&'a Texture<'a>,
}

impl Formation<'_>{
    pub fn init(&mut self){
        for i in 0..10{
            self.enemys.push(Enemy{
                rect: Rect::new(i*50+150,100,ENEMY_W,ENEMY_H),
                speed:2,
                lives:10,
                texture:self.texture,
                enemy_path:EnemyPath{data:Vec::new()},
                motion_counter:0,
                actions:0,
                last_time_shot:0,
            });
            self.enemys[i as usize].enemy_path.make_std();

        }
    } 
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        for enemy in self.enemys.iter(){
            enemy.draw(canvas).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn update(&mut self,shots: &mut Vec<Shot>,own_shots: &mut EnemyShots){
        self.enemys.iter_mut().for_each(
            |enemy| enemy.update(shots,own_shots)
        );
        //remove star witout the screen
        self.enemys.retain(|i| (i.lives) > 0);
    }
   
}
