extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
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

struct Stars{
    stars:Vec<Star>;
}
impl Stars{
    pub fn draw(self){
        
    }

}

fn main() -> Result<(), String> /*Error Handling*/{
    //inititlizing SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rusty starfeald Paralax", SCREEN_SIZE, SCREEN_SIZE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    

    //data
    let mut stars:Vec<Star> = Vec::new();
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        // A draw a rectangle which almost fills our window with it !
        for star in stars.iter(){
            canvas.fill_rect(Rect::new(star.x, star.y, star.size as u32, star.size as u32)).map_err(|e| e.to_string())?;

        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        //procces stars
        for star in stars.iter_mut(){
            star.y += (SPEED*star.size) as i32;

        }
        //remove star witout the screen
        stars.retain(|i| (i.y as u32) < SCREEN_SIZE);
        stars.push(Star{x:rand::thread_rng().gen_range(0..(SCREEN_SIZE as i32)-(MAX_STAR_SIZE as i32)),
        y:0,
        size:rand::thread_rng().gen_range(MIN_STAR_SIZE..MAX_STAR_SIZE)});
    }
    //println!("Hello, world!");
    
    Ok(())
}
