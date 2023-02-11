use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
};

// async fn fetch() -> Result<String, reqwest::Error> {}
fn fetch() -> Result<String, reqwest::Error> {
    // let client = reqwest::Client::new();
    // let results = client
    //     .get("https://www.youtube.com/results?search_query=brian+x+stephanie+swoard+art+online")
    //     .header("user-agent", "miniyt-scraper")
    //     .send().await?;

    // reqwest::blocking::get(
    //     // "https://www.youtube.com/@BrianxStephanie/playlists?view=1&sort=dd&shelf_id=0",
    //     "https://www.youtube.com/results?search_query=brian+x+stephanie+swoard+art+online",
    // )?
    // .text()

    // let client = reqwest::blocking::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("miniyt/scraper"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("utf-8"));
    
    let client = Client::new();

    let results = client
        .get("https://www.youtube.com/results?search_query=brian+x+stephanie+swoard+art+online")
        .headers(headers)
        // .header("User-Agent", "miniyt-scraper")
        // .header("Accept-Language", "en")
        // // .header("Accept-Encoding", "gzip")
        // .header("Accept-Encoding", "utf-8")
        .send()?;

    results.text()
}

fn parse_data(data: &String) -> String {
    // let start = "var ytInitialData = ";
    // let end = ";</script>";
    
    // data.split(&start);

    // let start_bytes = data.find("var ytInitialData = ").unwrap_or(0);
    // let end_bytes = data.find(";</script>").unwrap_or(data.len());
    // println!("{} {}", start_bytes, end_bytes);
    // let result = &data[start_bytes..end_bytes];
    // result.to_string()

    "".to_string()
}

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {}
fn main() -> Result<(), reqwest::Error> {
    // println!("Hello, world!");
    // let result =
    let result = match fetch() {
        // Ok(res) => {
        //     // let json = serde_json::from_str(&res);
        //     println!("{:#?}", res)
        // }
        Ok(res) => {
            let data = parse_data(&res);
            println!("{:#?}", data)
        },
        Err(err) => eprintln!("{}", err),
    };

    Ok(())
}
