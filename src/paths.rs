
pub enum Direction{  
    Right,
    Down,
    Left,
    Up,
}
pub struct Step{
    pub direction:Direction,
    pub time:u16,
}

pub struct EnemyPath{
    pub data:Vec<Step>,
}

impl EnemyPath {
    pub fn make_std(&mut self){
        self.data.push(Step{direction:Direction::Right,time:20u16});
        self.data.push(Step{direction:Direction::Down,time:20u16});
        self.data.push(Step{direction:Direction::Left,time:20u16});
        self.data.push(Step{direction:Direction::Up,time:20u16});
    }
}
