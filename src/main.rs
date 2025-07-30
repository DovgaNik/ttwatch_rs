use axum::extract::Path;
use axum::{Router, routing::get};
use reqwest::{Client, Error, Response};

async fn show_video(Path(link): Path<String>) -> String {
    let http_client = Client::new();
    // In this function I need to:
    // * check whether the url passed is valid
    // * get the downloadable link using the api (post request)
    let request = http_client
        .get("https://www.tikwm.com/api/?url=".to_owned() + &link + "&hd=1&web=1")
        .build();
    let response = http_client.execute(request.unwrap()).await;
    match response {
        Ok(s) => {
            println!("{}", s.status());
            println!("{}", s.text().await.unwrap());
        }
        Err(e) => {
            println!("{}", e);
            return "An error occurred".to_string();
        }
    }
    // * download the video (get request)
    // * return the video to the client

    return link;
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/{*link}", get(show_video));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
