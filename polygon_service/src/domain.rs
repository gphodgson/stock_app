
pub struct TickerData{
    ticker:String,
    query_count:i32,
    results_count:i32,
    adjusted:bool,
    results:TickerDataResult,
    status: String,
    request_id: String,
    count: i32
}

struct TickerDataResult{
    v:i32,
    vw:f32,
    o:f32,
    c:f32,
    h:f32,
    l:f32,
    t:f32,
    n:i32
}