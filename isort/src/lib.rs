use std::cmp::Ordering;
use std::collections::HashMap;

pub fn selection_sort<F>(xs: &mut [i32], mut cmp: F)
where
    F: FnMut(i32, i32) -> Ordering + Sized,
{
    for i in 0..xs.len() {
        let mut t = i;
        for j in i + 1..xs.len() {
            if cmp(xs[j], xs[t]) == Ordering::Less {
                t = j;
            }
        }
        xs.swap(i, t);
    }
}

pub fn merge_insertion_sort<F>(xs: &mut [i32], cmp: &mut F)
where
    F: FnMut(i32, i32) -> Ordering + Sized,
{
    if xs.len() < 2 {
        return;
    }

    // First, swap all the largest elements to the front.
    let mut partner = HashMap::new();
    let half = xs.len() / 2;
    for i in 0..half {
        if cmp(xs[i], xs[i + half]) == Ordering::Less {
            xs.swap(i, i + half);
        }
        partner.insert(xs[i], i + half);
    }

    // Now recursively sort those larger elements.
    merge_insertion_sort(&mut xs[..half], cmp);
    println!("done recursing: {:?}", xs);

    // The smallest element has a partner that we already know about, move that into place.
    xs.swap(half, partner[&xs[0]]);
    xs[..=half].rotate_right(1);

    println!("handled easy case: {:?}", xs);

    // Now do an insertion-sort to get the latter half of the array into order.
    for i in half + 1..xs.len() {
        let x = xs[i];
        let idx = find_insert_point(x, &xs[..i], cmp);
        println!("xs[{}] = {} belongs at {}", i, x, idx);
        xs[idx..=i].rotate_right(1);
    }
}

fn find_insert_point<F>(x: i32, xs: &[i32], cmp: &mut F) -> usize
where
    F: FnMut(i32, i32) -> Ordering + Sized,
{
    match xs.binary_search_by(|&y| cmp(y, x)) {
        Ok(idx) => idx,
        Err(idx) => idx,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pcg::Pcg;
    use rand::seq::SliceRandom;

    #[test]
    fn sorts_correctly_smoke() {
        let mut xs = vec![3, 5, 1, 2, 4];
        merge_insertion_sort(&mut xs, &mut |a, b| a.cmp(&b));
        assert_eq!(xs, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_correctly() {
        let init: Vec<i32> = (0..100).collect();
        let mut prng = Pcg::new(1, 2);
        for _ in 0..1000 {
            let mut xs = init.clone();
            xs.shuffle(&mut prng);
            merge_insertion_sort(&mut xs, &mut |a, b| a.cmp(&b));
            assert_eq!(xs, init);
        }
    }

    #[test]
    fn manual() {
        let mut xs: Vec<i32> = (0..4).collect();
        merge_insertion_sort(&mut xs, &mut |a: i32, b: i32| {
            println!("cmp {} vs {}", a, b);
            a.cmp(&b)
        });
    }

    fn count_cmps(mut xs: Vec<i32>) -> usize {
        let mut cnt = 0;
        merge_insertion_sort(&mut xs, &mut |a: i32, b: i32| {
            cnt += 1;
            a.cmp(&b)
        });
        cnt
    }

    #[test]
    fn right_number_of_comparisons_smoke() {
        assert_eq!(count_cmps(vec![3, 5, 1, 2, 4]), 9); // NB: this can be driven down to 7
    }

    #[test]
    fn right_number_of_comparisons_small() {
        let expected = vec![0, 1, 3, 5, 7, 10, 13, 16, 19, 22, 26, 30, 34];
        for (i, n) in expected.into_iter().enumerate() {
            let a = count_cmps((0..i as i32 + 1).collect());
            assert_eq!(
                a,
                n,
                "{} items can be sorted in {} cmps but we used {}",
                i + 1,
                n,
                a
            );
        }
    }

    #[test]
    fn right_number_of_comparisons_big() {
        let mut xs: Vec<i32> = (0..100).collect();
        xs.shuffle(&mut pcg::Pcg::new(3, 7));
        assert_eq!(count_cmps(xs), 736);
    }
}
