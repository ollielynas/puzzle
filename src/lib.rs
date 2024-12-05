use std::u64;
use std::fs::File;
use std::io::prelude::*;
use chapter::{Chapter, Page};
use connect_the_dots::ConnectTheDots;
use dyslexic_word_search::DyslexicWordSearch;
use itertools::Itertools;
use maze::Maze;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use strum::{self, IntoEnumIterator};
use strum_macros::EnumIter;
use waldo::Waldo;
use wasm_bindgen::prelude::wasm_bindgen;
use word_search::WordSearch;

#[cfg(not(target_arch = "wasm32"))]
use epub::gen_epub;
#[cfg(not(target_arch = "wasm32"))]
pub mod epub;


pub mod chapter;
pub mod word_search;
pub mod maze;
pub mod waldo;
pub mod dyslexic_word_search;
pub mod connect_the_dots;

#[derive(EnumIter, PartialEq, Eq, Copy, Clone)]
enum ChapterEnum {
    Maze,
    Waldo,
    WordSearch,
    DyslexicWordSearch,
    ConnectTheDots,
}

impl ChapterEnum {
    fn gen_pages(&self, seed: u64) -> Vec<Page> {
        match self {
            ChapterEnum::Maze => Maze::gen(seed).pages_owned(),
            ChapterEnum::WordSearch => WordSearch::gen(seed).pages_owned(),
            ChapterEnum::Waldo => Waldo::gen(seed).pages_owned(),
            ChapterEnum::DyslexicWordSearch => DyslexicWordSearch::gen(seed).pages_owned(),
            ChapterEnum::ConnectTheDots => ConnectTheDots::gen(seed).pages_owned(),
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

    fn repeat_chapter(&mut self, chapter: ChapterEnum, count: usize) {
        self.structure = self.structure.iter().map(|x| if x==&chapter {[x.clone()].repeat(count)} else {vec![x.clone()]}).concat();
    }

    pub fn gen_pages(&self) -> Vec<Page> {
        self.structure.par_iter().enumerate().map(|(i,x)| x.gen_pages(self.seed + i as u64)).collect::<Vec<Vec<Page>>>().concat()
    }
}


fn default_book_structure(seed: u64) -> Book {
    let mut book = Book::create(seed);
    book.repeat_chapter(ChapterEnum::ConnectTheDots, 0);

    return book;
}

#[wasm_bindgen]
pub fn gen_wasm_book(seed: u64) -> Vec<String> {
    let book: Book = default_book_structure(seed);
    book.gen_pages().iter().map(|x| x.to_string()).collect()
}


fn main() {
    
    
    let seed = 11;

    let book = default_book_structure(seed);

    let mut file = File::create("book.epub").unwrap();
    // file.write_all(&gen_epub(book).unwrap());
    let mut file = File::create("book.txt").unwrap();
    for page in book.gen_pages() {
        let _ = file.write_all(b"\n");
        let _ = file.write_all(page.to_string().as_bytes());
    }
}
