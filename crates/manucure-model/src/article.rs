use chrono::{DateTime, Utc};
use manucure_db::article::Article as DbArticle;
use manucure_index::model::IndexArticle;
use manucure_index::IntoIndexable;

#[derive(Debug)]
pub struct Article {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
    pub title: String,
    pub text: String,
    pub starred: bool,
    pub archived: bool,
    pub created: DateTime<Utc>,
    pub content: String,
}

impl From<DbArticle> for Article {
    fn from(article: DbArticle) -> Self {
        Self {
            id: article.id,
            user_id: article.user_id,
            url: article.url,
            title: article.title,
            text: article.text,
            starred: article.starred,
            archived: article.archived,
            created: article.created,
            content: article.content,
        }
    }
}

impl From<Article> for IndexArticle {
    fn from(article: Article) -> Self {
        IndexArticle {
            id: article.id,
            user_id: article.user_id,
            title: article.title,
            text: article.text,
        }
    }
}

impl IntoIndexable for Article {
    type Output = IndexArticle;
}
