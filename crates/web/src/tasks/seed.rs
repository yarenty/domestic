//! This task implements data seeding functionality for initializing new
//! development/demo environments.
//!
//! # Example
//!
//! Run the task with the following command:
//! ```sh
//! cargo run task
//! ```
//!
//! To override existing data and reset the data structure, use the following
//! command with the `refresh:true` argument:
//! ```sh
//! cargo run task seed_data refresh:true
//! ```

use std::path::Path;
use async_trait::async_trait;
use loco_rs::{
    app::AppContext,
    task::{Task, TaskInfo, Vars},
    db,
    Result,
};

use crate::app::App;

pub struct SeedData;

#[async_trait]
impl Task for SeedData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed:data".to_string(),
            detail: "Task for seeding data".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _vars: &Vars) -> Result<()> {
        let path = Path::new("fixtures/development");
        db::run_app_seed::<App>(ctx, path).await?;
        Ok(())
    }
}
