
use chrono::*;

#[repr(u32)]
pub enum MsgType {
    Ask,
    Bid,
    Trade,
}

pub struct msg_header {
    type: MsgType,
    timestamp: DateTime<Local>,
}

struct tick_payload {
    exchangeTime: DataTime<Local>,
    providerId: u32,
    instrumentId: u32,
    price: f64,
    size: f64,
}

pub struct Ask {
    head: msg_header,
    payload: tick_payload,
}

pub struct Bid {
    head: msg_header,
    payload: tick_payload,
}

pub struct Trade {
    head: msg_header,
    payload: tick_payload,
}


/*pub enum Message {
    None,
    Ask {
        head: msg_header,
        payload: tick_payload,
    },
    Bid {
        head: msg_header,
        payload: tick_payload,
    },
    Trade {
        head: msg_header,
        payload: tick_payload,
    },
}*/


