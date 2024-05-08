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
use std::collections::HashMap;
use std::fs;
use std::time::Duration;

mod sky;
mod config;
mod player;
mod gun;
mod enemy;
use crate::config::*;
use crate::sky::*;
use crate::player::*;
use crate::gun::*;
use crate::enemy::*;

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
    
    //data
    let mut sky:Sky = Sky{stars:Vec::new()};
    let mut player:Player = Player{
        x:400,
        y:400,
        speed:10,
        texture:&img["Ship"],
        gun:Gun{
            shots:Vec::new(),
            texture:&img["Shot"],
            last_time_shot:0u8
        },
    };
    let mut enemy:Enemy = Enemy{
        x:400,
        y:100,
        speed:7,
        lives:1,
        texture:&img["Enemy"],
        motion_counter:0
    };
    //debuging:   
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.x -= 1;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        sky.draw(&mut canvas).unwrap();
        player.draw(&mut canvas).unwrap();
        enemy.draw(&mut canvas).unwrap();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        //procces stars
        sky.update();
        player.update(&event_pump);
        enemy.update();
    }
    
    Ok(())
}
