use project_tracker_core::{id::Id, models::project::Project};
use crate::{Result, Error, db::traits::ProjectRepository};
use project_tracker_db::database::Database;
use async_trait::async_trait;

// TODO: Hold a reference to the database client

