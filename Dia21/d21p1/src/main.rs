struct DeterministicDie {
    current_pos: u8,
    roll_count: usize
}

impl DeterministicDie {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for DeterministicDie {
    fn default() -> Self {
        Self { current_pos: 1, roll_count: 0 }
    }
}

trait Die {
    fn next(&mut self) -> u8;
    fn number_of_rolls(&self) -> usize;
}

impl Die for DeterministicDie {
    fn next(&mut self) -> u8 {
        let r = self.current_pos;
        self.roll_count += 1;
        self.current_pos = (self.current_pos + 1) % 100;
        if r == 0 {
            100
        }else{
            r
        }
    }

    fn number_of_rolls(&self) -> usize {
        self.roll_count
    }
}

fn main() {
    let mut starting_pos = include_str!("../../input.txt").lines().map(|x| x.trim()[28..].parse::<u8>().unwrap());
    let mut player1_pos = starting_pos.next().unwrap();
    let mut player2_pos = starting_pos.next().unwrap();
    let mut die = DeterministicDie::new();
    let mut player1_points: usize = 0;
    let mut player2_points: usize = 0;
    let mut losing_points = 0;
    while player2_points < 1000 {
        let movement: usize = (0..3).map(|_| die.next() as usize).sum();
        player1_pos = ((player1_pos as usize + movement) % 10) as u8;
        player1_points += if player1_pos == 0 {10} else {player1_pos} as usize;
        if player1_points >= 1000 {
            losing_points = player2_points;
            break
        }
        let movement: usize = (0..3).map(|_| die.next() as usize).sum();
        player2_pos = ((player2_pos as usize + movement) % 10) as u8;
        player2_points += if player2_pos == 0 {10} else {player2_pos} as usize;
        losing_points = player1_points;
    }
    println!(">>> {}", die.number_of_rolls() * losing_points);
}
