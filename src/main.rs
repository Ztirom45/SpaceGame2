extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode,KeyboardState};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use std::time::Duration;

use rand;
use rand::Rng;
//use std::vec::Vec;

//consts
const SCREEN_SIZE:u32 = 800;
const MIN_STAR_SIZE:f32 = 2.0;
const MAX_STAR_SIZE:f32 = 6.0;
const SPEED:f32 = 2.5;

//structs
struct Star{
    x:i32,
    y:i32,
    size:f32,
}
struct Sky{
    stars:Vec<Star>
}
impl Sky{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        // A draw a rectangle which almost fills our window with it !

        for star in self.stars.iter(){
            canvas.fill_rect(Rect::new(star.x, star.y, star.size as u32, star.size as u32)).map_err(|e| e.to_string())?;

        }
        Ok(())
    }

    pub fn update(&mut self){
        for star in self.stars.iter_mut(){
            star.y += (SPEED*star.size) as i32;

        }
        //remove star witout the screen
        self.stars.retain(|i| (i.y as u32) < SCREEN_SIZE);
        self.stars.push(Star{x:rand::thread_rng().gen_range(0..(SCREEN_SIZE as i32)-(MAX_STAR_SIZE as i32)),
        y:0,
        size:rand::thread_rng().gen_range(MIN_STAR_SIZE..MAX_STAR_SIZE)});
    }
}


struct Player<'a>{
    x:i32,
    y:i32,
    speed:i32,
    texture:Texture<'a>,
}
impl Player<'_>{
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.copy(&self.texture,None,Rect::new(self.x,self.y,52,56)).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn update(&mut self,e: &sdl2::EventPump){
        if e.keyboard_state().is_scancode_pressed(Scancode::A){
            self.x -= self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::D){
            self.x += self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::W){
            self.y -= self.speed;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::S){
            self.y += self.speed;
        }
    }
}

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
        .window("rusty starfeld paralax", SCREEN_SIZE, SCREEN_SIZE)
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
    
    //textures
    let surface = Surface::load_bmp("rsc/img/Ship.bmp").map_err(|e| e.to_string())?;
    let prism_fighter = Texture::from_surface(&surface, &texture_creator).unwrap();
    //data
    let mut sky:Sky = Sky{stars:Vec::new()};
    let mut player:Player = Player{x:1,y:1,speed:10,texture:prism_fighter};
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

        sky.draw(&mut canvas).map_err(|e| e.to_string())?;
        player.draw(&mut canvas).map_err(|e| e.to_string())?;
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        //procces stars
        sky.update();
        player.update(&event_pump);
    }
    
    Ok(())
}
