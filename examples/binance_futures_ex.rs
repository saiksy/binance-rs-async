#[cfg(feature = "futures_api")]
#[macro_use]
extern crate tracing;

use env_logger::Builder;
use binance::futures::general::FuturesGeneral;

#[tokio::main]
async fn main() {
    Builder::new().parse_default_env().init();
    #[cfg(feature = "futures_api")]
    market_data().await;
}

#[cfg(feature = "futures_api")]
async fn market_data() {
    use binance::api::*;
    use binance::config::Config;
    use binance::futures::market::*;
    use binance::futures::rest_model::*;

    let config = &Config::default().set_proxy("http://127.0.0.1:18809".to_string());
    let general = FuturesGeneral::new_with_config(None, None, config);
    let pong = general.ping().await.unwrap();
    println!("Ping result {pong}");
    let market: FuturesMarket =
        Binance::new_with_config(None, None, config);

    match market.get_asset_index().await {
        Ok(answer) => info!("Asset Index: {:?}", answer),
        Err(e) => error!("Error: {:?}", e),
    }
}