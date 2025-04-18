use chrono::Utc;
use project_tracker_core::{
    builders::project_builder::ProjectBuilder, factories::{
        person_factory::*, 
        project_factory::*, 
        tag_factory::*, 
        task_factory::*
    }, models::{milestone,person,project,tag,task}, HasId
};
use project_tracker_backend::{
    app, db, dto::project_dto::{ProjectDTO}, errors, handlers, routes, services
};

// use project_tracker_core::{models::{milestone, person, project, tag, task}, HasId};
// use person::Person;
// use project::Project;
// use tag::Tag;
// use task::Task;

#[tokio::main]
async fn main() {
    /* --- What to do --- 
    1. get HTTP response and send to router to match
    2. router sends request to handler based on what it is (GET, POST, DELETE, etc.)
    3. Handles takes care of the parsing and sends the data to the appropriate service
    4. Service manages relation to the repository (db, memory, etc.)
        4.1 here the domain logic is processed to create, validate, update, delete items
    5. the db sends a response to the service, which sends a response to the handler, which encodes and sends an HTTP response to the client
    */

    app::run().await; // will bootstrap the server to start running
}