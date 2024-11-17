use std::collections::HashSet;


use crate::chapter::{Chapter, Page};

pub struct Maze {
    maze_page: Page,
}

impl Chapter for Maze {
    fn gen(seed: u64) -> Self {
        let mut mz = Maze {
            maze_page: Page::default(),
        };

        let mut walls = [[[1_i32, 1_i32]; 30]; 30];
        let mut cells = [[0_i32; 30]; 30];

        let mut front = HashSet::new();
        
        for i in 0..1 {
            front.insert((fastrand::usize(0..30), fastrand::usize(0..30)));
        }
        // front.insert((1,0));
        let mut add_to_front: Vec<Vec<(usize, usize)>> = vec![vec![], vec![]];
        let mut scores: Vec<i32> = vec![0,0];
        while front.len() > 0 {
            let index = fastrand::usize(0..front.len());
            let index2 = fastrand::usize(0..front.len());
            let cell1 = front.iter().nth(index).unwrap().clone();
            let cell2 = front.iter().nth(index2).unwrap().clone();
            // front.remove(&cell);

            // cells[cell.1][cell.0] = 1;

            let mut neighbors = [(0, 1), (1, 0), (-1, 0), (0, -1)];
            for j in 0..1 {
                let cell = match j {
                    0 => cell1,
                    _ => cell2,
                };

                fastrand::shuffle(&mut neighbors);
                let mut connected = false;
                for n in neighbors {
                    if !(0..30_i32).contains(&(cell.0 as i32 + n.0))
                        || !(0..30_i32).contains(&(cell.1 as i32 + n.1))
                    {
                        continue;
                    }
                    let p = (
                        (cell.0 as i32 + n.0) as usize,
                        (cell.1 as i32 + n.1) as usize,
                    );
                    if cells[p.1][p.0] == 0 {
                        if !front.contains(&p) {
                            add_to_front[j].push(p);
                            scores[j] += 1;
                        }
                    } else {
                        if !connected {
                            match n {
                                (-1, 0) => {
                                    walls[p.1][p.0][0] = 0;
                                }
                                (1, 0) => {
                                    walls[p.1][p.0 - 1][0] = 0;
                                }
                                (0, 1) => {
                                    walls[p.1 - 1][p.0][1] = 0;
                                }
                                (0, -1) => {
                                    walls[p.1][p.0][1] = 0;
                                }
                                _ => {}
                            }
                            connected = true;
                        }
                    }
                }
            }
            for j in 0..1 {
            if scores[j] == 2 {
                scores[j] += 10;
            }}
            if scores[0] > scores[1] {
                cells[cell1.1][cell1.0] = 1;
                front.remove(&cell1);
                for p in add_to_front[0].clone() {
                    if cells[p.1][p.0] == 0 {
                        front.insert(p);
                    }
                }
            } else {
                cells[cell2.1][cell2.0] = 1;
                front.remove(&cell2);
                for p in add_to_front[1].clone() {
                    if cells[p.1][p.0] == 0 {
                        front.insert(p);
                    }
                }
            }
        }

        mz.maze_page.title("MAZE");
        mz.maze_page.paragraph("connect the matching letters");
        mz.maze_page.set_margins(8, 5);
        mz.maze_page.set_cursor_y(5);
        mz.maze_page.paragraph(" ".to_owned() + &"_".repeat(59));
        for (index, row) in walls.iter().enumerate() {
            let s = row.map(|x| match x {
                [0, 0] => if fastrand::f32() > 0.7 {"  "} else {"  "},
                [1, 0] => " |",
                [0, 1] => if index == 29 {"__"} else {"__"},
                [1, 1] => "_|",
                _ => "??",
            });
            mz.maze_page.paragraph("|".to_owned() + &s.join(""));
        }
        let letters = "ABCDEFG".chars().collect::<Vec<char>>();
        mz.maze_page.set_cursor_y(5);
        for i in 0..7 {
            let mut a = (0_i32,0_i32);
            let mut b = (0_i32,0_i32);
            while ((a.0-b.0).pow(2)+(a.1-b.1).pow(2)) < i * i * 5 + 1 
            || mz.maze_page.get(a.0 * 2 + 8, a.1 + 5) != ' '
            || mz.maze_page.get(b.0 * 2 + 8, b.1 + 5) != ' '
            {
                a = (fastrand::i32(1..30),fastrand::i32(1..29));
                b = (fastrand::i32(1..30),fastrand::i32(1..29));
            }

            mz.maze_page.set_cursor(a.0 * 2 + 8, a.1 + 5);
            mz.maze_page.write(letters[i as usize].to_string());
            mz.maze_page.set_cursor(b.0 * 2 + 8, b.1 + 5);
            mz.maze_page.write(letters[i as usize].to_string());

        }
    

        return mz;
    }

    fn pages(&self) -> Vec<&Page> {
        vec![&self.maze_page]
    }
}
