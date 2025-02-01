
use num::{integer::Roots, traits::ConstZero, BigInt};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::chapter::{Chapter, Page};





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
        

        let mut primes: Vec<BigInt> = vec![2.into()];
        
        
        let mut p2 = 4;
        let mut num = [0;30];
        fastrand::fill(&mut num);
        
        let small_n = fastrand::u32(1000..10000);
        let n = BigInt::from(small_n);


        let mut x = BigInt::from(4);
        for _ in 0..(small_n - 1) {
            x = x.pow(2) - 2;
        }



        let is_prime = false;
        while !is_prime {
            
        }



        
        wl.num_page.title("WHERES WALDO's prime numbers (V2)");
        
        wl.num_page.paragraph(format!("Opps! Only two numbers this time, which one of them is prime?"));
        
        
        
        wl.num_page.paragraph(primes[primes.len()-1].to_string());

        

        return wl;
    }

    fn pages(&self) -> Vec<&Page> {
        vec![&self.num_page]
    }
}