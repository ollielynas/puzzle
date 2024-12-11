use std::{collections::HashMap, hash::Hash};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::chapter::{Chapter, Page, HEIGHT, WIDTH};

pub struct Crossword {
    list: Page,
    crossword: Page,
}

impl Chapter for Crossword {
    fn gen(seed: u64) -> Self {
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

        let mut num_words = 0;

        let mut crossword_map: HashMap<(i32, i32), (char, bool)> = HashMap::new();
        let index = fastrand::usize(0..8);
        let par = paragraphs[index];

        let words = par.split(" ").collect::<Vec<&str>>();
        let word = fastrand::choice(words.clone()).unwrap_or("error");

        let numbers: HashMap<(i32, i32), usize> = HashMap::new();

        for (i, c) in word.chars().enumerate() {
            crossword_map.insert((i as i32, 0), (c, false));
        }

        let mut min_bound_x = 0;
        let mut max_bound_y = 0;
        let mut max_bound_x = 0;
        let mut min_bound_y = 0;

        while num_words < 5 {

            println!("{}", num_words);

            let index = fastrand::usize(0..8);
            let par = paragraphs[index];

            let words = par.split(" ").collect::<Vec<&str>>();

            for (x,y) in crossword_map.keys() {
                max_bound_x = *x.max(&max_bound_x);
                min_bound_x = *x.min(&max_bound_x);
                max_bound_y = *y.max(&max_bound_y);
                min_bound_y = *y.min(&max_bound_y);
            }

            let best = (0..50)
                .into_par_iter()
                .map(|_| {
                    let word = fastrand::choice(words.clone()).unwrap_or("error");
                    let intersect = fastrand::choice(crossword_map.keys()).unwrap_or(&(0, 0));
                    
                    let mut score = -1;
                    if word.len() > 4 && word.chars().all(char::is_alphanumeric) {
                    if let Some(i) = word.find(crossword_map.get(intersect).unwrap().0)  {
                        let mut valid_x = true;
                        let mut valid_y = true;
                        for (l, c) in word.chars().enumerate() {
                            
                            if valid_x
                                && ![None, Some(&(c, true))].contains(
                                    &crossword_map
                                        .get(&(intersect.0 - i as i32 + l as i32, intersect.1)),
                                )
                            {
                                valid_x = false;
                            }
                            if valid_x
                                && matches!(
                                    &crossword_map
                                        .get(&(intersect.0 - i as i32 + l as i32, intersect.1 + 1)), Some(&(_, false)))
                                
                            {
                                valid_x = false;
                            }
                            if valid_x
                                && matches!(
                                    &crossword_map
                                        .get(&(intersect.0 - i as i32 + l as i32, intersect.1 - 1)), Some(&(_, false)))
                                
                            {
                                valid_x = false;
                            }
                            if valid_y
                                && matches!(
                                    &crossword_map
                                        .get(&(intersect.0 - 1, intersect.1  - i as i32 + l as i32)), Some(&(_, true)))
                                
                            {
                                valid_y = false;
                            }
                            if valid_y
                                && matches!(
                                    &crossword_map
                                        .get(&(intersect.0 + 1, intersect.1  - i as i32 + l as i32)), Some(&(_, true)))
                                
                            {
                                valid_x = false;
                            }

                            

                            if valid_y
                                && ![None, Some(&(c, false))].contains(
                                    &crossword_map
                                        .get(&(intersect.0, intersect.1 - i as i32 + l as i32)),
                                )
                            {
                                valid_y = false;
                            }

                        }

                        if valid_x && valid_y {
                            valid_x = fastrand::bool();
                        }

                        // note to self: this is bade code, never do this to yourself again
                        if valid_x {
                            for (l, c) in word.chars().enumerate() {
                                if Some(&(c, true))
                                    == crossword_map
                                        .get(&(intersect.0 - i as i32 + l as i32, intersect.1))
                                {
                                    if max_bound_x.max(intersect.0 - i as i32 + l as i32) - min_bound_x.min(intersect.0 - i as i32 + l as i32) > WIDTH/4 {
                                        score -= 100;
                                    }
                                    score += 1;
                                }
                            }
                            return (score, word, (intersect.0 - i as i32, intersect.1), false);
                        }
                        if valid_y {
                            for (l, c) in word.chars().enumerate() {
                                if Some(&(c, false))
                                == crossword_map
                                .get(&(intersect.0, intersect.1 - i as i32 + l as i32))
                                {
                                    if max_bound_y.max(intersect.0 - i as i32 + l as i32) - min_bound_y.min(intersect.0 - i as i32 + l as i32) > HEIGHT/2 {
                                        score -= 100;
                                    }
                                    score += 1;
                                }
                            }
                            return (score, word, (intersect.0, intersect.1  - i as i32), true);
                        }
                    }}

                    (score, word, (0, 0), false)
                })
                .max_by_key(|x| x.0);

                if let Some((score, word, pos, vertical)) = best {
                    println!("{}", score);
                    if score > -1 {
                        if vertical {
                            for (i, c) in word.chars().enumerate() {
                                crossword_map.insert((pos.0, pos.1 + i as i32), (c, true));
                            }
                        }else {
                            for (i, c) in word.chars().enumerate() {
                                crossword_map.insert((pos.0 + i as i32, pos.1), (c, false));
                            }
                        }

                        num_words += 1;
                    }
                }

        }

        for (x,y) in crossword_map.keys() {
            max_bound_x = *x.max(&max_bound_x);
            min_bound_x = *x.min(&max_bound_x);
            max_bound_y = *y.max(&max_bound_y);
            min_bound_y = *y.min(&max_bound_y);
        }

        let offset_x =   WIDTH / 2 - (min_bound_x + max_bound_x);
        let offset_y =   HEIGHT / 2 - (min_bound_y + max_bound_y);

        for k in crossword_map.keys() { 
            let char = crossword_map.get(k).unwrap();

            crossword.set_cursor(k.0 * 4 + offset_x, k.1 * 2 + offset_y);
            crossword.write(char.0.to_string());
        }

        for p in paragraphs {
            list.paragraph_ex("...", true);
            list.paragraph(p);
        }

        return Crossword { list, crossword };
    }

    fn pages(&self) -> Vec<&crate::chapter::Page> {
        vec![&self.list, &self.crossword]
    }
}
