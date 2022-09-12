#![feature(test)]

pub fn linear_search(vec: &Vec<i32>, target: i32) -> Option<usize> {
    for (index, num) in vec.iter().enumerate() {
        if target == *num {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let vec = vec![1, 6, 2, 5, 8];
        let target = 8;

        match linear_search(&vec, target) {
            Some(index) => {
                if vec.get(index).unwrap() != &target {
                    panic!("the index value does not match the target value!")
                }
            },
            None => panic!("no match was found!"),
        }
    }

    #[test]
    #[should_panic]
    fn empty_test() {
        let vec = vec![];

        linear_search(&vec, 1).unwrap();
    }


    extern crate test;
    use rand::Rng;
    use test::Bencher;

    #[bench]
    fn bench_100_item_list(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut list = vec![];
        
        for _ in 0..1000 {
            list.push(rng.gen());
        }
        let target = list.get(499).unwrap();

        b.iter(|| {
        test::black_box(linear_search(&list, *target));
    });
    }
}
