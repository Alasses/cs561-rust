mod model;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //
    // Try to fetch from the mock server
    //

    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("user", "John Doe");
    map.insert("password", "abc123");

    let response = client
        .post("http://ec2-3-93-143-170.compute-1.amazonaws.com:3000/v1/auth")
        .json(&map)
        .send()
        .await?;
    
    let auth_info = response
        .json::<model::Auth>()
        .await?;

    let token = &auth_info.auth;

    println!("\nAuth from Mock Server:\n {:?}", token);


    let response = client
        .get("http://ec2-3-93-143-170.compute-1.amazonaws.com:3000/v1/hello")
        .bearer_auth(token)
        .send()
        .await?;

    let message = response
        .json::<model::Greet>()
        .await?;

    println!("\nGreeting from Mock Server:\n {:?}", message); 


    let response = client
        .get("http://ec2-3-93-143-170.compute-1.amazonaws.com:3000/v1/weather")
        .bearer_auth(token)
        .send()
        .await?;

    let weather = response
        .json::<model::Weather>()
        .await?;

    println!("\nWeather from Mock Server:\n {:?}", weather);

    Ok(())
}
