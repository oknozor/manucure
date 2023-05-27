use manucure_db::tag::Tag as DbTAg;
use manucure_index::model::IndexTag;
use manucure_index::IntoIndexable;

#[derive(Debug)]
pub struct Tag {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
}

impl From<DbTAg> for Tag {
    fn from(tag: DbTAg) -> Self {
        Self {
            id: tag.id,
            user_id: tag.user_id,
            name: tag.name,
        }
    }
}

impl From<Tag> for IndexTag {
    fn from(tag: Tag) -> Self {
        IndexTag {
            id: tag.id,
            user_id: tag.user_id,
            name: tag.name,
        }
    }
}

impl IntoIndexable for Tag {
    type Output = IndexTag;
}
