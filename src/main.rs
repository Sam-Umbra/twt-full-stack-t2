use spring::{App, auto_config};
use spring_sea_orm::SeaOrmPlugin;
use spring_web::{WebPlugin, WebConfigurator};

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .run()
        .await
}
