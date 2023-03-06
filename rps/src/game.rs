use crate::hand::{Hand, Result, decide, random_hand};

use rand::prelude::{ThreadRng, thread_rng};

use std::io::stdin;

pub struct Game {
    rng: ThreadRng,
    score: isize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            rng: thread_rng(),
            score: 0,
        }
    }

    pub fn run(&mut self) -> () {
        println!("YO! let's do a bit of rockin, paperin, and yknow what >:)");

        while let Some(hand) = self.choose() {
            let (cpu, res) = self.play(hand);

            println!("\nyou played: {}", hand.to_string().to_lowercase());
            println!("i played: {}", cpu.to_string().to_lowercase());
            println!("result: you {}.", res.to_string().to_lowercase());
            println!("score: {}", self.score);
        }

        println!("\nrun away coward.");
    }

    fn choose(&self) -> Option<Hand> {
        println!("\nyour move bucko: ");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.to_lowercase().trim() {
            "rock" => Some(Hand::Rock),
            "paper" => Some(Hand::Paper),
            "scissors" => Some(Hand::Scissors),
            "e" | "ex" | "exit" | "q" | "quit" => None,
            _ => {
                println!("\nnot a valid input bozo! try that again.");
                self.choose()
            }
        }
    }

    fn play(&mut self, hand: Hand) -> (Hand, Result) {
        let cpu = random_hand(&mut self.rng);
        let res = decide(hand, cpu);
        let delta = match res {
            Result::Win => 1,
            Result::Lose => -1,
            Result::Draw => 0,
        };

        self.score += delta;

        (cpu, res)
    }
}

