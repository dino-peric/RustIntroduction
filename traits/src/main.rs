using std::fmt::Display;

pub trait Summary {
    // default implementation
    fn summarize(&self) -> String {
        String::from("(Read more...)")
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

pub struct WeatherForecast {
    pub high_temp: f64,
    pub low_temp: f64,
    pub chance_of_precipitation: f64,
}

impl Summary for WeatherForecast {
    // default
}

// equivalent to pub fn notify<T: Summary>(item: &T) {
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This one can get two different types that both implement summary
// pub fn notify(item1: &impl Summary, item2: &impl Summary)

// this one has to get two items of the same type that implement summary
// pub fn notify<T: Summary>(item1: &T, item2: &T)

pub fn notify2(item: &(impl Summary + std::fmt::Display)) {
    println!("Breaking news! {}", item.summarize());
}

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let weather = WeatherForecast {
        high_temp: 75.0,
        low_temp: 55.0,
        chance_of_precipitation: 0.2,
    };

    println!("The weather is: {}", weather.summarize());
}
