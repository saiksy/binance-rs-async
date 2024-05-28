#[cfg(feature = "futures_api")]
#[macro_use]
extern crate tracing;

use env_logger::Builder;

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

    let market: FuturesMarket =
        Binance::new_with_config(None, None, &Config::default().set_proxy("127.0.0.1:7890".to_string()));
    match market.get_asset_index().await {
        Ok(answer) => info!("Asset Index: {:?}", answer),
        Err(e) => error!("Error: {:?}", e),
    }
}
