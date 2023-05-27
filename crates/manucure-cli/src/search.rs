use manucure_db::PgPool;
use manucure_index::MeiliClient;
use manucure_model::article::Article;
use manucure_model::tag::Tag;
use tracing::info;

pub async fn reindex_all(client: &MeiliClient, db: PgPool) -> anyhow::Result<()> {
    delete_indexes(client, &db).await?;
    reindex_articles(client, &db).await?;
    reindex_tags(client, &db).await?;
    Ok(())
}

async fn delete_indexes(client: &MeiliClient, db: &PgPool) -> anyhow::Result<()> {
    let users = manucure_db::user::all(db).await?;
    for user in users {
        client.delete_user_indexes(user.id).await?;
    }

    Ok(())
}

async fn reindex_articles(client: &MeiliClient, db: &PgPool) -> anyhow::Result<()> {
    info!("enqueuing indexation tasks for articles");
    let articles = manucure_db::article::all(db)
        .await?
        .into_iter()
        .map(Article::from)
        .collect();

    client.index_many::<Article>(articles).await;

    Ok(())
}

async fn reindex_tags(client: &MeiliClient, db: &PgPool) -> anyhow::Result<()> {
    info!("enqueuing indexation tasks for tags");
    let tags = manucure_db::tag::all(db)
        .await?
        .into_iter()
        .map(Tag::from)
        .collect();

    client.index_many::<Tag>(tags).await;

    Ok(())
}
