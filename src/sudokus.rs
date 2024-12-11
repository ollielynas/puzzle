use crate::chapter::HEIGHT;
use std::{collections::HashSet, hash::Hash, mem::swap};

use fastrand::{choice, shuffle};
use itertools::Itertools;

use crate::chapter::{Chapter, Page, WIDTH};

pub struct Sudoku {
    ez: Page,
    hd_pg2: Page,
    mega: Page,
}

pub fn solved_sodoku(size: i32) -> Vec<Vec<String>> {
    let mut options: Vec<(usize, usize)> = Vec::new();

    let mut grid = (0..size * size)
        .map(|y| {
            (0..size * size)
                .map(|x| {
                    options.push((x as usize, y as usize));
                    ((x + y / size) % size + (y + x / size) % size * size) as u32 + 1
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    for _ in 0..1000 * size * size {
        let offset1 = size as usize * fastrand::usize(0..3);
        let offset2 = fastrand::usize(0..size as usize - 1);
        let offset3 = fastrand::usize(offset2..size as usize);
        if offset2 == offset3 {
            continue;
        }

        if fastrand::bool() {
            let (a, b) = grid.split_at_mut((offset1 + offset2 + offset1 + offset3) / 2 + 1);
            let l = a.len();
            swap(&mut a[offset1 + offset2], &mut b[offset1 + offset3 - l]);
        } else {
            for i in 0..size * size {
                let (a, b) =
                    grid[i as usize].split_at_mut((offset1 + offset2 + offset1 + offset3) / 2 + 1);
                let l = a.len();
                swap(&mut a[offset1 + offset2], &mut b[offset1 + offset3 - l]);
            }
        }
    }

    return grid
        .into_iter()
        .map(|a| {
            a.iter()
                .map(|b| format!("{:01x}", if size > 3 { b - 1 } else { *b }))
                .collect()
        })
        .collect();
}

fn make_solvable_sodoku(solved_sodoku: &mut Vec<Vec<String>>, mut difficulty: u32) {
    let sample = solved_sodoku[0][0].clone();
    let set: HashSet<String> = HashSet::from_iter(solved_sodoku[0].clone().into_iter());

    let size = (solved_sodoku.len() as f32).sqrt() as usize;

    let mut options = (0..size * size)
        .map(|y| {
            (0..size * size)
                .map(|x| (x as usize, y as usize))
                .collect::<Vec<(usize, usize)>>()
        })
        .concat();

    shuffle(&mut options);

    while difficulty > 0 && options.len() > 0 {
        let (x, y) = options.remove(fastrand::usize(0..options.len()));

        difficulty -= 1;

        let mut possible_values = set.clone();

        for x2 in 0..size * size {
            possible_values.remove(&solved_sodoku[y][x2]);
            if possible_values.len() == 1 {
                break;
            }
        }
        for y2 in 0..size * size {
            possible_values.remove(&solved_sodoku[y2][x]);
            if possible_values.len() == 1 {
                break;
            }
        }

        let base_x = x - x % size;
        let base_y = y - y % size;

        for x2 in 0..size {
            for y2 in 0..size {
                possible_values.remove(&solved_sodoku[base_y + y2][base_x + x2]);
                if possible_values.len() == 1 {
                    break;
                }
            }
        }

        if possible_values.len() == 1 {
            solved_sodoku[y][x] = " ".repeat(sample.len()).to_owned();
        }
    }
}

impl Page {
    fn draw_sudoku(
        &mut self,
        size: i32,
        do_fifty_fifty: bool,
        full_fifty_fifty: bool,
        difficulty: u32,
    ) {
        let min = if size >= 4 { 0 } else { 1 };
        let mut sodoku = solved_sodoku(size);
        if difficulty > 0 {
            make_solvable_sodoku(&mut sodoku, difficulty);
        }
        self.newline();
        self.write(
            "╔".to_string()
                + &(0..size)
                    .map(|_| (0..size).map(|_| "═══").join("╤"))
                    .join("╦")
                + "╗",
        );
        self.newline();
        let mut ln = 0;
        for line in &mut sodoku {
            for l in &mut *line {
                if !(fastrand::bool() || full_fifty_fifty) || !do_fifty_fifty {
                    *l = (" ".to_owned() + l + " ").to_owned();
                } else {
                    if fastrand::bool() {
                        *l = (format!("{}/{:0x}", l, fastrand::i32(0..size * size) + min))
                            .to_owned();
                    } else {
                        *l = (format!("{:0x}/{}", fastrand::i32(0..size * size) + min, l))
                            .to_owned();
                    }
                    while *l
                        == format!(
                            "{}/{}",
                            l.chars().next().unwrap(),
                            l.chars().next().unwrap()
                        )
                    {
                        *l = (format!(
                            "{}/{:0x}",
                            l.chars().next().unwrap(),
                            fastrand::i32(0..size * size) + min
                        ))
                        .to_owned()
                    }
                }
            }

            self.write(
                "║".to_string()
                    + &(0..size)
                        .map(|b| {
                            (0..size as usize)
                                .map(|a| line[a + b as usize * size as usize].clone())
                                .join("│")
                        })
                        .join("║")
                    + "║",
            );
            self.newline();

            ln += 1;
            if ln != size * size {
                self.write(
                    (if ln % size != 0 { "╟" } else { "╠" }).to_string()
                        + &(0..size)
                            .map(|_| {
                                (0..size)
                                    .map(|_| {
                                        if ln % size != 0 {
                                            "───"
                                        } else {
                                            "═══"
                                        }
                                    })
                                    .join(if ln % size != 0 { "┼" } else { "╪" })
                            })
                            .join(if ln % size != 0 { "╫" } else { "╬" })
                        + if ln % size != 0 { "╢" } else { "╣" },
                );
                self.newline();
            }
        }

        self.write(
            "╚".to_string()
                + &(0..size)
                    .map(|_| (0..size).map(|_| "═══").join("╧"))
                    .join("╩")
                + "╝",
        );
    }
}

impl Chapter for Sudoku {
    fn gen(seed: u64) -> Self {
        fastrand::seed(seed);
        let mut page1 = Page::default();
        let mut page2 = Page::default();
        let mut page3 = Page::default();

        page1.title("Sudoku");

        page1.set_margins((WIDTH - (4 * 4 * 4 + 1)) / 2, 0);
        page1.draw_sudoku(4, true, false, 0);

        let page2_y = page2.y();

        page2.set_margins((WIDTH / 2 - (4 * 3 * 3 + 1)) / 2 + 1, 0);
        page2.draw_sudoku(3, true, true, 0);

        page2.newline();

        page2.set_margins((WIDTH / 2 - (4 * 3 * 3 + 1)) / 2 + 1, 0);
        page2.draw_sudoku(3, false, false, 70);

        page2.set_cursor_y(page2_y);
        page2.set_margins(WIDTH / 2 + WIDTH / 4 - (4 * 3 * 3 + 1) / 2, 0);
        page2.draw_sudoku(3, false, false, 50);

        page2.newline();

        page2.set_margins(WIDTH / 2 + WIDTH / 4 - (4 * 3 * 3 + 1) / 2, 0);
        page2.draw_sudoku(3, false, false, 10000);
        
        
        let size = ((WIDTH.min(HEIGHT) / 2) as f32).sqrt() as i32;
        page2.set_margins(WIDTH / 2 + WIDTH / 4 - (size* size * 3 + 1) / 2, 0);
        
        page3.set_cursor_y(0);
        page3.set_cursor_x(page3.left_margin());
        
        let mut mega_sudoku = solved_sodoku(size);

        for a in &mut mega_sudoku {
            for v in a {
                *v = usize::from_str_radix(v, 16).unwrap_or(0).to_string();
                if v.len() < 2 {
                    *v += " ";
                }
            }
        }

        make_solvable_sodoku(&mut mega_sudoku, 10000);
        page3.write(
            "╔".to_string()
                + &(0..size)
                    .map(|_| (0..size).map(|_| "══").join("╤"))
                    .join("╦")
                + "╗",
        );
        page3.newline();
        let mut ln = 0;
        for line in &mut mega_sudoku {
            page3.write(
                "║".to_string()
                    + &(0..size)
                        .map(|b| {
                            (0..size as usize)
                                .map(|a| line[a + b as usize * size as usize].clone())
                                .join("│")
                        })
                        .join("║")
                    + "║",
            );
            page3.newline();

            ln += 1;
            if ln != size * size {
                page3.write(
                    (if ln % size != 0 { "╟" } else { "╠" }).to_string()
                        + &(0..size)
                            .map(|_| {
                                (0..size)
                                    .map(|_| {
                                        if ln % size != 0 {
                                            "──"
                                        } else {
                                            "══"
                                        }
                                    })
                                    .join(if ln % size != 0 { "┼" } else { "╪" })
                            })
                            .join(if ln % size != 0 { "╫" } else { "╬" })
                        + if ln % size != 0 { "╢" } else { "╣" },
                );
                page3.newline();
            }
        }

        page3.write(
            "╚".to_string()
                + &(0..size)
                    .map(|_| (0..size).map(|_| "══").join("╧"))
                    .join("╩")
                + "╝",
        );

        page3.set_cursor_y(0);
        page3.reset_margins();

        Sudoku {
            ez: page1,
            hd_pg2: page2,
            mega: page3,
        }
    }

    fn pages(&self) -> Vec<&Page> {
        vec![&self.ez, &self.hd_pg2, &self.mega]
    }
}
