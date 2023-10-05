use std::collections::HashMap;
use std::io::Read;
use std::fs;
use serde_json::Value;

// https://www.youtube.com/watch?v=dYVJQ-KQpdc REQWEST
// https://www.youtube.com/watch?v=JYnwbczRAfo SERENITY (DISCORD) 

fn main()  {
    canvas_get_test().unwrap();
}

fn canvas_get_test() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://canvas.qub.ac.uk/api/v1/users/self/".to_owned();
    let token = "12025~wkXXFsMQHgm1BudZwof76FdmzuB3KMdSv91zM9vst4z29xa7YyjWpPsUjGHHSYAP";
    let mut info: HashMap<&str, &str> = HashMap::new();
    info.insert("course", "courses");
    info.insert("planner", "planner");
    info.insert("activities", "activity_stream");
    info.insert("activities_summary", "activity_stream/summary");
    info.insert("events", "upcoming_events");
    info.insert("user", "");
    info.insert("profile", "profile");
    info.insert("avatar", "avatars");
    let merged = url+info["course"]+"?access_token="+token;

    println!("{}", merged);
    let mut res = reqwest::blocking::get(merged)?;
    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());

    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let parsed: Value = serde_json::from_str(&body)?;
    println!("Body:\n{}", serde_json::to_string_pretty(&parsed).unwrap());
    fs::write("output.json", serde_json::to_string_pretty(&parsed).unwrap()).expect("Unable to write file");
    
    Ok(())
}