use crate::chapter::{Chapter, Page, HEIGHT, HEIGHT_M1, WIDTH, WIDTH_M1};



pub struct Shape {
    empty: Page,
    instruction: Page,
}

fn get_x_y_z(offset: [[i32;3];3],height: [[i32;3];3], x: i32, y: i32, z: i32) -> bool {
    if !(0..3).contains(&x) || !(0..3).contains(&z) ||!(0..3).contains(&y){return false}
    offset[z as usize][x as usize] <= y && y - offset[z as usize][x as usize] < height[z as usize][x as usize] 
} 

impl Chapter for Shape {
    fn gen(seed: u64) -> Self {
        fastrand::seed(seed);

        let mut empty = Page::default();

        for y in 0..HEIGHT {
                empty.set_cursor(0,y);            
                empty.write((if y%2==0 { "  . " } else {".   "}).repeat(WIDTH as usize/2));
            
        }

        let mut s = [[0;3];3];
        let mut h = [[0;3];3];

        for row in 0..3 {
            for col in 0..3 {
                h[row][col] = fastrand::i32(0..2);
                
                s[row][col] = fastrand::i32(0..=(3-h[row][col]));

 
        }
        }

        let mut instruction: Page = Page::default();


        instruction.title("Isometric Drawing");


        let center = (WIDTH/2, HEIGHT/2);
        
        for z in -1..=3 {
        for x in -1..=3 {
            for y in -1..=3 {
                    let pos = (x*4 + center.0 - 16, y * 2 + center.1 + 5);

                    

                    
                    let solid = get_x_y_z(h, s, x, y, z);
                    let solid_above = get_x_y_z(h, s, x, y - 1, z);
                    let solid_right = get_x_y_z(h, s, x + 1, y, z);
                    

                    
                    match (solid, solid_above)  {
                        (true, true) => {
                            if instruction.get(pos.0, pos.1 - 1) == '─' {
                                instruction.set_cursor(pos.0 - 1, pos.1 - 1);
                                instruction.write("┄┄┄");
                            }
                        }
                        (true, false) | (false, true) => {
                            instruction.set_cursor(pos.0 - 1, pos.1 - 1);
                            instruction.write("───");
                        },
                        (false, false) => {},
                    }
                    match (solid, solid_right)  {
                        (true, true) => {
                            if instruction.get(pos.0 + 2, pos.1) == '│' {
                                instruction.set_cursor(pos.0 + 2, pos.1);
                                instruction.write("┊");
                            }
                        }
                        (true, false) | (false, true) => {
                            instruction.set_cursor(pos.0 + 2, pos.1);
                            instruction.write("│");
                        },
                        (false, false) => {},
                    }


                }
            }

        }
        for x in -1..=3 {
            for z in -1..=3 {
            for y in -1..=3 {
                    let pos = (z*4 + center.0 + 3, y * 2 + center.1 + 5);

                    

                    
                    let solid = get_x_y_z(h, s, x, y, z);
                    let solid_above = get_x_y_z(h, s, x, y - 1, z);
                    let solid_right = get_x_y_z(h, s, x, y, z + 1);
                    

                    match (solid, solid_above)  {
                        (true, true) => {
                            if instruction.get(pos.0, pos.1 - 1) == '─' {
                                instruction.set_cursor(pos.0 - 1, pos.1 - 1);
                                instruction.write("┄┄┄");
                            }
                        }
                        (true, false) | (false, true) => {
                            instruction.set_cursor(pos.0 - 1, pos.1 - 1);
                            instruction.write("───");
                        },
                        (false, false) => {},
                    }
                    match (solid, solid_right)  {
                        (true, true) => {
                            if instruction.get(pos.0 + 2, pos.1) == '│' {
                                instruction.set_cursor(pos.0 + 2, pos.1);
                                instruction.write("┊");
                            }
                        }
                        (true, false) | (false, true) => {
                            instruction.set_cursor(pos.0 + 2, pos.1);
                            instruction.write("│");
                        },
                        (false, false) => {},
                    }


                }
            }

        }
        for y in (-1..=3).rev() {
        for x in -1..=3 {
            for z in -1..=3 {
                    let pos = (x*4 + center.0 - 16, (z) * 2 + center.1 - 5);

                    


                    
                    let solid = get_x_y_z(h, s, x, y, z);
                    let solid_above = get_x_y_z(h, s, x, y, z - 1);
                    let solid_right = get_x_y_z(h, s, x + 1, y, z);
                    

                    match (solid, solid_above)  {
                        (true, true) => {
                            if instruction.get(pos.0, pos.1 - 1) == '─' {
                                instruction.set_cursor(pos.0 - 1, pos.1 - 1);
                                instruction.write("┄┄┄");
                            }
                        }
                        (true, false) | (false, true) => {
                            instruction.set_cursor(pos.0 - 1, pos.1 - 1);
                            instruction.write("───");
                        },
                        (false, false) => {},
                    }
                    match (solid, solid_right)  {
                        (true, true) => {
                            if instruction.get(pos.0 + 2, pos.1) == '│' {
                                instruction.set_cursor(pos.0 + 2, pos.1);
                                instruction.write("┊");
                            }
                        }
                        (true, false) | (false, true) => {
                            instruction.set_cursor(pos.0 + 2, pos.1);
                            instruction.write("│");
                        },
                        (false, false) => {},
                    }


                }
            }

        }

        // TODO: do this in a more efficient way, its kinda annoying
        for x in 1..WIDTH_M1 {
            for y in 1..HEIGHT_M1 {
                if instruction.get(x, y) == ' ' {
                
                    let text = match (
                        instruction.get(x - 1, y),
                        instruction.get(x + 1, y),
                        instruction.get(x, y - 1),
                        instruction.get(x, y + 1)
                    ) {
                        ('─'|'┄', '─'|'┄', ' ', ' ') => "─",
                        ('─'|'┄', '─'|'┄', '│'|'┊', '│'|'┊') => "┼",
                        ('─'|'┄', '─'|'┄', ' ', '│'|'┊') => "┬",
                        (' ', ' ', '│'|'┊', '│'|'┊') => "│",
                        (' ', '─'|'┄', ' ', '│'|'┊') => "┌",
                        ('─'|'┄', ' ', ' ', '│'|'┊') => "┐",
                        (' ', '─'|'┄', '│'|'┊', ' ') => "└",
                        ('─'|'┄', ' ', '│'|'┊', ' ') => "┘",
                        ('─'|'┄', '─'|'┄', '│'|'┊', ' ') => "┴",
                        (' ', '─'|'┄', ' ', ' ') => "─",
                        ('─'|'┄', ' ', ' ', ' ') => "─",
                        (' ', ' ', '│'|'┊', ' ') => "│",
                        (' ', '─'|'┄', '│'|'┊', '│'|'┊') => "├",
                        ('─'|'┄', ' ', '│'|'┊', '│'|'┊') => "┤",
                        _ => " "
                    };
                    
                instruction.set_cursor(x, y);   
                instruction.write(text);
            }
            }
        }



        Shape {
            empty,
            instruction

        }
    }

    fn pages(&self) -> Vec<&crate::chapter::Page> {
        [&self.instruction, &self.empty].to_vec()
    }
}




