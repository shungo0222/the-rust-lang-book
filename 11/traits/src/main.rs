use std::fmt::{Display, Debug};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }
}

// ========================================================

// Syntax sugar
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// ========================================================

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
// }

fn some_function<T, U>(t: &T, u: &U) -> i32
    where 
        T: Display + Clone,
        U: Clone + Debug,
{
    // ...
    10
}

// ========================================================

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@pochi"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The sky is falling!!"),
        content: String::from("The sky is not actually falling."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);

    println!("{}", returns_summarizable().summarize());
}
