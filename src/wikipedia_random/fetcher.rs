use std::error::Error;
use std::fmt::{Formatter, Result as FmtResult, Display, Debug};

use super::Article;


pub enum FetcherType {
    SingleBlocking,
    Multiple(i32)
}

pub enum FetcherError {
    RequestError(String),
    ParseError(String),
}

impl FetcherError {
    fn message(&self) -> &str {

       match self {
           Self::RequestError(reason) => reason,
           Self::ParseError(reason) => reason,
       }
    }
}

impl Display for FetcherError {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}",self.message())
    }
 }

 impl Debug for FetcherError {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}",self.message())
    }
 }

impl Error for FetcherError {
     
}

impl From<serde_json::Error> for FetcherError {

    fn from(e: serde_json::Error) -> Self {
        Self::ParseError(e.to_string())
    }
}

pub struct Fetcher {
    path: String,
    fetcher_type: FetcherType


}

impl Fetcher {
    pub fn new(path: String, fetcher_type: FetcherType) -> Self {
        Fetcher { path, fetcher_type }
    }

    pub fn fetch(&self) -> Result<Article, FetcherError> {

        match &self.fetcher_type {

            FetcherType::SingleBlocking => {

                let request = reqwest::blocking::get(&self.path);
                let res = serde_json::from_str::<Article>(&request.unwrap().text().unwrap());
        
                match res {
                    Ok(article) => Ok(article),
                    Err(error) => Err(FetcherError::RequestError(error.to_string())),
                }

            },
            FetcherType::Multiple(_count) => {
                unimplemented!()
            }
        }
    

        

    }

}

