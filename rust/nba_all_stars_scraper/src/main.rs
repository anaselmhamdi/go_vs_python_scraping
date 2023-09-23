use dotenv::dotenv;
use futures::future::join_all;
use reqwest::{Client, Proxy};
use std::error::Error;
use std::time::{Duration, Instant};

// fn main() {
//     let start_time = Instant::now();

//     dotenv().ok();
//     let response = reqwest::blocking::get("https://en.wikipedia.org/wiki/List_of_NBA_All-Stars")
//         .unwrap()
//         .text()
//         .unwrap();
//     let html = scraper::Html::parse_document(&response);
//     let table_selector = scraper::Selector::parse(".wikitable.sortable").unwrap();
//     let links_selector = scraper::Selector::parse("span.fn > a").unwrap();
//     let table = html.select(&table_selector).next().unwrap();
//     let links = table.select(&links_selector);
//     for link in links {
//         let href = link.value().attr("href").unwrap();
//         let response_all_star =
//             reqwest::blocking::get(format!("{}{}", "https://en.wikipedia.org", href))
//                 .unwrap()
//                 .text()
//                 .unwrap();
//         let html_all_star = scraper::Html::parse_document(&response_all_star);
//         let name_selector = scraper::Selector::parse("caption").unwrap();
//         let info_selector = scraper::Selector::parse("td.infobox-data").unwrap();

//         let name_element = html_all_star.select(&name_selector).next();
//         if let Some(_element) = name_element {
//         } else {
//             continue;
//         }
//         let info_element = html_all_star.select(&info_selector).next();
//         if let Some(_element) = info_element {
//         } else {
//             continue;
//         }
//         let info = info_element.unwrap().text().collect::<Vec<_>>();
//         let name = name_element.unwrap().text().collect::<String>();
//         println!("{:?}{:?}", name, info);
//     }
//     let end_time = Instant::now();
//     let duration = end_time.duration_since(start_time);
//     println!(
//         "Execution time: {}.{:03} seconds",
//         duration.as_secs(),
//         duration.subsec_millis()
//     );
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    dotenv().ok();
    let proxy_url = std::env::var("PROXY_URL").expect("PROXY_URL must be set");
    let proxy = Proxy::http(proxy_url)?;
    let client = Client::builder().proxy(proxy).build()?;
    let response = client
        .get("https://en.wikipedia.org/wiki/List_of_NBA_All-Stars")
        .send()
        .await?
        .text()
        .await?;
    let html = scraper::Html::parse_document(&response);
    let table_selector = scraper::Selector::parse(".wikitable.sortable").unwrap();
    let links_selector = scraper::Selector::parse("span.fn > a").unwrap();
    let table = html.select(&table_selector).next().unwrap();
    let links = table.select(&links_selector);

    let mut futures = vec![];

    for link in links {
        let client = client.clone();
        let href = link.value().attr("href").unwrap();
        let response_all_star = client
            .get(format!("{}{}", "https://en.wikipedia.org", href))
            .send()
            .await?
            .text()
            .await?;
        let future = tokio::spawn(async move {
            let html_all_star = scraper::Html::parse_document(&response_all_star);
            let name_selector = scraper::Selector::parse("caption").unwrap();
            let info_selector = scraper::Selector::parse("td.infobox-data").unwrap();

            let name_element = html_all_star.select(&name_selector).next();
            if let Some(_element) = name_element {
            } else {
                return;
            }
            let info_element = html_all_star.select(&info_selector).next();
            if let Some(_element) = info_element {
            } else {
                return;
            }
            let info = info_element.unwrap().text().collect::<Vec<_>>();
            let name = name_element.unwrap().text().collect::<String>();
            println!("{:?}{:?}", name, info);
        });
        futures.push(future);
    }

    join_all(futures).await;
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    println!(
        "Execution time: {}.{:03} seconds",
        duration.as_secs(),
        duration.subsec_millis()
    );
    Ok(())
}
