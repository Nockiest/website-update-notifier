use reqwest;
use scraper::{
    Html, Selector  , ElementRef 
};
use std::fs::File;
use std::io::Write; // for File

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.jidelna.cz/jidelni-listek/?jidelna=53";
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    let document: Html = Html::parse_document(&response);

    let selector: Selector = Selector::parse("div[class*='datum']").unwrap();
    let last_element: Option<ElementRef<'_>> = document.select(&selector).last();    
    // if  last_element.is_none() {
    //     println!("No elements found");
    //     return Ok(());
    // }
    println!("Last element: {:?}", last_element);
    if let Some(element) = last_element {
        let date: String = element.inner_html();
        println!("Date: {}", date);
        let mut file = File::create("last_date.txt").unwrap();
        file.write_all(date.as_bytes() ).unwrap();
    }
    Ok(())
}
