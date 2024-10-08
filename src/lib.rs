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

    fn get_population(&self)->u64{
        let mut pop = 0;
        for v in &self.state{
            for c in v{
                match c{
                    Cell::Alive => pop+=1,
                    Cell::Dead => ()
                }
            }
        }
        return pop;
    }

    pub fn render(&self, i:u32){
        //clears the screen
        print!("\x1B[2J\x1B[1;1H");
        print!("generation: {}\n", i);
        print!("population: {}\n", self.get_population());
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
                if rng.gen::<f32>()>0.8 {state[y][x]=Cell::Alive}
            }
        }
        self.state = state;
    }

    pub fn next_state(&mut self){
        let mut new_state = self.state.clone();
        //let mut new_state = vec![vec![0;self.width];self.height];
        let corners = [[0,0], [0,self.width-1], [self.height-1,0], [self.height-1, self.width-1]];

        for corner in corners{
            let x = corner[1];
            let y = corner[0];
            new_state[y][x] = self.check_corners(x, y);
        }

        for y in 1..self.height-1{
            let x = 0;
            new_state[y][x] = self.check_edges(x, y);
            let x = self.width-1;
            new_state[y][x] = self.check_edges(x, y);
        }

        for x in 1..self.width-1{
            let y = 0;
            new_state[y][x] = self.check_edges(x, y);
            let y = self.height-1;
            new_state[y][x] = self.check_edges(x, y);
        }

        for y in 1..self.height-1{
            for x in 1..self.width-1{
                new_state[y][x] = self.check_cent_nghbrs(x, y);
            }
        }
        self.state = new_state;
    }

    fn check_edges(&self, x:usize, y:usize)->Cell{
        let mut alive_nbors = 0;
        if y>0 && y<self.height-1{
            if x==0{
                if self.state[y-1][x]==Cell::Alive {alive_nbors+=1}
                if self.state[y+1][x]==Cell::Alive {alive_nbors+=1}
                if self.state[y][x+1]==Cell::Alive {alive_nbors+=1}
                if self.state[y-1][x+1]==Cell::Alive {alive_nbors+=1}
                if self.state[y+1][x+1]==Cell::Alive {alive_nbors+=1}
            }
            if x==self.width-1{
                if self.state[y-1][x]==Cell::Alive {alive_nbors+=1}
                if self.state[y+1][x]==Cell::Alive {alive_nbors+=1}
                if self.state[y][x-1]==Cell::Alive {alive_nbors+=1}
                if self.state[y-1][x-1]==Cell::Alive {alive_nbors+=1}
                if self.state[y+1][x-1]==Cell::Alive {alive_nbors+=1}
            }
        }
        if x>0 && x<self.width-1{
            if y==0{
                if self.state[y][x-1]==Cell::Alive {alive_nbors+=1}
                if self.state[y+1][x-1]==Cell::Alive {alive_nbors+=1}
                if self.state[y+1][x]==Cell::Alive {alive_nbors+=1}
                if self.state[y+1][x+1]==Cell::Alive {alive_nbors+=1}
                if self.state[y][x+1]==Cell::Alive {alive_nbors+=1}
            }
            if y==self.height-1{
                if self.state[y][x-1]==Cell::Alive {alive_nbors+=1}
                if self.state[y-1][x-1]==Cell::Alive {alive_nbors+=1}
                if self.state[y-1][x]==Cell::Alive {alive_nbors+=1}
                if self.state[y-1][x+1]==Cell::Alive {alive_nbors+=1}
                if self.state[y][x+1]==Cell::Alive {alive_nbors+=1}
            }
        }
        return match self.state[y][x]{
            Cell::Alive => {
                if alive_nbors<2 {Cell::Dead}
                else if alive_nbors==2 || alive_nbors==3 {Cell::Alive}
                else {Cell::Dead}
            },
            Cell::Dead => {
                if alive_nbors==3 {Cell::Alive}
                else {Cell::Dead}
            }
        };
    }

    fn check_corners(&self, x:usize, y:usize)->Cell{
        let mut alive_nbrs = 0;

        if x==0 && y==0{
            if &self.state[y][x+1]==&Cell::Alive{alive_nbrs+=1}
            if &self.state[y+1][x]==&Cell::Alive{alive_nbrs+=1}
            if &self.state[y+1][x+1]==&Cell::Alive{alive_nbrs+=1}
        } else if x==self.width-1 && y==0{
            if &self.state[y][x-1]==&Cell::Alive{alive_nbrs+=1}
            if &self.state[y+1][x]==&Cell::Alive{alive_nbrs+=1}
            if &self.state[y+1][x-1]==&Cell::Alive{alive_nbrs+=1}
        } else if x==self.width-1 && y==self.height-1{
            if &self.state[y][x-1]==&Cell::Alive{alive_nbrs+=1}
            if &self.state[y-1][x]==&Cell::Alive{alive_nbrs+=1}
            if &self.state[y-1][x-1]==&Cell::Alive{alive_nbrs+=1}
        } else if x==0 && y==self.height-1{
            if &self.state[y][x+1]==&Cell::Alive{alive_nbrs+=1}
            if &self.state[y-1][x]==&Cell::Alive{alive_nbrs+=1}
            if &self.state[y-1][x+1]==&Cell::Alive{alive_nbrs+=1}
        }
        
        return match alive_nbrs{
            _ => Cell::Dead
        };
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
