use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use fastrand::shuffle;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::chapter::{Chapter, Page, HEIGHT, WIDTH};

pub struct Crossword {
    list: Page,
    crossword: Page,
}



pub struct Word {
    word: Vec<char>,
    vertical: bool,
    chars: HashSet<char>,
    pos: (i32, i32),
    paragraph: (usize, usize),
    /// 2 point rect (x,y - x,y)
    bounds: (i32, i32, i32, i32),
}

impl Chapter for Crossword {
    fn gen(seed: u64) -> Self {
        fastrand::seed(seed);

        let mut list = Page::default();
        let mut crossword = Page::default();

        list.title("Public Domain Crossword");

        list.paragraph_ex(
            "What are the words missing from these excerpts of famous books?",
            true,
        );
        list.newline();
        let word_list = include_str!("book_para.txt");
        let mut paragraphs = word_list.lines().collect::<Vec<&str>>();

        paragraphs = fastrand::choose_multiple(paragraphs, 8);

        

        let mut used_words = HashSet::new();

        let mut crossword_words: Vec<Word> = vec![];
        let mut iter = 0;
        while crossword_words.len() < 30 {

            if iter > 100000 {
                break
            }else {
                iter += 1;
            }

            let paragraph = fastrand::usize(0..paragraphs.len());
            let words = paragraphs[paragraph].split(" ").collect::<Vec<&str>>();
            let word_index = fastrand::usize(0..words.len());
            let word = words[word_index];
            let word_chars = words[word_index].chars().collect::<Vec<char>>();
            let letters: HashSet<char> = HashSet::from_iter(word.chars());

            if used_words.contains(word) {
                continue;
            }

            // shuffle(&mut crossword_words);

            if !(word.len() > 5 && word.chars().all(char::is_alphanumeric)) {
                continue;
            }

            if crossword_words.len() == 0 && word.len() > 8 {
                let pos = (WIDTH/12,HEIGHT/3);
                crossword_words.push(
                    Word { word: word_chars, vertical: false, chars: letters, pos, paragraph: (paragraph, word_index), bounds: (pos.0 - 1, pos.1 - 1, pos.0 + word.len() as i32 + 1, pos.1 + 1) }
                );
                continue;
            }

            for word2 in &crossword_words {
                let intersection: Vec<&char> = word2.chars.intersection(&letters).collect();
                if intersection.len() == 0 {
                    continue;
                }
                let int_char = fastrand::choice(intersection).unwrap();
                let (new_word_index, _) = word.chars().find_position(|x| x == int_char).unwrap();
                let (old_word_index, _) = word2
                    .word
                    .clone()
                    .into_iter()
                    .find_position(|x| x == int_char)
                    .unwrap();
                let vertical = !word2.vertical;

                let pos = if vertical {
                    (
                        word2.pos.0 + old_word_index as i32,
                        word2.pos.1 - new_word_index as i32,
                    )
                } else {
                    (
                        word2.pos.0 - new_word_index as i32,
                        word2.pos.1 + old_word_index as i32,
                    )
                };

                let bounds = if vertical {
                    (pos.0 - 1, pos.1 - 1, pos.0 + 1, pos.1 + word.len() as i32 + 1)
                } else {
                    (pos.0 - 1, pos.1 - 1, pos.0 + word.len() as i32 + 1, pos.1 + 1)
                };

                let mut valid = true;

                if !(bounds.0 > 0 && bounds.1 > 0 && bounds.2 < WIDTH/4 && bounds.3 < HEIGHT/2) {
                    continue;
                }

                for word3 in &crossword_words {
                    let intersect_width = (bounds.2.min(word3.bounds.2) - bounds.0.max(word3.bounds.0)).max(0);
                    let intersect_height = (bounds.3.min(word3.bounds.3) - bounds.1.max(word3.bounds.1)).max(0);
                    let intersect_area = intersect_width * intersect_height;
                    if word3.word == word_chars {
                        valid = false;
                        break;
                    }

                    if intersect_area > 1 {
                        if word3.vertical == vertical {
                            valid = false;
                            break;
                        }

                        let new_intersect_index = if vertical {
                            (word3.pos.1 - pos.1) as usize
                        } else {
                            (word3.pos.0 - pos.0) as usize
                        };
                        let old_intersect_index = if vertical {
                            (pos.0 - word3.pos.0) as usize
                        } else {
                            (pos.1 - word3.pos.1) as usize
                        };

                        if new_intersect_index < word_chars.len() && old_intersect_index < word3.word.len() {

                        if new_word_index == 0 && new_intersect_index == old_intersect_index {
                            valid = false;
                            break;
                        }
                        
                        if word_chars[new_intersect_index] != word3.word[old_intersect_index] {
                            valid = false;
                            break;
                        }}else {
                            valid = false;
                            break;
                        }
                    }
                }

                if !valid {
                    continue;
                }

                // beyond this point we assume all checks are done and the word is valid

                used_words.insert(word);

                crossword_words.push(Word {
                    word: word_chars.clone(),
                    vertical,
                    chars: letters.clone().clone(),
                    pos,
                    paragraph: (paragraph, word_index),
                    bounds,
                });

                break
            }
        }
        let new_words: Vec<String> = crossword_words.iter().map(|x| x.word.iter().collect::<String>()).collect::<Vec<String>>();

        let mut paragraphs = paragraphs.into_iter().map(|text| text.split(" ").map(|x| x.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

        crossword_words.sort_by_key(|x| x.paragraph.0 * 1000 + x.paragraph.1);

        
        for w in &crossword_words {
            for (i, _) in w.word.iter().enumerate() {
                let pos = if w.vertical 
                {(w.pos.0, w.pos.1 + i as i32)} else {
                    (w.pos.0 + i as i32, w.pos.1)
                };


                crossword.set_cursor(pos.0 * 4 - 2, pos.1 * 2);
                crossword.write("│");
                crossword.set_cursor(pos.0 * 4 + 2, pos.1 * 2);
                crossword.write("│");
                crossword.set_cursor(pos.0 * 4, pos.1 * 2 + 1);
                crossword.write("─");
                crossword.set_cursor(pos.0 * 4 , pos.1 * 2 - 1);
                crossword.write("─");
                crossword.set_cursor(pos.0 * 4 + 1, pos.1 * 2 + 1);
                crossword.write("─");
                crossword.set_cursor(pos.0 * 4 - 1 , pos.1 * 2 - 1);
                crossword.write("─");
                crossword.set_cursor(pos.0 * 4 - 1, pos.1 * 2 + 1);
                crossword.write("─");
                crossword.set_cursor(pos.0 * 4 + 1 , pos.1 * 2 - 1);
                crossword.write("─");
            }


        }

        
        
        for (i, w) in crossword_words.iter().enumerate() {
            for (i, _) in w.word.iter().enumerate() {
                let pos = if w.vertical 
                {(w.pos.0, w.pos.1 + i as i32)} else {
                    (w.pos.0 + i as i32, w.pos.1)
                };


                crossword.set_cursor(pos.0 * 4, pos.1 * 2);
                if crossword.get(pos.0 * 4, pos.1 * 2) == ' ' {
                    crossword.write("X");
                }
            }
        }

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                match (
                    crossword.get(x, y),
                    crossword.get(x, y + 1),
                    crossword.get(x, y - 1),
                    crossword.get(x + 2, y),
                    crossword.get(x - 2, y),
                ) {
                    ('X',
                    _,
                    _,
                    _,
                    _,
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write(" ");
                    }
                    (' ',
                    '─',
                    '─',
                    '│',
                    '│',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("X");
                    }
                    (' ',
                    '│',
                    '│',
                    '─',
                    '─',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("┼");
                    }
                    (' ',
                    ' ',
                    '│',
                    '─',
                    '─',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("┴");
                    }
                    (' ',
                    '│',
                    ' ',
                    '─',
                    '─',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("┬");
                    }
                    (' ',
                    '│',
                    '│',
                    ' ',
                    '─',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("┤");
                    }
                    (' ',
                    '│',
                    '│',
                    '─',
                    ' ',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("├");
                    }
                    
                    (' ',
                    ' ',
                    '│',
                    '─',
                    ' ',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("└");
                    }
                    (' ',
                    ' ',
                    '│',
                    ' ',
                    '─',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("┘");
                    }
                    (' ',
                    '│',
                    ' ',
                    ' ',
                    '─',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("┐");
                    }
                    (' ',
                    '│',
                    ' ',
                    '─',
                    ' ',
                    )  
                    => {
                        crossword.set_cursor(x, y);
                        crossword.write("┌");
                    }



                    _ => {}
                }
            }
        }

        let mut unused_paragraphs: HashSet<usize> = HashSet::from_iter(0..paragraphs.len());

        for (i, w) in crossword_words.iter().enumerate() {
            paragraphs[w.paragraph.0][w.paragraph.1] = format!("[{},{}]", i, if w.vertical {"down"} else {"across"});

            crossword.set_cursor(w.pos.0 * 4 - if i >= 10 {1} else {0}, w.pos.1 * 2);
            crossword.write(i.to_string());

            unused_paragraphs.remove(&w.paragraph.0);
        }



        for (i, text) in paragraphs.into_iter().enumerate() {
            if unused_paragraphs.contains(&i) {
                continue;
            }
            let text = text.join(" ").replace("\n", ";");
            list.newline();
            list.paragraph(text);
        }

        return Crossword { list, crossword };
    }

    fn pages(&self) -> Vec<&crate::chapter::Page> {
        vec![&self.list, &self.crossword]
    }
}
