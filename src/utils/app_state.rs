use surrealdb::{engine::remote::ws::Client, Surreal};

use super::{db_instance::unified_db_instance, jwt::Claims};

#[derive(Clone)]// making it clonable for state exraction
pub struct AppState {
    pub surreal_client: Surreal<Client>,
   
}

// impl AppState {
//     pub async fn db_instance(){
//         let db =  unified_db_instance().await;
//         db
//     }
// }





