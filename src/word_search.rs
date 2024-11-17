use fastrand::shuffle;

use crate::chapter::{Chapter, Page, HEIGHT, HEIGHT_M1, WIDTH, WIDTH_M1};




pub struct WordSearch {
    list_page: Page,
    grid_page: Page,
}

impl Chapter for WordSearch {
    fn gen(seed: u64) -> Self {

        fastrand::seed(seed);

        let mut ws = WordSearch {
            list_page: Page::default(),
            grid_page: Page::default(),
        };

        let word_list = include_str!("most_used_passwords.txt");

        let mut words = word_list.lines().collect::<Vec<&str>>();
        shuffle(&mut words);
        ws.list_page.reset_margins();
        ws.list_page.title("WORD SEARCH OF COMMON PASSWORDS");
        let mut new_word_list: Vec<&str> = vec![];
        while new_word_list.len() < 45*4 {
            let word = words.pop().unwrap_or_default();
            if word.len() > 16 {
                continue;
            }
            // for i in 0..10 {
                let vertical = fastrand::i32(0..=1);
                let horizontal = fastrand::i32(if vertical==1 {0} else {1}..=1);

                let mut valid = true;
                let x = fastrand::i32(1..(WIDTH- word.len() as i32 * horizontal - 1));
                let y = fastrand::i32(1..(HEIGHT- word.len() as i32 * vertical - 1));

                for l in 0..word.len() as i32 {
                    let c = ws.grid_page.get(x + l * horizontal, y + l*vertical);
                    if c!=word.chars().nth(l as usize).unwrap_or(' ') && c!=' ' {
                        valid = false;
                    }
                }

                if !valid {
                    continue;
                }
                new_word_list.push(word);
                for l in 0..word.len() as i32 {
                    ws.grid_page.set_cursor(x + l * horizontal, y + l*vertical);
                    ws.grid_page.write(word.chars().nth(l as usize).unwrap_or(' ').to_string());
                    
                }


            // }
        }
        let chars = "@#$%&*1234567890abcdefghijklmenpqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string().chars().collect::<Vec<char>>();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let rand = fastrand::choice(chars.clone()).unwrap_or('#').to_string();
                let t =
                match (x,y, ws.grid_page.get(x, y)) {
                    (0,0,_) => "┌",
                    (0,HEIGHT_M1,_) => "└",
                    (WIDTH_M1,0,_) => "┐",
                    (WIDTH_M1,HEIGHT_M1,_) => "┘",
                    (_,0,_) | (_,HEIGHT_M1,_) => "─" ,
                    (0,_,_) | (WIDTH_M1,_,_) => "│",
                    (_,_,' ') => &rand,
                    (_,_,a) => &a.to_string(),
                };
                ws.grid_page.set_cursor(x, y);
                ws.grid_page.write(t);
            }
        }

        new_word_list.sort();

        ws.list_page.set_margins(2, 19);
        let mut index = 0;
        for i in 5..50 {
            ws.list_page.set_cursor_y(i);
            ws.list_page.paragraph(new_word_list[index]);
            index += 1;
        }

        ws.list_page.set_margins(21, 41);

        for i in 5..50 {
            ws.list_page.set_cursor_y(i);
            ws.list_page.paragraph(new_word_list[index]);
            index += 1;
        }
        ws.list_page.set_margins(41, 59);

        for i in 5..50 {
            ws.list_page.set_cursor_y(i);
            ws.list_page.paragraph(new_word_list[index]);
            index += 1;
        }

        ws.list_page.set_margins(61, 79);

        for i in 5..50 {
            ws.list_page.set_cursor_y(i);
            ws.list_page.paragraph(new_word_list[index]);
            index += 1;
        }
        return ws;
    }

     fn pages(&self) -> Vec<&Page> {
        vec![&self.list_page, &self.grid_page]
    }
}