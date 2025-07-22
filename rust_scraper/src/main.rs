use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use csv::Writer;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
   let url = "https://blog.rust-lang.org/";

   let response = reqwest::get(url).await?;

   let body = response.text().await?;

   let document =  Html::parse_document(&body);

   let selector = Selector::parse("a").unwrap();

   let mut wtr = Writer::from_path("articles.csv")?;

   wtr.write_record(&["Title", "URL"])?;

   for element in document.select(&selector)
   {

        let title = element.text().collect::<Vec<_>>().join(" ");
        let link = element.value().attr("href").unwrap_or(" ");

        println!("Title {}, Link {}", title, link);

        wtr.write_record(&[title.trim(), link])?;
   }

   let _ = wtr.flush();

   println!("Scrapping is Done");


   Ok(())
}
