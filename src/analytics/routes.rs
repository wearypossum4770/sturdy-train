use crate::analytics::page_view::{CreatePageView, PageView};
use crate::config::postgres::DbPool;
use crate::core::response::GenericResponse;
use crate::schema::create_page_view::dsl::*;
use crate::schema::page_view::dsl::*;
use actix_web::{get, post, web, Responder};
use diesel::prelude::*;
// use crate::user::person::User;

pub mod page_routes {
    use super::*;
    #[post("")]
    pub async fn create_new_page_view(
        pool: web::Data<DbPool>, data: web::Json<CreatePageView>,
    ) -> impl Responder {
        let resp: GenericResponse = GenericResponse::successful();
        let inserted_rows = web::block(move || {
            let payload = data.into_inner();
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            diesel::insert_into(create_page_view)
                .values(&payload)
                .execute(&mut conn)
                .expect("Error inserting page_view");
        })
        .await
        .unwrap();
        web::Json((resp, inserted_rows))
    }
    #[get("")]
    pub async fn new_page_view(
        pool: web::Data<DbPool>, data: web::Json<PageView>,
    ) -> impl Responder {
        let resp: GenericResponse = GenericResponse::successful();
        let inserted_rows = web::block(move || {
            let payload = data.into_inner();
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            diesel::insert_into(page_view)
                .values(&payload)
                .get_result::<PageView>(&mut conn)
                .expect("Error inserting page_view");
        })
        .await
        .unwrap();
        web::Json((resp, inserted_rows))
    }
}
