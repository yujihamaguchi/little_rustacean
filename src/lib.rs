use std::collections::HashMap;

// 0010. Write a function named `my_sum` summing list of usize elements.
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

// refer: https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html
// 0020. Write a function named my_mean calculating the average of list of usize elements.
#[allow(dead_code)]
fn my_mean(ns: &[usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        n => Some(my_sum(ns) / n),
    }
}

// 0030. Write a function nemed my_median calculating the median of elements of list.
#[allow(dead_code)]
fn my_median(ns: &mut [usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        length => {
            let index = length / 2;
            ns.sort();
            Some(ns[index])
        }
    }
}

// 0040. Write a function named my_mode calclating the mode of elements of list.
#[allow(dead_code)]
fn my_mode(ns: &[usize]) -> Option<usize> {
    if ns.is_empty() {
        None
    } else {
        let mut occurrences = HashMap::new();
        for &n in ns {
            *occurrences.entry(n).or_insert(0) += 1;
        }
        let (&k, _) = occurrences.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
        Some(k)
    }
}

// 0050. Write a function named to_pig_latin converting word to Pig Latin word.
#[allow(dead_code)]
fn pig_latin_from(s: &str) -> String {
    match s.as_bytes().first() {
        None => s.to_string(),
        Some(&c_byte) => match c_byte as char {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", s),
            _ => format!("{}-{}ay", &s[1..], c_byte as char),
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
    fn test_my_median() {
        assert_eq!(None, my_median(&mut vec![]));
        assert_eq!(Some(1), my_median(&mut vec![1]));
        assert_eq!(Some(2), my_median(&mut vec![3, 1, 2]));
    }
    #[test]
    fn test_my_mode() {
        assert_eq!(None, my_mode(&vec![]));
        assert_eq!(Some(1), my_mode(&vec![1]));
        assert_eq!(Some(2), my_mode(&vec![1, 2, 3, 2, 3, 2]));
    }
    #[test]
    fn test_pig_latin_from() {
        assert_eq!("", pig_latin_from(""));
        assert_eq!("apple-hay", pig_latin_from("apple"));        
        assert_eq!("irst-fay", pig_latin_from("first"));
        assert_eq!("econd-say", pig_latin_from("second"));
    }
}
