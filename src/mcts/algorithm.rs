use super::*;

impl<GameState> Arena<GameState> {

    pub fn add_children(&mut self, node: usize) {
        
        let available_moves = self[node].game_state.available_moves().into_iter();
        
        for gs in available_moves {
            //let child_id = self.next_id();
            self.child_from_with_gamestate(node, gs)
            //self.rollout(child_id);
        }
        
    }
    
        fn rollout(&mut self, node: usize) {
            self[node].score = self[node].game_state.terminate();
            self[node].visits += 1;
            self.propagate_from(node);
        }
    
        pub fn propagate_from(&mut self, node: usize) {
            assert!(&self[node].is_leaf());
             
            let scr = self[node].score;
            let mut node = node;
    
            loop {
                match self[node].parent {
                    Some(parent) => {
                        self[parent].score += scr;
                        self[parent].visits += 1;
                        node = self[parent].id;
                    }
                    None => break
                }
            }
        }
    
    pub fn iterate(&mut self, n: usize) {
        
        let mut current = 0;
        
        for _ in 0..n {
    
            print!("*");
    
            while !self[current].is_leaf() {
                current = self[current].children.iter()
                    .max_by_key(|child|self.ucb1_of(**child)).unwrap().to_owned();
            }
            
            match self[current].visits {
                0 => {
                    self.rollout(current);
                    current = 0;
                }
                1 => {
                    self.add_children(current);
                    current = 0;
                }
                _ => panic!(),
            }
        }
    }

}


