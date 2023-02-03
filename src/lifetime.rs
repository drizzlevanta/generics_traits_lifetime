use std::fmt::Display;

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

//When returning a reference from a function, the lifetime parameter for the return type needs to match
//the lifetime parameter for one of the parameters
// fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     s1
// }

fn longest2(s1: &str, s2: &str) -> String {
    String::from("really long string")
}

pub fn use_longest() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);

    println!("The longest string is {}", result);

    let result2 = longest2(s1.as_str(), s2);
    println!("Longest string 2 is {}", result2);
}

//lifetime in struct
struct Excerpt<'a> {
    part: &'a str,
}

pub fn use_excerpt() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Can't find first sentence...");
    println!("excerpt is: {}", first_sentence);
    println!("novel is {}", novel);

    let i = Excerpt {
        part: first_sentence,
    };

    println!("excerpt struct first_sentence is: {}", i.part);
}

//define generic type parameters, traits, and lifetimes all in one function
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//TODO no lifetime for return type
fn longest_with_an_announcement2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//TODO experiments with lifetime

//variant1
fn foo1<'a>(x: &'a u32, y: &'a u32) -> &'a u32 {
    x
}

//variant2
fn foo2<'a, 'b>(x: &'a u32, y: &'b u32) -> &'a u32 {
    x
}

//variant3
// fn foo3<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {
//     x
// }

fn use_foo() {
    let x = 12;
    let z = {
        let y = 42;
        foo2(&x, &y)
    };
}

fn t_ref<'a, T>(t: &'a T) {}

//TODO illegal, can't determine output lifetime, no inputs
// fn get_str() -> &str {
//     "fdjakl"
// }

//OK
fn get_str<'a>() -> &'a str {
    "fdjakl"
}

fn overlap<'a>(s: &'a str, t: &str) -> &'a str {
    "fdjakl"
} // output can't outlive s
fn overlap2<'a>(s: &str, t: &'a str) -> &'a str {
    "dajkhk"
} // output can't outlive t

fn overlap3<'a>(s: &str, t: &str) -> &'a str {
    "fdasjl"
} // no relationship between input & output lifetimes

fn demo() {
    let s1 = String::from("first string");
    let ol;
    {
        let s2 = String::from("second string");
        ol = overlap3(&s1, &s2); //TODO
    }
    println!("overlap is {ol}");
}

trait LifetimeDemo {
    fn overlap<'a>(s: &'a str, t: &'a str) -> &'a str; // output can't outlive s & t
    fn overlap2(s: &str, t: &str) -> &'static str; // output can outlive s & t
    fn overlap3<'a>(s: &str, t: &str) -> &'a str; // no relationship between input & output lifetimes
}
