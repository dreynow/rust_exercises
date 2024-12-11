use aggregator::{Summary, Tweet};

fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    // let number_list = vec![22,34,42,56,32,13];

    // let mut largest = &number_list[0];

}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

