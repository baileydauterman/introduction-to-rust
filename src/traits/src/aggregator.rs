pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        return String::from("(Read more...)");
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        return format!("{}", self.author);
    }

    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }

    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking new! {}", item.summarize());
}
