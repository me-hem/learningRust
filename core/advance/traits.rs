trait Summary {
    fn summarize(&self) -> String{
        String::from("Read more..")
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}

//Normal
// pub fn notify<T:Summary>(item: &T){
//     println!("\nBreaking News! {}", item.summarize());
// }

//Using where
pub fn notify<T>(item: &T)
where T:Summary {
    println!("\nBreaking News! {}", item.summarize());
}


fn main(){
    let tweet = Tweet{
        username: String::from("its_me"),
        content: String::from("I don't know"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
    println!("\n1 new tweet {}", tweet.summarize())
}