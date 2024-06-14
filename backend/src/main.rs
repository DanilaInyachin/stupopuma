use actix_web::{
    delete, dev::Response, get, post, put, web, App, HttpResponse, HttpServer, Responder,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;

mod models;
use models::{
    AddCourses, AddPrepodCourses, CangeUserEnrollment, ChangePrepodCourses, DeleteUser, LoginUser,
    RegisterUser, RegisterUserCourses, Token, User, СhangeUserData, СhangeUserRole, DeleteUserAdmin
};

#[post("/register")]
async fn register_user(
    user: web::Json<RegisterUser>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO users (mail, password) VALUES ($1, $2)",
        user.mail,
        hashed_password
    )
    .execute(&**db_pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

#[post("/login")]
async fn login_user(user: web::Json<LoginUser>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!(
        "SELECT mail, password FROM users WHERE mail = $1",
        user.mail
    )
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(record)) => {
            if verify(&user.password, &record.password).unwrap() {
                HttpResponse::Ok().json(Token {
                    token: user.mail.clone(),
                })
            } else {
                HttpResponse::Unauthorized().body("Invalid username or password")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid username or password"),
        Err(_) => HttpResponse::InternalServerError().body("Error logging in"),
    }
}

#[put("/change_user_data")]
async fn change_user_data(
    user: web::Json<СhangeUserData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    //println!("user: {:?}", user);

    let result = sqlx::query!(
        "UPDATE users SET (surname, firstname, lastname) = ($1, $2, $3) WHERE mail = $4;",
        user.surname,
        user.firstname,
        user.lastname,
        user.token,
    )
    .execute(&**db_pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User data update"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating data user"),
    }
}

#[put("/change_user_role")]
async fn change_user_role(
    user: web::Json<СhangeUserRole>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token,)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => match record.role {
            r if r == "Администратор".to_string() => {
                let response = sqlx::query!(
                    "UPDATE users SET role = $1 WHERE mail = $2;",
                    user.role,
                    user.mail,
                )
                .execute(&**db_pool)
                .await;

                match response {
                    Ok(_) => HttpResponse::Ok().body("User data update"),
                    Err(_) => HttpResponse::InternalServerError().body("Error updating data user"),
                }
            }
            _ => HttpResponse::Unauthorized().body("Invalid role"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error logging in"),
    }
}

#[post("/add_courses")]
async fn add_courses(user: web::Json<AddCourses>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token,)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => match record.role {
            r if r == "Администратор".to_string() => {
                let response = sqlx::query!(
                    "INSERT INTO courses (namecourses) VALUES ($1);",
                    user.nameCourses,
                )
                .execute(&**db_pool)
                .await;

                match response {
                    Ok(_) => HttpResponse::Ok().body("New course add"),
                    Err(_) => HttpResponse::InternalServerError().body("Error new course add"),
                }
            }
            _ => HttpResponse::Unauthorized().body("Invalid role"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error logging in"),
    }
}

#[get("/view_courses")]
async fn view_courses(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!("SELECT namecourses FROM courses;")
        .fetch_all(&**db_pool)
        .await;

    match result {
        Ok(courseslist) => {
            let courses: Vec<String> = courseslist
                .into_iter()
                .map(|course| course.namecourses)
                .collect();
            HttpResponse::Ok().json(courses)
        }
        Err(_) => HttpResponse::InternalServerError().body("Ошибка при получении данных курсов"),
    }
}

#[post("/register_user_courses")]
async fn register_user_courses(
    user: web::Json<RegisterUserCourses>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!("SELECT id, role FROM users WHERE mail = $1", user.token,)
        .fetch_one(&**db_pool)
        .await;

    let course_response = sqlx::query!(
        "SELECT id FROM courses WHERE namecourses = $1",
        user.nameCourses,
    )
    .fetch_one(&**db_pool)
    .await;

    match (result, course_response) {
        (Ok(user_data), Ok(course_data)) => {
            if user_data.role == "Ученик" {
                let insert_result = sqlx::query!(
                    "INSERT INTO listcourses (users_id, courses_id) VALUES ($1, $2);",
                    user_data.id,
                    course_data.id,
                )
                .execute(&**db_pool)
                .await;

                match insert_result {
                    Ok(_) => HttpResponse::Ok().body("New course added"),
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Ошибка при добавлении курса")
                    }
                }
            } else {
                HttpResponse::Forbidden().body("Пользователь не имеет прав для добавления курса")
            }
        }
        _ => HttpResponse::InternalServerError()
            .body("Ошибка при получении данных пользователя или курса"),
    }
}

#[put("/change_user_enrollment")]
async fn change_user_enrollment(
    user: web::Json<CangeUserEnrollment>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token,)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => match record.role {
            r if r == "Администратор".to_string() => {
                let response = sqlx::query!(
        
                    "UPDATE listcourses SET enrollment = $1 WHERE users_id = (SELECT id FROM users WHERE mail = $2);",
                    user.enrollment, user.mail
                )
                .execute(&**db_pool)
                .await;

                match response {
                    Ok(_) => HttpResponse::Ok().body("Add new enrollment"),
                    Err(_) => HttpResponse::InternalServerError().body("Error add new enrollment"),
                }
            }
            _ => HttpResponse::Unauthorized().body("Invalid role"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error logging in"),
    }
}

// #[post("/add_user_documents")]
// async fn add_user_documents(user: web::Json<AddUserDocuments>, db_pool: web::Data<PgPool>) -> impl Responder {

//     let result = sqlx::query!(
//         "SELECT id, role FROM users WHERE mail = $1",
//         user.token,
//     )
//     .fetch_one(&**db_pool)
//     .await;

