// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project deployment API endpoints.
//!
//! These endpoints are used for querying deployments.

mod create;
mod delete;
mod deployment;
mod edit;

pub use self::create::CreateDeployment;
pub use self::create::CreateDeploymentBuilder;
pub use self::create::CreateDeploymentBuilderError;

pub use self::delete::DeleteDeployment;
pub use self::delete::DeleteDeploymentBuilder;
pub use self::delete::DeleteDeploymentBuilderError;

pub use self::deployment::Deployment;
pub use self::deployment::DeploymentBuilder;
pub use self::deployment::DeploymentBuilderError;

pub use self::edit::DeploymentStatus;
pub use self::edit::EditDeployment;
pub use self::edit::EditDeploymentBuilder;
pub use self::edit::EditDeploymentBuilderError;
