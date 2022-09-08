pub mod argom;
use crate::argom::Args;
use downnd::*;

use anyhow::Result;
use clap::Parser;
use futures::stream::StreamExt;
use std::io::Cursor;
use tokio::io::copy;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(url) = args.url.as_deref() {
        let response = reqwest::get(url).await?;

        let mut dest = create_file(&response).await?;

        let mut content = Cursor::new(response.bytes().await?);
        copy(&mut content, &mut dest).await?;
    }

    if let Some(path) = args.path.as_deref() {
        let path = read_line(path)?;
        let fetches = futures::stream::iter(path.into_iter().map(|path| async move {
            match reqwest::get(&path).await {
                Ok(resp) => {
                    let mut dest = create_file(&resp).await.unwrap();
                    let mut content = Cursor::new(resp.bytes().await.unwrap());
                    copy(&mut content, &mut dest).await.unwrap();
                }
                Err(_) => println!("Error in {}", path),
            }
        }))
        .buffer_unordered(100)
        .collect::<Vec<()>>();

        fetches.await;
    }

    Ok(())
}