#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
enum FrameScore {
    Open(u16, u16),
    Spare(u16),
    Strike,
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<FrameScore>,
    score_buffer: Option<u16>,
    n_fill_balls: u8,
    fill_balls_score: u16,
    double_first_ball: bool,
    double_first_ball_score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            score_buffer: None,
            n_fill_balls: 0,
            fill_balls_score: 0,
            double_first_ball: false,
            double_first_ball_score: 0,
        }
    }

    fn done(&self) -> bool {
        self.frames.len() == 10 && self.n_fill_balls == 0
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.done() {
            return Err(Error::GameComplete);
        }
        if self.frames.len() == 10 && self.n_fill_balls > 0 {
            if self.n_fill_balls == 1
                && self.fill_balls_score != 10
                && self.fill_balls_score + pins > 10
            {
                return Err(Error::NotEnoughPinsLeft);
            }
            if self.n_fill_balls == 2 && self.double_first_ball {
                self.double_first_ball_score += pins;
            }
            self.fill_balls_score += pins;
            self.n_fill_balls -= 1;
            return Ok(());
        }
        match self.score_buffer {
            Some(f) => match f + pins {
                score if score > 10 => return Err(Error::NotEnoughPinsLeft),
                10 => {
                    let frame_score = FrameScore::Spare(f);
                    self.frames.push(frame_score);
                    self.score_buffer = None;
                    if self.frames.len() == 10 {
                        self.n_fill_balls = 1;
                    }
                }
                _ => {
                    let frame_score = FrameScore::Open(f, pins);
                    self.frames.push(frame_score);
                    self.score_buffer = None;
                }
            },
            None => match pins {
                10 => {
                    let frame_score = FrameScore::Strike;
                    self.frames.push(frame_score);
                    if self.frames.len() == 9 {
                        self.double_first_ball = true;
                    }
                    if self.frames.len() == 10 {
                        self.n_fill_balls = 2;
                    }
                }
                _ => self.score_buffer = Some(pins),
            },
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.done() {
            return None;
        }
        let mut score = 0_u16;
        let mut double_next_n = 0_u64;
        for frame in self.frames.iter() {
            match frame {
                FrameScore::Open(s1, s2) => {
                    if double_next_n == 3 {
                        score += 3 * s1 + 2 * s2;
                        double_next_n = 0;
                    } else if double_next_n == 2 {
                        score += 2 * (s1 + s2);
                        double_next_n = 0;
                    } else if double_next_n == 1 {
                        score += 2 * s1 + s2;
                        double_next_n = 0;
                    } else {
                        score += s1 + s2;
                    }
                }
                FrameScore::Spare(s) => {
                    if double_next_n == 3 {
                        score += 3 * s + 2 * (10 - s);
                        double_next_n = 1;
                    } else if double_next_n == 2 {
                        score += 20;
                        double_next_n = 1;
                    } else if double_next_n == 1 {
                        score += 2 * s + (10 - s);
                        // We do not need to change double_next_n
                        // Spare increases it by one, here we should decrease it also by one
                        // It balances out
                    } else {
                        score += 10;
                        double_next_n += 1;
                    }
                }
                FrameScore::Strike => {
                    if double_next_n == 3 {
                        score += 30;
                    } else if double_next_n >= 1 {
                        score += 20;
                        double_next_n += 1;
                    } else {
                        score += 10;
                        double_next_n += 2;
                    }
                }
            }
        }
        Some(score + self.fill_balls_score + self.double_first_ball_score)
    }
}
