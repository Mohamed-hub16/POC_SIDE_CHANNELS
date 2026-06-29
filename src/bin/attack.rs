use std::time::Instant;
use std::time::Duration;




#[tokio::main]
async fn main(){

    let lenght_of_the_secret = guess_the_length_of_secret().await;
    println!("{}", lenght_of_the_secret);

}

async fn guess_the_length_of_secret() -> usize{

    let mut guess_characters = "a".to_string();
    let mut times_of_response: Vec<(usize, Duration)> = Vec::new();
    for i in 1..31{
        let url = format!("http://localhost:3000/seiko_secret?code={}", guess_characters);
        let start = Instant::now();
        let _ = reqwest::get(&url).await;
        let end = start.elapsed();

        println!("{:?}",end);
        times_of_response.push((i,end));
        guess_characters+="a";

    }

    return 0



}