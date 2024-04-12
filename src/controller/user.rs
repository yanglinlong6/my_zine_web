use zino::{Request, Response, Result};

pub async fn new(req: Request) -> Result {
    let res = Response::default().context(&req);
    Ok(res.into())
}
