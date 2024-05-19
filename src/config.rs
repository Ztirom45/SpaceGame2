/*
Code written by Ztirom45
LICENSE: GPL4
contact: https://github.com/Ztirom45
*/

pub const SCREEN_SIZE:u32 = 800;
pub const MIN_STAR_SIZE:f32 = 2.0;
pub const MAX_STAR_SIZE:f32 = 6.0;

//stars
pub const STARS_SPEED:f32     = 2.5;
//player
pub const PLAYER_W:u32   = 52;
pub const PLAYER_H:u32   = 56;
pub const PLAYER_LIVES:i8 = 10;

//gun
pub const SHOT_W:u32 = 3;
pub const SHOT_H:u32 = 14;
pub const SHOT_SPWAN_OFFSET:i32 = 16;
pub const SHOT_SPAWN_DELAY:u8 = 4;
pub const SHOT_START_SPEED:i32 = 12;

//Enemy
pub const ENEMY_W:u32 = 19*2;
pub const ENEMY_H:u32 = 21*2;
pub const SHOT_SPAWN_DELAY_ENEMY:u8 = 10;
pub const HIT_SHOW_DELAY:u8 = 2;

//Menu

//font
pub const FONT_SCALE_FAKTOR:u32 = 4;
pub const FONT_Y_DISTANCE:i32 = 120/FONT_SCALE_FAKTOR as i32;
