use regex::Regex;
use reqwest;
use scraper::{ElementRef, Html, Selector};
use std::fs::File;
use std::io::{Read, Write}; // for File
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env; // Import the env module
use dotenv::dotenv;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let url = "https://www.jidelna.cz/jidelni-listek/?jidelna=53";
    let response = reqwest::get(url).await?.text().await?;
    println!("Response: {}", response);
    let document: Html = Html::parse_document(&response);
    let previous_date = read_previous_date();
    println!("Previous date: {}", previous_date);
    let selector: Selector = Selector::parse("div[class*='datum']").unwrap();
    let last_element: Option<ElementRef<'_>> = document.select(&selector).last();
    println!("Last element: {:?}", last_element);
    if let Some(element) = last_element {
        let raw_date: String = element.inner_html();
        let new_date: String = format_date(&raw_date);
        if new_date.is_empty() {
            println!("Date format is incorrect or date doesnt exist");
            return Ok(());
        }
        if new_date == previous_date {
            println!(
                "Date is the same as previous date {} {}",
                new_date, previous_date
            );
        } else {
            println!(
                "Date is different from previous date {} {}",
                new_date, previous_date
            );
            send_email(&new_date)?;
        }
        if let Ok(file) = File::open("previous_date.txt") {
            println!("File found! {}", file.metadata().unwrap().len());
        } else {
            println!("File not found!");
        }
        let mut file = File::create("previous_date.txt").unwrap();
        file.write_all(new_date.as_bytes()).unwrap();
    }

 

    Ok(())
}

fn format_date(date: &str) -> String {
    let re = Regex::new(r"(\d{1,2})\.\s*(\d{1,2})\.\s*(\d{4})").unwrap();
    if let Some(captures) = re.captures(date) {
        let day = &captures[1];
        let month = &captures[2];
        let year = &captures[3];
        format!("{}-{}-{}", year, month, day)
    } else {
        String::new()
    }
}

fn read_previous_date() -> String {
    let mut contents: String = String::new();
    if let Ok(mut file) = File::open("previous_date.txt") {
        file.read_to_string(&mut contents).unwrap();
    } else {
        return String::new();
    }
    contents
}

fn send_email(new_date: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Reads the .env file
    
    let EMAIL: String  = env::var("EMAIL").expect("EMAIL environment variable not set");
    let password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD environment variable not set");
   let to_email: String = env::var("TO_EMAIL").expect("TO_EMAIL environment variable not set");
    let email_message = Message::builder()
        .from(EMAIL.parse()? ) // Replace with your EMAIL
        .to(to_email.parse()? ) // Replace with the recipient's email
        .subject("New Meal Found!")
        .body(format!(
            "A new meal is available for the date: {}",
            new_date
        ))?;

    let creds = Credentials::new(EMAIL.to_string(), password.to_string());
    let mailer = SmtpTransport::relay("smtp.gmail.com")? // Gmail's SMTP server
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email_message)?;
    println!("Email sent successfully!");
    
    println!("Press Enter to exit...");
    
    Ok(())
}
