use std::{array, cell::Cell};

use rand::random_bool;

const CELLS_IN_BASTION: i16 = 100;
const MAX_SEARCHES_PER_DAY: usize = 10;
const MUNCHER_LEN: usize = 2;
const NUM_MUNCHERS: usize = MAX_SEARCHES_PER_DAY / MUNCHER_LEN;

/*  
    Probably should have used a scripting language 🙃.
    Trying to stop the munchers from overlapping
    costed me thirty minutes of skirmishing ⚔ with the borrow checker.
*/
fn main(){
    let mut max_days = i16::MIN;

    // The Spy checks adjacent cells randomly. So the simulation must be run multiple times
    for _ in 0..7 {
        for spy_starting_cell  in 0..100 {
            let days = simulate(spy_starting_cell);
            max_days = max_days.max(days);
        }
    }
    println!("{max_days}"); // Output: 24
    
}

/**
 * Returns the number of days to catch the spy
 */
fn simulate(spy_cell:i16) -> i16 {
    
    let mut spy = Spy::new(spy_cell);

    let munchers: [Muncher; NUM_MUNCHERS] = [

        // Set 1
        Muncher::new([0, 1], 1), // Muncher A 
        Muncher::new([48, 49], -1), // Muncher B 
        Muncher::new([25, 26], 0), // Muncher C 

        // Set 2
        Muncher::new([50, 51], 1),  // Muncher D 
        Muncher::new([98, 99], -1), // Muncher E 

    ];

    
    for day in 0..  { 

        let searches: Vec<i16> = munchers
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
    };

    i16::MAX

}

struct Spy {
    current_cell: i16,
}
impl Spy {
    pub fn new(current_cell:i16) -> Spy {
        Self {
            current_cell,
        }
    }
    pub fn overhear(&mut self, searches: &[i16]) {

        if !searches.contains(&self.current_cell) {
            return;
        }

        // Probably not a perfect rendition of a brilliant spy
        let mut check_direction = if random_bool(0.5) { 1 } else { -1 };

        let possible_cell = shift_cell_index(&self.current_cell,&check_direction);
        let switched = self.switch_to_cell_if_unsearched(possible_cell, searches);
        if switched {
            return;
        }

        check_direction *= -1;
        let possible_cell = shift_cell_index(&self.current_cell,&check_direction);
        let _ = self.switch_to_cell_if_unsearched(possible_cell, searches);
    }

    fn switch_to_cell_if_unsearched(
        &mut self,
        possible_cell: i16,
        searches: &[i16],
    ) -> bool {
        if !searches.contains(&possible_cell) {
            self.current_cell = possible_cell;
            return true;
        }
        false
    }
    pub fn is_in_cell(&self, searched_cell: i16) -> bool {
        searched_cell == self.current_cell
    }
}

struct Muncher {
    body: Cell<[i16; MUNCHER_LEN]>,
    velocity: i16,
}
impl Muncher {
    fn new(body: [i16; MUNCHER_LEN], velocity: i16) -> Muncher {
        Self { body:Cell::new(body), velocity, }
    }
    fn move_forwards(&self, other_munchers: &[&Muncher;NUM_MUNCHERS-1]) {

        let current_body = self.body.get();
        let new_body:[i16;MUNCHER_LEN] = array::from_fn(|i|{
            shift_cell_index(&current_body[i], &self.velocity)
        });

        let other_segments:[i16;(NUM_MUNCHERS-1)*2] = other_munchers.iter()
            .flat_map(|other_muncher|{
                other_muncher.body.get()
            })
            .collect::<Vec<i16>>()
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
fn shift_cell_index(current_position:&i16,shift:&i16) -> i16 {
    let next_position =(current_position + shift).rem_euclid(CELLS_IN_BASTION);
    
    assert!(next_position >= 0, "Next position is negative = {}",next_position);
    next_position
}
