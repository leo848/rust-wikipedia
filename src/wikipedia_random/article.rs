use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Namespace {
    id: i32,
    text: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Titles {
    canonical: String,
    normalized: String,
    display: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Image {
    source: String,
    width: i32,
    height: i32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Coordinates {
    lat: f32,
    lon: f32,
}


#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Article {
    pub title: String,
    pub extract: String,
    pub displaytitle: String,
    namespace: Namespace,
    wikibase_item: String,
    titles: Titles,
    pageid: i128,
    thumbnail: Image,
    coordinates: Option<Coordinates>,
    lang: String,
    dir: String,
    revision: String,
    tid: String,
    timestamp: String,
    description: String,
    description_source: String,
}