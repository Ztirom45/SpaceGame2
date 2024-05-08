/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use rand::Rng;

use crate::config::*;

pub struct Star{
    x:i32,
    y:i32,
    size:f32,
}
pub struct Sky{
    pub stars:Vec<Star>
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
            star.y += (STARS_SPEED*star.size) as i32;

        }
        //remove star witout the screen
        self.stars.retain(|i| (i.y as u32) < SCREEN_SIZE);
        self.stars.push(Star{x:rand::thread_rng().gen_range(0..(SCREEN_SIZE as i32)-(MAX_STAR_SIZE as i32)),
        y:0,
        size:rand::thread_rng().gen_range(MIN_STAR_SIZE..MAX_STAR_SIZE)});
    }
}