//     let course_response = sqlx::query!(
//         "SELECT id FROM courses WHERE namecourses = $1",
//         user.nameCourses,
//     )
//     .fetch_one(&**db_pool)
//     .await;

//     match (result, course_response) {
//         (Ok(user_data), Ok(course_data)) => {
//             if user_data.role == "Ученик" {
//                 let insert_result = sqlx::query!(
//                     "INSERT INTO listcourses (users_id, courses_id) VALUES ($1, $2);",
//                     user_data.id,
//                     course_data.id,
//                 )
//                 .execute(&**db_pool)
//                 .await;

//                 match insert_result {
//                     Ok(_) => HttpResponse::Ok().body("New course added"),
//                     Err(_) => HttpResponse::InternalServerError().body("Ошибка при добавлении курса"),
//                 }
//             } else {
//                 HttpResponse::Forbidden().body("Пользователь не имеет прав для добавления курса")
//             }
//         },
//         _ => HttpResponse::InternalServerError().body("Ошибка при получении данных пользователя или курса"),
//     }
// }

#[put("/add_prepod_courses")]
async fn add_prepod_courses(
    user: web::Json<AddPrepodCourses>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token,)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => match record.role.as_str() {
            "Администратор" | "Преподаватель" => {
                let id_courses = match sqlx::query!(
                    "SELECT id FROM courses WHERE namecourses = $1",
                    user.nameCourses,
                )
                .fetch_one(&**db_pool)
                .await
                {
                    Ok(response) => response.id,
                    _ => {
                        return HttpResponse::InternalServerError().body("Error fetching course ID")
                    }
                };

                for new_theme in &user.nametheme {
                    let insert_result = sqlx::query!(
                        "INSERT INTO themes (nametheme, id_courses) VALUES ($1, $2)",
                        new_theme,
                        id_courses
                    )
                    .execute(&**db_pool)
                    .await;

                    if insert_result.is_err() {
                        return HttpResponse::InternalServerError().body("Error adding new theme");
                    }
                }

                HttpResponse::Ok().body("New themes added successfully")
            }
            _ => HttpResponse::Unauthorized().body("Invalid role"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error checking user role"),
    }
}

#[put("/change_prepod_courses")]
async fn change_prepod_courses(
    user: web::Json<ChangePrepodCourses>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token,)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => match record.role.as_str() {
            "Администратор" | "Преподаватель" => {
                let response = sqlx::query!(
                    "UPDATE themes SET nametheme = $1 WHERE nametheme = $2;",
                    user.nametheme,
                    user.oldnametheme
                )
                .execute(&**db_pool)
                .await;
                HttpResponse::Ok().body("Theme udated")
            }
            _ => HttpResponse::Unauthorized().body("Invalid role"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error checking user role"),
    }
}

#[delete("/delete_user")]
async fn delete_user(user: web::Json<DeleteUser>, db_pool: web::Data<PgPool>) -> impl Responder {
    let users_id = match sqlx::query!("SELECT id FROM users WHERE mail = $1", user.token,)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(response) => response.id,
        _ => return HttpResponse::Unauthorized().body("Invalid role"),
    };

    let delete_documents = sqlx::query!(
        "DELETE FROM public.documents WHERE users_id = $1;",
        users_id,
    )
    .execute(&**db_pool)
    .await;

    let delete_listcourses = sqlx::query!(
        "DELETE FROM public.listcourses WHERE users_id = $1;",
        users_id,
    )
    .execute(&**db_pool)
    .await;

    let delete_user = sqlx::query!("DELETE FROM public.users WHERE id = $1;", users_id,)
        .execute(&**db_pool)
        .await;

    if delete_documents.is_err() || delete_listcourses.is_err() || delete_user.is_err() {
        return HttpResponse::InternalServerError().body("Error deleting user or related data");
    }

    HttpResponse::Ok().body("User deleted")
}


#[delete("/delete_user_admin")]
async fn delete_user_admin(user: web::Json<DeleteUserAdmin>, db_pool: web::Data<PgPool>) -> impl Responder {
    let role = match sqlx::query!(
        "SELECT role FROM users WHERE mail = $1",
        user.token,
    )
    .fetch_one(&**db_pool)
    .await {
        Ok(response) => response.role,
        _ => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    if role != "Администратор" {
        return HttpResponse::Unauthorized().body("Only administrators can delete users");
    }

    let users_id = match sqlx::query!(
        "SELECT id FROM users WHERE mail = $1",
        user.mail,
    )
    .fetch_one(&**db_pool)
    .await {
        Ok(response) => response.id,
        _ => return HttpResponse::NotFound().body("User not found"),
    };

    let delete_documents = sqlx::query!(
        "DELETE FROM public.documents WHERE users_id = $1;",
        users_id,
    )
    .fetch_one(&**db_pool)
    .await;

    let delete_listcourses = sqlx::query!(
        "DELETE FROM public.listcourses WHERE users_id = $1;",
        users_id,
    )
    .fetch_one(&**db_pool)
    .await;

    let delete_user_data = sqlx::query!(
        "DELETE FROM public.users WHERE id = $1;",
        users_id,
    )
    .fetch_one(&**db_pool)
    .await;

    

    HttpResponse::Ok().body("User deleted")
}


pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(register_user)
            .service(login_user)
            .service(change_user_data)
            .service(change_user_role)
            .service(add_courses)
            .service(view_courses)
            .service(register_user_courses)
            .service(change_user_enrollment)
            .service(add_prepod_courses)
            .service(change_prepod_courses)
            .service(delete_user)
            .service(delete_user_admin)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
