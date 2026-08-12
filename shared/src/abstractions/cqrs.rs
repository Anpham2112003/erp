use async_trait::async_trait;

pub trait Request {
    type Output;
}


#[async_trait]
pub trait RequestHandler<R: Request>: Send + Sync {
   async  fn handle(&self, request: R) -> R::Output;
}
    