/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::config::*;
use crate::gun::*;

pub enum Direction{  
    Right,
    Down,
    Left,
    Up,
}
pub struct Step{
    direction:Direction,
    time:u16,
}

pub struct EnemyPath{
    data:Vec<Step>,
}

impl EnemyPath {
    pub fn make_std(&mut self){
        self.data.push(Step{direction:Direction::Right,time:20u16});
        self.data.push(Step{direction:Direction::Down,time:20u16});
        self.data.push(Step{direction:Direction::Left,time:20u16});
        self.data.push(Step{direction:Direction::Up,time:20u16});
    }
}

pub struct Enemy<'a>{
    pub rect:Rect,
    pub speed:i32,
    pub lives:i8,
    pub texture:&'a Texture<'a>,
    pub enemy_path:EnemyPath,
    motion_counter:u16,
    actions:usize,
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
    pub fn update(&mut self,shots_referece:&mut Vec<Shot>){
        match self.enemy_path.data[self.actions].direction{
               Direction::Up=>self.rect.y -= self.speed,
               Direction::Down=>self.rect.y += self.speed,
               Direction::Right=>self.rect.x += self.speed,
               Direction::Left=>self.rect.x -= self.speed,
        }

        /*
        if self.motion_counter < 20{
            self.rect.x -= self.speed;
        }else if self.motion_counter > 20 && self.motion_counter < 40{
            self.rect.y += self.speed;
        }else if self.motion_counter > 40 && self.motion_counter < 60{
            self.rect.x += self.speed;
        }else if self.motion_counter > 60 && self.motion_counter < 80{ 
            self.rect.y -= self.speed;
        }*/

        let shots_len = shots_referece.len();
        shots_referece.retain(|i| self.rect.contains_rect(i.rect)==false);
        self.lives -= (shots_len-shots_referece.len()) as i8;
        self.motion_counter += 1;
        if self.motion_counter > self.enemy_path.data[self.actions].time{
            self.motion_counter = 0;
            self.actions += 1;
            println!("{}",self.enemy_path.data.len());
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

    pub fn update(&mut self,shots: &mut Vec<Shot>){
        self.enemys.iter_mut().for_each(
            |enemy| enemy.update(shots)
        );
        //remove star witout the screen
        self.enemys.retain(|i| (i.lives) > 0);
    }
   
}
