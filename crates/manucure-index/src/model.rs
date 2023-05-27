use crate::Indexable;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct IndexArticle {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub text: String,
}

impl Indexable for IndexArticle {
    fn index(&self) -> String {
        format!("articles-{}", self.user_id)
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[derive(Debug, Serialize)]
pub struct IndexTag {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
}

impl Indexable for IndexTag {
    fn index(&self) -> String {
        format!("tags-{}", self.user_id)
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn user_id(&self) -> i64 {
        self.user_id
    }
}
