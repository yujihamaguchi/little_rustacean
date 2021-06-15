pub mod helper;

use core::panic;
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
#[allow(dead_code)]
fn my_mean(ns: &[usize]) -> Option<usize> {
    match ns.len() {
        0 => None,
        n => Some(my_sum(ns) / n),
    }
}

// 0025. Write a function named 'my_sort' to sort array of some numbers.
#[allow(dead_code)]
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

// https://doc.rust-jp.rs/book-ja/ch17-03-oo-design-patterns.html
// 以下の要件で作ってみる
// 1.ブログ記事は、空の草稿から始まる。
// 2.草稿ができたら、査読が要求される。
// 3.記事が承認されたら、公開される。
// 4.公開されたブログ記事だけが表示する内容を返すので、未承認の記事は、誤って公開されない。
/* #[allow(dead_code)]
struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
#[allow(dead_code)]
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
    fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
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
 */
// その後、状態とふるまいを型としてコード化する(https://doc.rust-jp.rs/book-ja/ch17-03-oo-design-patterns.html#%E7%8A%B6%E6%85%8B%E3%81%A8%E6%8C%AF%E3%82%8B%E8%88%9E%E3%81%84%E3%82%92%E5%9E%8B%E3%81%A8%E3%81%97%E3%81%A6%E3%82%B3%E3%83%BC%E3%83%89%E5%8C%96%E3%81%99%E3%82%8B)
// 要件
// 1.ブログ記事は、空の草稿から始まる。
// 2.草稿ができたら、査読が要求される。
// 3.記事が承認されたら、公開される。
// 4.公開されたブログ記事だけが表示する内容を返すので、未承認の記事は、誤って公開されない。

// その後、このバージョンで追加要件を実現してみる

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

// 0095. The Book: Ch12. An I/O Project: Building a Command Line Program
// ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
// 例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。
// それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。
/*
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

// 0100. The Book: Ch.13.1 Closures: Anonymous Functions that Can Capture Their Environment
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

// Cacher実装の限界
// 0110. 単独の値ではなく、ハッシュマップを保持するようにCacherを改変してみてください。
#[allow(dead_code)]
struct Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

#[allow(dead_code)]
impl<T> Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher2<T> {
        Cacher2 {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, v: u32) -> u32 {
        self.value.entry(v).or_insert((self.calculation)(v));
        self.value[&v]
    }
}

// 0112. 現在のCacher実装の2番目の問題は、引数の型にu32を一つ取り、u32を返すクロージャしか受け付けないことです。
//       例えば、文字列スライスを取り、usizeを返すクロージャの結果をキャッシュしたくなるかもしれません。
//       この問題を修正するには、Cacher機能の柔軟性を向上させるためによりジェネリックな引数を導入してみてください。
#[allow(dead_code)]
struct Cacher3<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    value: HashMap<U, U>,
}

#[allow(dead_code)]
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

// 0120. 絶対に1から5をカウントするだけのイテレータを作成しましょう。
#[allow(dead_code)]
struct Counter {
    count: u32,
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

#[allow(dead_code)]
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

// Genbade yakudatsu ch.2

#[cfg(test)]
mod tests {

    use super::*;
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
        /*         assert_eq!(2, helper::execution_seconds(|| generate_workout(24, 0)));
               assert_eq!(2, helper::execution_seconds(|| generate_workout(25, 0)));
               assert_eq!(0, helper::execution_seconds(|| generate_workout(25, 3)));
        */
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

    /*    #[test]
       fn test_blog() {
           // 1.ブログ記事は、空の草稿から始まる。
           // 2.草稿ができたら、査読が要求される。
           // 3.記事が承認されたら、公開される。
           // 4.公開されたブログ記事だけが表示する内容を返すので、未承認の記事は、誤って公開されない。
           let content = "hello, world!";
            let mut post = Post::new();
           post.add_text(content);
           assert_eq!("", post.content());
           post.request_review();
           assert_eq!("", post.content());
           post.approve();
           assert_eq!(content, post.content());
    */
    // その後、以下の要件を満たしてみる
    // - 記事の状態をPendingReviewからDraftに戻すrejectメソッドを追加する。
    /*         let mut post = Post::new();
           post.add_text(content);
           post.request_review();
           post.reject();
           assert_eq!("", post.content());
           post.approve();
           assert_eq!("", post.content());
    */
    // - 状態がPublishedに変化させられる前にapproveを2回呼び出す必要があるようにする。
    /*         let mut post = Post::new();
           post.add_text(content);
           post.request_review();
           post.approve();
           assert_eq!("", post.content());
           post.approve();
           assert_eq!(content, post.content());
    */
    // - 記事がDraft状態の時のみテキスト内容をユーザが追加できるようにする。
    //   ヒント: ステートオブジェクトに内容について変わる可能性のあるものの責任を持たせつつも、 Postを変更することには責任を持たせない。
    /*         let mut post = Post::new();
           post.add_text(content);
           post.request_review();
           post.add_text(content);
           post.approve();
           post.approve();
           assert_eq!(content, post.content());
    }
    */
    #[test]
    fn text_blog_type() {
        // 1.ブログ記事は、空の草稿から始まる。
        // 2.草稿ができたら、査読が要求される。
        // 3.記事が承認されたら、公開される。
        // 4.公開されたブログ記事だけが表示する内容を返すので、未承認の記事は、誤って公開されない。
        struct Post {
            content: String,
        }
        struct DraftPost {
            content: String,
        }
        struct PendingReviewPost {
            content: String,
        }
        impl Post {
            fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }
            fn content(&self) -> &str {
                &self.content
            }
        }
        impl DraftPost {
            fn add_text(&mut self, text: &str) {
                self.content.push_str(text)
            }
            fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }
        impl PendingReviewPost {
            fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }
        let mut post = Post::new();
        let content = "hello world!";
        post.add_text(content);
        let post = post.request_review();
        let post = post.approve();
        assert_eq!(content, post.content());
        // その後、以下の要件を満たしてみる
        // - 記事の状態をPendingReviewからDraftに戻すrejectメソッドを追加する。
        // - 状態がPublishedに変化させられる前にapproveを2回呼び出す必要があるようにする。
        // - 記事がDraft状態の時のみテキスト内容をユーザが追加できるようにする。
        //   ヒント: ステートオブジェクトに内容について変わる可能性のあるものの責任を持たせつつも、 Postを変更することには責任を持たせない。
    }
}
