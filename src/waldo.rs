
use std::collections::HashSet;

use itertools::Itertools;
use num::{integer::Roots, traits::ConstZero, BigInt};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::chapter::{Chapter, Page, HEIGHT, WIDTH};





pub struct Waldo {
    num_page: Page,
}


impl Chapter for Waldo {
    fn gen(seed: u64) -> Self {
        let mut wl = Waldo {
            num_page: Page::default(),
        };

        fastrand::seed(seed);

        let mut primes = vec![2];
        let mut pow2 = vec![];
        let mut m5 = vec![];
        let mut normal = vec![];
        // let mut  = vec![];
        let mut p2 = 4;
        for i in 3..1000 {
            if !primes.par_iter().any(|x| i%x == 0) {
                primes.push(i);
            }else if i == p2 {
                pow2.push(i);
                p2 *= 2;
            }else if i % 5 == 0 {
                m5.push(i);
            }else {
                normal.push(i);
            }
        }




        let primes_count = fastrand::usize(5..primes.len().min(20));
        let m5_count = fastrand::usize(5..m5.len().min(20));
        let pow2_count = fastrand::usize(5..pow2.len().min(20));
        fastrand::shuffle(&mut primes);
        fastrand::shuffle(&mut pow2);
        fastrand::shuffle(&mut m5);

        primes.truncate(primes_count);
        pow2.truncate(pow2_count);
        m5.truncate(m5_count);
        let mut numbers = [
            primes,
            m5,
            pow2,
            normal
        ].concat();

        
        wl.num_page.title("WHERES WALDO ('s prime numbers)");
        
        wl.num_page.paragraph(format!("\n Can you help waldo find his {primes_count} prime numbers? \n "));
        
        
        numbers.truncate(560);
        fastrand::shuffle(&mut numbers);
        
        
        wl.num_page.paragraph_ex(numbers.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(if fastrand::bool() {"  "} else {" "}), true);
        
        

        

        return wl;
    }

    fn pages(&self) -> Vec<&Page> {
        vec![&self.num_page]
    }
}


pub struct Waldo2 {
    num_page: Page,
}


impl Chapter for Waldo2 {
    fn gen(seed: u64) -> Self {
        let mut wl = Waldo2 {
            num_page: Page::default(),
        };

        fastrand::seed(seed);
        

        let mut prime = BigInt::ZERO;
        let mut non_prime = BigInt::ZERO;

        // let mut non_prime_set = HashSet::new();
        
        while (prime==BigInt::ZERO) || (non_prime == BigInt::ZERO) {
        
        
        let small_n = fastrand::u32(500..=800);
        let mut n: BigInt = BigInt::from(2);

        n = n.pow(small_n) - 1;

        if &n % BigInt::from(5) == BigInt::ZERO {
            // return(BigInt::ZERO ,BigInt::ZERO);
            continue;
        }
        if &n % BigInt::from(3) == BigInt::ZERO {
            // return(BigInt::ZERO ,BigInt::ZERO);
            continue;
        }

        
        
        let mut x: BigInt = BigInt::from(4);
        for _ in 0..(small_n - 2) {
            x = (&x*&x - 2) % &n;
        }
        // (n, x)
        // }).collect::<Vec<(BigInt, BigInt)>>();

        // for (n,x) in nums {
        if x == BigInt::ZERO {
            prime = n;
        }else {
            // non_prime_set.insert(n.clone());
            if non_prime == BigInt::ZERO {
                        
        
                non_prime = n;
            }
        // }
        }
        }



        
        wl.num_page.title("WHERES WALDO's prime numbers (V2)");
        
        wl.num_page.paragraph_ex("Waldo accidentally dropped his prime number into a bag containing exactly one compound number. Can you help him figure out which number is prime?", true);
        wl.num_page.newline();
        wl.num_page.newline();
        
        let mut order = [prime, non_prime];
        fastrand::shuffle(&mut order);
        
        wl.num_page.set_margins(5, 5);
        wl.num_page.paragraph_ex("number 1", true);
        wl.num_page.paragraph(format!("{:?}", &order[0]).chars().chunks(3).into_iter().map(|x| {x.collect::<String>()}).join(" "));
        wl.num_page.newline();
        wl.num_page.newline();
        wl.num_page.paragraph_ex("number 2", true);
        wl.num_page.paragraph(format!("{:?}", &order[1]).chars().chunks(3).into_iter().map(|x| {x.collect::<String>()}).join(" "));
        
        

        return wl;
    }

    fn pages(&self) -> Vec<&Page> {
        vec![&self.num_page]
    }
}