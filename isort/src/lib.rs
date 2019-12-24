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

pub fn merge_insertion_sort<F>(xs: &mut [i32], cmp: &mut F)
    where
        F: FnMut(i32, i32) -> Ordering + Sized,
{
    if xs.len() < 2 {
        return;
    }

    // First, swap all the largest elements to the front.
    let half = xs.len() / 2;
    for i in 0..half {
        if cmp(xs[i], xs[i + half]) == Ordering::Less {
            xs.swap(i, i + half);
        }
    }

    // Now recursively sort those larger elements.
    merge_insertion_sort(&mut xs[..half], cmp);

    // Now do an insertion-sort to get the latter half of the array into order.
    for i in half .. xs.len() {
        let x = xs[i];
        let idx = find_insert_point(x, &xs[..i], cmp);
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
    use rand::seq::SliceRandom;

    #[test]
    fn sorts_correctly() {
        let mut xs = vec![3, 5, 1, 2, 4];
        merge_insertion_sort(&mut xs, &mut |a, b| a.cmp(&b));
        assert_eq!(xs, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn right_number_of_comparisons() {
        let mut cnt = 0;
        let counting = &mut |a: i32, b: i32| {
            cnt += 1;
            a.cmp(&b)
        };

        let mut xs = vec![3, 5, 1, 2, 4];
        merge_insertion_sort(&mut xs, counting);
        assert_eq!(cnt, 10);
    }

    #[test]
    fn right_number_of_comparisons_big() {
        let mut cnt = 0;
        let counting = &mut |a: i32, b: i32| {
            cnt += 1;
            a.cmp(&b)
        };

        let mut xs: Vec<i32> = (0..100).collect();
        xs.shuffle(&mut pcg::Pcg::new(0, 1));
        merge_insertion_sort(&mut xs, counting);
        assert_eq!(xs, (0..100).collect::<Vec<_>>());
        assert_eq!(cnt, 4950);
    }
}
