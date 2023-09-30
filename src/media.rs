use core::fmt;

#[derive(Clone)]
pub struct Media {
    pub title: String,
    pub link: String,
}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title - {} ; Link - {}", self.title, self.link)
    }
}
