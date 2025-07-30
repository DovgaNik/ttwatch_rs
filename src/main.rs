use axum::{Router, extract::Path, routing::get, response::Redirect};
use reqwest::Client;
use serde::Deserialize;

// Structs required to parse the JSON returned from the API
#[derive(Deserialize)]
struct ApiResponse {
    data: VideoData,
}

#[derive(Deserialize)]
struct VideoData {
    hdplay: String,
}

async fn show_video(Path(link): Path<String>) -> Redirect {
    let http_client = Client::new();
    // In this function I need to:
    // * check whether the url passed is valid
    // * get the downloadable link using the api (post request)
    let request = http_client
        .get("https://www.tikwm.com/api/?url=".to_owned() + &link + "&hd=1&web=1")
        .build();
    let response = http_client.execute(request.unwrap()).await.unwrap();
    let json_parsed = response.json::<ApiResponse>().await.unwrap();
    let hd_link = json_parsed.data.hdplay;

    Redirect::permanent(&format!("https://www.tikwm.com{}", hd_link))
    // * download the video (get request)
    // * return the video to the client
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/{*link}", get(show_video));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
