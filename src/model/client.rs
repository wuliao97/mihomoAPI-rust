use crate::model::json::Base;

#[derive(Debug)]
pub struct Client {
    id: String,
    lang: String,
}

impl Client {
    #[must_use]
    #[allow(dead_code)]
    pub fn new(id: String, lang: String) -> Self {
        Self { id, lang, }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn default(id: String) -> Self {
        Self { id, lang: String::from("jp"), }
    }

    fn get_url(&self) -> String {
        let id: String = self.id.clone();
        let lang: String = self.lang.clone();

        format!("https://api.mihomo.me/sr_info_parsed/{}?lang={}", id, lang)
    }

    async fn request(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = self.get_url();
        let client = reqwest::Client::new();

        Ok(client
            .get(url)
            .send()
            .await?
            .text()
            .await
            .unwrap()
        )
    }

    pub async fn get_user(&self) -> serde_json::error::Result<Base> {
        let req: String = self.request().await.unwrap();
        let base: Base = serde_json::from_str(&req).unwrap();
        Ok(base)
    }
}

