// use regex::Regex;
// use reqwest::{
//     blocking::Client,
//     header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
// };

// fn fetch() -> Result<String, reqwest::Error> {
//     let mut headers = HeaderMap::new();
//     // User-Agent miniyt-scraper or miniyt/scraper
//     headers.insert(USER_AGENT, HeaderValue::from_static("miniyt/scraper"));
//     headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
//     headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("utf-8"));

//     let client = Client::new();

//     let results = client
//         .get("https://www.youtube.com/results?search_query=brian+x+stephanie+swoard+art+online")
//         .headers(headers)
//         .send()?;

//     results.text()
// }

// fn parse_data(data: &String) -> String {
//     let re = Regex::new(r#"var ytInitialData = (.+?);</script>"#).unwrap();
//     let captures = re.captures(data).unwrap();
//     let result = &captures[1];
//     result.to_string()
// }

fn main() -> Result<(), reqwest::Error> {
    // let result = match fetch() {
    //     Ok(res) => {
    //         let data = parse_data(&res);
    //         println!("{:#?}", data)
    //     }
    //     Err(err) => eprintln!("{}", err),
    // };

    Ok(())
}
