use std::error::Error;
use newsapi::get_articles;
use newsapi::render_articles;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://api.hackerwebapp.com/news";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
} 
