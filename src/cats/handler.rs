use crate::cats;
use crate::mongo_connection::Conn;
use cats::Cat;
use mongodb::{doc, error::Error, oid::ObjectId};
use rocket::http::Status;
use rocket_contrib::json::Json;

fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all(connection: Conn) -> Result<Json<Vec<Cat>>, Status> {
    match cats::repository::all(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[get("/<id>")]
pub fn get(id: String, connection: Conn) -> Result<Json<Cat>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match cats::repository::get(res, &connection) {
            Ok(res) => Ok(Json(res.unwrap())),
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[post("/", format = "application/json", data = "<cats>")]
pub fn post(cats: Json<Cat>, connection: Conn) -> Result<Json<ObjectId>, Status> {
    match cats::repository::insert(cats.into_inner(), &connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[put("/<id>", format = "application/json", data = "<cats>")]
pub fn put(id: String, cats: Json<Cat>, connection: Conn) -> Result<Json<Cat>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match cats::repository::update(res, cats.into_inner(), &connection) {
            Ok(res) => Ok(Json(res)),
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[delete("/<id>")]
pub fn delete(id: String, connection: Conn) -> Result<Json<String>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match cats::repository::delete(res, &connection) {
            Ok(_) => Ok(Json(id)),
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[delete("/")]
pub fn delete_all(connection: Conn) -> Result<Json<bool>, Status> {
    match cats::repository::delete_all(&connection) {
        Ok(_) => Ok(Json(true)),
        Err(err) => Err(error_status(err)),
    }
}
