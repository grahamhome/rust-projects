// Implementation of the Command pattern using function pointers and closures
// This approach may be preferred if the commands are small and may be defined as simple functions or closures
// because it does not use dynamic dispatch and so has faster performance.

// A custom type
type FnPtr = fn() -> String;

// Struct defining the interface for a command
struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

struct Schema {
    commands: Vec<Command>
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, execute: FnPtr, rollback:FnPtr) {
        self.commands.push(Command { execute, rollback })
    }
    fn execute(&self) -> Vec<String> {
        // Function pointers must be invoked using parentheses(!)
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }
    fn rollback(&self) -> Vec<String> {
        self.commands.iter().rev().map(|cmd| (cmd.rollback)()).collect()
    }
}

fn create_table() -> String {
    "create table".to_string()
}

fn drop_table() -> String {
    "drop table".to_string()
}

pub fn run() -> () {
    let mut schema = Schema::new();
    // Creating an instance of the interface using function pointers
    schema.add_migration(create_table, drop_table);
    // Creating an instance of the interface using closures
    schema.add_migration(|| "add field".to_string(), || "remove field".to_string());

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}