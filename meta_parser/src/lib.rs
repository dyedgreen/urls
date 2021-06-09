use html_escape::decode_html_entities;
use nom::{error::ErrorKind, Err};
use parser::Tag;

mod parser;

const MAX_BUFFER_SIZE: usize = 4096;

/// Contains meta information about a
/// parsed HTML page.
#[derive(Debug, Clone)]
pub struct Meta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,

    buffer: Vec<u8>,
}

impl Meta {
    /// Create a new `Meta` object, which
    /// can be used to incrementally parse
    /// html.
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            image: None,
            buffer: Vec::new(),
        }
    }

    /// Parse additional HTML. If the parser needs more data
    /// to finish matching, the HTML is retained in a buffer,
    /// and this function returns.
    ///
    /// The amount of bytes held in this buffer will always be
    /// truncated to `MAX_BUFFER_SIZE`. If the buffer overflows
    /// after an incomplete parse, bytes from the beginning of
    /// the buffer are discarded.
    pub fn parse(&mut self, html: &[u8]) {
        for &byte in html {
            self.buffer.push(byte);
        }

        'outer: loop {
            'buffer: for offset in 0..self.buffer.len() {
                match parser::run::<(&[u8], ErrorKind)>(&self.buffer[offset..]) {
                    Result::Err(Err::Incomplete(_)) => {
                        // wait for more data
                        break 'outer;
                    }
                    Result::Err(_) => {
                        // try from next offset
                        continue 'buffer;
                    }
                    Ok((rest, tag)) => {
                        // interpret tag
                        match tag {
                            Tag::Meta(meta_tag) => match meta_tag.name {
                                b"twitter:title" | b"og:title" => {
                                    self.title = decode_str_bytes(meta_tag.content);
                                }
                                b"twitter:description" | b"og:description" => {
                                    self.description = decode_str_bytes(meta_tag.content);
                                }
                                b"twitter:image" | b"twitter:image:src" | b"og:image" => {
                                    self.image = decode_str_bytes(meta_tag.content);
                                }
                                _ => {}
                            },
                            Tag::Title(title_tag) => {
                                if self.title.is_none() {
                                    self.title = decode_str_bytes(title_tag.title);
                                }
                            }
                        }
                        // clean up used buffer
                        self.buffer = rest.to_vec();
                        continue 'outer;
                    }
                }
            }
            // all offsets were tried and failed
            // return and reset buffer
            self.buffer.clear();
            break;
        }

        // trim buffer if it exceeds the maximal allowed size
        if self.buffer.len() > MAX_BUFFER_SIZE {
            let start = self.buffer.len() - MAX_BUFFER_SIZE;
            self.buffer.copy_within(start.., 0);
            self.buffer.truncate(MAX_BUFFER_SIZE);
        }
    }
}

fn decode_str_bytes(bytes: &[u8]) -> Option<String> {
    let html = String::from_utf8_lossy(bytes);
    let clean = decode_html_entities(&html).trim().to_string();
    if clean.is_empty() {
        None
    } else {
        Some(clean)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_example() {
        let twitter_example = r#"
            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:site" content="@nytimes">
            <meta name="twitter:creator" content="@SarahMaslinNir">
            <meta name="twitter:title" content="Parade of Fans for Houston’s Funeral">
            <meta name="twitter:description" content="NEWARK - The guest list and parade of limousines with celebrities emerging from them seemed more suited to a red carpet event in Hollywood or New York than than a gritty stretch of Sussex Avenue near the former site of the James M. Baxter Terrace public housing project here.">
            <meta name="twitter:image" content="http://graphics8.nytimes.com/images/2012/02/19/us/19whitney-span/19whitney-span-articleLarge.jpg">
        "#;

        let mut meta = Meta::new();
        meta.parse(twitter_example.as_bytes());

        assert_eq!(
            meta.title,
            Some("Parade of Fans for Houston’s Funeral".into())
        );
        assert_eq!(
            meta.description,
            Some("NEWARK - The guest list and parade of limousines with celebrities emerging from them seemed more suited to a red carpet event in Hollywood or New York than than a gritty stretch of Sussex Avenue near the former site of the James M. Baxter Terrace public housing project here.".into())
        );
        assert_eq!(meta.image, Some("http://graphics8.nytimes.com/images/2012/02/19/us/19whitney-span/19whitney-span-articleLarge.jpg".into()));
    }

    #[test]
    fn test_facebook_example() {
        let facebook_example = r#"
            <meta property="og:url"                content="http://www.nytimes.com/2015/02/19/arts/international/when-great-minds-dont-think-alike.html" />
            <meta property="og:type"               content="article" />
            <meta property="og:title"              content="When Great Minds Don’t Think Alike" />
            <meta property="og:description"        content="How much does culture influence creative thinking?" />
            <meta property="og:image"              content="http://static01.nyt.com/images/2015/02/19/arts/international/19iht-btnumbers19A/19iht-btnumbers19A-facebookJumbo-v2.jpg" />
        "#;

        let mut meta = Meta::new();
        meta.parse(facebook_example.as_bytes());

        assert_eq!(
            meta.title,
            Some("When Great Minds Don’t Think Alike".into())
        );
        assert_eq!(
            meta.description,
            Some("How much does culture influence creative thinking?".into())
        );
        assert_eq!(meta.image, Some("http://static01.nyt.com/images/2015/02/19/arts/international/19iht-btnumbers19A/19iht-btnumbers19A-facebookJumbo-v2.jpg".into()));
    }

    #[test]
    fn test_html_entities() {
        let html_entities_example = r#"
            <title>Hello &middot; world!</title>
        "#;

        let mut meta = Meta::new();
        meta.parse(html_entities_example.as_bytes());

        assert_eq!(meta.title, Some("Hello · world!".into()));
    }

    #[test]
    fn test_white_space() {
        let html_entities_example = r#"
            <title> Badly Aligned </title>
        "#;

        let mut meta = Meta::new();
        meta.parse(html_entities_example.as_bytes());

        assert_eq!(meta.title, Some("Badly Aligned".into()));
    }

    #[test]
    fn test_empty_meta() {
        let html_entities_example = r#"
            <meta name="twitter:title" content="  " />
        "#;

        let mut meta = Meta::new();
        meta.parse(html_entities_example.as_bytes());

        assert_eq!(meta.title, None);
    }
}
