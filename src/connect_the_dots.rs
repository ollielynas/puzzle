use bresenham::Bresenham;
use fastrand::{choice, shuffle};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use resvg::{
    tiny_skia::{self, Mask, Pixmap},
    usvg::{Options, Size, Transform},
};

use crate::chapter::{Chapter, Page, HEIGHT, HEIGHT_M1, WIDTH, WIDTH_M1};

pub struct ConnectTheDots {
    page: Page,
}

impl Chapter for ConnectTheDots {
    fn gen(seed: u64) -> Self {
        fastrand::seed(seed);

        let mut page = Page::default();
        page.title("CONNECT THE DOTS");
        page.paragraph("connect the dots with matching ids to create the image");

        let svg_list = include_str!("svg_list.txt").lines();
        let svg_text =
            choice(svg_list.filter(|x| x.len() > 3).collect::<Vec<&str>>()).unwrap_or_default();
        let tree = resvg::usvg::Tree::from_str(svg_text, &Options {
            default_size: Size::from_wh(256.0, 256.0).unwrap(),
            style_sheet: Some("scale: 2; width: 256px !important; height: 256px !important;".to_owned()),
            ..Default::default()
        }
        
    ).unwrap();
        
        let mut px_map_nmt = Pixmap::new(256, 256).unwrap();
        let mut pixel_map = px_map_nmt.as_mut();
        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixel_map);

        let mut pixels = vec![];

        for px in pixel_map.pixels_mut() {
            pixels.push(px.alpha() > 254);
        }

        let size = (WIDTH / 2).min(HEIGHT - 5) / 3;

        let mut pos_list = (0..size)
            .map(|x| {
                (0..size)
                    .map(move |y| (x.clone(), y))
                    .collect::<Vec<(i32, i32)>>()
            })
            .collect::<Vec<Vec<(i32, i32)>>>()
            .concat();
        shuffle(&mut pos_list);
        let mut connections = vec![];

        let mut pos1 = pos_list.remove(fastrand::usize(0..pos_list.len()));
        while pos_list.len() > 0 {

            let connection = (0..50.min(pos_list.len()))
                .into_par_iter()
                .map(|_index| {
                    let index = fastrand::usize(0..pos_list.len());
                    let pos2 = pos_list[index].clone();
                    let mut score = 0;
                    for (x, y) in Bresenham::new(
                        (
                            ((pos1.0 as f32 / size as f32) * 256.0 as f32 / 7.0) as isize,
                            ((pos1.1 as f32 / size as f32) * 256.0 as f32 / 7.0) as isize,
                             ),
                        (
                            ((pos2.0 as f32 / size as f32) * 256.0 as f32 / 7.0) as isize,
                            ((pos2.1 as f32 / size as f32) * 256.0 as f32 / 7.0) as isize,
                            ),
                    ) {
                        if pixels[(x + y * 256) as usize] {
                            score += 1;
                        } else {
                            score -= 30;
                        }
                    }

                    return (score, index);
                })
                .max_by_key(|x| x.0);

            connections.push((pos1.clone(), connection.unwrap().0));
            pos1 = pos_list.remove(connection.unwrap().1);

        }

        for (i, (pos, score)) in connections.iter().enumerate() {
           
                page.set_cursor((pos.0 * 2) * 3, HEIGHT_M1 - pos.1 * 3);
                page.write(format!("{}{}", if *score < 0 {"."} else {"*"}, i+1));
            
        }


        return ConnectTheDots { page };
    }

    fn pages(&self) -> Vec<&Page> {
        vec![&self.page]
    }
}
