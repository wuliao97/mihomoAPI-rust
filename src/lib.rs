pub mod model;



#[cfg(test)]
mod tests {
    use crate::model::{
        json::Base,
        client::{
            self,
            get_image
        }
    };

    #[tokio::test]
    async fn it_works() {
        let client = client::Client::default(String::from("801671759"));
        let user: Base = client.get_user().await.unwrap();

        // dbg!(&user.characters.get(0).unwrap().skills.get(0).unwrap().element);
        dbg!(get_image(user.characters.get(0).unwrap().preview.clone()));
    }
}