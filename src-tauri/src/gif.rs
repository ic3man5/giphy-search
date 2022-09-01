pub mod giphy {
    use giphy::v1::gifs::SearchRequest;
    use giphy::v1::sync::*;
    use reqwest;

    /// Default API Key to use
    pub const DEFAULT_API_KEY: &str = "WPWGNRtoMq37sWokCR2GGIiHIWXQlPRG";

    pub struct Giphy {
        api_key: String,
    }
    #[derive(Debug)]
    pub enum GiphyURLType {
        Url,
        Bitly,
        Embed,
        Original,
        Downsized,
        DownsizedLarge,
        DownsizedMedium,
        DownsizedSmall,
    }
    #[derive(Debug)]
    pub enum GiphySearchError {
        NoResults,
        SearchFailure(String),
        SearchTypeNotFound,
    }

    impl Giphy {
        pub fn new(api_key: String) -> Self {
            Giphy { api_key }
        }

        fn get_api(&self) -> SyncApi {
            let client = reqwest::blocking::Client::new();
            SyncApi::new(self.api_key.clone(), client)
        }

        pub fn search_url(
            &self,
            search_string: &str,
            url_type: GiphyURLType,
            limit: Option<u32>,
        ) -> Result<Vec<String>, GiphySearchError> {
            let api = self.get_api();
            // Get the response object from the search_string
            let response = SearchRequest::new(search_string)
                .with_limit(limit.unwrap_or(20))
                .send_to(&api);
            let response = match &response {
                Ok(r) => r,
                Err(e) => {
                    return Err(GiphySearchError::SearchFailure(e.to_string()));
                }
            };
            // We can't do anything if didn't get any results
            if response.data.is_empty() {
                return Err(GiphySearchError::NoResults);
            }

            let mut found_gif_urls = Vec::new();
            for gif in &response.data {
                // Get the URL requested
                let url = match &url_type {
                    GiphyURLType::Url => gif.url.clone(),
                    GiphyURLType::Bitly => gif.bitly_url.clone(),
                    GiphyURLType::Embed => gif.embed_url.clone(),
                    GiphyURLType::Original => match &gif.images.original.url {
                        Some(u) => u.clone(),
                        None => String::new(),
                    },
                    GiphyURLType::Downsized => match &gif.images.downsized.url {
                        Some(u) => u.clone(),
                        None => String::new(),
                    },
                    GiphyURLType::DownsizedLarge => match &gif.images.downsized_large.url {
                        Some(u) => u.clone(),
                        None => String::new(),
                    },
                    GiphyURLType::DownsizedMedium => match &gif.images.downsized_medium.url {
                        Some(u) => u.clone(),
                        None => String::new(),
                    },
                    GiphyURLType::DownsizedSmall => match &gif.images.downsized_small.url {
                        Some(u) => u.clone(),
                        None => String::new(),
                    },
                };
                // Push the URL only if its actually there. Invalid URLs will be an Empty String
                if !url.is_empty() {
                    found_gif_urls.push(url);
                }
            }
            Ok(found_gif_urls)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gif::giphy::*;

    #[test]
    fn test_get_client() {
        let giphy = Giphy::new("WPWGNRtoMq37sWokCR2GGIiHIWXQlPRG".to_string());
        let search_text = String::from("lmao");
        let urls = giphy
            .search_url(&search_text, GiphyURLType::Original, Some(300))
            .unwrap();
        for url in &urls {
            println!("url: {}", url);
        }
    }
}
