use std::io::Cursor;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn download_file(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();

    let url = &args[1];
    let file_name = &args[2];

    download_file(url.to_string(), file_name.to_string()).await.unwrap();
}