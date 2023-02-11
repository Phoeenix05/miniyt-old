use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
};

fn fetch() -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    // User-Agent miniyt-scraper or miniyt/scraper
    headers.insert(USER_AGENT, HeaderValue::from_static("miniyt/scraper"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("utf-8"));

    let client = Client::new();

    let results = client
        .get("https://www.youtube.com/results?search_query=brian+x+stephanie+swoard+art+online")
        .headers(headers)
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

    // let re = Regex::new("(?s)var ytInitialData = .*;</script>").unwrap();
    // let re = Regex::new("(?<=var ytInitialData) = (.*)(?=;</script>)").unwrap();
    // let re = Regex::new("var ytInitialData = (?s)(.*);</script>").unwrap();
    // let re = Regex::new(
    //     "/var\\sytInitialData\\s=\\s(.*);<\\/script>/
    // ",
    // )
    // .unwrap();
    // let result = re.split(data);

    let re = Regex::new(r#"var ytInitialData = (.+?);</script>"#).unwrap();
    let captures = re.captures(data).unwrap();
    let result = &captures[1];
    // &captures[1].to_string()
    result.to_string()
}

fn main() -> Result<(), reqwest::Error> {
    let result = match fetch() {
        Ok(res) => {
            let data = parse_data(&res);
            println!("{:#?}", data)
        }
        Err(err) => eprintln!("{}", err),
    };

    Ok(())
}
