use anyhow::Result;
use clap::Clap;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;

const BASE_URL: &str = "https://xkcd.com";
const LATEST_COMIC: usize = 0;

#[derive(Clap)]
pub struct Args {
    #[clap(short, long)]
    pub save: bool,

    #[clap(long, short, default_value = "30")]
    pub timeout: u64,

    #[clap(short, long, default_value = "0")]
    pub num: usize,

    #[clap(short, long, arg_enum, default_value = "text")]
    pub output: OutFormat,
}

#[derive(Clap, Copy, Clone)]
pub enum OutFormat {
    JSON,
    Text,
}

struct XkcdCli {
    args: Args,
}

//comic response holder
#[derive(Deserialize)]
pub struct ComicResponse {
    month: String,
    num: usize,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

struct Comic {
    title: String,
    num: usize,
    date: String,
    desc: String,
    img_url: String,
}

impl From<ComicResponse> for Comic {
    fn from(cr: ComicResponse) -> Self {
        Comic {
            title: cr.title,
            num: cr.num,
            date: format!("{}-{}-{}", cr.day, cr.month, cr.year),
            desc: cr.alt,
            img_url: cr.img,
        }
    }
}

impl TryFrom<String> for ComicResponse {
    type Error = anyhow::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json).map_err(|e| e.into())
    }
}

//constructor
impl XkcdCli {
    fn new(args: Args) -> Self {
        XkcdCli { args }
    }

    fn run(&self) -> Result<()> {
        let url = if let Some(n) = self.args.num {
            format!("{}/{}/info.0.json", BASE_URL, n)
        } else {
            format!("{}/info.0.json", BASE_URL)
        };
        let http_client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(self.args.timeout))
            .build()?;
        let resp: ComicResponse = http_client.get(&url).send()?.text()?.try_into()?;
        let comic: Comic = resp.into();
        if self.args.save {
            comic.save()?;
        }
        comic.print(self.args.output)?;
        Ok(())
    }
}

//impl comic
impl Comic {
    fn print(&self, of: OutFormat) -> Result<()> {
        match of {
            OutFormat::Text => println!("{}", todo!("print self as Text")),
            OutFormat::Json => println!("{}", todo!("print self as JSON")),
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = XkcdCli::new(args);
    client.run();
}
