use rand::prelude::*;

pub struct Board{
    width: usize,
    height: usize,
    state: Vec<Vec<u8>>
}

impl Board{
    pub fn new(width: usize, height: usize)->Self{
        return Self{width, height, state: vec![vec![0;width];height]};
    }

    pub fn render(&self){
        print!("\x1B[2J\x1B[1;1H");
        for y in 0..self.height{
            for x in 0..self.width{
                if self.state[y][x]==1 {print!("# ")} else {print!("  ")}
            }
            println!();
        }
    }
    pub fn random_state(&mut self){
        let mut rng = rand::thread_rng();
        let mut state = vec![vec![0;self.width]; self.height];
        for y in 0..self.height{
            for x in 0..self.width{
                if rng.gen::<f32>()>0.9 {state[y][x]=1}
            }
        }
        self.state = state;
    }

    pub fn next_state(&mut self){
        //let old_state = self.state.clone();
        let new_state = vec![vec![0;self.width];self.height];
        for y in 1..self.height-1{
            for x in 1..self.width-1{
                self.check_nbrs();
            }
        }
    }

    fn check_nbrs(&self){

    }
}
