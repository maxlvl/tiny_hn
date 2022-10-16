use newsapi::get_articles;
use newsapi::render_articles;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://api.hackerwebapp.com/news";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
