use rand::prelude::{ThreadRng, SliceRandom};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use self::Hand::*;
use self::Result::*;

#[derive(Debug, Display, Eq, PartialEq)]
pub enum Result {
    Win,
    Lose,
    Draw,
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

pub fn decide(own: Hand, other: Hand) -> Result {
    // if our beats == the other hand, we won. if their beats == our hand, we lost.
    if own.beats() == other { Win }
    else if other.beats() == own { Lose }
    else { Draw }
}

pub fn random_hand(mut rng: &mut ThreadRng) -> Hand {
    *Hand::iter().collect::<Vec<Hand>>().choose(&mut rng).unwrap()
}

