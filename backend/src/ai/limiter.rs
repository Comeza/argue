use std::time::{Duration, Instant};

// Based on the images (i am to lazy to read) of https://medium.com/geekculture/system-design-design-a-rate-limiter-81d200c9d392
pub struct TokenLimiter {
    last_requst: Instant,
    tokens_per_minute: f32,
    tokens: u32,
    max_tokens: u32,
}

impl TokenLimiter {
    pub fn new(max_tokens: u32) -> Self {
        Self {
            max_tokens,
            tokens: 0,
            tokens_per_minute: 0.0,
            last_requst: Instant::now(),
        }
    }

    pub fn inital_tokens(mut self, tokens: u32) -> Self {
        self.tokens = tokens;
        self
    }

    pub fn token_refresh_rate(mut self, tokens: u32, per: Duration) -> Self {
        self.tokens_per_minute = tokens as f32 / per.as_secs_f32();
        self
    }

    pub fn update_tokens(&mut self) {
        self.tokens = (self.tokens_since(self.last_requst) + self.tokens).clamp(0, self.max_tokens);
    }

    pub fn update_time(&mut self) {
        self.last_requst = Instant::now()
    }

    pub fn tokens_since(&self, last: Instant) -> u32 {
        let tokens = (self.last_requst.duration_since(Instant::now()).as_secs_f32() / 60.0) * self.tokens_per_minute;
        (tokens).clamp(0.0, (self.max_tokens) as f32) as u32
    }

    pub fn is_limited(&self) -> bool {
        (self.tokens + self.tokens_since(Instant::now())) == 0
    }
}
