mod aggregator;

use aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false, 
        retweet: false,
    };

    println!("1 new tweet {}", tweet.summarize());

    println!("1 new tweet: {}", tweet.summarize_author());

    let article = NewsArticle {
        headline: String::from("New super cool language coming soon"),
        location: String::from("Your moms house"),
        author: String::from("Me!"),
        content: String::from(
            "There is a cool new language coming to homes \
            near you soon. Be sure to be on the lookout for \
            this super cool new language. It's called Rust."
            ),
    };

    println!("New aritcle available: {}", article.summarize());
    println!("New aritcle available: {}", article.summarize_author());
}

// This will return an implementation for a summary trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
            ),
        reply: false,
        retweet: false,
    }
}

