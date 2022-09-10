pub mod prelude {
    use anyhow::Result;
    use futures::stream::StreamExt;
    use indicatif::{ProgressBar, ProgressStyle};
    use reqwest::Response;
    use std::path::Path;
    use std::{cmp::min, io::BufRead, io::Cursor};
    use tokio::{fs::File, io::copy};

    pub async fn create_file(resp: &Response) -> Result<File> {
        let file = {
            let fname = resp
                .url()
                .path_segments()
                .and_then(|f| f.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("Temp");

            println!(
                "File scaricato: {fname} ({:.2} MB)",
                match resp.content_length() {
                    Some(num) => num as f64 / (1024 * 1024) as f64,
                    None => 0 as f64,
                }
            );

            File::create(fname).await?
        };
        Ok(file)
    }

    pub fn read_line(path: &Path) -> Result<Vec<String>> {
        let file = std::fs::File::open(path)?;
        let buf = std::io::BufReader::new(file);

        Ok(buf.lines().filter_map(Result::ok).collect())
    }

    pub async fn download(response: Response) -> Result<()> {
        let total_size = response.content_length().unwrap_or(0);

        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
          .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
          .unwrap()
          .progress_chars("#>-"));

        let mut dest = create_file(&response).await?;

        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunck = item.unwrap();
            let mut data = Cursor::new(&chunck);
            copy(&mut data, &mut dest).await?;
            let new = min(downloaded + (chunck.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_and_clear();
        Ok(())
    }
}