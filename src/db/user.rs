use crate::{config::crypto::CryptoService,
    models::user::{User,SearchUsers,FriendAdd,YourFriend,SelectFriendAdd,FindFullName},
    errors::AppError,
    errors::AppErrorCode,
    };


use actix_web::{web::Data, FromRequest ,HttpResponse, web::Json};
use sqlx::{PgPool, postgres::PgQueryAs,query_as};
use std::sync::Arc;
use std::ops::Deref;
use color_eyre::Result;
use uuid::Uuid;
use futures::future::{ready,Ready};
use tracing::instrument;
use serde_json::{Value,json};


pub struct UserRepository {
    pool: Arc<PgPool>
}

impl UserRepository {
    pub fn new(pool:Arc<PgPool>) -> Self {
        Self {pool}
    }

    #[instrument(skip(self))]
    pub async fn find_by_username(&self, username: &str) -> Result<User> {
        let maybe_user = sqlx::query_as::<_, User>("select * from users_info where username = $1")
            .bind(username)
            .fetch_one(&*self.pool)
            .await?;

        Ok(maybe_user)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let maybe_user = sqlx::query_as::<_, User>("select * from users_info where id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(maybe_user)
    }
    #[instrument(skip(self))]
    pub async fn find_full_name_by_id(&self, id: Uuid) -> Result<Option<FindFullName>> {
        println!("dans find full name by id ");
        let maybe_user = sqlx::query_as::<_, FindFullName>("select full_name from users_info where id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(maybe_user)
    }


    #[instrument(skip(self))]
    pub async fn get_all_users(&self,full_name : String) -> Vec<SearchUsers>{

        println!("{}", &full_name);
        let full_name_queriable = String::from(full_name + "%");

        let all_users = sqlx::query_as::<_,SearchUsers>("select full_name,id from users_info where full_name like $1")
            .bind(full_name_queriable)
            .fetch_all(&*self.pool)
            .await;

        // println!("{:#?}",format!("{:?}", all_users));
        // //println!("{:#?}",format!("{:?}", &all_users.unwrap()));
        // //let json = serde_json::from_str(all_users.unwrap());
        //let users = format!("{:?}", all_users.unwrap());
        // let users = json!(all_users).as_array();
        // println!("{:?}",&users);
        // //users
        all_users.unwrap()
       

    //    let maybe_user = sqlx::query_as::<_,SearchUsers >("select full_name from users_info")
    //    .fetch_all(&*self.pool)
    //    .await?;

    //     Ok(maybe_user)
    }
    pub async fn check_if_friends(&self, id1 : Uuid, id2: Uuid) -> bool {
        let maybe_user = sqlx::query_as::<_, SelectFriendAdd>("select * from friends_list where id_user = $1 AND friend = $2;")
        .bind(id1.to_string())
        .bind(id2.to_string())
        .fetch_optional(&*self.pool)
        .await;
        
        if maybe_user.unwrap().is_none(){
            println!("pas amis\n");
            false
        }
        else{                 
                println!("DEJA AMIS");
            true      
        }

    }
    pub async fn db_add_friend(&self, id_asker : Uuid, id_to_add: Uuid) -> Result<FriendAdd,()>{
        println!("{} ask {} to be friend", id_asker,id_to_add);
    

        let id_asker_2 = id_asker.clone();
        let id_to_add_2 = id_to_add.clone();
        


        if self.check_if_friends(id_asker,id_to_add_2).await {
            Err(())
        }
        else {
            let adding_friend = sqlx::query_as::<_, FriendAdd>(
                      "insert into friends_list (id_user,friend) values ($1,$2),($2,$1) returning *;"
                        
                     )
                     .bind(&id_asker)
                     .bind(&id_to_add)        
                     .fetch_one(&*self.pool)
                     .await;
            
                     Ok(adding_friend.unwrap()) 

        }
        // let maybe_user = sqlx::query_as::<_, SelectFriendAdd>("select * from friends_list where id_user = $1 AND friend = $2;")
        // .bind(id_asker_2.to_string())
        // .bind(id_to_add_2.to_string())
        // .fetch_optional(&*self.pool)
        // .await;

        // let user_to_err = format!("{:?}",maybe_user);
        
        // if maybe_user.unwrap().is_none(){
        //     println!("adding friend.....\n");
        //     println!("{} : {}", id_asker,id_to_add);

        //     let adding_friend = sqlx::query_as::<_, FriendAdd>(
        //          "insert into friends_list (id_user,friend) values ($1,$2),($2,$1) returning *;"
                
        //     )
        //     .bind(&id_asker)
        //     .bind(&id_to_add)        
        //     .fetch_one(&*self.pool)
        //     .await?;
    
        //     Ok(adding_friend) 
        // }
        // else{                 
        //         println!("DEJA AMIS");
        //         Ok(user_to_err).expect_err("already friends")      
        // }

      
    }

    pub async fn db_get_your_friend(&self, id_asker : Uuid) -> Vec<YourFriend>{
        //println!("dans get friend db");
        let id_asker = id_asker.to_string();
        let your_friend = sqlx::query_as::<_,YourFriend>("select friend from friends_list where id_user=$1")
            .bind(id_asker)
            .fetch_all(&*self.pool)
            .await;
        
           your_friend.unwrap()


    }
    pub async fn db_supp_a_friend(&self, id_asker: Uuid, id_to_supp: Uuid) {//-> Result<FriendAdd,()>{
            println!("dans db_supp_a_friend");

            let id_asker2 = &id_asker.clone();
            let id_to_supp2 = &id_to_supp.clone();

            println!("id asker 2: {}",&id_asker2);
            println!("id_to supp: {}",&id_asker2);

            if self.check_if_friends(id_asker,id_to_supp).await && self.check_if_friends(id_to_supp,id_asker).await{
                println!("amis je supprime...");
                let delete_friendship = sqlx::query_as::<_, FriendAdd>(
                    "delete from friends_list where id_user = $1 and friend = $2 returning *;"
                          
                       )
                       .bind(&id_asker.to_string())
                       .bind(&id_to_supp.to_string())        
                       .fetch_one(&*self.pool)
                       .await;
                let delete_friendship2 = sqlx::query_as::<_, FriendAdd>(
                    "delete from friends_list where id_user = $1 and friend = $2 returning *;"
                        
                    )
                    .bind(&id_to_supp2.to_string()) 
                    .bind(&id_asker2.to_string())
                    .fetch_one(&*self.pool)
                    .await;
            }
            else {
                println!("pas amis de base");
            }
            //  delete from friends_list where created_at < '2030-01-01 18:01:01.000000' and id_user = $1 and friend = $2;
            // let adding_friend = sqlx::query_as::<_, FriendAdd>(
            //     "insert into friends_list (id_user,friend) values ($1,$2),($2,$1) returning *;"
                  
            //    )
            //    .bind(&id_asker)
            //    .bind(&id_to_add)        
            //    .fetch_one(&*self.pool)
            //    .await;
    }
}

impl FromRequest for UserRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req, payload))]
    fn from_request(    
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PgPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(UserRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}