use axum::Json;
use serde::Serialize;
use serde_json::json;

fn main() {
    let s = format!(
        "{:?}",
        Helllo {
            m: "shafi".to_string()
        }
    );
    println!(" {}", s);

    let f = serde_json::to_string(&s).expect("wrong");
    println!(" {:?}", f);

    let hello = Helllo {
        m: "shafi".to_string(),
    };
    let s = serde_json::to_string(&hello).expect("Failed to serialize");
    println!("{}", s);

  
}

#[derive(Debug, Serialize)]
struct Helllo {
    m: String,
}
