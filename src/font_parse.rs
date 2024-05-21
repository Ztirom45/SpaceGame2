use sdl2::{rect::Rect, render::Texture};

pub struct Text<'a>{
    pub texture:Texture<'a>,
    pub rect:Rect,
}
pub enum Gamemode{
    Normal,
    Hard,
}

pub struct MenuText<'a>{
    pub texture:Texture<'a>,
    pub rect:Rect,
    pub gamemode:Gamemode,
}
