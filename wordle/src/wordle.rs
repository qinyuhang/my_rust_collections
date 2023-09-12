use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ResultCharRepresentation {
    char: String,
    is_position_correct: bool,
    is_contained: bool,
}
// 每一次guess的结果
#[derive(Debug)]
pub struct GuessResult<'a> {
    chars: Vec<ResultCharRepresentation>,
    answer: &'a str,
}
impl<'a> GuessResult<'a> {
    pub fn new(chars: Vec<ResultCharRepresentation>, answer: &'a str) -> Self {
        GuessResult { chars, answer }
    }
    pub fn test_mode(&self) -> String {
        // println!("chch {:?}", self.chars);
        self.chars
            .iter()
            .map(|val| format!("{}", val.char))
            .collect()
    }
    pub fn interactive_mode(&self) -> String {
        format!(
            "{} {}",
            self.test_mode(),
            ('A'..='Z')
                .map(|ch| {
                    let r = self
                        .chars
                        .iter()
                        .map(|chi| {
                            if chi.char == ch.to_string() {
                                if chi.is_contained {
                                    return "c";
                                }
                                if chi.is_position_correct {
                                    return "y";
                                }
                            }
                            return "";
                        })
                        .collect::<String>();
                    if r != "" {
                        r
                    } else {
                        ch.to_string()
                    }
                })
                .collect::<String>()
        )
    }
}

pub struct Wordle<'a> {
    pub current_guest_times: Cell<usize>,
    pub guess_records: RefCell<Vec<&'a str>>,
}

pub const MAX_TIMES: usize = 6;
pub const WORDS: &str = include_str!("./wordlist.txt");

impl<'a> Wordle<'a> {
    pub fn new() -> Self {
        Wordle {
            current_guest_times: Cell::new(0),
            guess_records: RefCell::new(vec![]),
        }
    }
    pub fn make_guess(&self, word: &'a str) -> Result<GuessResult, String> {
        self.current_guest_times
            .set(self.current_guest_times.get() + 1);
        if self.current_guest_times.get() > MAX_TIMES {
            return Err(format!("exceed max limit {}", WORDS));
        }
        self.guess_records.borrow_mut().push(word);
        let mut chars = vec![];
        for (i, w) in word.chars().enumerate() {
            let mut is_contained = false;
            let mut is_position_correct = false;
            for (ii, ww) in WORDS.chars().enumerate() {
                is_contained = w == ww;
                is_position_correct = w == ww && i == ii;
                if w == ww {
                    break;
                }
            }
            chars.push(ResultCharRepresentation {
                char: String::from(w),
                is_position_correct,
                is_contained,
            });
        }
        Ok(GuessResult {
            chars,
            answer: WORDS,
        })
    }
}

mod test {

    use super::Wordle;
    use super::MAX_TIMES;
    use super::WORDS;

    #[test]
    pub fn a() {
        assert_eq!(WORDS, "CIGAR");
    }

    #[test]
    pub fn test_make_guess_incress_guess_times() {
        let game = Wordle::new();
        assert_eq!(game.current_guest_times.get(), 0);

        for i in 1..=MAX_TIMES {
            let r = game.make_guess("ABCDE").unwrap();

            println!("{} {}", r.test_mode(), r.interactive_mode());

            // println!("r: {:?} {:?}", r, r.chars[0].is_contained);
            assert_eq!(game.current_guest_times.get(), i);
        }

        assert!(game.make_guess("ABCDE").is_err());
    }
}
