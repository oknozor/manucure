use manucure_db::article::AsArticle;
use readability::extractor::{scrape, ReadableHtmlPage};

pub struct HtmlArticle {
    pub title: String,
    pub text: String,
    pub content: String,
}

impl HtmlArticle {
    pub async fn scrape(url: &str) -> Result<HtmlArticle, readability::error::Error> {
        scrape(url).await.map(HtmlArticle::from)
    }
}

impl From<ReadableHtmlPage> for HtmlArticle {
    fn from(value: ReadableHtmlPage) -> Self {
        Self {
            title: value.title,
            text: value.text,
            content: value.content,
        }
    }
}

impl AsArticle for HtmlArticle {
    fn title(&self) -> &str {
        &self.title
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn content(&self) -> &str {
        &self.content
    }
}
