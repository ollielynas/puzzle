use crate::chapter::{Chapter, Page, HEIGHT_M1, WIDTH};
use crate::connect_the_dots::ConnectTheDots;
use crate::dyslexic_word_search::DyslexicWordSearch;
use crate::maze::Maze;
use crate::waldo::Waldo;
use crate::word_search::WordSearch;
use chrono::Duration;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::u64;
use strum::{self, IntoEnumIterator};
use strum_macros::EnumIter;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_timer::Instant;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(EnumIter, PartialEq, Eq, Copy, Clone, Hash)]
enum ChapterEnum {
    Maze,
    Waldo,
    WordSearch,
    DyslexicWordSearch,
    ConnectTheDots,
}

impl ChapterEnum {
    pub fn gen_pages(&self, seed: u64) -> Vec<Page> {
        match self {
            ChapterEnum::Maze => Maze::gen(seed).pages_owned(),
            ChapterEnum::WordSearch => WordSearch::gen(seed).pages_owned(),
            ChapterEnum::Waldo => Waldo::gen(seed).pages_owned(),
            ChapterEnum::DyslexicWordSearch => DyslexicWordSearch::gen(seed).pages_owned(),
            ChapterEnum::ConnectTheDots => ConnectTheDots::gen(seed).pages_owned(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ChapterEnum::Maze => "Maze",
            ChapterEnum::Waldo => "Where's Waldo",
            ChapterEnum::WordSearch => "Word Search",
            ChapterEnum::DyslexicWordSearch => "Scramble Search",
            ChapterEnum::ConnectTheDots => "Connect The Dots",
        }
    }
}

pub struct Book {
    structure: Vec<ChapterEnum>,
    seed: u64,
}

impl Book {
    pub fn create(seed: u64) -> Book {
        Book {
            seed,
            structure: ChapterEnum::iter().collect::<Vec<ChapterEnum>>(),
        }
    }

    pub fn repeat_chapter(&mut self, chapter: ChapterEnum, count: usize) {
        self.structure = self
            .structure
            .iter()
            .map(|x| {
                if x == &chapter {
                    [x.clone()].repeat(count)
                } else {
                    vec![x.clone()]
                }
            })
            .concat();
    }

    fn stats_page(&self, gen_time: Duration) -> Page {
        let mut page = Page::default();
        page.title("Stats");

        page.large_margins();

        page.key_value_element("Author", "Ollie Lynas");
        page.key_value_element("Seed", self.seed.to_string());
        page.key_value_element(
            "Date Printed",
            format!("{}", chrono::offset::Local::now())
                .split(" ")
                .next()
                .unwrap(),
        );
        page.key_value_element("Gen Time", format!("{}ms", gen_time.num_milliseconds()));

        return page;
    }
    pub fn index(&self, offset: i32) -> Page {
        let mut page = Page::default();
        page.title("Index");

        page.large_margins();
        let mut set: HashSet<ChapterEnum> = HashSet::new();
        for p in self.structure.iter().enumerate() {
            if (offset + p.0 as i32) < 0 {
                continue;
            }
            if set.insert(*p.1) {
                page.key_value_element(p.1.name(), (p.0 as i32 + offset).to_string());
            }
        }

        return page;
    }

    fn cover() -> Page {
        let mut page = Page::default();
        page.title("ACTIVITY BOOK");
        page.paragraph_ex(format!("v{}", VERSION), true);

        return page;
    }

    fn add_page_numbers(pages: &mut Vec<Page>) {

        for (i, page) in pages.iter_mut().enumerate() {
            page.reset_margins();
            page.set_cursor_y(HEIGHT_M1);
            page.paragraph_ex(format!("- pg. {} -", i), true);
        }

    }

    pub fn gen_pages(&self) -> Vec<Page> {
        let before: chrono::DateTime<chrono::Local> = chrono::Local::now();
        let pages = self
            .structure
            .par_iter()
            .enumerate()
            .map(|(i, x)| x.gen_pages(self.seed + i as u64))
            .collect::<Vec<Vec<Page>>>()
            .concat();
        let elapsed = chrono::Local::now().signed_duration_since(before);

        let mut content = [vec![self.index(2), self.stats_page(elapsed)], pages].concat();
        Book::add_page_numbers(&mut content);
        return [vec![Book::cover()], content].concat();
    }
}

pub fn default_book_structure(seed: u64) -> Book {
    let mut book = Book::create(seed);
    book.repeat_chapter(ChapterEnum::ConnectTheDots, 0);
    return book;
}

#[wasm_bindgen]
pub fn gen_wasm_book(seed: u64) -> Vec<String> {
    let book: Book = default_book_structure(seed);
    book.gen_pages().par_iter().map(|x| x.to_string()).collect()
}
