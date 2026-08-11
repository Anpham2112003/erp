pub trait Request {
    type Output;
}

pub trait RequestHandler<R: Request> {
    fn handle(&self, request: R) -> R::Output;
}
    