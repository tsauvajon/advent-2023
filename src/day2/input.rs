use super::dice::{Bag, BagBuilder, Color, Game};

#[derive(Debug, PartialEq)]
pub(crate) struct NumberedGame {
    pub id: u64,
    game: Game,
}

impl NumberedGame {
    pub(crate) fn is_possible_for(&self, bag: &Bag) -> bool {
        self.game.fits_in(bag)
    }

    pub(crate) fn get_requirements(&self) -> Bag {
        self.game.get_requirements()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Error {
    MissingParts,
    TooManyParts,
    BadlyFormattedTitle,
    BadlyFormattedDie,
    UnknownColor,
}

pub(crate) fn parse_input(input: &str) -> Result<Vec<NumberedGame>, Error> {
    let parsed_games = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(parse_line);
    let mut games = vec![];
    for game in parsed_games {
        games.push(game?);
    }

    Ok(games)
}

#[test]
fn ignores_empty_lines() {
    let with_empty_lines = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red

        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green


    "#;

    parse_input(with_empty_lines).unwrap();
}

#[test]
fn can_parse_example_input() {
    let example = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    let parsed_input = parse_input(example).unwrap();
    assert_eq!(5, parsed_input.len());
    let fourth_game = &parsed_input[3];
    assert_eq!(4, fourth_game.id);

    let sets = vec![
        BagBuilder::new()
            .with_dice(Color::Green, 1)
            .with_dice(Color::Red, 3)
            .with_dice(Color::Blue, 6)
            .build(),
        BagBuilder::new()
            .with_dice(Color::Green, 3)
            .with_dice(Color::Red, 6)
            .build(),
        BagBuilder::new()
            .with_dice(Color::Green, 3)
            .with_dice(Color::Blue, 15)
            .with_dice(Color::Red, 14)
            .build(),
    ];
    assert_eq!(Game::new(sets), fourth_game.game)
}

fn parse_line(line: &str) -> Result<NumberedGame, Error> {
    let mut parts = line.trim().split(':');
    let Some(title) = parts.next() else {
        return Err(Error::MissingParts);
    };
    let Some(sets) = parts.next() else {
        return Err(Error::MissingParts);
    };
    if parts.next().is_some() {
        return Err(Error::TooManyParts);
    }

    let id = parse_title(title)?;
    let game = parse_game(sets)?;

    Ok(NumberedGame { id, game })
}

#[cfg(test)]
mod parse_line_tests {
    use super::super::dice::BagBuilder;
    use super::{parse_line, Color, Error, Game, NumberedGame};

    #[test]
    fn can_parse_a_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let got = parse_line(line).unwrap();

        let want = NumberedGame {
            id: 1,
            game: Game::new(vec![
                BagBuilder::new()
                    .with_dice(Color::Blue, 3)
                    .with_dice(Color::Red, 4)
                    .build(),
                BagBuilder::new()
                    .with_dice(Color::Red, 1)
                    .with_dice(Color::Green, 2)
                    .with_dice(Color::Blue, 6)
                    .build(),
                BagBuilder::new().with_dice(Color::Green, 2).build(),
            ]),
        };
        assert_eq!(want, got);
    }

    #[test]
    fn detects_missing_parts() {
        let line = "";
        assert_eq!(Err(Error::MissingParts), parse_line(line));

        let line = "Game 1";
        assert_eq!(Err(Error::MissingParts), parse_line(line));

        let line = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(Err(Error::MissingParts), parse_line(line));
    }

    #[test]
    fn detects_too_many_parts() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green: Game 2";
        assert_eq!(Err(Error::TooManyParts), parse_line(line));
    }
}

fn parse_title(raw: &str) -> Result<u64, Error> {
    let raw = raw.trim();

    if !raw.starts_with("Game ") {
        return Err(Error::BadlyFormattedTitle);
    }

    let parts: Vec<&str> = raw.split(' ').collect();
    if parts.len() != 2 {
        return Err(Error::BadlyFormattedTitle);
    }

    parts[1]
        .parse::<u64>()
        .map_err(|_| Error::BadlyFormattedTitle)
}

#[cfg(test)]
mod parse_title_tests {
    use super::{parse_title, Error};

    #[test]
    fn empty_title() {
        let title = "";
        assert_eq!(Err(Error::BadlyFormattedTitle), parse_title(title));
    }

    #[test]
    fn bad_title() {
        let title = "Gamestop 1";
        assert_eq!(Err(Error::BadlyFormattedTitle), parse_title(title));
    }

    #[test]
    fn too_much_stuff() {
        let title = "Game 1 2";
        assert_eq!(Err(Error::BadlyFormattedTitle), parse_title(title));
    }

    #[test]
    fn not_enough_stuff() {
        let title = "Game";
        assert_eq!(Err(Error::BadlyFormattedTitle), parse_title(title));

        let title = "1";
        assert_eq!(Err(Error::BadlyFormattedTitle), parse_title(title));
    }

    #[test]
    fn parses_valid_title() {
        let title = "Game 1";
        assert_eq!(Ok(1), parse_title(title));

        let title = "Game 23";
        assert_eq!(Ok(23), parse_title(title));

        let title = "Game 99999999";
        assert_eq!(Ok(99999999), parse_title(title));
    }
}

