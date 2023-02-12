use crate::views::PageQuery;

pub mod article;
pub mod user;

const ARTICLES_PER_PAGE: i64 = 12;

pub struct Page(i64);

impl From<PageQuery> for Page {
    fn from(value: PageQuery) -> Self {
        Page(value.page)
    }
}

impl Page {
    fn to_limit_offset(&self) -> (i64, i64) {
        let offset = (self.0 - 1) * ARTICLES_PER_PAGE;
        (ARTICLES_PER_PAGE, offset)
    }

    fn nth(&self) -> i64 {
        self.0
    }

    fn get_pagination(&self, total_items: Option<i64>) -> Vec<i64> {
        let total = Self::total(total_items);
        let pages: Vec<i64> = (1..total).collect();
        if pages.is_empty() {
            vec![1]
        } else {
            pages
        }
    }

    fn total(item_count: Option<i64>) -> i64 {
        if let Some(count) = item_count {
            let has_offset = (count % ARTICLES_PER_PAGE) > 0;
            let mut total_pages = count / ARTICLES_PER_PAGE;
            if has_offset {
                total_pages += 1;
            }

            total_pages
        } else {
            1
        }
    }
}
