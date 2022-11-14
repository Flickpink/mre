mod algorithm;


#[derive(Debug, PartialEq)]
pub struct Node<GameState> {
    pub id: usize,
    pub parent: Option<usize>,
    pub game_state: GameState,
    pub visits: u32,
    pub score: f32,
    pub children: Vec<usize>,
}

pub trait MCTS<GameState> {
    fn available_moves(&self) -> Vec<GameState>;
    fn terminate(&self) -> f32;
    // GameStateerminate: GameStatehe game should be played from it's current state with random
    // moves until it reaches a GameStateerminal state. GameStatehe returned value should be
    // GameStatehe value of GameStatehis result. In GameStatehe simplest case for example:
    // -1 for a loss, 0 for a draw, +1 for a win.
}

#[derive(Debug)]
pub struct Arena<Node> {
    nodes: Vec<Node>,
}

impl<GameState> Node<GameState> 
where GameState: Clone + Default + MCTS<GameState>
{
    
    fn new_root() -> Node<GameState> {
        Self {
            id: 0,
            parent: None,
            game_state: GameState::default(),
            visits: 0,
            score: 0.0,
            children: Vec::new(),
        }
    }

    fn new_with_gs(gs: GameState) -> Node<GameState> {
        Self {
            id: 0,
            parent: None,
            game_state: GameState::default(),
            visits: 0,
            score: 0.0,
            children: Vec::new(),
        }
    }

    fn new_child_with_gamestate(&self, arena: &Arena<Node<GameState>>, gs: GameState) -> Node<GameState> {
        Self {
            id: arena.next_id(),
            parent: Some(self.id),
            game_state: gs,
            visits: 0,
            score: 0.0,
            children: Vec::new(),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

}

impl<GameState> Arena<Node<GameState>>
where GameState: Clone + Default + MCTS<GameState>
{
    fn new() -> Self {
        Arena {
            nodes: vec![Node::new_root()],
        }
    }

    fn new_with_gamestate(gs: GameState) -> Self {
        Arena {
            nodes: vec![Node::new_with_gs(gs)]
        }
    }

    // TODO: Find a better (multithreadable) solution for getting the next id.
    fn next_id(&self) -> usize {
        self.nodes.len()
    }

    fn child_from_with_gamestate(&mut self, parent: usize, gs: GameState) {
        let child_id = self.next_id();
        self.nodes[parent].children.push(child_id);
        self.nodes.push(self[parent].new_child_with_gamestate(&self, gs))
    }

}

use std::ops::{Index,IndexMut};

impl<GameState> Index<usize> for Arena<Node<GameState>> {
    type Output = Node<GameState>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<GameState> IndexMut<usize> for Arena<Node<GameState>> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

