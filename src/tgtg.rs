use hyper::Client;

pub struct TGTGAuth {
    access_token: String,
    refresh_token: String,
    user_id: String
}

impl TGTGAuth {
    fn login(&self) {}
}

pub struct TGTG<C> {
    client: Client<C>,
    auth: TGTGAuth,
}

impl<C> TGTG<C> {
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.

    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse()?;

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    Ok(())
}

