use number_theory::NumberTheory;

use crate::chapter::{Chapter, Page};




pub struct Waldo {
    num_page: Page,
    key_page: Page,
}


impl Chapter for Waldo {
    fn gen(seed: u64) -> Self {
        let mut wl = Waldo {
            num_page: Page::default(),
            key_page: Page::default(),
        };

        let mut primes = vec![];
        let mut pow2 = vec![];
        let mut m5 = vec![];
        let mut normal = vec![];
        // let mut  = vec![];
        let mut p2 = 4;
        for i in 2..1000 {
            if i.is_prime() {
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

        numbers.truncate(570);
        fastrand::shuffle(&mut numbers);


        wl.num_page.paragraph(numbers.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(if fastrand::bool() {"  "} else {" "}));

        wl.key_page.title("WHERES WALDO ('s prime numbers)");
        wl.key_page.paragraph("\n\nOh no! Waldo has had an accident and he needs you help! can you help him find the folowing items?");
        wl.key_page.set_margins(5, 3);

        wl.key_page.paragraph(" ");
        wl.key_page.paragraph(format!("{pow2_count} powers of 2"));
        wl.key_page.paragraph(format!("{m5_count} multiples of 5"));
        wl.key_page.paragraph(format!("{primes_count} prime numbers"));

        

        

        return wl;
    }

    fn pages(&self) -> Vec<&Page> {
        vec![&self.key_page, &self.num_page]
    }
}