pub mod prelude {
    use anyhow::Result;
    use reqwest::Response;
    use std::io::BufRead;
    use std::path::Path;
    use tokio::fs::File;

    pub async fn create_file(resp: &Response) -> Result<File> {
        let file = {
            let fname = resp
                .url()
                .path_segments()
                .and_then(|f| f.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("Temp");

            println!(
                "File da scaricare: {fname} ({:.2} MB)",
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
}
