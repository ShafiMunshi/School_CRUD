use bcrypt::hash;
use chrono::DateTime;


#[tokio::main]
async fn main() {


    let pass = "pass1234";

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::hash(pass, bcrypt::DEFAULT_COST);
        let _ = send.send(result);
    });

    let recc= recv.await.unwrap().unwrap();

    println!(" {}",recc);
    tracing::debug!("end");
}
