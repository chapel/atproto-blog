use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};
use serde::*;

use super::state::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct Records {
    pub records: Vec<Record>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Record {
    pub uri: String,
    pub cid: String,
    pub value: RecordValue,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RecordValue {
    #[serde(rename = "$type")]
    pub r#type: String,
    pub title: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub visibility: String,
}

#[server]
#[worker::send]
pub async fn get_posts() -> Result<Records, ServerFnError> {
    web_sys::console::log_1(&"get_posts start".into());

    let state = expect_context::<AppState>();
    web_sys::console::log_1(&"expect_context end".into());
    let AppState { fetch_proxy, .. } = state;
    web_sys::console::log_1(&"AppState end".into());

    let resp = fetch_proxy.send_with_url("https://shiitake.us-east.host.bsky.network/xrpc/com.atproto.repo.listRecords?repo=did:plc:zqduigrgzgbxnhhnwujnpzo2&collection=com.whtwnd.blog.entry").await.expect("get posts failure");

    web_sys::console::log_1(&"fetch end".into());

    let body = resp.body();
    web_sys::console::log_1(&format!("{:?}", body).into());

    let records: Records = Records { records: vec![] };
    web_sys::console::log_1(&"get_posts end".into());
    Ok(records)
}
