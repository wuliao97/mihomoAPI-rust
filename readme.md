<div align="center">
    <h1>Mihomo API with Rust</h1>
    <p align="right">version. 0.1.0</p>
</div>


### Sample Code

`Cargo.tomi`

```
mihomo_api = { git = "https://github.com/wuliao97/mihomoAPI-rust" }
tokio = {version= "1.29.1", features=["full"]}
```

`main.rs`

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

    dbg!(result.player);
}
```

### Output
```
[src\main.rs:14] result.player = Player {       
    uid: "801671759",
    nickname: "Ennui",
    level: 69,
    world_level: 6,
    friend_count: 37,
    avatar: Avatar {
        id: "200101",
        name: "三月なのか・歓迎",
        icon: "icon/avatar/200101.png",
    },
    signature: "入って！ discord.gg/starrailjp",
    is_display: Bool(true),
    space_info: SpaceInfo {
        challenge_data: ChallengeData {
            maze_group_id: 10,
            maze_group_index: 21,
            pre_maze_group_index: 115,
        },
        pass_area_progress: 7,
        light_cone_count: 76,
        avatar_count: 30,
        achievement_count: 245,
    },
}

```