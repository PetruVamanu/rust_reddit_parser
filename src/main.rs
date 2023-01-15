use chrono::{DateTime, Local, NaiveDateTime, Utc};
use clap::Parser;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    r#subreddit: String,

    #[arg(long, default_value_t = String::from("hot"))]
    sort: String,
   
    #[arg(long, default_value_t = -1)]
    seconds: i32,
}

use serde_derive::Deserialize;
#[derive(Deserialize)]
struct RedditInfo {
    title: String,
    author: String,
    permalink: String,
    created_utc: f32,
}

#[derive(Deserialize)]
struct RedditPost {
    // kind: String,
    data: RedditInfo,
}

#[derive(Deserialize)]
struct RedditPostWrapper {
    children: Vec<RedditPost>,
}

#[derive(Deserialize)]
struct RedditResponse {
    data: RedditPostWrapper,
}

fn get_api(url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(url)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    Ok(body)
}

fn get_local_time(timestamp: f32) -> String {
    
    let datetime: DateTime<Utc> =
        DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp as i64, 0), Utc);
    let local_datetime = datetime.with_timezone(&Local);
    local_datetime.format("%d/%m/%Y %H:%M").to_string()
}
fn main() {
    let args = Args::parse();
    let url = format!(
        "https://www.reddit.com/r/{}/{}.json",
        args.subreddit, args.sort
    );

    let seconds = args.seconds;

    let mut posts_set = HashSet::<String>::new();

    loop {
        let response_data = get_api(url.as_str());
        let mut json_data = String::new();
        match response_data {
            Ok(response) => json_data = response,
            Err(_) => {
                //  !panic!("Request failed!");
                }
        };

        let json_string: RedditResponse = serde_json::from_str(&json_data).unwrap();

        for item in json_string.data.children.into_iter() {
            if !posts_set.contains(&item.data.permalink) {
                posts_set.insert(item.data.permalink.clone());
                println!(
                    "{}, {} \n{} \nhttps://www.reddit.com{}\n\n",
                    item.data.title,
                    item.data.author,
                    get_local_time(item.data.created_utc),
                    item.data.permalink
                );
            }
        }
        if seconds == -1
        {
            break;
        }
        sleep(Duration::from_secs(seconds as u64));
    }
}
