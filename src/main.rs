use std::u64;
use std::fs::File;
use std::io::prelude::*;
use chapter::{Chapter, Page};
use epub::gen_epub;
use maze::Maze;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::{self, IntoEnumIterator};
use strum_macros::EnumIter;
use waldo::Waldo;
use word_search::WordSearch;

pub mod chapter;
pub mod word_search;
pub mod maze;
pub mod waldo;
pub mod epub;

#[derive(EnumIter)]
enum ChapterEnum {
    WordSearch,
    Waldo,
    Maze,
}

impl ChapterEnum {
    fn gen_pages(&self, seed: u64) -> Vec<Page> {
        match self {
            ChapterEnum::Maze => Maze::gen(seed).pages_owned(),
            ChapterEnum::WordSearch => WordSearch::gen(seed).pages_owned(),
            ChapterEnum::Waldo => Waldo::gen(seed).pages_owned(),
        }
    }
}

fn main() {
    

    let seed = 0;
    let book = ChapterEnum::iter().collect::<Vec<ChapterEnum>>().par_iter().map(|x| x.gen_pages(seed)).collect::<Vec<Vec<Page>>>().concat();

    let mut file = File::create("foo.epub").unwrap();
    file.write_all(&gen_epub(book).unwrap());
}
