#![feature(type_name_of_val)]
#![allow(dead_code)]

pub mod helper;

use core::slice;
use std::{collections::HashMap, thread, time::Duration, u32};

// 0010. Write a function named `my_sum` summing array of usize elements.
// use reduce
fn my_sum(ns: &[usize]) -> usize {
    // I use `to_vec` because Array ([T: N] type) doesn't implement IntoIterator.
    // https://doc.rust-lang.org/std/primitive.array.html#impl-IntoIterator
    // `IntoIterator (implemented for &[T; N] and &mut [T; N])`
    ns.to_vec()
        .into_iter()
        .reduce(|acc, n| acc + n)
        .unwrap_or(0)
}

//  use recursion
/* fn my_sum(ns: &[usize]) -> usize {
    fn my_sum(acc: usize, ns: &[usize]) -> usize {
        match ns {
            [] => acc,
            [n, ns @ ..] => my_sum(n + acc, ns),
        }
    }
    my_sum(0, ns)
}
 */

// 0020. Write a function named 'my_mean' to calculate an average for array of some numbers.
fn my_mean(ns: &[usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        n => Some(my_sum(ns) / n),
    }
}

// 0025. Write a function named 'my_sort' to sort array of some numbers.
fn my_sort(ns: &[usize]) -> Vec<usize> {
    match ns {
        [] => vec![],
        [n, ns @ ..] => {
            let v = ns.to_vec();
            let lt = v.iter().filter(|&m| m < n).cloned().collect::<Vec<_>>();
            let ge = v.iter().filter(|&m| n <= m).cloned().collect::<Vec<_>>();
            [my_sort(&lt), vec![*n], my_sort(&ge)].concat()
        }
    }
}

// 0030. Write a function nemed 'my_median' to calculate a median for list of some numbers.
fn my_median(ns: &mut [usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        n => {
            ns.sort();
            Some(ns[n / 2])
        }
    }
}

// 0031. go to test_blog_oop

// 0032. go to test_blog_not_oop

// 0040. Write a function named my_mode calclating the mode of elements of list.
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

// 0050. Write a function named `pig_latin_from` to convert word into Pig Latin fashion.
enum Vowel {
    A,
    E,
    U,
    I,
    O,
}
impl Vowel {
    fn value(&self) -> char {
        match *self {
            Vowel::A => 'a',
            Vowel::E => 'e',
            Vowel::U => 'u',
            Vowel::I => 'i',
            Vowel::O => 'o',
        }
    }
    fn contains(c: &char) -> bool {
        c.eq(&Vowel::A.value())
            || c.eq(&Vowel::E.value())
            || c.eq(&Vowel::U.value())
            || c.eq(&Vowel::I.value())
            || c.eq(&Vowel::O.value())
    }
}

fn pig_latin_from(word: &str) -> String {
    match word.chars().nth(0) {
        Some(c) if Vowel::contains(&c) => format!("{}-hay", word),
        Some(c) => format!("{}-{}ay", word.chars().skip(1).collect::<String>(), c),
        _ => word.to_string(),
    }
}

// 0060. Write a function named `first_word` to return first word of string passed as parameter.
fn first_word(s: &str) -> Option<&str> {
    if s.is_empty() {
        return None;
    }
    for (i, c) in s.bytes().enumerate() {
        if b' ' == c {
            return Some(&s[..i]);
        }
    }
    Some(s)
}

// 0070. Write a function named `largest_for_copy` returning largest one of elements which implements Copy trait.
// use for loop
fn largest_for_copy<T: Ord + Copy>(list: &[T]) -> Option<T> {
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

// use reduce
/* fn largest_for_copy<T: Copy + Ord>(xs: &[T]) -> Option<T> {
    xs.to_owned()
        .into_iter()
        .reduce(|x, y| if x > y { x } else { y })
}
 */

// 0080. Write a function named `largest_for_clone` returning largest one of elements which implements Clone trait.
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
fn largest<T: PartialOrd>(xs: &[T]) -> Option<&T> {
    if xs.is_empty() {
        return None;
    }
    let mut largest = &xs[0];
    for x in xs {
        if largest < x {
            largest = x;
        }
    }
    Some(largest)
}

// 0095. The Book: Ch12. An I/O Project: Building a Command Line Program
// ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????
// ????????????"Add Sally to Engineering"(?????????????????????????????????)???"Add Amir to Sales"(????????????????????????????????????)???????????????
// ???????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????
/*
use std::{collections::HashMap, io};
fn main() {
    let mut org: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line!");
        match command.trim().split(' ').collect::<Vec<_>>().as_slice() {
            &["Add", name, "to", dept] => {
                org.entry(String::from(dept))
                    .or_insert(Vec::new())
                    .push(String::from(name));
            }
            &["list", dept] => {
                if let Some(names) = org.get(dept) {
                    for name in names {
                        println!("{}", name);
                    }
                }
            }
            &["list"] => {
                for (dept, names) in &org {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{} {}", dept, name);
                    }
                }
            }
            _ => println!("Invalid command!"),
        }
    }
}
*/

// 0100. The Book: Ch.13.1 Closures: Anonymous Functions that Can Capture Their Environment
// ??????????????????????????????????????????????????????????????????

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// Before tuning.
/* fn generate_workout(intensity: u32, random_number: u32) -> u32 {
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
    intensity
}
*/

fn generate_workout(intensity: u32, random_number: u32) -> u32 {
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
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    value: Option<u32>,
    calc: T,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calc: T) -> Self {
        Cacher { value: None, calc }
    }
}
trait Cached<T> {
    fn value(&mut self, key: T) -> T;
}

