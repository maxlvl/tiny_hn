use std::error::Error;
use colour:: { dark_green, yellow, blue,red };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Article {
    id: u32,
    title: String,
    time_ago: String,
    comments_count: u16,
    url: String, 
}

pub fn get_articles(url: &str) -> Result<Vec<Article>, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Vec<Article> = serde_json::from_str(&response).unwrap();
    Ok(articles)
}

pub fn render_articles(articles: &Vec<Article>) {
    for a in articles {
        dark_green!("Title:          {}\n", a.title);
        yellow!("URL:            {}\n", a.url);
        red!("Comment Count:  {}\n", a.comments_count);
        blue!("Time Posted:    {}\n\n", a.time_ago);
    }
}
