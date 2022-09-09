#![feature(is_sorted)]
#![feature(test)]

pub fn merge_sort(vec: Vec<i32>) -> Vec<i32> {

    if vec.len() == 1 || vec.is_empty() {
        return vec;
    }

    let d = vec.len() / 2;

    let v1 = vec[0..d].to_vec();
    let v2 = vec[d..].to_vec();

    let v1 = merge_sort(v1);
    let v2 = merge_sort(v2);

    let (mut k, mut i, mut j) = (0, 0, 0);

    let size = v1.len() + v2.len();
    let mut retvec = vec![0; size];

    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
          retvec[k] = v1[i];
          i+=1;
        } else {
          retvec[k] = v2[j];
          j+=1;
        }
        k+=1;
      }

    while i < v1.len() {
        retvec[k] = v1[i];
        i+=1;
        k+=1;
      }
      while j < v2.len() {
        retvec[k] = v2[j];
        j+=1;
        k+=1;
      }

    retvec
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_simple_vec() {
        let list = vec![1, 7, 2, 5];
        let list = merge_sort(list);

        assert!(list.is_sorted());
    }

    #[test]
    fn test_with_negatives() {
        let list = vec![8, -42, 5, 163, 9, -3];
        let list = merge_sort(list);

        assert!(list.is_sorted());
    }

    #[test]
    fn test_empty() {
        let list = vec![];
        let list = merge_sort(list);

        assert!(list.is_sorted());
    }

    #[test]
    fn test_one_element() {
        let list = vec![1];
        let list = merge_sort(list);

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
        test::black_box(merge_sort(list.clone()));
    });
    }
} 
