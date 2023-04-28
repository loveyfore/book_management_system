use crate::daos::book_dao::BookDao;
use crate::entities::book::Model as Book;
use sea_orm::*;

pub struct BookService;

impl BookService {
    pub async fn find_all(db: &DbConn) -> Result<Vec<Book>, DbErr> {
        BookDao::find_all(db).await
    }

    pub async fn create(db: &DbConn, book: &Book) -> Result<(), DbErr> {
        BookDao::create(db, book).await
    }

    pub async fn update(db: &DbConn, book: &Book) -> Result<(), DbErr> {
        BookDao::update(db, book).await
    }

    pub async fn delete(db: &DbConn, book_id: i64) -> Result<DeleteResult, DbErr> {
        BookDao::delete(db, book_id).await
    }
}