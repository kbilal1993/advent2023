#[derive(Debug)]
#[derive(PartialEq)]
pub struct Game{
    id: u8,
    handfuls: Vec<Handful>
}

impl Default for Game {
    fn default() -> Game {
        Game {
            id: 0,
            handfuls: vec![]
        }
    }
}

impl Game{
    fn is_possible(&self, full_set: &Handful) -> bool {
        if self.handfuls.len() == 0 {
            true
        } else {
            self.handfuls.iter().all(|x| x.is_possible(&full_set))
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Handful {
    blue: u8,
    green: u8,
    red: u8
}

impl Default for Handful {
    fn default() -> Handful {
        Handful {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}

impl Handful {
    fn is_possible(&self, full_set: &Handful) -> bool {
        self.blue <= full_set.blue &&
            self.red <= full_set.red &&
            self.green <= full_set.green
    }
}

fn parse_game(game_string: &str) -> Game {
    if game_string.len() == 0 {
        return Game::default()
    };
    let temp_id: u8;
    let temp_handful_strings: Vec<&str>;
    // Centre split. 0 = game, 1 = handfuls
    let temp_split: Vec<&str> = game_string.split(":").map(|x| x.trim()).collect();

    temp_id = temp_split[0].matches(char::is_numeric).collect::<String>().parse().unwrap();
    temp_handful_strings = temp_split[1].split(";").collect::<Vec<_>>().iter().map(|x| x.trim()).collect();

    Game {
        id: temp_id,
        handfuls: temp_handful_strings.iter().map(|x| parse_handful(x)).collect()
    }

}

// Takes "x blue, y green, z red", returns a Handful containing the values
fn parse_handful(handful_string: &str) -> Handful {
    if handful_string.len() == 0 {
        return Handful::default()
    };
    let mut temp_blue: u8 = 0;
    let mut temp_green: u8 = 0;
    let mut temp_red: u8 = 0;
    let colours_strings = handful_string.split(",").map(|x| x.trim());
    for colour_string in colours_strings {
        let temp_split: Vec<_> = colour_string.split_whitespace().collect();
        let (number, colour) = (temp_split[0], temp_split[1]);
        match colour {
            "red" => temp_red = number.parse().unwrap(),
            "green" => temp_green= number.parse().unwrap(),
            "blue" => temp_blue = number.parse().unwrap(),
            _ => println!("This is impossible")
        }
    }
    Handful {red: temp_red, green: temp_green, blue: temp_blue}
}

fn part1() {
    const COMPARISON: Handful = Handful {red: 12, green: 13, blue: 14};
    let input = include_str!("input.txt").lines();
    let mut counter: u32 = 0; // contains the IDs we sum up as part of the question
    for i in input {
        let g = parse_game(i);
        if g.is_possible(&COMPARISON) {
            counter = counter + u32::from(g.id);
        }
    }
    println!("{}", counter);
}


fn part2() {
    let input = include_str!("input.txt").lines();
    let mut counter: u32 = 0;
    for i in input {
        let mut minimum_handful = Handful::default();
        let game = parse_game(i);
        for h in game.handfuls {
            if h.red > minimum_handful.red {
                minimum_handful.red = h.red
            }
            if h.green > minimum_handful.green{
                minimum_handful.green = h.green
            }
            if h.blue > minimum_handful.blue {
                minimum_handful.blue = h.blue
            }
        }
        counter = counter + (u32::from(minimum_handful.red) * u32::from(minimum_handful.green) * u32::from(minimum_handful.blue));
    }
    println!("{}", counter);
}

fn main() {
    println!("Part 1");
    part1();
    println!("Part 2");
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;
    const COMPARISON: Handful = Handful {red: 10, green: 11, blue: 12};
    #[test]
    fn handful_is_possible() {
        let my_handful = Handful {red: 5, blue: 4, green: 7};
        assert_eq!(true, my_handful.is_possible(&COMPARISON));
    }
    #[test]
    fn handful_is_impossible() {
        let my_handful= Handful {red: 11, green: 4, blue: 7};
        assert_eq!(false, my_handful.is_possible(&COMPARISON));
    }
    #[test]
    fn parse_to_handful_works() {
        let my_str = "4 blue, 3 red, 5 green";
        assert_eq!(Handful {blue: 4, red: 3, green: 5}, parse_handful(my_str));
    }
    #[test]
    fn parse_to_handful_works_with_missing_values() {
        let my_str = "3 red, 5 green";
        assert_eq!(Handful {blue: 0, red: 3, green: 5}, parse_handful(my_str));
    }
    #[test]
    fn parse_to_handful_works_with_no_values() {
        let my_str = "";
        assert_eq!(Handful {blue: 0, red: 0, green: 0}, parse_handful(my_str));
    }
    #[test]
    fn game_is_possible() {
        let my_game = Game {
            id: 5,
            handfuls: vec![
                Handful {red: 3, green: 5, blue: 7},
                Handful {green: 4, blue: 10, red: 10},
                Handful {red: 3, green: 0, blue: 0}]
        };
        assert_eq!(true, my_game.is_possible(&COMPARISON));
    }
    #[test]
    fn game_is_impossible() {
        let my_game = Game {
            id: 5,
            handfuls: vec![
                Handful {red: 3, green: 5, blue: 7},
                Handful {green: 4, blue: 10, red: 10},
                Handful {red: 11, green: 0, blue: 0}]
        };
        assert_eq!(false, my_game.is_possible(&COMPARISON));
    }
}
