use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct AddUserDocuments {
//     pub mail: String,
//     pub token: String,
//     pub enrollment: bool,
// }

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