fn parse_die(raw: &str) -> Result<Bag, Error> {
    let mut parts = raw.split_whitespace();
    let Some(count) = parts.next() else {
        return Err(Error::BadlyFormattedDie);
    };
    let Some(color) = parts.next() else {
        return Err(Error::BadlyFormattedDie);
    };
    if parts.next().is_some() {
        return Err(Error::BadlyFormattedDie);
    }

    let count = count.parse::<u64>().map_err(|_| Error::BadlyFormattedDie)?;

    let color = Color::try_from_str(color).map_err(|()| Error::UnknownColor)?;

    Ok(BagBuilder::new().with_dice(color, count).build())
}

#[cfg(test)]
mod parse_die_tests {
    use super::super::dice::BagBuilder;
    use super::{parse_die, Color, Error};

    #[test]
    fn detects_unknown_color() {
        let set = "2 yellow";
        assert_eq!(Err(Error::UnknownColor), parse_die(set));
    }

    #[test]
    fn detects_bad_number() {
        let set = "two red";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_die(set));
    }

    #[test]
    fn detects_missing_parts() {
        let set = "0";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_die(set));

        let set = "2";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_die(set));

        let set = "red";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_die(set));
    }

    #[test]
    fn detects_extra_parts() {
        let set = "blue 2 red";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_die(set));

        let set = "2 red blue";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_die(set));
    }

    #[test]
    fn parses_valid_game() {
        let set = "2 green";
        assert_eq!(
            Ok(BagBuilder::new().with_dice(Color::Green, 2).build()),
            parse_die(set)
        );

        let set = "3 blue";
        assert_eq!(
            Ok(BagBuilder::new().with_dice(Color::Blue, 3).build()),
            parse_die(set)
        );
    }
}

fn parse_set(raw: &str) -> Result<Bag, Error> {
    let dice = raw.split(',');

    let mut bag = BagBuilder::new();

    for die in dice {
        let die = parse_die(die.trim())?;
        bag = bag.with_bag(&die);
    }

    Ok(bag.build())
}

#[cfg(test)]
mod parse_set_tests {
    use super::{parse_set, BagBuilder, Color, Error};

    #[test]
    fn detects_invalid_die() {
        let set = "2 yellow";
        assert_eq!(Err(Error::UnknownColor), parse_set(set));

        let set = "two red";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_set(set));
    }

    #[test]
    fn parses_single_die() {
        let set = "2 green";
        assert_eq!(
            Ok(BagBuilder::new().with_dice(Color::Green, 2).build()),
            parse_set(set)
        );

        let set = "3 blue";
        assert_eq!(
            Ok(BagBuilder::new().with_dice(Color::Blue, 3).build()),
            parse_set(set)
        );
    }

    #[test]
    fn detects_bad_dice() {
        let set = "3 blue, 4 red blue";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_set(set));

        let set = "3 blue,";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_set(set));
    }

    #[test]
    fn parses_correct_set() {
        let set = "3 blue, 4 red";
        assert_eq!(
            Ok(BagBuilder::new()
                .with_dice(Color::Blue, 3)
                .with_dice(Color::Red, 4)
                .build()),
            parse_set(set)
        );

        let set = "    1      red      ,     2     green  ,  6  blue ";
        assert_eq!(
            Ok(BagBuilder::new()
                .with_dice(Color::Red, 1)
                .with_dice(Color::Green, 2)
                .with_dice(Color::Blue, 6)
                .build()),
            parse_set(set)
        );
    }
}

fn parse_game(raw: &str) -> Result<Game, Error> {
    let sets_str = raw.trim().split(';');
    let mut sets = vec![];
    for set in sets_str {
        let set = parse_set(set.trim())?;
        sets.push(set);
    }

    Ok(Game::new(sets))
}

#[cfg(test)]
mod parse_game_tests {
    use super::{parse_game, BagBuilder, Color, Error, Game};

    #[test]
    fn detects_incorrect_sets() {
        let game = "3 blue, 4 red blue";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_game(game));

        let game = "3 blue, 4 red blue;";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_game(game));

        let game = "3 blue, 4 red blue; hello";
        assert_eq!(Err(Error::BadlyFormattedDie), parse_game(game));
    }

    #[test]
    fn parses_game_with_single_set() {
        let game = "3 blue, 4 red";
        assert_eq!(
            Ok(Game::new(vec![BagBuilder::new()
                .with_dice(Color::Blue, 3)
                .with_dice(Color::Red, 4)
                .build()])),
            parse_game(game),
        );
    }

    #[test]
    fn parses_game_with_multiple_sets() {
        let game = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let sets = vec![
            BagBuilder::new()
                .with_dice(Color::Blue, 3)
                .with_dice(Color::Red, 4)
                .build(),
            BagBuilder::new()
                .with_dice(Color::Red, 1)
                .with_dice(Color::Green, 2)
                .with_dice(Color::Blue, 6)
                .build(),
            BagBuilder::new().with_dice(Color::Green, 2).build(),
        ];

        assert_eq!(Ok(Game::new(sets)), parse_game(game));
    }

    #[test]
    fn ignores_all_extra_and_missing_whitespace() {
        let normal = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let extra_whitespace = "  3  blue   ,   4  red ;  1  red,  2  green,  6 blue;  2  green";
        let compact = "3 blue,4 red;1 red,2 green,6 blue;2 green";

        let normal = parse_game(normal);
        let extra_whitespace = parse_game(extra_whitespace);
        let compact = parse_game(compact);

        assert_eq!(normal, extra_whitespace);
        assert_eq!(normal, compact);
        assert_eq!(extra_whitespace, compact);
    }
}
