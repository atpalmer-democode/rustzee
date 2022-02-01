use super::entries;
use crate::roll::Roll;

pub struct ScoreCard {
    entries: Vec<Box<dyn entries::ScoreCardEntry>>
}

impl ScoreCard {
    pub fn new() -> ScoreCard {
        return ScoreCard {
            entries: vec![
                Box::new(entries::Aces::default()),
                Box::new(entries::Twos::default()),
                Box::new(entries::Threes::default()),
                Box::new(entries::Fours::default()),
                Box::new(entries::Fives::default()),
                Box::new(entries::Sixes::default()),
                Box::new(entries::ThreeOfAKind::default()),
                Box::new(entries::FourOfAKind::default()),
                Box::new(entries::FullHouse::default()),
                Box::new(entries::SmallStraight::default()),
                Box::new(entries::LargeStraight::default()),
                Box::new(entries::Rustzee::default()),
                Box::new(entries::Chance::default()),
            ]
        };
    }

    fn top_subtotal(&self) -> i32 {
        return self.entries[0..6].iter()
            .filter_map(|x| x.get())
            .sum();
    }

    fn top_bonus(&self) -> i32 {
        return match self.top_subtotal() >= 63 {
            true => 35,
            false => 0,
        };
    }

    fn top_total(&self) -> i32 {
        return self.top_subtotal() + self.top_bonus();
    }

    fn bottom_total(&self) -> i32 {
        return self.entries[6..].iter()
            .filter_map(|x| x.get())
            .sum();
    }

    pub fn total(&self) -> i32 {
        return self.top_total() + self.bottom_total();
    }

    pub fn get_options(&self, roll: &Roll) -> Vec<(usize, &str, i32, i32)> {
        let base_total = self.total();
        return self.entries.iter()
            .enumerate()
            .filter_map(|(i, e)| {
                self.entries[i].clone().try_score(roll).ok().and_then(|score| {
                    let total = base_total - (*e).get().unwrap_or(0) + score;
                    return Some((i + 1, (*e).text(), score, total));
                })
            }).collect();
    }

    pub fn score_by_option(&mut self, roll: &Roll, choice: usize) -> Option<i32> {
        let index = choice.checked_sub(1)?;
        return self.entries.get_mut(index)?.try_score(roll).ok();
    }
}

