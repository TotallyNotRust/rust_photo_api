use actix_web::{
    get, post,
    web::{Json, Query},
    FromRequest, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::Serializer;

use crate::get_photos;

#[derive(Deserialize)]
pub struct Indent {
    indent: i32,
}
#[derive(Serialize)]
pub struct ImageList {
    images: Vec<String>,
}

pub async fn photos(indent: Query<Indent>) -> impl Responder {
    let image_list: ImageList = ImageList {
        images: get_photos(),
    };

    return HttpResponse::Ok().body(serde_json::to_string(&image_list).unwrap());
}
