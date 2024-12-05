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
    text: [[char;WIDTH as usize];HEIGHT as usize],
    page_number: Option<i32>,
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
            page_number: None,
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
    pub fn large_margins(&mut self) {
        self.margin_left = WIDTH/6;
        self.margin_right = WIDTH/6;
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

    pub fn x(&self) -> i32 {
        self.cursor_pos.x
    }

    pub fn y(&self) -> i32 {
        self.cursor_pos.y
    }

    pub fn newline(&mut self) {
        self.set_cursor_y(self.y() + 1);
        self.set_cursor_x(self.margin_left);
    }

    pub fn width_with_margins(&self) -> i32 {
        WIDTH - self.margin_left - self.margin_right
    }

    pub fn key_value_element(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        self.set_cursor_x(self.margin_left);

        let k_len = key.as_ref().len();
        let v_len = value.as_ref().len();

        if k_len + v_len >= self.width_with_margins() as usize {
            self.write([key.as_ref(), &".".repeat(WIDTH as usize - k_len - self.margin_right as usize)].concat());
            self.newline();
            self.write([&".".repeat(WIDTH as usize - v_len - self.margin_right as usize), value.as_ref()].concat());
        }else {
            self.write([ key.as_ref(), &".".repeat(self.width_with_margins() as usize - v_len - k_len), value.as_ref()].concat());
        }
        self.newline();
        
    }

    pub fn title(&mut self, text:  impl AsRef<str>) {
        self.title = Some(text.as_ref().to_string());
        let a  = (WIDTH - self.margin_left - self.margin_right) / 2 - (text.as_ref().len() as i32 + 8) / 2;
        self.paragraph_ex(format!("--- {} ---", text.as_ref().to_string()), true);
        self.newline();
    }
    pub fn paragraph(&mut self, text:  impl AsRef<str>) {
        self.paragraph_ex(text, false);
    }
    pub fn paragraph_ex(&mut self, text:  impl AsRef<str>, center: bool) {
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

                        if center {
                            self.cursor_pos.x += (self.width_with_margins() - line.len() as i32).max(0) / 2;
                        }
                        self.write(&line.trim());
                        self.cursor_pos.x = self.margin_left;
                        self.cursor_pos.y += 1;
                        self.cursor_pos.y = self.cursor_pos.y.clamp(0, HEIGHT_M1);
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