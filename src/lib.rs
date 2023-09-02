pub mod model;



#[cfg(test)]
mod tests {
    use crate::model::{
        client,
        json::Base
    };

    #[tokio::test]
    async fn it_works() {
        let client = client::Client::default(String::from("801671759"));
        let user: Base = client.get_user().await.unwrap();

        dbg!(&user.characters.get(0).unwrap().skills.get(0).unwrap().element);
    }
}