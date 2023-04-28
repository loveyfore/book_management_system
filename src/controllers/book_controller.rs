use crate::services::book_service::BookService;
use crate::entities::book::Model as Book;
use askama::Template;
use salvo::hyper::header::CONTENT_TYPE;
use salvo::hyper::HeaderMap;
use salvo::prelude::*;
use sea_orm::{Database, DatabaseConnection};

type Result<T> = std::result::Result<T, StatusError>;

#[derive(Debug, Clone)]
pub struct AppState {
    // templates: tera::Tera,
    pub conn: DatabaseConnection,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    books: &'a Vec<Book>,
}

#[handler]
pub async fn all_books(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let conn = &state.conn;

    let books = BookService::find_all(conn).await.unwrap();
    let template = IndexTemplate { books: &books };
    let body = template.render().unwrap();
    res.render(Text::Html(&body));
    Ok(())
}

#[handler]
pub async fn add_book(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let conn = &state.conn;

    let book: Book = req
        .parse_form::<Book>()
        .await
        .map_err(|err| { eprintln!("{:?}", err); StatusError::bad_request()})?;

    if let Err(err) = BookService::create(conn, &book).await {
        res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(&format!("Failed to add book: {:?}", err));
        return Ok(());
    }

    res.render(Redirect::found("/"));
    Ok(())
}

#[handler]
pub async fn update_book(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let conn = &state.conn;

    let book: Book = req
        .parse_form::<Book>()
        .await
        .map_err(|_| StatusError::bad_request())?;

    if let Err(err) = BookService::update(conn, &book).await {
        res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(&format!("Failed to update book: {:?}", err));
        return Ok(());
    }

    res.render(Redirect::found("/"));
    Ok(())
}

#[handler]
pub async fn delete_book(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let conn = &state.conn;

    let book_id = req.param::<i64>("id").unwrap_or_default();

    BookService::delete(conn, book_id).await
        .map_err(|_| StatusError::internal_server_error())?;

    res.render(Redirect::found("/"));
    Ok(())
}