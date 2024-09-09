use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub(crate) fn try_from_str(raw: &str) -> Result<Color, ()> {
        match raw.to_lowercase().trim() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[test]
fn parses_color_from_str() {
    assert_eq!(Ok(Color::Red), Color::try_from_str("red"));
    assert_eq!(Ok(Color::Red), Color::try_from_str("Red"));
    assert_eq!(Ok(Color::Red), Color::try_from_str("RED"));

    assert_eq!(Ok(Color::Blue), Color::try_from_str("BLUE"));
    assert_eq!(Ok(Color::Blue), Color::try_from_str("   blUE  "));
    assert_eq!(Ok(Color::Blue), Color::try_from_str("BlUe"));

    assert_eq!(Err(()), Color::try_from_str("yellow"));
}

type Count = u64;

#[derive(Default, Clone, PartialEq, Debug, Eq)]
pub(crate) struct Bag {
    dice: HashMap<Color, Count>,
}

impl Bag {
    fn can_contain(&self, other: &Bag) -> bool {
        for (color, needed) in &other.dice {
            let Some(available) = self.dice.get(color) else {
                return false;
            };

            if needed.gt(available) {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod bag_tests {
    use super::{Bag, BagBuilder, Color};

    #[test]
    fn can_contain_empty_bags() {
        let bag = BagBuilder::new().with_dice(Color::Green, 3).build();
        assert!(bag.can_contain(&Bag::default()));
    }

    #[test]
    fn can_contain_itself() {
        let bag = BagBuilder::new()
            .with_dice(Color::Red, 12)
            .with_dice(Color::Green, 13)
            .with_dice(Color::Blue, 14)
            .build();

        assert!(bag.can_contain(&bag));
    }

    #[test]
    fn cannot_contain_more_colors() {
        let bag = BagBuilder::new().with_dice(Color::Red, 10).build();

        assert!(!bag.can_contain(
            &BagBuilder::new()
                .with_dice(Color::Red, 10)
                .with_dice(Color::Green, 10)
                .build()
        ));
    }

    #[test]
    fn cannot_contain_dice_of_different_colour() {
        let bag = BagBuilder::new().with_dice(Color::Red, 10).build();

        assert!(!bag.can_contain(&BagBuilder::new().with_dice(Color::Blue, 1).build()));
    }

    #[test]
    fn cannot_contain_more_dice_of_same_colour() {
        let bag = BagBuilder::new().with_dice(Color::Red, 10).build();

        assert!(!bag.can_contain(&BagBuilder::new().with_dice(Color::Red, 11).build()));
    }

    #[test]
    fn can_contain_dice_of_fewer_colours() {
        let bag = BagBuilder::new()
            .with_dice(Color::Red, 10)
            .with_dice(Color::Green, 10)
            .build();

        assert!(bag.can_contain(&BagBuilder::new().with_dice(Color::Red, 10).build()));
    }

    #[test]
    fn can_contain_fewer_dice_of_same_colour() {
        let bag = BagBuilder::new().with_dice(Color::Red, 10).build();

        assert!(bag.can_contain(&BagBuilder::new().with_dice(Color::Red, 9).build()));
    }
}

#[derive(Default)]
pub(crate) struct BagBuilder {
    dice: HashMap<Color, Count>,
}

impl BagBuilder {
    pub(crate) fn new() -> Self {
        BagBuilder::default()
    }

    pub(crate) fn with_dice(mut self, color: Color, count: Count) -> Self {
        self.dice.insert(color, count);
        self
    }

    pub(crate) fn with_bag(mut self, other: &Bag) -> Self {
        for (color, count) in &other.dice {
            self = self.with_dice(*color, *count);
        }
        self
    }

    pub(crate) fn build(&self) -> Bag {
        Bag {
            dice: self.dice.clone(),
        }
    }
}

#[test]
fn builds_a_bag() {
    let built = BagBuilder::new()
        .with_dice(Color::Red, 12)
        .with_dice(Color::Green, 13)
        .with_dice(Color::Blue, 14)
        .build();

    assert_eq!(
        built.dice,
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)])
    )
}

#[test]
fn builds_a_bag_from_other_bags() {
    let sub_bag_1 = BagBuilder::new()
        .with_dice(Color::Red, 12)
        .with_dice(Color::Green, 13)
        .build();
    let sub_bag_2 = BagBuilder::new().with_dice(Color::Blue, 14).build();
    let want = BagBuilder::new()
        .with_dice(Color::Red, 12)
        .with_dice(Color::Green, 13)
        .with_dice(Color::Blue, 14)
        .build();

    assert_eq!(
        want,
        BagBuilder::new()
            .with_bag(&sub_bag_1)
            .with_bag(&sub_bag_2)
            .build()
    )
}

#[test]
fn ignores_previous_color_count() {
    let built = BagBuilder::new()
        .with_dice(Color::Red, 12)
        .with_dice(Color::Red, 13)
        .with_dice(Color::Red, 14)
        .build();

    assert_eq!(built.dice, HashMap::from([(Color::Red, 14)]))
}

#[test]
fn order_does_not_matter() {
    let built = BagBuilder::new()
        .with_dice(Color::Green, 13)
        .with_dice(Color::Blue, 14)
        .with_dice(Color::Red, 12)
        .build();

    assert_eq!(
        built.dice,
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)])
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Game {
    sets: Vec<Bag>,
}

impl Game {
    pub(crate) fn new(sets: Vec<Bag>) -> Self {
        Self { sets }
    }

    pub(crate) fn fits_in(&self, bag: &Bag) -> bool {
        for set in &self.sets {
            if !bag.can_contain(set) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::{Bag, BagBuilder, Color, Game};

    /*
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
     */

    fn bag() -> Bag {
        BagBuilder::new()
            .with_dice(Color::Red, 12)
            .with_dice(Color::Green, 13)
            .with_dice(Color::Blue, 14)
            .build()
    }

    fn game1() -> Game {
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

        Game { sets }
    }

    #[test]
    fn game1_fits_in_bag() {
        assert!(game1().fits_in(&bag()));
    }
}
