use actix_web::{web,get, post, Responder};

use crate::analytics::page_view::{CreatePageView, PageView};
use crate::config::postgres::DbPool;
use crate::core::response::GenericResponse;
use crate::schema::page_view::dsl::*;
use crate::schema::create_page_view::dsl::*;
// use crate::user::person::User;

pub mod page_routes {
    use super::*;
    #[post("/")]
    pub async fn create_new_page_view(
        pool: web::Data<DbPool>, data: web::Json<CreatePageView>,
    ) -> impl Responder {
        let resp: GenericResponse = GenericResponse::successful();
        web::block(move || {
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            diesel::insert_into(create_page_view).values(&data).get_result::<CreatePageView>(&mut conn).unwrap()
        });
        web::Json(resp)
    }
    #[get("")]
    pub async fn new_page_view(
        pool: web::Data<DbPool>, data: web::Json<PageView>,
    ) -> impl Responder {
        let resp: GenericResponse = GenericResponse::successful();
        web::block(move || {
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            diesel::insert_into(page_view).values(&data).get_result::<PageView>(&mut conn).unwrap()
        });
        web::Json(resp)
    }
}
