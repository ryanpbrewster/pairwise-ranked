use std::cmp::Ordering;

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

#[cfg(test)]
mod test {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    fn sorts_correctly() {
        let mut xs = vec![3, 5, 1, 2, 4];
        selection_sort(&mut xs, |a, b| a.cmp(&b));
        assert_eq!(xs, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn right_number_of_comparisons() {
        let mut cnt = 0;
        let counting = |a: i32, b: i32| {
            cnt += 1;
            a.cmp(&b)
        };

        let mut xs = vec![3, 5, 1, 2, 4];
        selection_sort(&mut xs, counting);
        assert_eq!(cnt, 10);
    }

    #[test]
    fn right_number_of_comparisons_big() {
        let mut cnt = 0;
        let counting = |a: i32, b: i32| {
            cnt += 1;
            a.cmp(&b)
        };

        let mut xs: Vec<i32> = (0..100).collect();
        xs.shuffle(&mut pcg::Pcg::new(0, 1));
        selection_sort(&mut xs, counting);
        assert_eq!(cnt, 4950);
    }
}
