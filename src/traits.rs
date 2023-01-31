//define trait Summary
//One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type
//is local to our crate.
//we can’t implement external traits on external types.

use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String; //this method is implied public
                                   //if you can see the trait, you can always see all of it

    //default implementation, can ne overriden
    fn summarize_author(&self) -> String {
        format!("author is...")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub headline: String,
    pub location: String,
}

//implement trait Summary for struct Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

//implement trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//pass in traits as params
fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

//trait bound
fn notify_2(item: &impl Summary, item2: impl Summary) {}

//trait bound and reinforce item1 and item2 are of the same type
fn notify_3<T: Summary>(item1: T, item2: T) {}

//multiple trait bound
fn notify_4(item: &(impl Summary + Display)) {}

//or
fn notify_5<T: Summary + Display>(item: T) {}

//using where to organize trait bounds
fn notify_6<T: Display + Clone, U: Clone + Debug>(item1: T, item2: U) {}

//We can also conditionally implement a trait for any type that implements another trait.
//blanket implementation
//impl<T: Display> ToString for T {}
