use actix_multipart::Multipart;
use actix_web::web::Json;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use dotenv::dotenv;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::env;
use std::io::Write;

use crate::view_user;

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
    pub token: String,
    pub mail: String,
    pub enrollment: bool,
    pub course_name: String,
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
            role: user.role,
        }
    }
}

#[derive(Deserialize)]
pub struct UserAuthentication {
    pub token: String,
}

#[derive(Serialize)]
pub struct CoursesListResponse {
    pub enrolled_courses: Vec<String>,
    pub not_enrolled_courses: Vec<String>,
}

#[derive(Serialize)]
pub struct CourseTopic {
    pub namecourses: String,
    pub id: i32,
    pub nametheme: String,
}

#[derive(Serialize, Deserialize)]
pub struct NameCourses {
    pub namecourses: String,

}