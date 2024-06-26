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
use std::thread::sleep;


mod sky;
mod config;
mod player;
mod gun;
mod paths;
mod enemy;
mod menu;
mod font_parse;
use crate::config::*;
use crate::sky::*;
use crate::player::*;
use crate::gun::*;
use crate::enemy::*;
use crate::menu::*;
use crate::font_parse::*;

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
    //Fonts
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttf_context.load_font("rsc/font/FreeSans.ttf",128).unwrap();
    let font_color = Color::RGB(255, 255, 255);
    let surface_font_normal = font
        .render("Normal Mode")
        .blended(font_color)
        .map_err(|e| e.to_string())?;
    let text_normal_size = surface_font_normal.size();
    let surface_font_hard = font
        .render("Hard Mode")
        .blended(font_color)
        .map_err(|e| e.to_string())?;
    let text_hard_size = surface_font_hard.size();
    let surface_font_help = font
        .render("Return to start / [W,S], to select modes / controll with [W,A,S,D,SPACE]")
        .blended(font_color)
        .map_err(|e| e.to_string())?;
    let text_help_size = surface_font_help.size();
    
    //random
    let mut rng = rand::thread_rng();
    //objects
    let mut menu = Menu{
        image_mode_background:&img["Background"],
        modes:vec![
            MenuText{
            texture:texture_creator.create_texture_from_surface(surface_font_normal).unwrap(),
            rect:Rect::new(0,0,//initialized later
                text_normal_size.0/FONT_SCALE_FAKTOR,
                text_normal_size.1/FONT_SCALE_FAKTOR,
            ),
            gamemode:Gamemode::Normal,
            },
            MenuText{
            texture:texture_creator.create_texture_from_surface(surface_font_hard).unwrap(),
            rect:Rect::new(0,0,//initialized later
                text_hard_size.0/FONT_SCALE_FAKTOR,
                text_hard_size.1/FONT_SCALE_FAKTOR,
            ),
            gamemode:Gamemode::Hard,
            },
        ],
        selcted_mode:0,
        help_text:Text{
            texture:texture_creator.create_texture_from_surface(surface_font_help).unwrap(),
            rect:Rect::new(0,0,//initialized later
                text_help_size.0/HELP_TEXT_SCALE_FACTOR,
                text_help_size.1/HELP_TEXT_SCALE_FACTOR,
            )
        },

    };
    menu.init();
    let mut menu_active:bool = true;

    let mut sky:Sky = Sky{stars:Vec::new()};
    let mut player:Player = Player{
        rect:Rect::new(400,400,PLAYER_W,PLAYER_H),
        speed:10,
        texture:&img["Ship"],
        texture_heart:&img["LiveOn"],
        texture_heart_off:&img["LiveOff"],
        gun:Gun{
            shots:Vec::new(),
            texture:&img["Shot"],
            last_time_shot:0,
            sound:&sound["laser"],
        },
        lives:0,//initizilized later
        max_lives:0,//"
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
        texture:&img["Shot2"],
        sound_shot:&sound["laser2"],
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
                Event::KeyDown{
                    keycode: Some(Keycode::Return),
                    ..
                }=>{
                    if menu_active{
                        player.init(menu.get_gamemode());
                        formations.init();
                    }

                    menu_active = false;
                },
                _ => {}
            }

        }
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        
        if menu_active{
            sky.update();
            sky.draw(&mut canvas).unwrap();
            menu.main(&mut canvas,&event_pump).unwrap();
            canvas.set_draw_color(Color::RGBA(195, 217, 255, 255)); 
        }else{
            //procces objects
            sky.update();
            player.update(&event_pump,&mut enemy_shots.shots,&mut sl);
            enemy_shots.update();
            formations.update(&mut player.gun.shots,&mut enemy_shots,&mut rng,&mut sl);
            //cheak for end of game
            if player.lives<=0{
                menu_active = true;
                canvas.copy(&img["Lose"],None,None).unwrap();
                canvas.present();
                sleep(Duration::from_millis(300));
                //break 'running;
            }
            if formations.formations.len() == formations.formation_number{
                //break 'running;
                menu_active = true;
                canvas.copy(&img["Win"],None,None).unwrap();
                canvas.present();
                sleep(Duration::from_millis(300));

 
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
