use chrono::{DateTime, NaiveDate, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;
mod schema;

use self::schema::books;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i64,
    pub uuid: Uuid,
    pub title: String,
    pub author: String,
    pub translator: Option<String>,
    pub publisher: Option<String>,
    pub coverimage: Option<String>,
    pub purchasedate: Option<NaiveDate>,
    pub purchaselocation: Option<String>,
    pub isbn: Option<String>,
    pub createdat: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub translator: Option<&'a str>,
    pub publisher: Option<&'a str>,
    pub coverimage: Option<&'a str>,
    pub purchasedate: Option<NaiveDate>,
    pub purchaselocation: Option<&'a str>,
    pub isbn: Option<&'a str>,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tauri::command]
fn get_books() -> Result<Vec<Book>, String> {
    use self::schema::books::dsl::*;

    let connection = &mut establish_connection();
    books
        .order(createdat.desc())
        .load::<Book>(connection)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_book(
    title: String,
    author: String,
    translator: Option<String>,
    publisher: Option<String>,
    coverimage: Option<String>,
    purchasedate: Option<String>,
    purchaselocation: Option<String>,
    isbn: Option<String>,
) -> Result<(), String> {
    let purchase_date =
        purchasedate.and_then(|date| NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok());

    let new_book = NewBook {
        title: &title,
        author: &author,
        translator: translator.as_deref(),
        publisher: publisher.as_deref(),
        coverimage: coverimage.as_deref(),
        purchasedate: purchase_date,
        purchaselocation: purchaselocation.as_deref(),
        isbn: isbn.as_deref(),
    };

    let connection = &mut establish_connection();
    diesel::insert_into(books::table)
        .values(&new_book)
        .execute(connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_book(bookid: i64) -> Result<Book, String> {
    use self::schema::books::dsl::*;

    let connection = &mut establish_connection();
    books
        .find(bookid)
        .first::<Book>(connection)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_book(
    id: i64,
    title: String,
    author: String,
    translator: Option<String>,
    publisher: Option<String>,
    coverimage: Option<String>,
    purchasedate: Option<String>,
    purchaselocation: Option<String>,
    isbn: Option<String>,
) -> Result<(), String> {
    

    let connection = &mut establish_connection();

    let purchase_date =
        purchasedate.and_then(|date| NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok());

    diesel::update(books::table.find(id))
        .set((
            books::title.eq(&title),
            books::author.eq(&author),
            books::translator.eq(translator.as_deref()),
            books::publisher.eq(publisher.as_deref()),
            books::coverimage.eq(coverimage.as_deref()),
            books::purchasedate.eq(purchase_date),
            books::purchaselocation.eq(purchaselocation.as_deref()),
            books::isbn.eq(isbn.as_deref()),
        ))
        .execute(connection)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn delete_book(id: i64) -> Result<(), String> {
    let connection = &mut establish_connection();

    diesel::delete(books::table.find(id))
        .execute(connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
        get_books,
        get_book,
        add_book,
        update_book,
        delete_book
    ])
    .run(tauri::generate_context!())
    .expect("Error while running tauri application");
}
