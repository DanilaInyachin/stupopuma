use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use actix_multipart::Multipart;

use sqlx::PgPool;
use dotenv::dotenv;
use std::env;
use sha2::{Sha256, Digest};
use std::io::Write;

use crate::view_users;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub mail: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    pub mail: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub mail: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct СhangeUserData {
    pub surname: String,
    pub firstname: String,
    pub lastname: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct СhangeUserRole {
    pub mail: String,
    pub token: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCourses {
    pub nameCourses: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CangeUserEnrollment {
    pub mail: String,
    pub token: String,
    pub enrollment: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterUserCourses {
    pub token: String,
    pub nameCourses: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AddPrepodCourses {
    pub nametheme: Vec<String>,
    pub token: String,
    pub nameCourses: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangePrepodCourses {
    pub nametheme: String,
    pub token: String,
    pub oldnametheme: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUser {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserAdmin {
    pub token: String,
    pub mail: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UploadDocument {
    pub token: String,
    pub file_name: String, 
 
}

pub struct AppState {
    pub db_pool: PgPool,
}

#[derive(serde::Deserialize)]
struct LoginForm {
    mail: String,
    password: String,
}

#[derive(sqlx::FromRow)]
pub struct ViewUser {
    pub role: String,
    pub surname: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(serde::Serialize)]
pub struct ResponseUser {
    pub lastname: String,
    pub firstname: String,
    pub surname: String,
    pub role: String,
}

impl ResponseUser {
    fn from(user: ViewUser) -> Self {
        Self {
            lastname: user.lastname,
            firstname: user.firstname,
            surname: user.surname,
            role: user.role
        }
    }
}

#[derive(Deserialize)]
pub struct ChangeUserData {
    pub token: String,
}