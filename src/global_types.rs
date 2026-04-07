use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Source{
    Anime,
    Movies,
    Tv
}

impl Source {
    pub fn as_str(&self) -> &'static str { 
        match self {
            Source::Anime => "anime",
            Source::Movies => "movies",
            Source::Tv => "tv",
        }
    }
    
    pub fn to_string(&self) -> String { 
        match self {
            Source::Anime => "anime".to_string(),
            Source::Movies => "movies".to_string(),
            Source::Tv => "tv".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Option<Source> {
        match s {
            "anime" => Some(Source::Anime),
            "movies" => Some(Source::Movies),
            "tv" => Some(Source::Tv),
            _ => None,
        }
    }
    
}