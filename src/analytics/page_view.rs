use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::{Bytes, Uuid};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::page_view)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PageView {
    pub id: i32,
    pub page_title: String,
    pub page_location: String,
    pub page_path: String,
    pub page_referrer: Option<String>,
    pub user_agent: String,
    pub page_encoding: String,
    pub engagement_time_msec: i64,
    #[serde(with = "uuid::serde::compact")]
    pub anonymous_id: Uuid,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::create_page_view)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreatePageView {
    pub id: i32,
    #[serde(with = "uuid::serde::compact")]
    pub idempotent_id: Uuid,
    pub page_title: String,
    pub page_location: String,
    pub page_path: String,
    pub page_referrer: Option<String>,
    pub user_agent: String,
    pub page_encoding: String,
    pub engagement_time_msec: i64,
    #[serde(with = "uuid::serde::compact")]
    pub anonymous_id: Uuid,
}

impl From<CreatePageView> for PageView {
    fn from(obj: CreatePageView) -> Self {
        PageView {
            page_title: obj.page_title.to_owned(),
            page_location: obj.page_location.to_owned(),
            page_path: obj.page_path.to_owned(),
            page_referrer: obj.page_referrer,
            user_agent: obj.user_agent.to_owned(),
            page_encoding: obj.page_encoding.to_owned(),
            engagement_time_msec: obj.engagement_time_msec.to_owned(),
            anonymous_id: obj.anonymous_id,
            ..Default::default()
        }
    }
}
impl PageView {
    pub fn new() -> PageView {
        PageView::default()
    }
    fn init(
        page_title: &str, page_location: &str, page_path: &str, page_referrer: &str,
        user_agent: &str, page_encoding: &str,
    ) -> PageView {
        let mut page: PageView = PageView::new();
        page.set_page_title(page_title)
            .set_page_location(page_location)
            .set_page_path(page_path)
            .set_page_referrer(page_referrer)
            .set_user_agent(user_agent)
            .set_page_encoding(page_encoding)
            .clone()
    }

    pub fn set_page_title(&mut self, page_title: &str) -> &mut Self {
        self.page_title.clear();
        self.page_title.push_str(page_title);
        self
    }
    pub fn set_page_location(&mut self, page_location: &str) -> &mut Self {
        self.page_location.clear();
        self.page_location.push_str(page_location);
        self
    }
    pub fn set_page_path(&mut self, page_path: &str) -> &mut Self {
        self.page_path.clear();
        self.page_path.push_str(page_path);
        self
    }
    pub fn set_page_referrer(&mut self, page_referrer: &str) -> &mut Self {
        self.page_referrer = Some(page_referrer.to_owned());
        self
    }
    pub fn set_user_agent(&mut self, user_agent: &str) -> &mut Self {
        self.user_agent.clear();
        self.user_agent.push_str(user_agent);
        self
    }
    pub fn set_page_encoding(&mut self, page_encoding: &str) -> &mut Self {
        self.page_encoding.clear();
        self.page_encoding.push_str(page_encoding);
        self
    }
}

// "en_us",

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn private_init_test_helper() {
        let result: PageView = PageView::init(
            "Home",
            "https://example.com",
            "",
            "self",
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0",
            "",
        );
        assert_eq!(result.page_title, String::from("Home"));
    }
}
