#[macro_use(c)]
extern crate cute;

use std::collections::HashMap;
use crate::chapter_ten::Summary;

//--- Chapter 2: Guess Game ---
pub mod chapter_two;

//--- Chapter 5: Structs
pub mod chapter_five;

//--- Chapter 6: Enums and Pattern Matching
pub mod chapter_six;

//--- Chapter 10: Generic Types, Traits, and Lifetimes
pub mod chapter_ten;

//--- Chapter 11: Tests
#[cfg(test)]
pub mod chapter_eleven;

//--- Chapter 13: Iterators and Closures
pub mod chapter_thirteen;

//--- Chapter 15: Smart Pointers
pub mod chapter_fifteen;

//--- Chapter 17: OOP
pub mod chapter_seventeen;


fn main() {
    //-- Chapter 5: Structs
    println!("\n=== Chapter 5: Structs ===");
    let mut user1 = chapter_five::User::new(String::from("user1"),
                                                      String::from("user1@test.com"));
    user1.email = String::from("new_user1@test.com");
    user1.sign_in(); user1.deactivate();

    println!("user1 sign in count: {}", user1.get_sign_in_count());
    println!("user1 is {}",
             if user1.is_active() {String::from("activated")} else {String::from("deactivated")});

    //-- Chapter 6: Enums
    println!("\n=== Chapter 6: Enums ===");
    let ip1 = chapter_six::IpAddr::V4(192, 168, 1, 1);
    let ip2 = chapter_six::IpAddr::V6(String::from("::1"));
    match &ip1 {
        chapter_six::IpAddr::V4(a, b, c, d) => {
            println!("ip1: {}.{}.{}.{}", a, b, c, d);
        },
        chapter_six::IpAddr::V6(s) => {
            println!("ip1: {}", s);
        }
    }
    if let chapter_six::IpAddr::V4(a, b, c, d) = ip1 {
        println!("ip1: {}.{}.{}.{}", a, b, c, d);
    }
    println!("ip2: {}", ip2.get_str());

    //-- Chapter 8: Common Collections
    println!("\n=== Chapter 8: Common Collections ===");
    // Vector (w/ the use of cute: https://crates.io/crates/cute)
    let mut v = c![x, for x in 0..3];
    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }
    println!("v = {:?}", v);

    // String
    let s1 = String::from("hello ") + "world";
    let s2 = String::from("hello");
    let s3 = String::from("world");
    let s1 = format!("{} {} {}", s1, s2, s3);
    let s1 = &s1[..12];
    println!("{}", s1);

    // Hash Map
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 30);
    scores.insert(String::from("red"), 50);
    println!("{}", scores["blue"]);
    for (k, v) in &scores {
        println!("key: {}, value: {}", k, v);
    }

    //-- Chapter 10: Generics, Traits, Lifetimes
    println!("\n=== Chapter 10: Generics, Traits, Lifetimes");
    let tweet1 = chapter_ten::Tweet{ username: String::from("Donald"),
                                     content: String::from("Make US great again!"),
                                     retweet: false};
    let news1 = chapter_ten::NewsArticle { headline: String::from("Fake News"),
                                           location: String::from("Washington"),
                                           author: String::from("Donald Trump"),
                                           content: String::from("Make US great again!")};
    println!("tweet1 summary: {}", tweet1.summarize());
    println!("news1 summary: {}", news1.summarize());
    println!("{}", news1.get_useless_str());

    let lst_i32 = [1, 2, 3, 4];
    println!("Largest of lst_i32 = {}", chapter_ten::largest(&lst_i32));
    let lst_char = ['a', 'b', 'c', 'd'];
    println!("Largest of lst_char = {}", chapter_ten::largest(&lst_char));
    println!("The longest of abcd and xyz is: {}", chapter_ten::longest("abcd", "xyz"));

    //-- Chapter 13: Iterators and Closures
    chapter_thirteen::run_closure_ex();

    //-- Chapter 15: Smart Pointers

    // Boxes allow you to store data on the heap rather than the stack.
    // <b> has the value of a Box that points to the value 5, which is allocated on the heap.
    let b = Box::new(5);
    println!("b = {}", b);

    // Enabling Recursive Types with Boxes:
    // Recursive types can have infinitely big size, however boxes have a known size, so
    // inserting a box in a recursive type definition.
    chapter_fifteen::run_linked_list_ex();
    chapter_fifteen::run_my_box_ex();

    //-- Chapter 16: OOP
    chapter_seventeen::run_screen_ex();
    chapter_seventeen::run_states_ex();
}
