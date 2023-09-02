<div align="center">
    <h1>Mihomo API with Rust</h1>
    <p align="right">version. 0.1.0</p>
</div>


<h1 align="left">Requirements</h1>


```
serde = { version = "1.0.182", features = ["derive"] }
serde_json = "1.0.104"
reqwest = "0.11.18"
tokio = {version= "1.29.1", features=["full"]}
```

<h1 align="left">Sample Code</h1>

<p align="left">Cargo.tomi</p>

```
mihomo_api = { git = "https://github.com/wuliao97/mihomoAPI-rust" }
tokio = {version= "1.29.1", features=["full"]}
```

<p align="left">main.rs</p>

```rust
extern crate mihomo_api;

use crate::mihomo_api::model::{
    client::Client,
    json::Base
};

#[tokio::main]
async fn main() {
    let id: String = String::from("801671759");
    let client: Client = Client::default(id);
    let result: Base = client.get_user().await.unwrap();

    println!("{:?}", result);
}
```
