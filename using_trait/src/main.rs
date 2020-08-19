use using_trait::{Tweet, notify,notify_bound,notify_mutiple_variant,notify_mutiple};
use using_trait::Summary;
use using_trait::NewsArticle;
use using_trait::WXPublicAccount;
fn main() {
    let a_tweet = Tweet{
        username:"horse_ebooks".to_string(),
        content: String::from("of course, as you probably already know, people"),
        reply:false,
        retweet:false
    };
    let a_wxpost = WXPublicAccount{
        username: String::from("蒂姆先生"),
        content: String::from("海豚")
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };
    println!("1 new tweet:{}",a_tweet.summarize());
    println!("1 new wechat post:{}",a_wxpost.summarize());
    //对实现对应trait的对象调用notify方法
    notify(a_tweet);
    notify(a_wxpost);
    notify(article);
    let a_tweet = Tweet{
        username:"horse_ebooks".to_string(),
        content: String::from("of course, as you probably already know, people"),
        reply:false,
        retweet:false
    };
    let a_wxpost = WXPublicAccount{
        username: String::from("蒂姆先生"),
        content: String::from("海豚")
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };
    notify_bound(a_tweet);
    notify_bound(article);
    notify_bound(a_wxpost);
    let a_tweet = Tweet{
        username:"horse_ebooks".to_string(),
        content: String::from("of course, as you probably already know, people"),
        reply:false,
        retweet:false
    };
    let a_wxpost = WXPublicAccount{
        username: String::from("蒂姆先生"),
        content: String::from("海豚")
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };
    // notify_mutiple(article,a_wxpost);
    notify_mutiple_variant(article,a_wxpost);
    println!("Hello, world!");
}
