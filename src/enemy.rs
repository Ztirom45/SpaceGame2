/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
use sdl2::rect::Rect;
use sdl2::render::Texture;
use soloud::Soloud;
use soloud::audio;
use rand::{Rng,rngs::ThreadRng};
use crate::paths::*;
use crate::gun::*;
use crate::config::*;

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
    pub shot_probability:u8,//1:shoot_probability
}
impl Enemy<'_>{
    pub fn new_enemy1<'a>(x:i32,y:i32,texture:&'a Texture<'a>,texture_hit:&'a Texture<'a>) -> Enemy<'a>{
        Enemy{
            rect: Rect::new(x,y,ENEMY_W,ENEMY_H),
            speed:2,
            lives:3,
            texture,
            texture_hit,
            enemy_path:EnemyPath::new_std(),
            motion_counter:0,
            actions:0,
            last_time_shot:0,
            last_time_hit:0,
            shot_probability:20,
        }
    }
    pub fn new_enemy2<'a>(x:i32,y:i32,enemy_path:EnemyPath,texture:&'a Texture<'a>,texture_hit:&'a Texture<'a>) -> Enemy<'a>{
        Enemy{
            rect: Rect::new(x,y,ENEMY_W,ENEMY_H),
            speed:8,
            lives:10,
            texture,
            texture_hit,
            enemy_path,
            motion_counter:0,
            actions:0,
            last_time_shot:0,
            last_time_hit:0,
            shot_probability:10,
        }
    }

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
    pub fn update(&mut self,player_shots:&mut Vec<Shot>,own_shots:&mut EnemyShots,rng:&mut ThreadRng,sl:&mut Soloud){
        //shot

        if (self.last_time_shot > SHOT_SPAWN_DELAY_ENEMY) && (rng.gen_range(0..self.shot_probability) == 0){
            own_shots.shot(self.rect.x,self.rect.y,sl);
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

        //motion
        if self.enemy_path.data.len()>0{
            //move
            match self.enemy_path.data[self.actions].direction{
                   Direction::Up=>self.rect.y -= self.speed,
                   Direction::Down=>self.rect.y += self.speed,
                   Direction::Right=>self.rect.x += self.speed,
                   Direction::Left=>self.rect.x -= self.speed,
            }
            //cheak next motion
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
}


pub struct Formation<'a>{
    pub enemys:Vec<Enemy<'a>>,
    pub sound_enemy_die:&'a audio::Wav,
}

impl Formation<'_>{
    pub fn new<'a>(sound_enemy_die:&'a audio::Wav) -> Formation{
        Formation{
            enemys:Vec::new(),
            sound_enemy_die
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
            |enemy| enemy.update(shots,own_shots,rng,sl)
        );
        //remove enemys witout the screen
        let enemys_len_befor = self.enemys.len();
        self.enemys.retain(|i| (i.lives) > 0);
        if enemys_len_befor>self.enemys.len(){
            sl.play(self.sound_enemy_die);
        }
    }
   
}

pub struct Formations<'a>{
    pub formations:Vec<Formation<'a>>,
    pub formation_number:usize,
    pub texture_enemy:&'a Texture<'a>,
    pub texture_enemy2:&'a Texture<'a>,
    pub texture_enemy_hit:&'a Texture<'a>,
    pub sound_enemy_die:&'a audio::Wav,
}

