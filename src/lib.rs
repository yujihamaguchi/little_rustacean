pub mod helper;

use core::panic;
use std::{collections::HashMap, thread, time::Duration};

// 0010. Write a function named `my_sum` summing list of usize elements.

// use reduce
fn my_sum(ns: &[usize]) -> usize {
    ns.to_vec()
        .into_iter()
        .reduce(|acc, n| acc + n)
        .unwrap_or(0)
}

/*
//  use recursion
fn my_sum(ns: &[usize]) -> usize {
    fn my_sum(acc: usize, ns: &[usize]) -> usize {
        match ns {
            [] => acc,
            [n, ns @ ..] => my_sum(n + acc, ns),
        }
    }
    my_sum(0, ns)
}
 */
// 0020. Write a function named 'my_mean' to calculate an average for list of some numbers.
#[allow(dead_code)]
fn my_mean(ns: &[usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        n => Some(my_sum(ns) / n),
    }
}

// 0025. Write a function named 'my_sort' to sort list of some numbers.
#[allow(dead_code)]
fn my_sort(ns: &[usize]) -> Vec<usize> {
    match ns {
        [] => vec![],
        [n, ns @ ..] => {
            let v = ns.to_vec();
            let lt: Vec<usize> = v.iter().filter(|&m| m < n).cloned().collect();
            let ge: Vec<usize> = v.iter().filter(|&m| n <= m).cloned().collect();
            [my_sort(&lt), vec![*n], my_sort(&ge)].concat()
        }
    }
}

// 0030. Write a function nemed 'my_median' to calculate a median for list of some numbers.
#[allow(dead_code)]
fn my_median(ns: &mut [usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        n => {
            ns.sort();
            Some(ns[n / 2])
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
        let (&k, _) = occurrences
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();
        Some(k)
    }
}

// 0050. Write a function named to_pig_latin converting word to Pig Latin word.
#[allow(dead_code)]
fn pig_latin_from(s: &str) -> String {
    match s.chars().nth(0) {
        Some('a') | Some('e') | Some('i') | Some('u') | Some('o') => format!("{}-hay", s),
        Some(c) => format!("{}-{}ay", s.chars().skip(1).collect::<String>(), c),
        None => s.to_string(),
    }
}

// 0060. Write a function named `first_word` returning first word of string passed as parameter.
#[allow(dead_code)]
fn first_word(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        match s.split(' ').collect::<Vec<_>>().as_slice() {
            [first, _rest @ ..] => Some(first),
            _ => panic!("unexpected pattern!"),
        }
    }
}

/*
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
*/

// 0070. Write a function named `largest_for_copy` returning largest one of elements which implements Copy trait.
#[allow(dead_code)]
fn largest_for_copy<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        None
    } else {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        Some(largest)
    }
}

// 0080. Write a function named `largest_for_clone` returning largest one of elements which implements Clone trait.
#[allow(dead_code)]
fn largest_for_clone<T: PartialOrd + Clone>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        None
    } else {
        let mut largest = list[0].clone();
        for item in list.iter() {
            let item = item.clone();
            if item > largest {
                largest = item;
            }
        }
        Some(largest)
    }
}

// 0090. Write a function named `largest` returning largest one of elements.
#[allow(dead_code)]
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        Some(largest)
    }
}

// The Book: Ch.13.1 Closures: Anonymous Functions that Can Capture Their Environment
/*
// Before tuning.
#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }
    if intensity < 25 {

        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );

        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
*/

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, v: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(v);
                self.value = Some(v);
                v
            }
        }
    }
}

#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    let mut cacher = Cacher::new(|_| simulated_expensive_calculation(intensity));

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));

        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

// The Book: Ch12. An I/O Project: Building a Command Line Program

/*
// ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
// 例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。
// それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。

use std::{collections::HashMap, io};

fn main() {
    let mut depts: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Command?");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line!");

        match command.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["add", name, "to", dept] => {
                depts
                    .entry(dept.to_string())
                    .or_insert(Vec::new())
                    .push(name.to_string());
            }
            ["list"] => list(&depts),
            ["list", target_dept] => list(
                &depts
                    .iter()
                    .filter(|(dept, _)| dept == target_dept)
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<HashMap<_, _>>(),
            ),
            _ => println!("nothing to do!"),
        }
    }
}
*/

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
    fn test_my_sort() {
        assert_eq!(vec![] as Vec<usize>, my_sort(&vec![]));
        assert_eq!(vec![1], my_sort(&vec![1]));
        assert_eq!(vec![1, 2, 3], my_sort(&vec![2, 1, 3]));
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
        assert_eq!("egg-hay", pig_latin_from("egg"));
        assert_eq!("issue-hay", pig_latin_from("issue"));
        assert_eq!("useless-hay", pig_latin_from("useless"));
        assert_eq!("out-hay", pig_latin_from("out"));
    }

    #[test]
    fn test_first_word() {
        assert_eq!(None, first_word(""));
        assert_eq!(Some("foo"), first_word("foo"));
        assert_eq!(Some("foo"), first_word("foo bar baz"));
    }

    #[test]
    fn test_largest_for_copy() {
        // assert_eq!(None, largest(vec![]));
        assert_eq!(Some(100), largest_for_copy(&vec![34, 50, 25, 100, 65]));
        assert_eq!(Some('y'), largest_for_copy(&vec!['y', 'm', 'a', 'q']));
    }

    #[test]
    fn test_largest_for_clone() {
        assert_eq!(
            Some(String::from("foo")),
            largest_for_clone(&vec![
                String::from("bar"),
                String::from("foo"),
                String::from("baz")
            ])
        );
    }

    #[test]
    fn test_largest() {
        assert_eq!(Some(&100), largest(&vec![34, 50, 25, 100, 65]));
        assert_eq!(Some(&'y'), largest(&vec!['y', 'm', 'a', 'q']));
        assert_eq!(
            Some(&String::from("foo")),
            largest(&vec![
                String::from("bar"),
                String::from("foo"),
                String::from("baz")
            ])
        );
    }

    #[test]
    fn test_list_workout_in_specific_secs() {
        assert_eq!(2, helper::execution_seconds(|| generate_workout(24, 0)));
        assert_eq!(2, helper::execution_seconds(|| generate_workout(25, 0)));
        assert_eq!(0, helper::execution_seconds(|| generate_workout(25, 3)));
    }
}
