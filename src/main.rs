use reqwest;
use scraper::{
    Html, Selector
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.jidelna.cz/jidelni-listek/?jidelna=53";
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    let document: Html = Html::parse_document(&response);

    let selector: Selector = Selector::parse("div[class*='datum']").unwrap();
    let last_element: Option<scraper::ElementRef<'_>> = document.select(&selector).last();    
    println!("Last element: {:?}", last_element);
    // {
    //     let date = element.text().collect::<Vec<_>>().join(" ");
    //     println!("Date: {}", date);
    // }
    Ok(())
}
