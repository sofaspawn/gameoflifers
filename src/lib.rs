use rand::prelude::*;

pub struct Board{
    width: usize,
    height: usize,
    state: Vec<Vec<Cell>>
}

#[derive(Clone, PartialEq)]
pub enum Cell{
    Alive,
    Dead
}

impl Board{
    pub fn new(width: usize, height: usize)->Self{
        return Self{width, height, state: vec![vec![Cell::Dead;width];height]};
    }

    pub fn render(&self){
        //clears the screen
        print!("\x1B[2J\x1B[1;1H");
        for y in 0..self.height{
            for x in 0..self.width{
                match self.state[y][x]{
                    Cell::Alive => {print!("■ ")},
                    Cell::Dead => {print!("□ ")}
                }
                //if self.state[y][x]==1 {print!("■ ")} else {print!("□ ")}
            }
            println!();
        }
    }
    pub fn random_state(&mut self){
        let mut rng = rand::thread_rng();
        let mut state = vec![vec![Cell::Dead;self.width]; self.height];
        for y in 0..self.height{
            for x in 0..self.width{
                if rng.gen::<f32>()>0.9 {state[y][x]=Cell::Alive}
            }
        }
        self.state = state;
    }

    pub fn next_state(&mut self){
        let mut new_state = self.state.clone();
        //let mut new_state = vec![vec![0;self.width];self.height];
        for y in 1..self.height-1{
            for x in 1..self.width-1{
                new_state[y][x] = self.check_cent_nghbrs(x, y);
            }
        }
        self.state = new_state;
    }

    fn check_cent_nghbrs(&self, x:usize, y:usize)->Cell{
        let mut alive_nbrs = 0;

        if &self.state[y][x-1]==&Cell::Alive{alive_nbrs+=1}
        if &self.state[y][x+1]==&Cell::Alive{alive_nbrs+=1}
        if &self.state[y-1][x]==&Cell::Alive{alive_nbrs+=1}
        if &self.state[y+1][x]==&Cell::Alive{alive_nbrs+=1}
        if &self.state[y-1][x-1]==&Cell::Alive{alive_nbrs+=1}
        if &self.state[y+1][x+1]==&Cell::Alive{alive_nbrs+=1}
        if &self.state[y-1][x+1]==&Cell::Alive{alive_nbrs+=1}
        if &self.state[y+1][x-1]==&Cell::Alive{alive_nbrs+=1}

        return match self.state[y][x]{
            Cell::Alive => {
                if alive_nbrs<2 {Cell::Dead}
                else if alive_nbrs==2 || alive_nbrs==3 {Cell::Alive}
                else {Cell::Dead}
            },
            Cell::Dead => {
                if alive_nbrs==3 {Cell::Alive}
                else {Cell::Dead}
            }
        };
    }
}
