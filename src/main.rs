use rand::Rng;
use std::collections::HashMap;
use std::io;

extern crate rand;

const BOARD_SIZE: usize = 10;
const UNKNOWN_POINT: &str = "  .  ";
const SUNK_POINT: &str = "  ✔  ";
const MISSED_POINT: &str = "  x  ";
const WATER_POINT: &str = "  ~  ";

/* Used to for AI to keep track of results
and displaying correct character for each OceanPoint.*/
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum AttackResult {
    Miss,
    Hit,
    Sunk,
}
impl AttackResult {
    /* Used to print meaningful display to the terminal.
    Helps the player remember whether or not each point was
    hit, missed, or sunk. */
    fn to_string(&self) -> String {
        match self {
            AttackResult::Miss => String::from("  ⚬  "),
            AttackResult::Hit => String::from("  X  "),
            AttackResult::Sunk => String::from(SUNK_POINT),
        }
    }
}
/* Used for UI to display different symbols on player side,
or to determine if an attack was a hit on either side fo the board.
*/
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ShipType {
    Destroyer,
    Submarine,
    Cruiser,
    BattleShip,
    Carrier,
    NoShip,
    Unknown,
}
impl ShipType {
    /* Prints meaningful characters on locations of the player's ships.  */
    pub fn to_string(&self) -> String {
        match self {
            ShipType::Destroyer => String::from("  D  "),
            ShipType::Submarine => String::from("  S  "),
            ShipType::Cruiser => String::from("  U  "),
            ShipType::BattleShip => String::from("  B  "),
            ShipType::Carrier => String::from("  C  "),
            ShipType::NoShip => String::from("  X  "),
            ShipType::Unknown => String::from(" ? "),
        }
    }
    fn get_ship_size(&self) -> usize {
        match self {
            ShipType::Destroyer => 2,
            ShipType::Submarine => 3,
            ShipType::Cruiser => 3,
            ShipType::BattleShip => 4,
            ShipType::Carrier => 5,
            ShipType::NoShip => 0,
            ShipType::Unknown => 0,
        }
    }
    fn get_name(&self) -> String {
        match self {
            ShipType::Destroyer => String::from("Destroyer"),
            ShipType::Submarine => String::from("Submarine"),
            ShipType::Cruiser => String::from("Cruiser"),
            ShipType::BattleShip => String::from("BattleShip"),
            ShipType::Carrier => String::from("Carrier"),
            ShipType::NoShip => String::from("No Ship"),
            ShipType::Unknown => String::from("Unknown Ship"),
        }
    }
}
/* AI has its own board to keep track of previous attacks and results */
struct AI {
    board: [[Guess; BOARD_SIZE]; BOARD_SIZE],
    stack: Vec<(usize, usize)>,
}
impl AI {
    pub fn new() -> Self {
        let mut board = [[Guess::new(); 10]; 10];
        let mut i_index: usize = 0;
        for i in board.iter_mut() {
            let mut j_index: usize = 0;
            for j in i.iter_mut() {
                j.x = i_index;
                j.y = j_index;
                j_index += 1;
            }
            i_index += 1;
        }
        Self {
            board: board,
            stack: Vec::new(),
        }
    }

