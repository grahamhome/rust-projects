// Implementation of the Command pattern using a Trait object.
// This approach may be preferred if the commands are structs with data and functions and cannot be expressed as simple
// functions or closures.

// Trait - to be used as an interface for commandsS
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

// A struct implementing the trait
pub struct CreateTable;

impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }

    fn rollback(&self) -> &str {
        "drop table"
    }
}

// Another struct implementing the trait
pub struct AddField;

impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }

    fn rollback(&self) -> &str {
        "remove field"
    }
}

// Struct using the Trait object
struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    // Box is used to allow Migration to function as an interface
    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd)
    }

    // execute() and rollback() call methods of the Trait/interface
    fn execute(&self) -> Vec<&str> {
        // We don't need to specify the type to .collect() into since it can be inferred from the method signature :)
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.commands.iter().rev().map(|cmd| cmd.rollback()).collect()
    }
}

pub fn run() {
    let mut schema = Schema::new();
    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}