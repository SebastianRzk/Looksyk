use crate::io::http::page::search::dtos::SearchTermDto;
use crate::state::state::AppState;
use actix_web::web::Data;
use actix_web::{Result, post, web, Responder};
use crate::io::http::page::search::mapper::{search_result_to_dto, to_search_term};
use crate::looksyk::search;

#[post("/api/search")]
async fn search_in_files(body: web::Json<SearchTermDto>, data: Data<AppState>)-> Result<impl Responder> {
    let search_term = to_search_term(body.into_inner());

    let page_guard = data.a_user_pages.lock().unwrap();
    let journal_guard = data.b_journal_pages.lock().unwrap();

    let result = search::search(search_term, &journal_guard, &page_guard);

    drop(page_guard);
    drop(journal_guard);

    Ok(web::Json(search_result_to_dto(result)))
}