impl<T> Cached<u32> for Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn value(&mut self, key: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let value = (self.calc)(key);
                self.value = Some(value);
                value
            }
        }
    }
}
/*
fn generate_workout(intensity: u32, random_number: u32) -> u32 {
    let mut cacher = Cacher::new(|intensity: u32| simulated_expensive_calculation(intensity));
    execute(intensity, random_number, &mut cacher)
}

fn execute(intensity: u32, random_number: u32, cacher: &mut dyn Cached<u32>) -> u32 {
    let mut result: u32 = 0;
    if intensity < 25 {
        cacher.value(intensity);
        result = cacher.value(intensity + 1);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            result = cacher.value(intensity);
        }
    }
    result
}

trait Cached<T> {
    fn value(&mut self, key: T) -> T;
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calc: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calc: T) -> Self {
        Cacher { calc, value: None }
    }
}

impl<T> Cached<u32> for Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn value(&mut self, key: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let value = (self.calc)(key);
                self.value = Some(value);
                value
            }
        }
    }
}
*/
// Cacher???????????????
// 0110. ????????????????????????????????????????????????????????????????????????Cacher????????????????????????????????????
/*
fn generate_workout2(intensity: u32, random_number: u32) -> u32 {
    let mut cacher = Cacher2::new(|intensity: u32| simulated_expensive_calculation(intensity));
    execute(intensity, random_number, &mut cacher)
}

struct Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    calc: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calc: T) -> Self {
        Cacher2 {
            calc,
            value: HashMap::new(),
        }
    }
}

impl<T> Cached<u32> for Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    fn value(&mut self, key: u32) -> u32 {
        match self.value.get(&key) {
            Some(v) => *v,
            None => {
                let value = (self.calc)(key);
                self.value.entry(key).or_insert(value);
                value
            }
        }
    }
}
*/

// 0112. ?????????Cacher?????????2????????????????????????????????????u32??????????????????u32???????????????????????????????????????????????????????????????
//       ?????????????????????????????????????????????usize??????????????????????????????????????????????????????????????????????????????????????????
//       ????????????????????????????????????Cacher??????????????????????????????????????????????????????????????????????????????????????????????????????????????????
struct Cacher3<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U> Cacher3<T, U>
where
    T: Fn(U) -> U,
    U: Eq + std::hash::Hash + Copy,
{
    fn new(calculation: T) -> Cacher3<T, U> {
        Cacher3 {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, v: U) -> U {
        self.value.entry(v).or_insert((self.calculation)(v));
        self.value[&v]
    }
}

// 0120. ?????????1??????5????????????????????????????????????????????????????????????????????????
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 4 {
            return None;
        }
        self.count += 1;
        Some(self.count)
    }
}

// 0130. Write the function named `my_split_at_mut` as [i32]'s method which will split a mutable slice(self) to two mutable slices at a passed index.
trait MySlice {
    fn my_split_at_mut(&mut self, _: usize) -> (&mut [i32], &mut [i32]);
}
impl MySlice for [i32] {
    fn my_split_at_mut(&mut self, n: usize) -> (&mut [i32], &mut [i32]) {
        let len = self.len();
        let ptr = self.as_mut_ptr();
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, n),
                slice::from_raw_parts_mut(ptr.offset(n as isize), len - n),
            )
        }
    }
}

