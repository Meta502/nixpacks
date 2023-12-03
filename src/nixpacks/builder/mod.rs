use std::process::Output;

use super::{environment::Environment, plan::BuildPlan};
use anyhow::Result;

pub mod docker;

/// Types that impl this trait can produce Docker images.
pub trait ImageBuilder {
    fn create_image(
        &self,
        app_source: &str,
        plan: &BuildPlan,
        env: &Environment,
    ) -> Result<Output>;
}
