use super::{Bybit, ExchangeServer};
use crate::exchange::ExchangeId;

/// [`BybitFuturesUsd`] WebSocket server base url.
///
/// See docs: <https://bybit-exchange.github.io/docs/v5/ws/connect>
pub const WEBSOCKET_BASE_URL_BYBIT_FUTURES_USD: &str = "wss://stream.bybit.com/v5/public/linear";

/// [`Bybit`](super::Bybit) futures exchange.
pub type BybitFuturesUsd = Bybit<BybitServerFuturesUsd>;

/// [`Bybit`](super::Bybit) futures [`ExchangeServer`](super::super::ExchangeServer).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BybitServerFuturesUsd;

impl ExchangeServer for BybitServerFuturesUsd {
    const ID: ExchangeId = ExchangeId::BybitFuturesUsd;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_BYBIT_FUTURES_USD
    }
}
