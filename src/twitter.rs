use json::{object, stringify};
use reqwest::header::CONTENT_TYPE;
use reqwest_oauth1::{self, Error, OAuthClientProvider};

const TWITTER_API_URL: &str = "https://api.twitter.com/2/tweets";
const FLASHBOTS_EXPLORER_URL: &str = "https://flashbots-explorer.marto.lol";

pub struct BigRewardTweet {
    pub value: String,
    pub block_number: u32,
}

impl BigRewardTweet {
    pub fn build_tweet_content(&self) -> String {
        let mut tweet_string = String::from("Big reward! 🚨\n\n");
        tweet_string.push_str(&format!("Value: Ξ {:.1$}\n", self.value, 7));
        tweet_string.push_str(&format!(
            "🔎 {}/?block={}",
            FLASHBOTS_EXPLORER_URL, self.block_number
        ));
        tweet_string
    }
}

pub async fn publish_tweet(tweet: &BigRewardTweet) -> Result<(), Error> {
    let consumer_key = std::env::var("TWITTER_API_KEY").expect("TWITTER_API_KEY must be set.");
    let consumer_secret =
        std::env::var("TWITTER_API_KEY_SECRET").expect("TWITTER_API_KEY_SECRET must be set.");
    let access_token_key =
        std::env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN must be set.");
    let access_token_secret =
        std::env::var("TWITTER_TOKEN_SECRET").expect("TWITTER_TOKEN_SECRET must be set.");

    let secrets = reqwest_oauth1::Secrets::new(consumer_key, consumer_secret)
        .token(access_token_key, access_token_secret);

    let client = reqwest::Client::new();

    let request_body = object! {
      text: tweet.build_tweet_content()
    };

    match client
        .oauth1(secrets)
        .post(TWITTER_API_URL)
        .header(CONTENT_TYPE, "application/json")
        .body(stringify(request_body))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
