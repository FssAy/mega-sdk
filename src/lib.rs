pub mod api_async;
// pub mod api;
pub mod binds;

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::time::Duration;
    use crate::api_async::Api;
    use crate::api_async::error::Error;
    use crate::api_async::nodes::Node;

    #[tokio::test]
    async fn login() {}
}
