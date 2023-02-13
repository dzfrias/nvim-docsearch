use anyhow::Result;
use futures::StreamExt;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
};
use url::{ParseError, Url};
use voyager::{scraper::Selector, Collector, Crawler, CrawlerConfig, Response, Scraper};

#[derive(Debug)]
struct DocScraper {
    help_tag_selector: Selector,
    link_selector: Selector,
    visited: HashSet<Url>,
}

impl Default for DocScraper {
    fn default() -> Self {
        Self {
            help_tag_selector: Selector::parse(".help-tag, .help-tag-right")
                .expect("should be valid selector"),
            link_selector: Selector::parse("a").expect("should be valid selector"),
            visited: HashSet::new(),
        }
    }
}

impl Scraper for DocScraper {
    type State = ();
    type Output = HashSet<Url>;

    fn scrape(
        &mut self,
        response: Response<Self::State>,
        crawler: &mut Crawler<Self>,
    ) -> Result<Option<Self::Output>> {
        let html = response.html();

        // Get all help tags on page and turn them into links
        let help_tags = html
            .select(&self.help_tag_selector)
            .map(|el| {
                response
                    .response_url
                    .join(&format!("#{}", urlencoding::encode(&el.inner_html())))
                    .expect("should be valid joined url")
            })
            .collect::<HashSet<_>>();

        for href in html
            .select(&self.link_selector)
            .filter_map(|a| a.value().attr("href"))
        {
            let mut url = match Url::parse(href) {
                Ok(url) => url,
                Err(ParseError::RelativeUrlWithoutBase) => {
                    response.response_url.join(href).unwrap()
                }
                Err(_) => continue,
            };
            // Remove named anchor, not needed for crawling
            url.set_fragment(None);
            if self.visited.contains(&url) {
                continue;
            }
            self.visited.insert(url.clone());
            crawler.visit(url);
        }

        Ok(Some(help_tags))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = CrawlerConfig::default().allow_domain("neovim.io");
    let mut collector = Collector::new(DocScraper::default(), config);
    collector.crawler_mut().visit("https://neovim.io/doc/user");

    let mut out = BufWriter::new(File::create("./out.txt")?);
    while let Some(output) = collector.next().await {
        if let Ok(urls) = output {
            for url in urls {
                writeln!(out, "{}", url.to_string())?;
            }
        }
    }
    out.flush()?;

    Ok(())
}
