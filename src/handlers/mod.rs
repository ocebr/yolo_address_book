use actix_web::{web, web::ServiceConfig, HttpResponse};
mod authentication;
mod user;
use crate::errors::AppError;


use user::{get_users,test,add_friend,get_your_friend,supp_a_friend};



type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;


pub fn app_config(config : &mut ServiceConfig) {


    let get_users = web::resource("/get_users/{full_name}").route(web::get().to(get_users));
    //let search = web::resource("/get_users/{full_name}").route(web::get().to(get_users));

    let test = web::resource("/test").route(web::get().to(test));

    let add_friend = web::resource("/add_friend/").route(web::post().to(add_friend));

    let get_your_friend = web::resource("/get_your_friend").route(web::get().to(get_your_friend));

    let supp_friend = web::resource("/supp_friend/").route(web::post().to(supp_a_friend));

    config.service(get_users).service(test).service(add_friend).service(get_your_friend).service(supp_friend);
}