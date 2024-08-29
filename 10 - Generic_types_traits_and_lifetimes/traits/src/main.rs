use std::iter::Sum;

// Traits are essentially Interfaces from for example C#, but a bit different.
//
pub trait Summary {
    fn summarize(&self) -> String;
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
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

/*
 * you can also define default functionality for methods in a task.
 */

pub trait Trait2 {
    fn do_stuff() -> String {
        String::from("value")
    }
}

/*
 * you can have a function take a trait as an input, that way you know what the object you receive has/can do.
 */

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}
/*
 * which is short for
 */

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/*
 * or this
 */

pub fn notify3<T>(item: &T) 
where
    T:Summary
    {
        println!("Breaking news! {}", item.summarize());
}

/*
 * use &impl, unless you need two params that implement the same trait.
 * 
 * also, if you have something that implements two traits, just add a '+' between both traits.
 */

/*
 * to have a function that returns an object that implements a trait:
 */

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

/*
 * just be warned that you can't return two different types with this. 
 */