use axum::{
    routing::get,
    Router,
    extract::Query,
};

use serde::Deserialize;
use tokio::time::{sleep, Duration};


#[derive(Deserialize)]
struct Parameters {
    code : Option<String>,
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/seiko_home", get(seiko_home_page))
        .route("/seiko_secret", get(seiko_secret_page));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn seiko_home_page() ->  &'static str{
    "Pas de Seiko pas de N3"
}

async fn seiko_secret_page(Query(p) : Query<Parameters>) -> String{
    let secret =  "KstUvamdoiekldz";
    
    match p.code{
        Some(c)=>{
            if compare(secret, &c).await {
                "VOUS AVEZ REUSSI".to_string()
            }
            else {
                "Trompé".to_string()
            }
        }
        None=> "Trompé".to_string(),
    }
        
}

async fn compare(secret : &str, code : &str) -> bool{
    let secret = secret.as_bytes();
    let code = code.as_bytes();


    if secret.len() != code.len(){
        return false
    }

    sleep(Duration::from_millis(50)).await;


    let mut i = 0;
    while i < secret.len() {
        if secret[i] != code[i] {
            break;
        }

        sleep(Duration::from_millis(50)).await;
        i += 1;
    }
    return i==secret.len()
}