impl Formations<'_>{
    pub fn init(&mut self){
    self.formations.clear();
    self.formation_number = 0;
//FORMATION 0 -------------------------------------------------------------------------------        
        self.formations.push(Formation::new(self.sound_enemy_die));
        for i in 0..10{
            self.formations[0].enemys.push(Enemy::new_enemy1(i*50+150,100,self.texture_enemy,self.texture_enemy_hit));

        }
//FORMATION 1 -------------------------------------------------------------------------------        
        self.formations.push(Formation::new(self.sound_enemy_die));
        
        for y in 0..2{
            for x in 0..8{
                self.formations[1].enemys.push(Enemy::new_enemy1(x*50+150,100+y*50,self.texture_enemy,self.texture_enemy_hit));            }
        }
//FORMATION 3 -------------------------------------------------------------------------------        
        self.formations.push(Formation::new(self.sound_enemy_die));
        
        for i in 0..8{
            self.formations[2].enemys.push(Enemy::new_enemy1(i*50+200,100,self.texture_enemy,self.texture_enemy_hit));
        }
        self.formations[2].enemys.push(Enemy::new_enemy2(
            300,
            50,
            EnemyPath{
                    data: vec![
                        Step{direction:Direction::Down,time:10},
                        Step{direction:Direction::Left,time:30},
                        Step{direction:Direction::Up,time:10},
                        Step{direction:Direction::Right,time:30},
                    ]
            },
            self.texture_enemy2,
            self.texture_enemy_hit
        ));
        self.formations[2].enemys.push(Enemy::new_enemy2(
            500,
            50,
            EnemyPath{
                    data: vec![
                        Step{direction:Direction::Down,time:10},
                        Step{direction:Direction::Right,time:30},
                        Step{direction:Direction::Up,time:10},
                        Step{direction:Direction::Left,time:30},
                    ]
            },
            self.texture_enemy2,
            self.texture_enemy_hit
        ));
//FORMATION 4 -------------------------------------------------------------------------------        
        self.formations.push(Formation::new(self.sound_enemy_die));
        self.formations[3].enemys.push(Enemy::new_enemy2(
            480,50,
            EnemyPath{
                    data: vec![
                        Step{direction:Direction::Down,time:20},
                        Step{direction:Direction::Left,time:20},
                        Step{direction:Direction::Up,time:20},
                        Step{direction:Direction::Right,time:20},
                    ]
            },
            self.texture_enemy2,
            self.texture_enemy_hit
        ));

        self.formations[3].enemys.push(Enemy::new_enemy2(
            320,50,
            EnemyPath{
                    data: vec![
                        Step{direction:Direction::Right,time:20},
                        Step{direction:Direction::Down,time:20},
                        Step{direction:Direction::Left,time:20},
                        Step{direction:Direction::Up,time:20},
                    ]
            },
            self.texture_enemy2,
            self.texture_enemy_hit
        ));
        self.formations[3].enemys.push(Enemy::new_enemy2(
            320,210,
            EnemyPath{
                    data: vec![
                        Step{direction:Direction::Up,time:20},
                        Step{direction:Direction::Right,time:20},
                        Step{direction:Direction::Down,time:20},
                        Step{direction:Direction::Left,time:20},
                    ]
            },
            self.texture_enemy2,
            self.texture_enemy_hit
        ));
        self.formations[3].enemys.push(Enemy::new_enemy2(
            480,210,
            EnemyPath{
                    data: vec![
                        Step{direction:Direction::Left,time:20},
                        Step{direction:Direction::Up,time:20},
                        Step{direction:Direction::Right,time:20},
                        Step{direction:Direction::Down,time:20},
                    ]
            },
            self.texture_enemy2,
            self.texture_enemy_hit
        ));
        self.formations[3].enemys.push(Enemy::new_enemy2(
            400,130,
            EnemyPath{data:Vec::new()},
            self.texture_enemy2,
            self.texture_enemy_hit
        ));
    }

    pub fn update(&mut self,shots: &mut Vec<Shot>,own_shots: &mut EnemyShots,rng: &mut ThreadRng,sl:&mut Soloud){
        if self.formation_number < self.formations.len(){
            self.formations[self.formation_number].update(shots,own_shots,rng,sl);
            if self.formations[self.formation_number].enemys.len() == 0{
                self.formation_number+=1;
            }
        }
    }
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        
        if self.formation_number < self.formations.len(){
            self.formations[self.formation_number].draw(canvas).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
