#![feature(is_sorted)]
#![feature(test)]

pub fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    loop {
        let mut swapped = false;

        let mut i = 0;
        while i < vec.len() {
            if i >= vec.len() - 1 { break; }
            if vec[i] > vec[i + 1] {
                swapped = true;
                let value = vec.remove(i);
                vec.insert(i + 1, value);
                break;
            }
            i += 1;
        }

        if !swapped { break; }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_simple_list() {
        let list = vec![9, 4, 7, 1];
        let list = bubble_sort(list);

        assert!(list.is_sorted());
    }

    #[test]
    fn list_with_negatives() {
        let list = vec![28, 6, -2, 18, -69];
        let list = bubble_sort(list);

        assert!(list.is_sorted());
    }

    #[test]
    fn empty_list() {
        let list = vec![];
        let list = bubble_sort(list);

        assert!(list.is_sorted());
    }

    #[test]
    fn one_element_list() {
        let list = vec![1];
        let list = bubble_sort(list);

        assert!(list.is_sorted());
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_100_item_list(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut list = vec![];
        
        for _ in 0..100 {
            list.push(rng.gen());
        }

        b.iter(|| {
        test::black_box(bubble_sort(list.clone()));
    });
    }
}
