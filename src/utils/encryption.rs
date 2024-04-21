use bcrypt::DEFAULT_COST;

// as we know bcrption process is designed to run slowly , it is an expensive operation, 
// so we need use it iin a parrallelism block, so that it could encrypt / decrypt password fastly, 
// rayon is the lightweight tool to run concurrent task

// https://crates.io/crates/bcrypt



pub async fn hash_password(password: String) -> String {
    let (send, recv) = tokio::sync::oneshot::channel();// creating a channel for send message between threads 
    
    rayon::spawn(move || {// rayon is used for sequential work in parrallismly --- > https://crates.io/crates/rayon
        
        let result = bcrypt::hash(password, DEFAULT_COST).unwrap(); // hash the original password
        let _ = send.send(result);
    });
    let hased = recv.await.unwrap(); 

    println!("hash password is:  {}",hased);

    hased
}


// here we are taking , the password which comes fromm LoginInput, and the hash_password(Encrypted) from Database
pub async fn veriy_password(password: String, hashed: String) -> bool {

    println!("hased pass from db: {}",hashed);
    let (send, recv) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        // this verify() will decrypt the encrypt password and match with the LoginInput pass, it it's match return true, otherwise false;
        let result = bcrypt::verify(password, &hashed).expect("something went wrong to bcrypt") ;
        let _ = send.send(result);
    });

    let is_verified = recv.await.unwrap();

    is_verified
}