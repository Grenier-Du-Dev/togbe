use url::Url;


#[derive(Clone, Debug)]
pub struct Link {
    pub url: Url,
    pub depth: usize,
}