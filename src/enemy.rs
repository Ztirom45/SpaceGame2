use rand::Rng;
/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use sdl2::rect::Rect;
use sdl2::render::Texture;
use rand::thread_rng;
use rand::rngs::ThreadRng;
use soloud::Soloud;
use soloud::Wav;
use soloud::audio;
use crate::config::*;
use crate::paths::*;
use crate::gun::*;

pub struct Enemy<'a>{
    pub rect:Rect,
    pub speed:i32,
    pub lives:i8,
    pub texture:&'a Texture<'a>,
    pub texture_hit:&'a Texture<'a>,
    pub enemy_path:EnemyPath,
    pub motion_counter:u16,
    pub actions:usize,
    pub last_time_shot:u8,
    pub last_time_hit:u8,
}
impl Enemy<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        
        if self.last_time_hit <= HIT_SHOW_DELAY{
            canvas.copy(
                &self.texture_hit,
                None,
                self.rect,
            ).map_err(|e| e.to_string())?;
        }else{
            canvas.copy(
                &self.texture,
                None,
                self.rect,
            ).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
    pub fn update(&mut self,player_shots:&mut Vec<Shot>,own_shots:&mut EnemyShots,rng:&mut ThreadRng){
        //move
        match self.enemy_path.data[self.actions].direction{
               Direction::Up=>self.rect.y -= self.speed,
               Direction::Down=>self.rect.y += self.speed,
               Direction::Right=>self.rect.x += self.speed,
               Direction::Left=>self.rect.x -= self.speed,
        }
        //shot

        if (self.last_time_shot > SHOT_SPAWN_DELAY_ENEMY) && (rng.gen_range(0..20) == 0){
            own_shots.shot(self.rect.x,self.rect.y);
            self.last_time_shot = 0;
        }

        if self.last_time_shot <= SHOT_SPAWN_DELAY_ENEMY{
            self.last_time_shot += 1;
        }
        //colision with player shots and dying
        let shots_len = player_shots.len();
        player_shots.retain(|i| self.rect.contains_rect(i.rect)==false);
        let damage = (shots_len-player_shots.len()) as i8;
        if damage > 0{
            self.lives -= damage;
            self.last_time_hit = 0
        }else{
            if self.last_time_hit <= HIT_SHOW_DELAY{
                self.last_time_hit+=1
            }
        }
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
    pub textures:Vec<&'a Texture<'a>>,
    pub sounds:Vec<&'a audio::Wav>,
}

impl Formation<'_>{
    pub fn init(&mut self){
        for i in 0..10{
            self.enemys.push(Enemy{
                rect: Rect::new(i*50+150,100,ENEMY_W,ENEMY_H),
                speed:2,
                lives:3,
                texture:self.textures[0],
                texture_hit:self.textures[1],
                enemy_path:EnemyPath{data:Vec::new()},
                motion_counter:0,
                actions:0,
                last_time_shot:0,
                last_time_hit:0,
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

    pub fn update(&mut self,shots: &mut Vec<Shot>,own_shots: &mut EnemyShots,rng: &mut ThreadRng,sl:&mut Soloud){
        self.enemys.iter_mut().for_each(
            |enemy| enemy.update(shots,own_shots,rng)
        );
        //remove enemys witout the screen
        let enemys_len_befor = self.enemys.len();
        self.enemys.retain(|i| (i.lives) > 0);
        if enemys_len_befor>self.enemys.len(){
            sl.play(self.sounds[0]);
        }
    }
   
}
