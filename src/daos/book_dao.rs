use crate::entities::book::{Model as Book, ActiveModel, Entity as BookEntity};
use sea_orm::{ActiveModelTrait, EntityTrait, Database, DbConn, DbErr, DeleteResult};
use sea_orm::ActiveValue::Set;

pub struct BookDao;

impl BookDao {
    pub async fn find_all(db: &DbConn) -> Result<Vec<Book>, DbErr> {
        BookEntity::find().all(db).await
    }

    pub async fn create(db: &DbConn, book: &Book) -> Result<(), DbErr> {
        // post::ActiveModel {
        //     title: Set(form_data.title.to_owned()),
        //     text: Set(form_data.text.to_owned()),
        //     ..Default::default()
        // }
        //     .save(db)
        //     .await
        //
        // let active_model: ActiveModel = (*book).clone().into();
        let active_model: ActiveModel = book.clone().into();
        active_model.insert(db).await.map(|_| ())
    }

    pub async fn update(db: &DbConn, book: &Book) -> Result<(), DbErr> {
        let active_model: ActiveModel = ActiveModel {
            id: Set(book.id),
            title: Set(book.title.to_owned()),
            author: Set(book.author.to_owned()),
            publication_year: Set(book.publication_year),
        };
        active_model.update(db).await.map(|_| ())
    }

    pub async fn delete(db: &DbConn, book_id: i64) -> Result<DeleteResult, DbErr> {
        let book: ActiveModel = BookEntity::find_by_id(book_id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        book.delete(db).await
    }
}