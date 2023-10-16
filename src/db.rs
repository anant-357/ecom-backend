
#[derive(Clone)]
pub struct Db {
    db: mongodb::Client,
}

impl Db{
    pub async fn with_uri(uri: &str) -> actix_web::Result<Self> {
        let mongo =  mongodb::Client::with_uri_str(uri).await.unwrap();
        Ok(Self { db: mongo})
    }

    pub fn get_db(&self) -> &mongodb::Client {
        return &self.db
    }
}

