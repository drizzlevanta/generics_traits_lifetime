use traits::Summary; //traits must brought into scope

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_number(&number_list);
    println!("The largest number is {}", result);

    let pointm1 = generics::Point_M { x: 12, y: 13 };
    let pointm2 = generics::Point_M { x: 3.5, y: '1' };

    let mix_point = pointm1.mixup(pointm2);

    println!("mix point is: {:?} ", mix_point);

    //the following line would cause error
    // println!("pointm1 x is now: {}", pointm1.x);

    //demo of traits
    let elon_tweet = traits::Tweet {
        username: String::from("elonmusk"),
        content: String::from("late night tweet"),
        retweet: false,
        reply: true,
    };

    println!("tweet summary: {}", elon_tweet.summarize());

    let w_news = traits::NewsArticle {
        headline: String::from("what happened today"),
        location: String::from("new york"),
        author: String::from("Aaron"),
        content: String::from("What happened today in new york..."),
    };
    println!("article summary: {}", w_news.summarize());

    lifetime::use_longest();
    lifetime::use_excerpt();
}

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num
        }
    }

    largest
}

mod generics;
mod lifetime;
mod traits;
