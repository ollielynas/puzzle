use std::default;

pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 50;
pub const HEIGHT_M1: i32 = HEIGHT - 1;
pub const WIDTH_M1: i32 = WIDTH - 1;

#[derive(Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}
#[derive(Clone)]
pub struct Page {
    pub title: Option<String>,
    margin_left: i32,
    margin_right: i32,
    cursor_pos: Pos,
    text: [[char;WIDTH as usize];HEIGHT as usize]
}

impl Default for Page {
    fn default() -> Self {
        Page {
            margin_left: 2,
            title: None,
            margin_right: 3,
            cursor_pos: Pos {
                x: 3,
                y: 3,
            },
            text: [[' ';WIDTH as usize];HEIGHT as usize],
        }
    }
}

impl ToString for Page {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for r in self.text {
            if s != String::new() {
                s+="\n";
            }
            for c in r {
                s += &c.to_string();
            }
        }
        return s;
    }
}

impl Page {
    pub fn reset_margins(&mut self) {
        self.margin_left = 3;
        self.margin_right = 3;
    }
    pub fn set_margins(&mut self, left: i32, right: i32) {
        self.margin_left = left.clamp(0, 79);
        self.margin_right = right.clamp(0, 79);
    }
    pub fn set_cursor(&mut self, x: i32, y: i32) {
        self.cursor_pos = Pos {
            x:x.clamp(0, WIDTH - 1),
            y:y.clamp(0, HEIGHT - 1)
        }
    }
    pub fn get(&mut self, x: i32, y: i32) -> char {
        let p = Pos {
            x:x.clamp(0, WIDTH - 1),
            y:y.clamp(0, HEIGHT - 1)
        };
        self.text[p.y as usize][p.x as usize]
    }
    pub fn set_cursor_x(&mut self, x: i32) {
        self.cursor_pos.x = x.clamp(0, WIDTH - 1);
    }
    pub fn set_cursor_y(&mut self, y: i32) {
        self.cursor_pos.y = y.clamp(0, HEIGHT - 1);
    }
    pub fn write(&mut self, text:  impl AsRef<str>) {
        for c in text.as_ref().chars() {
            self.text[self.cursor_pos.y as usize][self.cursor_pos.x as usize] = c;
            self.set_cursor(self.cursor_pos.x + 1, self.cursor_pos.y);
        }
    }

    pub fn title(&mut self, text:  impl AsRef<str>) {
        self.title = Some(text.as_ref().to_string());
        let a  = (WIDTH - self.margin_left - self.margin_right) / 2 - (text.as_ref().len() as i32 + 8) / 2;
        self.paragraph(format!("{}--- {} ---"," ".repeat(a.clamp(0, WIDTH) as usize), text.as_ref().to_string()));
        
    }
    pub fn paragraph(&mut self, text:  impl AsRef<str>) {
        self.cursor_pos.x = self.margin_left;
        let max_width = WIDTH - (self.margin_left + self.margin_right);
        let mut word = String::new();
        let mut line = String::new();
        
        for c in text.as_ref().chars().chain(" \n".chars()) {
            if ![' ', '\n'].contains(&c) {
                word += &c.to_string();
            }else {
                if line.len() + word.len() + 1 > max_width as usize || c=='\n' {
                    let mut newline = c=='\n';
                    while line.len() + word.len() + 1 > max_width as usize || newline {
                        newline = false;
                        self.write(&line);
                        self.cursor_pos.x = self.margin_left;
                        self.cursor_pos.y += 1;
                        self.cursor_pos.y = self.cursor_pos.y.clamp(0, 39);
                        if word.len() > max_width as usize {
                        line = word.split_off(max_width as usize) + " ";
                        }else {
                            line = word + " ";
                            word = String::new();
                        }
                    }
                }else {
                    line += &word;
                    line += " ";
                    word = String::new();
                }
            }

        }
    }
}

pub trait Chapter {
    fn gen(seed: u64) -> Self;
    fn pages(&self) -> Vec<&Page>;
    fn pages_owned(&self) -> Vec<Page> {
        self.pages().iter().map(|x| (*x).clone()).collect()
    }
}