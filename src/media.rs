use core::fmt;

#[derive(Clone)]
pub struct Media {
    pub title: String,
    pub url: String,
    pub poster_url: String,
}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title - {} ; Link - {} ; Poster Url - {}",
            self.title, self.url, self.poster_url
        )
    }
}
