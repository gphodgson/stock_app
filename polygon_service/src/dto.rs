use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerData{
    ticker:String,
    query_count:i32,
    results_count:i32,
    adjusted:bool,
    results:Vec<TickerDataResult>,
    status: String,
    /**
    This is required as this is for some reason the only field that is not camelCase. Don't be mad
    at me, be mad at Polygon.
    **/
    #[serde(rename="request_id")]
    request_id: String,
    count: i32
}

#[derive(Serialize, Deserialize)]
struct TickerDataResult{
    v:f32,
    vw:f32,
    o:f32,
    c:f32,
    h:f32,
    l:f32,
    t:f32,
    n:i32
}