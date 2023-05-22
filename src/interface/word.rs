use crate::util::rgba8::RGBA8;
use crate::util::vector::Vector2;

pub struct Word {
    pub text: String,
    pub pos: Vector2,
    pub highlight: bool,
    pub fg_color: Option<RGBA8>
}

enum ParsingMod {
    Word,
    GetParam,
    Color,
}

impl Word {
    pub fn get_word_vec(text: &String, pos: Vector2, size: Vector2) -> Vec<Word> {
        let mut words = Vec::new();
        let mut last_word = String::new();
        let mut x_pos = pos.x;
        let mut y_pos = pos.y;
        let max_x = pos.x + size.x;
        let mut parsing_mod = ParsingMod::Word;
        let mut color = None;
        for (i, c) in text.chars().enumerate() {
            match parsing_mod {
                ParsingMod::Word => {
                    if c == '`' {
                        parsing_mod = ParsingMod::GetParam;
                    }
                    else if c.is_whitespace() || i == text.len() - 1 {
                        let new_line = c == '\n';

                        if i == text.len() - 1 {
                            last_word.push(c);
                        }

                        let word = Word {
                            text: last_word.clone(),
                            pos: Vector2::new(x_pos, y_pos),
                            highlight: false,
                            fg_color: color,
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
                ParsingMod::GetParam => {
                    match c {
                        'c' => {
                            parsing_mod = ParsingMod::Color;
                            last_word.clear();
                        }
                        _ => {
                            parsing_mod = ParsingMod::Word;
                        }
                    }
                }
                ParsingMod::Color => {
                    if last_word.len() < 6 {
                        // color information is 6 characters long
                        last_word.push(c);
                    }
                    else {
                        // register color
                        color = Some(RGBA8::from_hex_string(&last_word));
                        last_word.clear();
                        parsing_mod = ParsingMod::Word;
                    }
                }
            }
        }
        words
    }
}