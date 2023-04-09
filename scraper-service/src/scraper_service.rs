use std::error::Error;

use crate::scraper_proto::{scraper_server::Scraper, Content, ContentRequest};
use content_scraper::ContentScraper;
use reqwest;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ScraperService {}

impl ScraperService {
    /// A helper function for fetching an html document from
    /// a url.
    async fn fetch_html(url: &String) -> Result<String, reqwest::Error> {
        reqwest::get(url).await?.text().await
    }

    /// Generates and composes the content data using a ContentScraper 
    /// instance.
    fn generate_content(url: String, html: String) -> Result<Content, Box<dyn Error>> {
        let content_scraper = ContentScraper::new(&url, &html);

        match content_scraper {
            // Compose the content data.
            Ok(content_scraper) => Ok(Content {
                title: content_scraper.title(),
                image: content_scraper.image(),
                icon: content_scraper.icon(),
            }),
            Err(err) => Err(err),
        }
    }
}

#[tonic::async_trait]
impl Scraper for ScraperService {
    /// Implementation of the GetContent rpc.
    async fn get_content(
        &self,
        request: Request<ContentRequest>,
    ) -> Result<Response<Content>, Status> {
        let url = request.into_inner().url;
        let html = ScraperService::fetch_html(&url).await;

        match html {
            Ok(html) => {
                let content = ScraperService::generate_content(url, html);
                match content {
                    Ok(content) => Ok(Response::new(content)),
                    Err(err) => Err(Status::invalid_argument(err.to_string())),
                }
            }
            Err(error) => {
                return Err(Status::not_found(format!(
                    "HTML could not be retrieved from provided url. {}",
                    error
                )))
            }
        }
    }
}
