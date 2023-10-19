use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use std::env;
use dotenv;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

// https://www.youtube.com/watch?v=dYVJQ-KQpdc REQWEST
// https://www.youtube.com/watch?v=JYnwbczRAfo SERENITY (DISCORD) 


// const PING_MESSAGE: &str = "Pong!";
// const PING_COMMAND: &str = "!ping";

const URL: &str = "https://canvas.qub.ac.uk/api/v1/users/self/";
const DISCORD_TOKEN: &str = "MTE1OTIzNjI5Nzc0MzkzNzU5Ng.GKlK2B.P2NG84au7TwYg60D6q14B41F3ebxZu6K4F-Q24";

#[derive(Hash, PartialEq, Eq)]
enum Resource {
    Course,
    Planner,
    Activities,
    ActivitiesSummary,
    Events,
    User,
    Profile,
    Avatar,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" /* == PING_COMMAND */ {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


#[tokio::main]
async fn main() {
    canvas_request(Resource::Course).await.unwrap();

    dotenv::dotenv().ok();
    env::set_var("DISCORD_TOKEN", dotenv::var("DISCORD_TOKEN").unwrap());
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn canvas_request(resource: Resource) -> Result<(), Box<dyn std::error::Error>> {
    let token: &str = "12025~VLprWlL440cSbPD4B87gkX2di9H2SMDOylPHImgiZhzo9pkSxAPBnwLxDgGwqAso";
    let mut info = HashMap::new();
    info.insert(Resource::Course, "courses");
    info.insert(Resource::Planner, "planner");
    info.insert(Resource::Activities, "activity_stream");
    info.insert(Resource::ActivitiesSummary, "activity_stream/summary");
    info.insert(Resource::Events, "upcoming_events");
    info.insert(Resource::User, "");
    info.insert(Resource::Profile, "profile");
    info.insert(Resource::Avatar, "avatars");

    let merged_url = URL.to_owned()+info[&resource]+"?access_token="+token;
    println!("\n{}\n", merged_url);

    let body = reqwest::get(merged_url).await?.text().await?;
    let mut parsed: Vec<HashMap<String, Value>> = Vec::new();

    if body.chars().nth(0).unwrap() == '[' { // If returned format is arrayS
        parsed = serde_json::from_str(&body)?;
    } else {
        parsed.insert(0, serde_json::from_str(&body)?);
    }

    //println!("{:#?}", json_map);  // advanced print
    let pretty = serde_json::to_string_pretty(&parsed).unwrap();
    println!("Body:\n{}", pretty);
    fs::write("output.json", pretty).expect("Unable to write file");

    Ok(())
}

// fn canvas_get_test() -> Result<(), Box<dyn std::error::Error>> {
//     let url = "https://canvas.qub.ac.uk/api/v1/users/self/".to_owned();
//     let token = "12025~wkXXFsMQHgm1BudZwof76FdmzuB3KMdSv91zM9vst4z29xa7YyjWpPsUjGHHSYAP";
//     let mut info: HashMap<&str, &str> = HashMap::new();
//     info.insert("course", "courses");
//     info.insert("planner", "planner");
//     info.insert("activities", "activity_stream");
//     info.insert("activities_summary", "activity_stream/summary");
//     info.insert("events", "upcoming_events");
//     info.insert("user", "");
//     info.insert("profile", "profile");
//     info.insert("avatar", "avatars");
//     let merged = url+info["course"]+"?access_token="+token;

//     println!("{}", merged);
//     let mut res = reqwest::blocking::get(merged)?;
//     //println!("Status: {}", res.status());
//     //println!("Headers:\n{:#?}", res.headers());

//     let mut body = String::new();
//     res.read_to_string(&mut body)?;
//     let parsed: Value = serde_json::from_str(&body)?;
//     println!("Body:\n{}", serde_json::to_string_pretty(&parsed).unwrap());
//     fs::write("output.json", serde_json::to_string_pretty(&parsed).unwrap()).expect("Unable to write file");
    
//     Ok(())
// }