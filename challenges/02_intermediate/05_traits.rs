// Challenge: Traits
// Complete each method and function marked with `todo!()`.
// Verify with: rustc --test 05_traits.rs && ./05_traits

/// Exercise 1: Define a Trait
/// In TypeScript: `interface Summary { summarize(): string; }`
pub trait Summary {
    fn summarize(&self) -> String;
}

/// Exercise 2: Implement a Trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /// Format: "{headline}, by {author} ({location})"
    fn summarize(&self) -> String {
        todo!()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    /// Format: "{username}: {content}"
    fn summarize(&self) -> String {
        todo!()
    }
}

/// Exercise 3: Trait with Default Implementation
/// If an implementing type doesn't override `greet`, it uses this default string.
pub trait Greeter {
    fn greet(&self) -> String {
        String::from("Hello!")
    }
}

pub struct Person;
impl Greeter for Person {} // uses default

pub struct Robot;
impl Greeter for Robot {
    fn greet(&self) -> String {
        todo!()
    }
}

/// Exercise 4: Trait Bounds (`impl Trait` syntax)
/// Takes any type that implements `Summary` and returns a formatted alert: "[ALERT] {summary}".
pub fn notify(item: &impl Summary) -> String {
    todo!()
}

/// Exercise 5: Trait Bounds with where clause and multiple traits
/// Takes an item implementing BOTH `Summary` and `Greeter`, returning "{greet} - {summarize}".
pub fn announce<T>(item: &T) -> String
where
    T: Summary + Greeter,
{
    todo!()
}

fn main() {
    println!("Run `rustc --test 05_traits.rs && ./05_traits` to verify your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_article_summary() {
        let article = NewsArticle {
            headline: String::from("Rust 1.70 Released"),
            location: String::from("Internet"),
            author: String::from("Rust Team"),
            content: String::from("Exciting new features..."),
        };
        assert_eq!(article.summarize(), "Rust 1.70 Released, by Rust Team (Internet)");
    }

    #[test]
    fn test_tweet_summary() {
        let tweet = Tweet {
            username: String::from("ferris"),
            content: String::from("Rust is amazing!"),
        };
        assert_eq!(tweet.summarize(), "ferris: Rust is amazing!");
    }

    #[test]
    fn test_greeter_trait() {
        let p = Person;
        assert_eq!(p.greet(), "Hello!");

        let r = Robot;
        assert_eq!(r.greet(), "BEEP BOOP");
    }
    
    #[test]
    fn test_notify() {
        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("New release out!"),
        };
        assert_eq!(notify(&tweet), "[ALERT] rustlang: New release out!");
    }

    struct Announceable {
        name: String,
    }
    impl Summary for Announceable {
        fn summarize(&self) -> String {
            format!("Special announcement from {}", self.name)
        }
    }
    impl Greeter for Announceable {
        fn greet(&self) -> String {
            String::from("Attention please")
        }
    }

    #[test]
    fn test_announce() {
        let item = Announceable { name: String::from("Rust Foundation") };
        assert_eq!(announce(&item), "Attention please - Special announcement from Rust Foundation");
    }
}
