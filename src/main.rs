use reqwest;
use scraper::{
    Html, Selector  , ElementRef 
};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write}; // for File
use regex::Regex;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.jidelna.cz/jidelni-listek/?jidelna=53";
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    let document: Html = Html::parse_document(&response);
    let previous_date = read_previous_date();
    println!("Previous date: {}", previous_date);
    let selector: Selector = Selector::parse("div[class*='datum']").unwrap();
    let last_element: Option<ElementRef<'_>> = document.select(&selector).last();    
    println!("Last element: {:?}", last_element);
    if let Some(element) = last_element {
        let raw_date: String = element.inner_html();
        println!("new_date: {}", raw_date);
        
        let re = Regex::new(r"(\d{1,2})\.\s*(\d{1,2})\.\s*(\d{4})").unwrap();
        
        if let Some(captures) = re.captures(&raw_date) {
            let day = &captures[1];
            let month = &captures[2];
            let year = &captures[3];
            println!("day: {}, month: {}, year: {}", day, month, year);
            let formatted_date = format!("{}-{}-{}", year, month, day);
            let mut file = File::create("previous_date.txt").unwrap();
            file.write_all(formatted_date.as_bytes() ).unwrap();
        } else {
            println!("No match found");
        }
        
        
       
    }
    Ok(())
}

fn read_previous_date() -> String {
    let mut contents = String::new();
    if let Ok(mut file) = File::open("previous_date.txt") {
        file.read_to_string(&mut contents).unwrap();
    } else   {
        return String::new();
    }
    contents
}