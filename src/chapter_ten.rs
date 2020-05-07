
pub trait Summary {
    fn summarize(&self) -> String;

    fn get_useless_str(&self) -> String {
        String::from("I am useless.")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn get_useless_str(&self) -> String {
        String::from("Now I am kinda useful.")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// This function will compile as long as the types of the values in the slice implements the
// PartialOrd and Copy traits, ex. i32 and char
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Lifetime annotation in the signature: all the references in the parameters and the return value
// must have the same lifetime
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
