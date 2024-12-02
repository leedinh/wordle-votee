use http_client::VoteeResponse;


mod http_client;


    // need guess size -> allowed characters -> full word

    async fn guess_word_characters(client: &http_client::VoteeHttpClient, size: i32, seed: i32) -> anyhow::Result<Vec<String>> {
        let mut allowed_chars = "abcdefghijklmnopqrstuvwxyz".to_string();
        if allowed_chars.len() % size as usize != 0 {
            allowed_chars = format!("{:width$}", allowed_chars, width = (allowed_chars.len() / size as usize + 1) * size as usize).replace(" ", "z");
        }
        let mut result = Vec::new();
        for i in 0..allowed_chars.len() / size as usize {
            let guess = allowed_chars.chars().skip(i * size as usize).take(size as usize).collect::<String>();
            let params = http_client::VoteeParams {
                guess: guess,
                size: Some(size),
                seed: Some(seed),
            };
            let resp = client.guess_random_word(&params).await?;
            match resp {
                http_client::HttpResult::Success(responses) => {
                    for response in responses {
                        if response.result != "absent" && !result.contains(&response.guess) {
                            result.push(response.guess);
                        }
                    }
                }
                http_client::HttpResult::Failure(_) => {
                    println!("Failed to guess word characters");
                }
                
            }
        }

        Ok(result)

    }

    async fn guess_word(client: &http_client::VoteeHttpClient, size: i32, seed: i32, allowed_chars: Vec<String>) -> anyhow::Result<String> {
        let mut result: Vec<VoteeResponse> = Vec::new();
        for i in allowed_chars {
            let guess = i.repeat(size as usize);
            let params = http_client::VoteeParams {
                guess: guess,
                size: Some(size),
                seed: Some(seed),
            };

            let resp = client.guess_random_word(&params).await?;
            match resp {
                http_client::HttpResult::Success(responses) => {
                    for response in responses {
                       if response.result == "correct" {
                           result.push(response.clone());
                       }
                    }
                }
                http_client::HttpResult::Failure(_) => {
                    println!("Failed to guess word");
                }
                
            }
        }

        result.sort_by(|a, b| a.slot.cmp(&b.slot));
        let word = result.iter().map(|x| x.guess.clone()).collect::<String>();
        Ok(word)
        
    }


#[tokio::main]
async  fn main() {
    let client = http_client::VoteeHttpClient::new();
    let size = 5;
    let seed = 1;
    let allowed_chars = guess_word_characters(&client, size, seed).await.unwrap();
    println!("Allowed characters: {:?}", allowed_chars);
    let final_word = guess_word(&client, size, seed, allowed_chars).await.unwrap();
    println!("Final word: {}", final_word);
    
}
