use surrealdb::{engine::remote::ws::{Client, Ws, Wss}, opt::auth::Root, Surreal};


use crate::utils;
pub async fn unified_db_instance()->Surreal<Client> {

    // getting all the important data from envirment 
    let db_url = utils::constants::DATABASE_URL.clone() ;
    let db_username = utils::constants::USERNAME.clone();
    let db_pass=utils::constants::PASSWORD.clone();
    let db_ns = utils::constants::NS.clone();
    let db_bd = utils::constants::DB.clone();


    let db = Surreal::new::<Ws>(db_url)
        .await
        .expect("Error : Unable to connect with Client ");

    db.signin(Root {
        username: "root",
        password: "preciq12345",
    })
    .await
    .expect("Error : Unable to Login");

    db.use_ns(db_ns)
        .use_db(db_bd)
        .await
        .expect("Unable to connect specified Namespace/Database");
    db
}