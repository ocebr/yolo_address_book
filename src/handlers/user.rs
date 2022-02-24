use super::{authentication::AuthenticatedUser, AppResponse};
use crate::{db,
            config::crypto::CryptoService,
            db::user::UserRepository,
            errors::AppError,
            models::user::{User,SearchUsers,QueryAddFriend,KillFriendship}};

use actix_web::{web::Data,web::Json,web::Form, HttpResponse,HttpRequest,Responder,HttpMessage,web::Query};
use color_eyre::Result;
use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument};
use validator::Validate;
use serde_json::Value;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use uuid::Uuid;

#[instrument[skip(repository)]]
pub async fn get_users(user: AuthenticatedUser,repository: UserRepository, req: HttpRequest) -> AppResponse {
    let full_name = req.match_info().get("full_name").unwrap_or("NONE");
    println!("Hello {}!", &full_name);
    let name : String= String::from(full_name);
    //let name2 : String = String::from(full_name);
    let user = repository
        .get_all_users(name)
        .await;
    
    println!("user:     {:?}", user);

    Ok(HttpResponse::Ok().header("Access-Control-Allow-Methods","*").header("Access-Control-Allow-Origin","*").json(user))
        
        //.ok_or(AppError::INTERNAL_ERROR)?;
    //let user = user.as_str();
    

    // let json = repository
    //     .get_all_users(name2)
    //     .await;
    // //println!("{}",json.len());
    
    // let mut v : Vec<SearchUsers> = Vec::new();
    // let mut i = 0;
    // while i< json.len(){
    //         println!("{:?}", json[i].id);
    //         println!("{:?}\n", json[i].full_name);

    //         //create a json
    //         let json = format!("{}{}{}{}{}",(r#"{"id":""#),(&*json[i].id.to_string()),(r#"","full_name":""#),(&*json[i].full_name.to_string()),(r#""}"#));
    //    // println!("{:#?}",json);
    //     let request : SearchUsers = serde_json::from_str(&*json).unwrap();
    //     v.push(request);
    //         //println!("demo :{:?}", request);
    //         i= i + 1 ;
    // };

    // println!("{:?}\n\n\n", v);

    
    // println!("je suis dans get_users de handlers");
    // repository.get_all_users(name).await;
    //Ok(HttpResponse::Ok().header("Access-Control-Allow-Methods","*").header("Access-Control-Allow-Origin","*").body("ok"))
}

pub async fn test (req : HttpRequest) -> impl Responder{
    let request = req;
    HttpResponse::Ok().header("Access-Control-Request-Methods","*").header("Access-Control-Allow-Origin","*").body(format!("{:?}",request))
}

pub async fn add_friend (user : AuthenticatedUser,repository: UserRepository,req: actix_web::HttpRequest,info: Query<QueryAddFriend>) -> impl Responder{
    //decoder l'id du token + celui envoyer et mettre in table friend   
    
    // let friend_id_to_add = req.match_info().get("friend_id_to_add").unwrap_or("NONE");
    // let friend_full_name = req.match_info().get("friend_full_name").unwrap_or("NONE");



    //println!("{} : {}", info.friend_id_to_add,info.friend_full_name);


    let full_name = info.friend_full_name.clone();
    // println!("FULL NAME: {} \n\n ",friend_full_name);
    
    // println!("id du friend a ajouter:  {}", &friend_id_to_add);
    // println!("id du mec qui fait la req{}",user.0);
    
    let add_friend = repository
                        .db_add_friend(user.0,info.friend_id_to_add)
                        .await;
                        
    //let request = req.clone();
    //let cookie_auth = request.cookie("JWT").unwrap().value();
    //let auto= req.headers().get("Authorization").unwrap().to_str().unwrap();
    //println!("{:#?}",request.cookie("JWT").unwrap().value());
    //println!("{:?}",auto);
    println!("sortie de add friend");
    HttpResponse::Ok()

}

pub async fn get_your_friend(user : AuthenticatedUser,repository: UserRepository,req: actix_web::HttpRequest) -> impl Responder {
    //println!("dans get_your_friend user\n");
    let get_your_friend_value = repository
                                    .db_get_your_friend(user.0)
                                    .await;
    
    //println!("{:?}",get_your_friend_value);
    let mut  i = 0;
    let mut v : Vec<SearchUsers> = Vec::new();
   while i < get_your_friend_value.len(){
       //println!("{:?} : ",get_your_friend_value[i].friend);
       let value = &get_your_friend_value[i].friend;
       let current_full_name = repository.find_full_name_by_id(Uuid::parse_str(value).unwrap()).await;
       let json = format!("{}{}{}{}{}",(r#"{"id":""#),(&*value),(r#"","full_name":""#),(&*current_full_name.unwrap().unwrap().full_name),(r#""}"#));
       let request : SearchUsers = serde_json::from_str(&*json).unwrap();
       v.push(request);
      // println!("{:?} \n",current_full_name.unwrap().unwrap().full_name);
       i = i +1;
   }
    HttpResponse::Ok().json(v)//.header("Access-Control-Allow-Methods","*").header("Access-Control-Allow-Origin","*")//.json(get_your_friend_value)
}

pub async fn supp_a_friend(user : AuthenticatedUser,repository: UserRepository,req: actix_web::HttpRequest,info: Query<KillFriendship>) -> impl Responder{
    println!("dans supp a friend user");
    println!("id2 : {}", info.id2);
    let id2 = info.id2.clone();
    let supp_friend = repository
                        .db_supp_a_friend(user.0,id2)
                        .await;

    HttpResponse::Ok()
}