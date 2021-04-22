// 0010. Write a function named my_sum summing elements of list.
#[allow(dead_code)]
fn my_sum(ns: &[usize]) -> usize {
    fn my_sum(acc: usize, ns: &[usize]) -> usize {
        match ns {
            [] => acc,
            [n, ns @ ..] => my_sum(n + acc, ns),
        }
    }
    my_sum(0, ns)
}

// https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html
// 0020. Write a function named my_mean calcurating the average of elements of list.
#[allow(dead_code)]
fn my_mean(ns: &[usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        n => Some(my_sum(ns) / n),
    }
}

// 0030. Write a function nemed my_median calcurating the median of elements of list.
#[allow(dead_code)]
fn my_median(ns: &mut [usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        length => {
            let index = length / 2;
            ns.sort();
            Some(ns[index])
        },
    }
}

// Genbade yakudatsu ch.2

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_sum() {
        assert_eq!(0, my_sum(&vec![]));
        assert_eq!(1, my_sum(&vec![1]));
        assert_eq!(6, my_sum(&vec![1, 2, 3]));
    }
    #[test]
    fn test_my_mean() {
        assert_eq!(None, my_mean(&vec![]));
        assert_eq!(Some(2), my_mean(&vec![2]));
        assert_eq!(Some(2), my_mean(&vec![1, 2, 3]));
    }
    #[test]
    fn test_median() {
        assert_eq!(None, my_median(&mut vec![]));
        assert_eq!(Some(1), my_median(&mut vec![1]));
        assert_eq!(Some(2), my_median(&mut vec![3, 1, 2]));
    }
}
