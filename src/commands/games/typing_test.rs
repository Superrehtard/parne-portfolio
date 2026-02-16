use crate::commands::{games, CommandOutput, LineStyle};

const SENTENCES: &[&str] = &[
    "the quick brown fox jumps over the lazy dog",
    "rust is a systems programming language focused on safety",
    "webassembly enables high performance applications on the web",
    "leptos is a reactive framework for building web applications",
    "practice makes perfect when learning to type faster",
    "the terminal is a powerful tool for developers",
    "coding is both an art and a science requiring creativity",
    "open source software powers most of the internet today",
];

#[derive(Clone)]
pub struct TypingTest {
    pub target: String,
    pub start_time: Option<f64>,
    pub finished: bool,
}

impl TypingTest {
    pub fn new() -> Self {
        let idx = (js_sys::Math::random() * SENTENCES.len() as f64) as usize;
        let idx = idx.min(SENTENCES.len() - 1);

        Self {
            target: SENTENCES[idx].to_string(),
            start_time: None,
            finished: false,
        }
    }

    pub fn start_output(&self) -> CommandOutput {
        CommandOutput::new(
            vec![
                (String::new(), LineStyle::Normal),
                ("  === TYPING SPEED TEST ===".to_string(), LineStyle::Accent),
                (
                    "  Type the following text as fast as you can:".to_string(),
                    LineStyle::Normal,
                ),
                ("  Type 'quit' to exit.".to_string(), LineStyle::Muted),
                (String::new(), LineStyle::Normal),
                (format!("  > {}", self.target), LineStyle::Accent),
                (String::new(), LineStyle::Normal),
            ],
            false,
            false,
        )
    }

    pub fn handle_input(&mut self, input: &str) -> CommandOutput {
        let input = input.trim();

        // Start timer on first input
        if self.start_time.is_none() {
            self.start_time = Some(js_sys::Date::now());
        }

        let elapsed_ms = js_sys::Date::now() - self.start_time.unwrap();
        let elapsed_secs = elapsed_ms / 1000.0;

        // Calculate accuracy
        let target_chars: Vec<char> = self.target.chars().collect();
        let input_chars: Vec<char> = input.chars().collect();
        let mut correct = 0;
        let total = target_chars.len();

        for (i, ch) in input_chars.iter().enumerate() {
            if i < target_chars.len() && *ch == target_chars[i] {
                correct += 1;
            }
        }

        let accuracy = if total > 0 {
            (correct as f64 / total as f64 * 100.0) as u32
        } else {
            0
        };

        // Calculate WPM (standard: 5 chars = 1 word)
        let word_count = input.len() as f64 / 5.0;
        let minutes = elapsed_secs / 60.0;
        let wpm = if minutes > 0.0 {
            (word_count / minutes) as u32
        } else {
            0
        };

        self.finished = true;

        let rating = match wpm {
            0..=29 => "Keep practicing!",
            30..=49 => "Not bad!",
            50..=69 => "Good speed!",
            70..=89 => "Impressive!",
            90..=119 => "Blazing fast!",
            _ => "Are you a robot?!",
        };

        CommandOutput::new(
            vec![
                (String::new(), LineStyle::Normal),
                ("  === RESULTS ===".to_string(), LineStyle::Accent),
                (String::new(), LineStyle::Normal),
                (format!("  Speed:    {} WPM", wpm), LineStyle::Normal),
                (format!("  Accuracy: {}%", accuracy), LineStyle::Normal),
                (
                    format!("  Time:     {:.1}s", elapsed_secs),
                    LineStyle::Normal,
                ),
                (format!("  Rating:   {}", rating), LineStyle::Accent),
                (String::new(), LineStyle::Normal),
                (
                    "  Type 'typing' to try again, or 'quit' to exit.".to_string(),
                    LineStyle::Muted,
                ),
            ],
            false,
            false,
        )
    }
}
