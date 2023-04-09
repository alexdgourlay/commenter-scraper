use std::error::Error;

use scraper::{Html, Selector};
use url::{ParseError, Url};

pub struct ContentScraper {
    url: Url,
    document: Html,
}

/// The different OpenGraph property types.
/// Defined in [protocol docs](https://ogp.me/).
///
/// Currently a subset of all valid types!
///
/// Serialized into camelCase values using strum for selecting within
/// a document.
#[derive(strum_macros::Display)]
#[strum(serialize_all = "camelCase")]
enum OgProperty {
    Image,
    Title,
}

impl ContentScraper {
    pub fn new(url: &str, html: &str) -> Result<Self, Box<dyn Error>> {
        // Parsing validates the supplied url.
        let url = Url::parse(url)?;

        // Parsing html document.
        let document = Html::parse_document(html);

        Ok(Self { url, document })
    }

    pub fn icon(&self) -> Option<String> {
        if let Some(icon_path) = self.get_shortcut_icon() {
            return self.absolute(&icon_path).ok();
        }
        None
    }

    pub fn title(&self) -> Option<String> {
        return self.get_og_title();
    }

    pub fn image(&self) -> Option<String> {
        if let Some(image_path) = self.get_og_image() {
            return self.absolute(&image_path).ok();
        }
        None
    }

    fn absolute(&self, path: &str) -> Result<String, ParseError> {
        let url = Url::parse(path);

        match url {
            Ok(_) => Ok(path.to_string()),
            Err(parse_error) => {
                // If the path is a relative path.
                if parse_error == ParseError::RelativeUrlWithoutBase {
                    let full_url = self.url.join(path)?;
                    return Ok(full_url.into());
                }
                return Err(parse_error);
            }
        }
    }

    fn get_shortcut_icon(&self) -> Option<String> {
        let selector_str = r#"link[rel="shortcut icon"]"#;
        let selector = Selector::parse(&selector_str).unwrap();

        if let Some(element) = self.document.select(&selector).next() {
            if let Some(content) = element.value().attr("href") {
                return Some(content.to_string());
            }
        }
        None
    }

    fn get_og_property(&self, property: OgProperty) -> Option<String> {
        let selector_str = format!(r#"meta[property="og:{}"]"#, property);
        let selector = Selector::parse(&selector_str).unwrap();

        if let Some(element) = self.document.select(&selector).next() {
            if let Some(content) = element.value().attr("content") {
                return Some(content.to_string());
            }
        }
        None
    }

    fn get_og_image(&self) -> Option<String> {
        self.get_og_property(OgProperty::Image)
    }

    fn get_og_title(&self) -> Option<String> {
        self.get_og_property(OgProperty::Title)
    }
}

#[cfg(test)]
mod tests {

    use super::ContentScraper;
    use std::{env, fs};

    /// Read test document contents from relative path.
    fn read_document(document_path: &str) -> String {
        let document_abs_path = env::current_dir().unwrap().join(document_path);
        fs::read_to_string(document_abs_path).unwrap()
    }

    /// Returns a ContentScraper for verge.com test document.
    fn get_verge_scraper() -> ContentScraper {
        ContentScraper::new(
            "https://www.theverge.com/",
            &read_document("assets/verge-article.html"),
        )
        .unwrap()
    }

    #[test]
    fn absolute_works_for_relative_path() {
        let scraper = get_verge_scraper();
        assert_eq!(
            scraper.absolute("/icons/favicon.ico"),
            Ok("https://www.theverge.com/icons/favicon.ico".to_string(),),
            "Appends the relative path to the scraper's url."
        );
    }

    #[test]
    fn absolute_works_for_url() {
        let scraper = get_verge_scraper();
        assert_eq!(
            scraper.absolute("https://www.theverge.com/icons/favicon.ico"),
            Ok("https://www.theverge.com/icons/favicon.ico".to_string(),),
            "Returns the supplied full url."
        );
    }

    #[test]
    fn icon() {
        let scraper = get_verge_scraper();
        assert_eq!(
            scraper.icon(),
            Some("https://www.theverge.com/icons/favicon.ico".into()),
            "Retrieved the first shortcut-icon path."
        )
    }

    #[test]
    fn title() {
        let scraper = get_verge_scraper();
        assert_eq!(
            scraper.title(),
            Some("Three new Star Wars movies are on the way".into()),
            "Retrieved the first OpenGraph title content."
        )
    }

    #[test]
    fn image() {
        let scraper = get_verge_scraper();
        assert_eq!(
            scraper.image(),
            Some("https://cdn.vox-cdn.com/thumbor/IWw0MWvB3UvbpYLKoeb36lRySrE=/0x0:2048x858/1200x628/filters:focal(393x522:394x523)/cdn.vox-cdn.com/uploads/chorus_asset/file/9855501/LastJediReyLightsaber.jpg".into()),
            "Retrieved the first OpenGraph image content."
        )
    }
}
