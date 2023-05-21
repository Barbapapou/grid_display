use crate::util::Vector2;

pub struct Word {
    pub text: String,
    pub pos: Vector2,
}

impl Word {
    pub fn get_word_vec(text: &String, pos: Vector2, size: Vector2) -> Vec<Word> {
        let mut words = Vec::new();
        let mut last_word = String::new();
        let mut x_pos = pos.x;
        let mut y_pos = pos.y;
        let max_x = pos.x + size.x;
        for (i, c) in text.chars().enumerate() {
            if c.is_whitespace() || i == text.len() - 1 {
                let new_line = c == '\n';

                let word = Word {
                    text: last_word.clone(),
                    pos: Vector2::new(x_pos, y_pos),
                };

                words.push(word);
                x_pos += last_word.len() as i32 + 1;
                last_word.clear();

                if new_line {
                    y_pos += -1;
                    x_pos = pos.x;
                }
            }
            else {
                last_word.push(c);
                if x_pos + last_word.len() as i32 > max_x {
                    x_pos = pos.x;
                    y_pos += -1;
                }
            }
        }
        words
    }
}