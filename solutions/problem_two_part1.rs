use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug)]
#[allow(dead_code)]
struct Cubes {
    blue: usize,
    red: usize,
    green: usize,
    game_state: bool,
}
impl Cubes {
    pub fn new(blue: usize, red: usize, green: usize) -> Self {
        let mut game_state = true;

        if blue > 14 || green > 13 || red > 12 {
            game_state = false;
        }
        Self {
            blue,
            red,
            green,
            game_state,
        }
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct Game {
    id: usize,
    cubes: Vec<Cubes>,
    possible: bool,
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ Game: {}", self.id)?;
        for cube in &self.cubes {
            write!(
                f,
                "(blue: {}, red: {}, green: {})",
                cube.blue, cube.red, cube.green
            );
        }
        write!(f, "possible: {}]", self.possible)
    }
}
impl Game {
    pub fn new(id: usize, cubes: Vec<Cubes>) -> Self {
        let mut possible = true;
        for each in &cubes {
            if !each.game_state {
                possible = false;
                break;
            }
        }
        Self {
            id,
            cubes,
            possible,
        }
    }
}
fn main() {
    let mut reader = BufReader::new(File::open("input_cubes_1").unwrap());
    let mut line = String::new();
    let mut games = Vec::new();

    loop {
        let mut scores = Vec::new();
        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }

        let v: Vec<&str> = line.split(':').collect();
        let v2: Vec<&str> = v[1].split(';').collect();

        for each in v2 {
            let v3: Vec<&str> = each.split(',').collect();
            let mut green: usize = 0;
            let mut blue: usize = 0;
            let mut red: usize = 0;
            for cube in v3 {
                let cube = cube.trim();
                if cube.contains("blue") {
                    let a: Vec<&str> = cube.split(' ').collect();
                    blue += a[0].parse::<usize>().unwrap();
                }
                if cube.contains("red") {
                    let a: Vec<&str> = cube.split(' ').collect();
                    red += a[0].parse::<usize>().unwrap();
                }
                if cube.contains("green") {
                    let a: Vec<&str> = cube.split(' ').collect();
                    green += a[0].parse::<usize>().unwrap();
                }
            }
            let cube = Cubes::new(blue, red, green);
            scores.push(cube);
        }

        let a: Vec<&str> = v[0].split(' ').collect();
        let id = a[1].parse::<usize>().unwrap();
        let game = Game::new(id, scores);
        games.push(game);
        line.clear();
    }
    let mut sum = 0;
    for game in &games {
        if game.possible {
            sum += game.id;
        }
    }
    // for game in &games {
    //     if !game.possible {
    //         println!("{game}");
    //     }
    // }
    println!("{sum}");
}
