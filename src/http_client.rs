use serde::Deserialize;


pub struct VoteeHttpClient {
    client: reqwest::Client
}

pub struct VoteeParams {
    pub guess: String,
    pub size: Option<i32>,
    pub seed: Option<i32>,
}

// [
//   {
//     "slot": 0,
//     "guess": "a",
//     "result": "absent"
//   },
//   {
//     "slot": 1,
//     "guess": "b",
//     "result": "absent"
//   },
//   {
//     "slot": 2,
//     "guess": "c",
//     "result": "absent"
//   },
//   {
//     "slot": 3,
//     "guess": "e",
//     "result": "correct"
//   },
//   {
//     "slot": 4,
//     "guess": "f",
//     "result": "absent"
//   }
// ]

#[derive(Debug, Deserialize, Clone)]
pub struct VoteeResponse {
    pub slot: i32,
    pub guess: String,
    pub result: String,
}

pub enum HttpResult {
    Success(Vec<VoteeResponse>),
    Failure(()),
}



impl VoteeHttpClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        VoteeHttpClient { client }
    }

    pub async fn guess_random_word(&self, query: &VoteeParams) -> anyhow::Result<HttpResult> {
        println!("Guessing word: {}", query.guess);
        let url = format!("https://wordle.votee.dev:8000/random");
        let params = [("guess", query.guess.clone())];
        let mut request = self.client.get(&url).query(&params);
        if let Some(size) = query.size {
            request = request.query(&[("size", size.to_string())]);
        }

        if let Some(seed) = query.seed {
            request = request.query(&[("seed", seed.to_string())]);
        }

        let response = request.send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let response = response.json::<Vec<VoteeResponse>>().await?;
                Ok(HttpResult::Success(response))
            }
            _ => {
                Ok(HttpResult::Failure(()))
            }
            
        }

    }
}