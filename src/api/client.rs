// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;

use reqwest::blocking::{RequestBuilder, Response};
use reqwest::Method;
use url::Url;

use crate::api::ApiError;

/// A trait representing a client which can communicate with a GitLab instance.
pub trait Client {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    /// Build a REST query from a URL and a given method.
    fn build_rest(&self, method: Method, url: Url) -> RequestBuilder;

    /// Send a REST query.
    fn rest(&self, request: RequestBuilder) -> Result<Response, ApiError<Self::Error>>;
}
