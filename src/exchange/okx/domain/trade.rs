use super::{
    OkxChannel,
    subscription_id,
};
use crate::{
    subscriber::subscription::SubscriptionIdentifier,
    model::{Market, MarketIter, PublicTrade},
    exchange::ExchangeId,
    Identifier

};
use barter_integration::model::{Exchange, Instrument, Side, SubscriptionId};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Terse type alias for an [`Okx`] real-time trades WebSocket message.
pub type OkxTrades = OkxMessage<OkxTrade>;

/// [`Okx`] market data WebSocket message.
///
/// Example Trade:
/// ```json
/// {
///   "arg": {
///     "channel": "trades",
///     "instId": "BTC-USDT"
///   },
///   "data": [
///     {
///       "instId": "BTC-USDT",
///       "tradeId": "130639474",
///       "px": "42219.9",
///       "sz": "0.12060306",
///       "side": "buy",
///       "ts": "1630048897897"
///     }
///   ]
/// }
/// ```
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel>
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct OkxMessage<T> {
    #[serde(rename = "arg", deserialize_with = "de_okx_message_arg_as_subscription_id")]
    pub subscription_id: SubscriptionId,
    pub data: Vec<T>,
}

impl<T> SubscriptionIdentifier for OkxMessage<T> {
    fn subscription_id(&self) -> SubscriptionId {
        self.subscription_id.clone()
    }
}

/// [`Okx`] real-time trade WebSocket message.
///
/// Example:
/// ```json
/// {
///   "instId": "BTC-USDT",
///   "tradeId": "130639474",
///   "px": "42219.9",
///   "sz": "0.12060306",
///   "side": "buy",
///   "ts": "1630048897897"
/// }
/// ```
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel-trades-channel>
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct OkxTrade {
    #[serde(rename = "tradeId")]
    pub id: String,
    #[serde(rename = "px", deserialize_with = "crate::util::de_str")]
    pub price: f64,
    #[serde(rename = "sz", deserialize_with = "crate::util::de_str")]
    pub amount: f64,
    pub side: Side,
    #[serde(rename = "ts", deserialize_with = "crate::util::de_str_epoch_ms_as_datetime_utc")]
    pub time: DateTime<Utc>,
}

impl Identifier<OkxChannel> for OkxMessage<OkxTrade> {
    fn id() -> OkxChannel {
        OkxChannel::TRADES
    }
}

impl From<(ExchangeId, Instrument, OkxMessage<OkxTrade>)> for MarketIter<PublicTrade> {
    fn from((exchange_id, instrument, message): (ExchangeId, Instrument, OkxMessage<OkxTrade>)) -> Self {
        message
            .data
            .into_iter()
            .map(|trade| {
                Ok(Market {
                    exchange_time: trade.time,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    event: PublicTrade {
                        id: trade.id,
                        price: trade.price,
                        amount: trade.amount,
                        side: trade.side
                    }
                })
            })
            .collect()
    }
}

/// Deserialize an [`OkxMessage`] "arg" field as a Barter [`SubscriptionId`].
fn de_okx_message_arg_as_subscription_id<'de, D>(deserializer: D) -> Result<SubscriptionId, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Arg<'a> {
        channel: &'a str,
        inst_id: &'a str,
    }

    Deserialize::deserialize(deserializer)
        .map(|arg: Arg<'_>| subscription_id(arg.channel, arg.inst_id))
}