use std::env;

use url::Url;
use anyhow::{Result, Context};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use tokio::sync::mpsc::{channel};   
 
pub async fn browser_login() -> Result<String> {
    let port = portpicker::pick_unused_port().context("Failed to pick unused port")?;

    let mut url = Url::parse("https://whop.com/oauth").unwrap();
    url.query_pairs_mut()
        .append_pair("redirect_uri", env::var("WHOP_REDIRECT_URI").expect("Please set the WHOP_REDIRECT_URI environment variable.").as_str())
        .append_pair("client_id", env::var("WHOP_CLIENT_ID").expect("Please set the WHOP_CLIENT_ID environment variable.").as_str());

    println!("Opening: {}", url.clone().as_str());

    if let Err(e) = webbrowser::open(&url.as_str()) {
        println!("Failed to open browser: {}", e); 
    } 

    let (tx, mut rx) = channel::<String>(1);

    let service = make_service_fn(move |_| {
        let tx = tx.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let tx = tx.clone();
                async move {
                    let response = Response::new(Body::from(
                        "<html><body>You may close this window and return to the terminal now.</body></html>\r\n"
                    ));
    
                    let query = req.uri().query();

                    if let Some(query) = query {
                        let query: Vec<(String, String)> = query
                            .split('&')
                            .map(|s| {
                                let mut split = s.split('=');
                                let key = split.next().unwrap().to_string();
                                let value = split.next().unwrap().to_string();
                                (key, value)
                            })
                            .collect::<Vec<_>>();

                        let code = query.iter().find(|(key, _)| key == "code").unwrap().1.clone();

                        tx.send(code).await.unwrap();

                    }
    
                    Ok::<_, Infallible>(response)
                }
            }))
        }
    });

    // create a hyper server to listen for incoming requests
    let server = Server::bind(&([127, 0, 0, 1], 8080).into()).serve(service);

    // start the server in a separate task
    let runtime = tokio::spawn(async move {
        if let Err(error) = server.await {
            eprintln!("Server error: {}", error);
        }

    });

    let response = rx.recv().await;

    runtime.abort();

    let code = response.unwrap();
    println!("Response: {:?}", &code);

    let base_uri = "https://data.whop.com/oauth/token";
    let mut client = reqwest::Client::new();
    let mut res = client.post(base_uri)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("client_id", env::var("WHOP_CLIENT_ID").expect("Please set the WHOP_CLIENT_ID environment variable.").as_str()),
            ("client_secret", env::var("WHOP_CLIENT_SECRET").expect("Please set the WHOP_CLIENT_SECRET environment variable.").as_str()),
            ("redirect_uri", env::var("WHOP_REDIRECT_URI").expect("Please set the WHOP_REDIRECT_URI environment variable.").as_str()),

        ])
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    let access_token = json["access_token"].as_str().unwrap_or_default().to_string();

    Ok(access_token)
}