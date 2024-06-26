// программа управления контингентом обучающихся

use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, App, Error, HttpResponse, HttpServer, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use uuid::Uuid;

mod models;
use models::{
    AddCourses, AddPrepodCourses, CangeUserEnrollment, ChangePrepodCourses, CourseTopic,
    CoursesListResponse, DeleteUser, DeleteUserAdmin, EditCourse, EnrolledCoursesList, LoginUser,
    NameCourses, RegisterUser, RegisterUserCourses, ResponseUser, Token, UnenrolledCourse,
    UserAuthentication, UserToken, СhangeUserData, СhangeUserRole,
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
        user.lastname,
        user.firstname,
        user.surname,
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
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token)
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
        Ok(record) => match record.role.as_str() {
            "Администратор" | "Преподаватель" => {
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

#[post("/edit_course")]
async fn edit_course(
    course_data: web::Json<EditCourse>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", course_data.token)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => match record.role.as_str() {
            "Администратор" | "Преподаватель" => {
                let response = sqlx::query!(
                    "UPDATE courses SET namecourses = $1 WHERE namecourses = $2",
                    course_data.new_name_courses,
                    course_data.name_courses,
                )
                .execute(&**db_pool)
                .await;

                match response {
                    Ok(_) => HttpResponse::Ok().body("Course name updated"),
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Failed to update course name")
                    }
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
            // Проверяем, что пользователь имеет право записаться на курс
            if user_data.role == "Ученик" || user_data.role == "Преподаватель" {
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
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => match record.role.as_str() {
            "Администратор" => {
                let response = sqlx::query!(
                    "UPDATE listcourses
                     SET enrollment = $1
                     WHERE users_id = (SELECT id FROM users WHERE mail = $2)
                     AND courses_id = (SELECT id FROM courses WHERE namecourses = $3)",
                    user.enrollment,
                    user.mail,
                    user.course_name
                )
                .execute(&**db_pool)
                .await;

                match response {
                    Ok(_) => HttpResponse::Ok().body("Enrollment status updated"),
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Error updating enrollment status")
                    }
                }
            }
            _ => HttpResponse::Unauthorized().body("Invalid role"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error logging in"),
    }
}

#[put("/add_prepod_courses")] //добавление новых тем в курс
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

                let insert_result = sqlx::query!(
                    "INSERT INTO themes (nametheme, id_courses) VALUES ($1, $2)",
                    user.nametheme,
                    id_courses
                )
                .execute(&**db_pool)
                .await;

                if insert_result.is_err() {
                    return HttpResponse::InternalServerError().body("Error adding new theme");
                }

                HttpResponse::Ok().body("New themes added successfully")
            }
            _ => HttpResponse::Unauthorized().body("Invalid role"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error checking user role"),
    }
}

#[put("/change_prepod_courses")] //изменение тем в курсе
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

#[post("/delete_user")]
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
async fn delete_user_admin(
    user: web::Json<DeleteUserAdmin>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let role = match sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token,)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(response) => response.role,
        _ => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    if role != "Администратор" {
        return HttpResponse::Unauthorized().body("Only administrators can delete users");
    }

    let users_id = match sqlx::query!("SELECT id FROM users WHERE mail = $1", user.mail,)
        .fetch_one(&**db_pool)
        .await
    {
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

    let delete_user_data = sqlx::query!("DELETE FROM public.users WHERE id = $1;", users_id,)
        .fetch_one(&**db_pool)
        .await;

    HttpResponse::Ok().body("User deleted")
}

#[post("/upload_documents")]
async fn upload_documents(
    mut payload: Multipart,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    // Переменная для хранения имени загружаемого файла
    let mut file_name = String::new();

    // Обрабатываем каждый частичный поток данных (файлы)
    while let Some(mut field) = payload.try_next().await? {
        // Генерируем уникальное имя файла на основе UUID и сохраняем расширение
        let content_type = field.content_type().type_().as_str();
        let file_extension = match content_type {
            "image" => ".jpg",
            "text" => ".txt",
            "application" => ".bin",
            _ => ".dat", // по умолчанию
        };

        let new_file_name = Uuid::new_v4().to_string() + file_extension;
        file_name = new_file_name.clone();

        // Создаем путь для сохранения файла
        let file_path = format!("./uploads/{}", &new_file_name);

        // Создаем файл и записываем в него данные
        let mut f = web::block(move || fs::File::create(&file_path)).await??;

        // Пишем данные в файл
        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    // Сохраняем имя файла в базе данных
    let _ = sqlx::query!(
        "INSERT INTO documents (namedocuments) VALUES ($1)",
        &file_name
    )
    .execute(&**db_pool)
    .await;

    Ok(HttpResponse::Ok().body("mission complete"))
}

#[post("/view_user")]
async fn view_user(
    db_pool: web::Data<PgPool>,
    user: web::Json<UserAuthentication>,
) -> impl Responder {
    // Первый запрос для получения ID пользователя по токену
    let user_id_result = sqlx::query!("SELECT id FROM users WHERE mail = $1", user.token)
        .fetch_one(&**db_pool)
        .await;

    match user_id_result {
        Ok(user_id) => {
            // Второй запрос для получения данных пользователя по ID
            let user_data_result = sqlx::query!(
                "SELECT role, surname, firstname, lastname FROM users WHERE id = $1",
                user_id.id
            )
            .fetch_one(&**db_pool)
            .await;

            match user_data_result {
                Ok(userinfo) => HttpResponse::Ok().json(ResponseUser {
                    role: userinfo.role,
                    lastname: userinfo.surname.unwrap_or_else(|| "".to_string()),
                    firstname: userinfo.firstname.unwrap_or_else(|| "".to_string()),
                    surname: userinfo.lastname.unwrap_or_else(|| "".to_string()),
                }),
                Err(_) => HttpResponse::InternalServerError()
                    .body("Ошибка при получении данных пользователя"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Ошибка при получении ID пользователя"),
    }
}

#[post("/get_user_courses_list")]
async fn get_user_courses_list(
    db_pool: web::Data<PgPool>,
    user: web::Json<UserAuthentication>,
) -> impl Responder {
    // Первый запрос для получения ID пользователя по токену
    let user_id_result = sqlx::query!("SELECT id FROM users WHERE mail = $1", user.token)
        .fetch_one(&**db_pool)
        .await;

    match user_id_result {
        Ok(user_id) => {
            // Второй запрос для получения списка курсов пользователя
            let user_courses_list = sqlx::query!(
                "SELECT courses.namecourses, listcourses.enrollment FROM courses JOIN listcourses ON courses.id = listcourses.courses_id WHERE listcourses.users_id = $1",
                user_id.id
            )
            .fetch_all(&**db_pool)
            .await;

            match user_courses_list {
                Ok(userinfo) => {
                    let (enrolled_courses, not_enrolled_courses): (Vec<_>, Vec<_>) =
                        userinfo.into_iter().partition(|course| course.enrollment);

                    let enrolled_courses: Vec<String> = enrolled_courses
                        .into_iter()
                        .map(|course| course.namecourses)
                        .collect();

                    let not_enrolled_courses: Vec<String> = not_enrolled_courses
                        .into_iter()
                        .map(|course| course.namecourses)
                        .collect();

                    HttpResponse::Ok().json(CoursesListResponse {
                        enrolled_courses,
                        not_enrolled_courses,
                    })
                }
                Err(_) => HttpResponse::InternalServerError()
                    .body("Ошибка при получении списка курсов пользователя"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Ошибка при получении ID пользователя"),
    }
}

#[post("/get_topics_by_course")]
async fn get_topics_by_course(
    db_pool: web::Data<PgPool>,
    course_name: web::Json<NameCourses>,
) -> impl Responder {
    let topics_result = sqlx::query_as!(
        CourseTopic,
        r#"
        SELECT courses.namecourses AS namecourses, themes.id AS id, themes.nametheme AS nametheme
        FROM themes
        JOIN courses ON themes.id_courses = courses.id
        WHERE courses.namecourses = $1
        ORDER BY themes.id ASC
        "#,
        course_name.namecourses
    )
    .fetch_all(&**db_pool)
    .await;

    match topics_result {
        Ok(topics) => HttpResponse::Ok().json(topics),
        Err(e) => {
            eprintln!("Error fetching topics: {:?}", e);
            HttpResponse::InternalServerError().body("Ошибка при получении тем")
        }
    }
}

#[post("/unenrolled_courses")]
async fn unenrolled_courses(
    user: web::Json<UserAuthentication>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => {
            if record.role == "Администратор" {
                let unenrolled_courses_result = sqlx::query!(
                    r#"
                    SELECT users.mail, users.surname, users.firstname, users.lastname, courses.namecourses AS course_name
                    FROM listcourses
                    JOIN users ON listcourses.users_id = users.id
                    JOIN courses ON listcourses.courses_id = courses.id
                    WHERE listcourses.enrollment = FALSE
                    "#
                )
                .fetch_all(&**db_pool)
                .await;

                match unenrolled_courses_result {
                    Ok(rows) => {
                        let courses: Vec<UnenrolledCourse> = rows
                            .into_iter()
                            .map(|row| UnenrolledCourse {
                                mail: row.mail.unwrap_or_else(|| "".to_string()),

                                lastname: row.surname.unwrap_or_else(|| "".to_string()),
                                firstname: row.firstname.unwrap_or_else(|| "".to_string()),
                                surname: row.lastname.unwrap_or_else(|| "".to_string()),

                                course_name: row.course_name.unwrap_or_else(|| "".to_string()),
                            })
                            .collect();

                        HttpResponse::Ok().json(courses)
                    }
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Ошибка при получении данных")
                    }
                }
            } else {
                HttpResponse::Forbidden().body("Недостаточно прав для просмотра данных")
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Ошибка при получении роли пользователя")
        }
    }
}

#[post("/enrolled_courses_list")]
async fn enrolled_courses_list(
    user: web::Json<UserToken>,
    db_pool: web::Data<sqlx::PgPool>,
) -> impl actix_web::Responder {
    let result = sqlx::query!("SELECT role FROM users WHERE mail = $1", user.token)
        .fetch_one(&**db_pool)
        .await;

    match result {
        Ok(record) => match record.role.as_str() {
            "Преподаватель" => {
                let enrolled_courses = sqlx::query!(
                    r#"
                    WITH user_id_query AS (
                        SELECT id
                        FROM users
                        WHERE mail = $1
                    ), course_ids_query AS (
                        SELECT id_course
                        FROM prepod
                        WHERE id_prepod = (SELECT id FROM user_id_query)
                    )
                    SELECT c.namecourses AS course_name,
                           u.surname || ' ' || u.firstname || ' ' || u.lastname AS student
                    FROM course_ids_query pc
                    JOIN courses c ON pc.id_course = c.id
                    JOIN listcourses lc ON c.id = lc.courses_id AND lc.enrollment = TRUE
                    JOIN users u ON lc.users_id = u.id
                    WHERE u.role = 'Ученик'
                    ORDER BY c.namecourses;
                    "#,
                    user.token
                )
                .fetch_all(&**db_pool)
                .await;

                match enrolled_courses {
                    Ok(rows) => {
                        let mut courses: Vec<EnrolledCoursesList> = Vec::new();
                        let mut current_course_name = String::new();
                        let mut students: Vec<String> = Vec::new();

                        for row in rows {
                            let course_name = row.course_name.clone();
                            let student = row.student.unwrap_or_else(|| "".to_string()); // Обработка Option<String>

                            if course_name != current_course_name && !current_course_name.is_empty()
                            {
                                courses.push(EnrolledCoursesList {
                                    course_name: current_course_name.clone(),
                                    students: students.clone(),
                                });
                                students.clear();
                            }

                            current_course_name = course_name;
                            students.push(student);
                        }

                        // Добавление последнего курса
                        if !current_course_name.is_empty() {
                            courses.push(EnrolledCoursesList {
                                course_name: current_course_name,
                                students,
                            });
                        }

                        HttpResponse::Ok().json(courses)
                    }
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Ошибка при получении данных")
                    }
                }
            }
            "Администратор" => {
                let all_users = sqlx::query!(
                    r#"
                    SELECT 
                        u.surname || ' ' || u.firstname || ' ' || u.lastname AS student,
                        CASE 
                            WHEN u.role = 'Преподаватель' THEN 'Преподаватель'
                            ELSE c.namecourses
                        END AS course_name
                    FROM users u
                    LEFT JOIN listcourses lc ON u.id = lc.users_id AND lc.enrollment = TRUE
                    LEFT JOIN courses c ON lc.courses_id = c.id
                    ORDER BY course_name, student;
                    "#
                )
                .fetch_all(&**db_pool)
                .await;

                match all_users {
                    Ok(rows) => {
                        let mut courses: Vec<EnrolledCoursesList> = Vec::new();
                        let mut current_course_name: Option<String> = None;
                        let mut students: Vec<String> = Vec::new();

                        for row in rows {
                            let course_name = row
                                .course_name
                                .clone()
                                .unwrap_or_else(|| "Без курса".to_string());
                            let student = row.student.unwrap_or_else(|| "".to_string()); // Обработка Option<String>

                            if current_course_name.is_some()
                                && course_name != current_course_name.clone().unwrap_or_default()
                            {
                                courses.push(EnrolledCoursesList {
                                    course_name: current_course_name.clone().unwrap_or_default(),
                                    students: students.clone(),
                                });
                                students.clear();
                            }

                            current_course_name = Some(course_name);
                            students.push(student);
                        }

                        // Добавление последнего курса
                        if current_course_name.is_some() {
                            courses.push(EnrolledCoursesList {
                                course_name: current_course_name.unwrap_or_default(),
                                students,
                            });
                        }

                        HttpResponse::Ok().json(courses)
                    }
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Ошибка при получении данных")
                    }
                }
            }
            _ => HttpResponse::Unauthorized().body("Неверная роль"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Ошибка при входе"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Получаем URL базы данных из переменной окружения
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Создаем пул подключений к базе данных
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Создаем и запускаем HTTP сервер
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone())) // Передаем пул подключений в приложение
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
            .service(view_user)
            .service(get_user_courses_list)
            .service(get_topics_by_course)
            .service(unenrolled_courses)
            .service(enrolled_courses_list)
            .service(edit_course)
            
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .route("/login", web::post().to(login_handler)) // Регистрируем маршрут для логина
    })
    .bind("127.0.0.1:8080")? // Привязываем сервер к адресу и порту
    .run() // Запускаем сервер
    .await
}

// Обработчик для логина
async fn login_handler() -> impl Responder {
    "Login handler"
}
