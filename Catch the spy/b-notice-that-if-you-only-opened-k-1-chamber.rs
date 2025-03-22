/**
  Changes:
  1. MAX_SEARCHES_PER_DAY = 2;
  2. MUNCHER_LEN = 2;
  3. Removed 3 Munchers

  Everything else is the same 👍
**/
use std::{array, cell::Cell};

use rand::random_bool;

const CELLS_IN_BASTION: i32 = 100;
const MAX_SEARCHES_PER_DAY: usize = 4; // Changed to 3 
const MUNCHER_LEN: usize = 2;
const NUM_MUNCHERS: usize = MAX_SEARCHES_PER_DAY / MUNCHER_LEN;

fn main(){
    let mut max_days = i32::MIN;

    // The Spy checks adjacent cells randomly. So the simulation must be run multiple times
    for _ in 0..7 {
        for spy_starting_cell  in 0..100 {
            let days = simulate(spy_starting_cell);
            max_days = max_days.max(days);
        }
    }
    println!("{max_days}"); // Output: 49
    
}

fn simulate(spy_cell:i32) -> i32 {
    
    let mut spy = Spy::new(spy_cell);

    let munchers: [Muncher; NUM_MUNCHERS] = [
        Muncher::new([0, 1], Direction::new(&Hand::Right)),
        Muncher::new([98, 99], Direction::new(&Hand::Left)),
    ];
    let mut day = 1;
    loop  { 
        let searches: Vec<i32> = munchers
            .iter()
            .flat_map(|muncher| muncher.body.get())
            .collect();

        spy.overhear(&searches);
        
        for cell in searches.iter()  {
            if spy.is_in_cell(*cell) {
                return day;
            }
        };
        for (index, muncher) in munchers.iter().enumerate() {
            let other_munchers: [&Muncher; NUM_MUNCHERS-1] = array::from_fn(|i| {
                if i < index { &munchers[i] } else { &munchers[i+1] }
            });
            muncher.move_forwards(&other_munchers);
        };
        day+=1;
    };
    
}

struct Spy {
    current_cell: i32,
}
impl Spy {
    pub fn new(current_cell:i32) -> Spy {
        Self {
            current_cell,
        }
    }
    pub fn overhear(&mut self, searches: &[i32]) {

        if !searches.contains(&self.current_cell) {
            return;
        }

        let mut shift_direction = if random_bool(0.5) { 
            Direction::new(&Hand::Left) 
        } else { 
            Direction::new(&Hand::Right) 
        };

        let possible_cell = next_position(&self.current_cell,&shift_direction);
        let switched = self.switch_to_cell_if_unsearched(possible_cell, searches);
        if switched {
            return;
        }

        shift_direction.flip();
        let possible_cell = next_position(&self.current_cell,&shift_direction);
        let _ = self.switch_to_cell_if_unsearched(possible_cell, searches);
    }

    fn switch_to_cell_if_unsearched(
        &mut self,
        possible_cell: i32,
        searches: &[i32],
    ) -> bool {
        if !searches.contains(&possible_cell) {
            self.current_cell = possible_cell;
            return true;
        }
        false
    }
    pub fn is_in_cell(&self, searched_cell: i32) -> bool {
        searched_cell == self.current_cell
    }
}

struct Muncher {
    body: Cell<[i32; MUNCHER_LEN]>,
    direction: Direction,
}
impl Muncher {
    fn new(body: [i32; MUNCHER_LEN], direction: Direction) -> Muncher {
        Self { body:Cell::new(body), direction }
    }
    fn move_forwards(&self, other_munchers: &[&Muncher;NUM_MUNCHERS-1]) {

        let current_body = self.body.get();
        let new_body:[i32;MUNCHER_LEN] = array::from_fn(|i|{
            next_position(&current_body[i], &self.direction)
        });

        let other_segments:[i32;(NUM_MUNCHERS-1)*2] = other_munchers.iter()
            .flat_map(|other_muncher|{
                other_muncher.body.get()
            })
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();

        // I could optimize this a little bit
        let has_collision = new_body
            .iter()
            .any(|new_segment| other_segments.contains(new_segment));

        if !has_collision {
            self.body.set(new_body);
        }
    }
}
enum Hand {
    Left,
    Right,
    Neutral,
}
struct Direction(i32);
impl Direction {
    fn new(hand: &Hand) -> Self {
        match hand {
            Hand::Left => Self(-1),
            Hand::Right => Self(1),
            Hand::Neutral => Self(0),
        }
    }
    fn flip(&mut self) {
        self.0 = -self.0 
    }
}
fn next_position(current_position:&i32,direction:&Direction) -> i32 {
    let shift = direction.0;
    let next_position =(current_position + shift).rem_euclid(CELLS_IN_BASTION);
    
    assert!(next_position >= 0, "Next position is negative = {}",next_position);
    next_position
}