// 1020: haskell ??? sum ???????????????????????????????????????(??????????????????????????????, reduce ????????????????????????)
// sum :: (Num a) => [a] -> a
// sum ns
//     ?????????????????? ns ?????????????????????
//     see also: product, foldl
//         sum [1, 2, 3]  = 6
//         sum []         = 0
// Using recursion.
/* fn sum(ns: &[u32]) -> u32 {
    if ns.is_empty() {
        return 0;
    }
    ns[0] + sum(&ns[1..])
}
 */

// Using reduce.
// refer: https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
fn sum(ns: &[u32]) -> u32 {
    ns.to_vec()
        .into_iter()
        .reduce(|acc, n| acc + n)
        .unwrap_or(0)
}

// Genbade yakudatsu ch.2

#[cfg(test)]
mod tests {

    use std::any::type_name_of_val;

    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(0, sum(&[]));
        assert_eq!(0, sum(&[0, 0]));
        assert_eq!(1, sum(&[1]));
        assert_eq!(10, sum(&[1, 2, 3, 4]));
    }
    #[test]
    fn test_my_zip() {
        assert_eq!(vec![(1, 2), (3, 4)], my_zip(&[&[1, 3], &[2, 4]]));
    }

    #[test]
    fn test_my_split_at_mut() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.my_split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    #[test]
    fn test_my_sum() {
        assert_eq!(0, my_sum(&[]));
        assert_eq!(1, my_sum(&[1]));
        assert_eq!(6, my_sum(&[1, 2, 3]));
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
        assert_eq!(Some("bar"), first_word("bar baz foo"));
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
        assert_eq!(2, helper::execution_seconds(|| generate_workout(25, 1)));
        assert_eq!(0, helper::execution_seconds(|| generate_workout(25, 3)));
        assert_eq!(24, generate_workout(24, 1));
        assert_eq!(2, helper::execution_seconds(|| generate_workout(24, 1)));
    }
    /*
       #[test]
       fn test_list_workout_in_specific_secs2() {
           assert_eq!(2, helper::execution_seconds(|| generate_workout2(25, 1)));
           assert_eq!(0, helper::execution_seconds(|| generate_workout2(25, 3)));
           assert_eq!(25, generate_workout2(24, 1));
           assert_eq!(4, helper::execution_seconds(|| generate_workout2(24, 1)));
       }

       #[test]
       fn test_cacher() {
           let mut cacher = Cacher2::new(|a| a);
           assert_eq!(1, cacher.value(1));
           assert_eq!(2, cacher.value(2));
       }

       #[test]
       fn test_generic_cacher() {
           let mut cacher = Cacher3::new(|a| a);
           assert_eq!("foo", cacher.value("foo"));
           let mut cacher = Cacher3::new(|a| a);
           assert_eq!(1, cacher.value(1));
       }
    */
    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }

    #[test]
    fn test_blog_oop() {
        // https://doc.rust-jp.rs/book-ja/ch17-03-oo-design-patterns.html
        // ?????????????????????????????????
        // 1.???????????????????????????????????????????????????
        let mut post = Post::new();
        let content = String::from("hello world!");
        post.add_text(&content);
        assert_eq!("", post.content());

        struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }
        impl Post {
            fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }
            fn add_text(&mut self, text: &str) {
                if self.state.as_ref().unwrap().editable() {
                    self.content.push_str(text)
                }
            }
            fn content(&self) -> &str {
                self.state.as_ref().unwrap().content(&self)
            }
            fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }
            fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }
        }
        trait State {
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&self, _: &'a Post) -> &'a str {
                ""
            }
            fn reject(self: Box<Self>) -> Box<dyn State>;
            fn editable(&self) -> bool {
                false
            }
        }
        struct Draft {}
        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview { approve_count: 0 })
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn editable(&self) -> bool {
                true
            }
        }
        struct PendingReview {
            approve_count: usize,
        }
        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(mut self: Box<Self>) -> Box<dyn State> {
                if self.approve_count > 0 {
                    Box::new(Published {})
                } else {
                    self.approve_count += 1;
                    self
                }
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                Box::new(Draft {})
            }
        }

        struct Published {}
        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }
        // 2.???????????????????????????????????????????????????
        post.request_review();
        assert_eq!("", post.content());

        // 3.????????????????????????????????????????????????
        post.approve();
        //assert_eq!(content, post.content());

        // 4.?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????
        let mut post = Post::new();
        let content = "hello rust!";
        post.add_text(content);
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!(content, post.content());

        // 5.??????????????????PendingReview??????Draft?????????reject??????????????????????????????
        impl Post {
            fn reject(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.reject())
                }
            }
        }

        let mut post = Post::new();
        post.request_review();
        post.reject();
        assert_eq!("", post.content());

        // 6.?????????Published??????????????????????????????approve???2????????????????????????????????????????????????
        let mut post = Post::new();
        let content = "hello rust!";
        post.add_text(content);
        post.request_review();
        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!(content, post.content());

        // 7.?????????Draft????????????????????????????????????????????????????????????????????????????????????
        //   ?????????: ????????????????????????????????????????????????????????????????????????????????????????????????????????????????????? Post??????????????????????????????????????????????????????
        let mut post = Post::new();
        let foo = "foo";
        post.add_text(foo);
        post.request_review();
        post.approve();
        post.approve();
        assert_eq!("foo", post.content());
        post.add_text(foo);
        assert_eq!("foo", post.content());
    }

    #[test]
    fn test_blog_not_oop() {
        // https://doc.rust-jp.rs/book-ja/ch17-03-oo-design-patterns.html#%E7%8A%B6%E6%85%8B%E3%81%A8%E6%8C%AF%E3%82%8B%E8%88%9E%E3%81%84%E3%82%92%E5%9E%8B%E3%81%A8%E3%81%97%E3%81%A6%E3%82%B3%E3%83%BC%E3%83%89%E5%8C%96%E3%81%99%E3%82%8B
        // ?????????????????????????????????
        // 1.???????????????????????????????????????????????????
        let post = Post::new();
        assert_eq!(
            "little_rustacean::tests::test_blog_not_oop::DraftPost",
            type_name_of_val(&post)
        );
        struct Post {
            content: String,
        }
        impl Post {
            fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }
        }
        struct DraftPost {
            content: String,
        }
        impl DraftPost {
            fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                    approved_count: 0,
                }
            }
            fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }
        }

        struct PendingReviewPost {
            content: String,
            approved_count: usize,
        }

        impl PendingReviewPost {
            fn approve(&mut self) -> Option<PublishedPost> {
                if self.approved_count > 0 {
                    Some(PublishedPost {
                        content: self.content.clone(),
                    })
                } else {
                    self.approved_count += 1;
                    None
                }
            }
            fn reject(self) -> DraftPost {
                DraftPost {
                    content: self.content,
                }
            }
        }

        struct PublishedPost {
            content: String,
        }

        impl PublishedPost {
            fn content(&self) -> &str {
                &self.content
            }
        }

        // 2.???????????????????????????????????????????????????
        let post = Post::new();
        let post = post.request_review();
        assert_eq!(
            "little_rustacean::tests::test_blog_not_oop::PendingReviewPost",
            type_name_of_val(&post)
        );

        // 3.????????????????????????????????????????????????
        let mut post = Post::new();
        let content = "hello not oop!";
        post.add_text(content);
        let mut post = post.request_review();
        post.approve();
        /*         assert_eq!(
                   "little_rustacean::tests::test_blog_not_oop::PublishedPost",
                   type_name_of_val(&post)
               );
        */
        //assert_eq!( content, post.content() );

        // 4.?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????
        // let draft = Post::new();
        // Compile Error!!: post.content();
        // let pending_review = draft.request_review();
        // Compile Error!!: pending_review.content();

        // 5.??????????????????PendingReview??????Draft?????????reject??????????????????????????????
        let draft = Post::new();
        let pending_review = draft.request_review();
        let draft = pending_review.reject();
        assert_eq!(
            "little_rustacean::tests::test_blog_not_oop::DraftPost",
            type_name_of_val(&draft)
        );

        // 6.?????????Published??????????????????????????????approve???2????????????????????????????????????????????????
        let mut draft = Post::new();
        let content = "hello not oop in rust!";
        draft.add_text(content);
        let mut pending_review = draft.request_review();
        pending_review.approve();
        assert_eq!(
            "little_rustacean::tests::test_blog_not_oop::PendingReviewPost",
            type_name_of_val(&pending_review)
        );
        if let Some(published) = pending_review.approve() {
            assert_eq!(content, published.content());
        }

        // 7.?????????Draft????????????????????????????????????????????????????????????????????????????????????
        //   ?????????: ????????????????????????????????????????????????????????????????????????????????????????????????????????????????????? Post??????????????????????????????????????????????????????
        let mut draft = Post::new();
        draft.add_text("foo");
        let mut pending_review = draft.request_review();
        // Compile Error!!: pending_review.add_text("foo");
        pending_review.approve();
        let _published = pending_review.approve();
        // Compile Error!!: published.add_text("foo");
    }
    #[test]
    fn test() {
        trait MyIterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
        struct Counter {}
        impl MyIterator for Counter {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                todo!()
            }
        }

        assert!(true);
    }
}
