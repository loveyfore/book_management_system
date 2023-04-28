use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Deserialize, serde::Serialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key)]
    // #[serde(skip_deserializing)]
    #[serde(default)]
    pub id: i64,
    pub title: String,
    pub author: String,
    pub publication_year: i32,
}

impl Model {
    pub fn new(title: String, author: String, publication_year: i32) -> Self {
        Self {
            id: 0,
            title,
            author,
            publication_year,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(has_many = "super::Book")]
    // User,
}

// impl Related<super::User> for Book {
//     fn to() -> RelationDef {
//         Relation::User.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}