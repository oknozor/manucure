use meilisearch_sdk::Client;
use serde::Serialize;
use std::fmt::Error;
use tracing::{debug, error, info};
pub mod model;

#[derive(Debug, Clone)]
pub struct MeiliClient {
    client: Client,
}

impl MeiliClient {
    pub fn new(url: String, key: &str) -> Self {
        MeiliClient {
            client: Client::new(url, Some(key)),
        }
    }
}

pub trait IntoIndexable {
    type Output: Indexable;

    fn into_indexable(self) -> Self::Output
    where
        Self: Into<Self::Output>,
    {
        self.into()
    }
}

pub trait Indexable: Serialize {
    fn index(&self) -> String;
    fn id(&self) -> i64;
    fn user_id(&self) -> i64;
}

impl MeiliClient {
    pub async fn index_many<T: IntoIndexable>(&self, items: Vec<T>)
    where
        <T as IntoIndexable>::Output: From<T>,
    {
        let items: Vec<_> = items
            .into_iter()
            .map(IntoIndexable::into_indexable)
            .collect();
        let index = &items[0].index();

        match self
            .client
            .index(index)
            .add_documents(items.as_slice(), Some("id"))
            .await
        {
            Ok(task) => debug!("Items indexed: {task:?}"),
            Err(err) => error!("Indexation failed for items: {err}"),
        }
    }

    pub async fn index_one<T: IntoIndexable>(&self, item: T)
    where
        <T as IntoIndexable>::Output: From<T>,
    {
        let item = item.into_indexable();
        let id = item.id();
        let index = item.index();

        match self
            .client
            .index(index)
            .add_documents(&[item], Some("id"))
            .await
        {
            Ok(task) => debug!("Item indexed: {task:?}"),
            Err(err) => error!("Indexation failed for item {id}: {err}"),
        }
    }

    pub async fn delete<T: IntoIndexable>(&self, item: T)
    where
        <T as IntoIndexable>::Output: From<T>,
    {
        let item = item.into_indexable();
        let id = item.id();
        let index = item.index();
        match self.client.index(index).delete_document(id).await {
            Ok(task) => debug!("Document deleted: {task:?}"),
            Err(err) => error!("Failed to delete document {id}: {err}"),
        };
    }

    pub async fn delete_user_indexes(&self, user_id: i64) -> Result<(), Error> {
        info!("cleaning indexes for user {}", user_id);
        let article_index = &format!("articles-{}", user_id);
        match self.client.delete_index(article_index).await {
            Ok(task) => debug!("Index {article_index} deleted: {task:?}"),
            Err(err) => error!("Failed to delete index {article_index}, err: {err}"),
        }

        let tag_index = &format!("tags-{}", user_id);
        match self.client.delete_index(tag_index).await {
            Ok(task) => debug!("Index {tag_index} deleted: {task:?}"),
            Err(err) => error!("Failed to delete index {tag_index}, err: {err}"),
        }

        Ok(())
    }
}
