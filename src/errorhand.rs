use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::DbErr;

#[derive(Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
pub enum ErrorResponder {
    #[response(status = 404)]
    NotFound(Json<ErrorMessage>),

    #[response(status = 500)]
    InternalError(Json<ErrorMessage>),

    #[response(status = 400)]
    BadRequest(Json<ErrorMessage>),
}

//tbh i dont even know why im returning json
impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => ErrorResponder::NotFound(Json(ErrorMessage {
                message: "Record not found".into(),
            })),
            _ => ErrorResponder::InternalError(Json(ErrorMessage {
                message: "server internal error".into(),
            })),
        }
    }
}

// #[derive(Responder)]
// #[response(status = 500, content_type = "json")]
// pub struct ErrorResponder {
//     message: String,
// }
//
// impl From<DbErr> for ErrorResponder {
//     fn from(err: DbErr) -> ErrorResponder {
//         ErrorResponder {
//             message: err.to_string(),
//         }
//     }
// }
//
// impl From<String> for ErrorResponder {
//     fn from(string: String) -> ErrorResponder {
//         ErrorResponder { message: string }
//     }
// }
//
// impl From<&str> for ErrorResponder {
//     fn from(str: &str) -> ErrorResponder {
//         str.to_owned().into()
//     }
// }