    /* Called when AI attacks player's ocean.
    If there are no adjacent points to a previous successful hit left to attack,
    a choose a random point that has not been attempted yet, and attack that point.
    Record the results on the AI board.
     */
    fn attack(&mut self, player_board: &mut Ocean) {
        let mut attack_point = (BOARD_SIZE + 1, BOARD_SIZE + 1);
        while !is_in_bounds(attack_point) || self.has_ship_at(attack_point) {
            attack_point = match self.stack.is_empty() {
                true => rand_point(),
                false => {
                    let mut test = self.stack.pop();
                    match test {
                        Some(thing) => thing,
                        None => rand_point(),
                    }
                }
            };
        }
        match player_board.attack(attack_point) {
            true => {
                println!(
                    "Computer has hit your ship at ({},{})!",
                    attack_point.0, attack_point.1
                );
                self.push_surrounding_points(attack_point);
                self.set_ship_type(attack_point, ShipType::Unknown);
            }
            false => {
                println!(
                    "Computer attacked ({},{}), and missed!",
                    attack_point.0, attack_point.1
                );
                self.set_ship_type(attack_point, ShipType::NoShip);
            }
        }
    }
    fn has_ship_at(&self, point: (usize, usize)) -> bool {
        self.board[point.0][point.1].ship_type.is_some()
    }
    fn get_ship_type(&self, point: (usize, usize)) -> Option<ShipType> {
        self.board[point.0][point.1].ship_type
    }
    fn set_ship_type(&mut self, point: (usize, usize), ship_type: ShipType) {
        self.board[point.0][point.1].ship_type = Some(ship_type);
    }
    /* Called when AI has struck a player ship. Push all surrounding points to
    a the stack of future high-priority strikes. */
    fn push_surrounding_points(&mut self, point: (usize, usize)) {
        let mut other_points = [
            (point.0 + 1, point.1),
            (point.0 - 1, point.1),
            (point.0, point.1 - 1),
            (point.0, point.1 + 1),
        ];
        for i in 0..other_points.len() {
            match is_in_bounds(other_points[i])
                && !self.has_ship_at((other_points[i].0, other_points[i].1))
                && !self.stack.contains(&other_points[i])
            {
                true => self.stack.push(other_points[i]),
                false => continue,
            }
        }
    }
}
/*2d array of OceanPoint structs, vector of ships, and identifier
for whether or not this side of the ocean belongs to the player. */
#[derive(Debug)]
struct Ocean {
    ocean: Vec<Vec<OceanPoint>>,
    fleet: Vec<Ship>,
    is_player: bool,
}
impl Ocean {
    fn new(is_player: bool) -> Self {
        Self {
            ocean: (0..10)
                .map(|_| (0..10).map(|_| OceanPoint::new()).collect())
                .collect(),
            fleet: vec![],
            is_player,
        }
    }
    /* Used when player attacks AI. Checks for a successful hit and
    reveals that point on AI side upon success.
    Prints a message about the result.*/
    fn attack(&mut self, (x, y): (usize, usize)) -> bool {
        match self.ocean[x][y].ship_type {
            Some(ship_type) => {
                self.ocean[x][y].set_attack_result(AttackResult::Hit);
                if self.check_for_sunk((x, y)) {
                    println!("a ship has been sunk!")
                }
                true
            }
            None => {
                self.ocean[x][y].set_attack_result(AttackResult::Miss);
                false
            }
        }
    }
    /* Find the ship that was hit, and see if all of the OceanPoint in
    that ship's location have been hit. */
    fn check_for_sunk(&mut self, tuple: (usize, usize)) -> bool {
        for ship in &mut self.fleet {
            if ship.location.contains(&tuple) {
                return ship.is_sunk(&self.ocean);
            }
        }
        false
    }
    /* Re-displays AI and Player maps to the terminal. */
    pub fn show(&mut self) {
        let spaces = "   ";
        let last_row_spaces = "  ";
        let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
        let mut alpha_line = format!(" {}", spaces);
        let mut horizontal_separator = String::from("");
        for i in 0..letters.len() {
            alpha_line = format!("{}   {} ", alpha_line, letters[i]);
            horizontal_separator = format!("{}_____", horizontal_separator);
        }
        horizontal_separator = format!("  {}{}", spaces, horizontal_separator);
        println!("{}\n{}", alpha_line, horizontal_separator);
        let mut point_chars: String;
        for i in 0..BOARD_SIZE {
            if i == BOARD_SIZE - 1 {
                print!("{}  |", i + 1);
            } else {
                print!("{}{}|", i + 1, spaces);
            }
            for j in 0..BOARD_SIZE {
                if self.is_player {
                    point_chars = match self.ocean[i][j].get_ship_type() {
                        None => String::from(WATER_POINT),
                        Some(ship_type) => match self.ocean[i][j].get_attack_result() {
                            Some(_attack_result) => SUNK_POINT.to_string(),
                            None => ship_type.to_string(),
                        },
                    };
                } else {
                    point_chars = match self.ocean[i][j].get_attack_result() {
                        Some(attack_result) => attack_result.to_string(),
                        None => String::from(UNKNOWN_POINT),
                    };
                }
                print!("{}", point_chars)
            }
            println!("|");
        }
        println!("{}\n\n", horizontal_separator);
    }
    /* Randomly places ships for both fleets in the ocean. */
    fn generate_fleet(&mut self) {
        let ships: [ShipType; 5] = [
            ShipType::Destroyer,
            ShipType::Submarine,
            ShipType::Cruiser,
            ShipType::BattleShip,
            ShipType::Carrier,
        ];
        for i in 0..ships.len() {
            let boat_size = ships[i].get_ship_size();

            self.fleet.push(Ship::new(
                ships[i],
                ships[i].get_name(),
                ships[i].to_string(),
            ));

            let location = &mut self.fleet[i].location;
            let mut rng = rand::thread_rng();
            let mut direction_start;
            let mut a_index = 0;

            while a_index < boat_size {
                direction_start = rng.gen_range(0, 4) % 4 as usize;
                let mut x = rng.gen_range(boat_size, BOARD_SIZE) as isize;
                let mut y = rng.gen_range(boat_size, BOARD_SIZE) as isize;
                loop {
                    if (x < 0 || x > BOARD_SIZE as isize - 1)
                        || (y < 0 || y > BOARD_SIZE as isize - 1)
                    {
                        location.clear(); // if out of bounds or already taken, start over
                        a_index = 0;
                        break;
                    } else if !self.ocean[x as usize][y as usize].ship_type.is_none()
                        || x as usize >= BOARD_SIZE
                        || y as usize >= BOARD_SIZE
                    {
                        location.clear(); // if out of bounds or already taken, start over
                        a_index = 0;
                        break;
                    }
                    location.push((x as usize, y as usize));
                    match direction_start {
                        0 => x -= 1, // traverse left
                        1 => y -= 1, // traverse up
                        2 => x += 1, // traverse right
                        _ => y += 1, // traverse down
                    };
                    a_index += 1;
                    if a_index == boat_size {
                        break;
                    } else {
                    }
                }
            }
            for tuple in location {
                self.ocean[tuple.0][tuple.1].set_ship_type(ships[i]);
            }
        }
    }
}
/* Struct that holds info on what type of ship is located at a particular (x,y)
coordinate (if any), and an attack result (if it has been attacked) */
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct OceanPoint {
    ship_type: Option<ShipType>, // represents a ship ID
    attack_result: Option<AttackResult>,
}
impl OceanPoint {
    pub fn new() -> Self {
        Self {
            ship_type: None, // represents a ship ID
            attack_result: None,
        }
    }
    /* This changes to Some(ShipType) if a ship has been placed at that location */
    pub fn set_ship_type(&mut self, ship_type: ShipType) {
        self.ship_type = Some(ship_type);
    }
    /* Used to see if a ship exists at the this OceanPoint */
    pub fn get_ship_type(&self) -> Option<ShipType> {
        self.ship_type
    }
    /* When this point is attacked, its "attack_result" is updated Hit or Miss,
    depending on whether or not there is a ship here */
    pub fn set_attack_result(&mut self, attack_result: AttackResult) {
        self.attack_result = Some(attack_result);
    }
    /* Used to find out if this point has been attacked yet. */
    pub fn get_attack_result(&self) -> Option<AttackResult> {
        self.attack_result
    }
}
/* Struct used by the AI to represent an 'OceanPoint' on the player's board,
and information about an attack on that point, if any.
 */
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Guess {
    x: usize,
    y: usize,
    ship_type: Option<ShipType>,
    adjacent_points: [Option<bool>; 4],
}
impl Guess {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            ship_type: None,
            adjacent_points: [None; 4],
        }
    }
}
/* Represents a ship in the fleet. Holds information about its location as
a vector of (x,y) coordinates. */
#[derive(Debug)]
struct Ship {
    location: Vec<(usize, usize)>,
    ship_type: ShipType,
    ship_name: String,
    display_string: String,
}
impl Ship {
    pub fn new(ship_type: ShipType, name: String, print_string: String) -> Self {
        Self {
            location: Vec::new(),
            ship_type,
            ship_name: name,
            display_string: print_string,
        }
    }
    /* Called by player and AI boards. Returns true if
    OceanPoints in ship's location has an 'AttackResult'.*/
    fn is_sunk(&mut self, ocean: &Vec<Vec<OceanPoint>>) -> bool {
        for point in &self.location {
            match ocean[point.0][point.1].get_attack_result() {
                Some(_thing) => continue,
                None => return false,
            }
        }
        true
    }
}
/* Used to make sure any randomly-generated points are within the bounds of the board. */
pub fn is_in_bounds(point: (usize, usize)) -> bool {
    point.0 < BOARD_SIZE && point.0 >= 0 && point.1 < BOARD_SIZE && point.1 >= 0
}
/* Used to generate a random (x,y) coordinate */
pub fn rand_point() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (
        rng.gen_range(0, BOARD_SIZE) as usize,
        rng.gen_range(0, BOARD_SIZE) as usize,
    )
}
fn main() {
    let alphas: HashMap<String, usize> = (b'A'..=b'J')
        .enumerate()
        .map(|(i, c)| ((c as char).to_string(), i + 1))
        .collect();
    println!("{:?}", alphas);
    let mut player_board = Ocean::new(true);
    let mut ai_board = Ocean::new(false);
    let mut ai = AI::new();
    player_board.generate_fleet();
    ai_board.generate_fleet();
    ai_board.show();
    player_board.show();
    let mut round_count = 1;

    loop {
        let mut guess = String::new();
        println!(
            "\n\nRound {}:\nPlease input your guess (x and y, separated by a space).",
            round_count
        );

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let mut substr_iter = guess.split_whitespace(); //.split_whitespace();
        let mut next_num = || -> usize {
            substr_iter
                .next()
                .expect("Not enough input numbers")
                .parse()
                .expect("Input is not a number")
        };
        let guess_y = next_num();
        let guess_x = next_num();

        println!("You guessed: ({},{})", guess_x, guess_y);

        let success = ai_board.attack((guess_x - 1, guess_y - 1));
        match success {
            true => println!("HIT!"),
            false => println!("Missed!"),
        }
        ai.attack(&mut player_board);
        ai_board.show();
        player_board.show();
        round_count += 1;
    }
}
