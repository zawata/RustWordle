

#[derive(PartialEq)]
pub enum MatchClass {
  NoMatch,
  PartialMatch,
  ExactMatch
}

pub struct Game {
  answer: [char; 5]
}

impl Game {
  pub fn new(answer: [char; 5]) -> Self {
    Self {
      answer
    }
  }

  pub fn guess(self: &Self, guess: [char; 5]) -> [MatchClass; 5] {
    let mut answer_matches: [bool; 5] = core::array::from_fn(|_| false);
    let mut result: [MatchClass; 5] = core::array::from_fn(|_| MatchClass::NoMatch);
    // enumerate guess characters
    for (g_idx, g_chr) in guess.iter().enumerate() {
      // enumerate answer characters
      for (a_idx, a_chr) in self.answer.iter().enumerate() {
        // if 2 characters match and that answer character hasn't been matched yet
        if g_chr == a_chr && !answer_matches[a_idx] {
          // mark this answer character as matched
          answer_matches[a_idx] = true;
          // if the characters share an index
          if g_idx == a_idx {
            // it's an exact match
            result[g_idx] = MatchClass::ExactMatch;
          } else {
            // otherwise it's a partial match
            result[g_idx] = MatchClass::PartialMatch;
          }
          break;
        }
      }
    }

    result
  }
}