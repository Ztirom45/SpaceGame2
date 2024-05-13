/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use soloud::*;


mod sky;
mod config;
mod player;
mod gun;
mod paths;
mod enemy;
mod menu;
use crate::config::*;
use crate::sky::*;
use crate::player::*;
use crate::gun::*;
use crate::enemy::*;
use crate::menu::*;

/*
//Just for debugug pourposes
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}*/


fn main() -> Result<(), String> /*Error Handling*/{
    //inititlizing SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rusty SpaceGame", SCREEN_SIZE, SCREEN_SIZE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?; 
    let texture_creator = canvas.texture_creator();
    
    //loads texture by name into the hashmap
    //use it by borrowing &img["texture name"]
    let mut img:HashMap<String, Texture> = HashMap::new();
    for file in fs::read_dir("rsc/img").unwrap() {
        let path = file.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let name:String = name[0..name.len()-4].to_string();//remove .bmp
        let path = path.to_str().unwrap().to_string();
        
        let surface = Surface::load_bmp(path.clone()).map_err(|e| e.to_string())?;
        img.entry(name.clone()).or_insert(
            Texture::from_surface(&surface, &texture_creator).unwrap()
        );
    }
    //audio

    let mut sl = Soloud::default().unwrap();
    let mut sound:HashMap<String, audio::Wav> = HashMap::new();
    for file in fs::read_dir("rsc/sound").unwrap() { 
        let path = file.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let name:String = name[0..name.len()-4].to_string();//remove .bmp
        let path = path.to_str().unwrap().to_string();
       
        sound.entry(name.clone()).or_insert(
            audio::Wav::from_path(path).unwrap()
        );


    } 
    //random
    let mut rng = rand::thread_rng();
    //objects
    let mut menu = Menu{
        image_buttons:&img["Buttons"],
        image_mode1:&img["NormalMode"],
    };
    let mut menu_active:bool = true;

    let mut sky:Sky = Sky{stars:Vec::new()};
    let mut player:Player = Player{
        rect:Rect::new(400,400,PLAYER_W,PLAYER_H),
        speed:10,
        texture:&img["Ship"],
        gun:Gun{
            shots:Vec::new(),
            texture:&img["Shot"],
            last_time_shot:0,
            sound:&sound["laser"],
        },
        lives:PLAYER_LIVES,
        sound_hit:&sound["hit"],

    };
    let mut formations:Formations = Formations{
        formations:Vec::new(),
        formation_number:0,
        texture_enemy:&img["Enemy"],
        texture_enemy2:&img["Enemy2"],
        texture_enemy_hit:&img["Enemy_hit"],
        sound_enemy_die:&sound["explosion"],
    
    };
    let mut enemy_shots = EnemyShots{
        shots:Vec::new(),
        texture:&img["Shot"],
        sound_shot:&sound["laser2"],
    };

    formations.init();
    //debuging:   
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown{
                    keycode: Some(Keycode::Space),
                    ..
                }=> menu_active = false,
                _ => {}
            }

        }
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        
        if menu_active{
            sky.update();
            sky.draw(&mut canvas).unwrap();
            menu.main(&mut canvas).unwrap();
        }else{
            //procces objects
            sky.update();
            player.update(&event_pump,&mut enemy_shots.shots,&mut sl);
            enemy_shots.update();
            formations.update(&mut player.gun.shots,&mut enemy_shots,&mut rng,&mut sl);
            //cheak for end of game
            if player.lives<=0{
                println!("Lose");
                break 'running;
            }
            if formations.formations.len() == formations.formation_number{
                println!("Win");
                break 'running;
            }
            //draw objects
            sky.draw(&mut canvas).unwrap();
            player.draw(&mut canvas).unwrap();
            formations.draw(&mut canvas).unwrap();
            enemy_shots.draw(&mut canvas).unwrap();


        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    
    Ok(())
}
