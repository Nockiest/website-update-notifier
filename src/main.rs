use reqwest;
use scraper::{
    Html, Selector  , ElementRef 
};
// use scraper::element_ref

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
    if  last_element.is_none() {
        println!("No elements found");
        return Ok(());
    }
    println!("Last element: {:?}", last_element);
    if let Some(element) = last_element {
        let date = element.inner_html();
        println!("Date: {}", date);
    }
    // let date = last_element.html() //last_element.html();  //.value().collect::<Vec<_>>().join(" ");
    // println!("Date: {}", date);
    // {
    //     let date = element.text().collect::<Vec<_>>().join(" ");
    //     println!("Date: {}", date);
    // }
    Ok(())
}
