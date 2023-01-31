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
