use std::u64;
use std::fs::File;
use std::io::prelude::*;
use book::{default_book_structure, new_book_structure};
use chapter::{Chapter, Page};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use strum::{self, IntoEnumIterator};
// pub use wasm_bindgen_rayon::init_thread_pool;

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
pub mod book;
pub mod sudokus;
pub mod crossword;
pub mod shape;

pub fn run() {
    
    let seed = 0;
    // let seed = fastrand::u64(0..1000);

    // let book = new_book_structure(seed, "10;0;0;0;0;1;1;1;".to_string());
    let book = default_book_structure(seed);


    let mut file = File::create("book.txt").unwrap();
    let mut first = true;
    for page in book.gen_pages() {
        if !first {
            let _ = file.write_all(b"\n");
        }
        first = false;
        let _ = file.write_all(page.to_string().as_bytes());
    }
}
