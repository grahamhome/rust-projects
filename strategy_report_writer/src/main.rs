// Report writer that uses the Strategy pattern to output the same data in a variety of formats.
use std::collections::HashMap;

// newtype pattern
type Data = HashMap<String, i32>;

// Struct interface using the Data type as a function parameter type
trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

struct Report;

impl Report {
    // Static method. Uses the formatting strategy of the Formatter-compliant object passed to it.
    fn generate<T: Formatter>(formatter: T, some_string: &mut String) {
        // Create some data
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        // generate report
        formatter.format(&data, some_string);
    }
}

struct Text;

impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{k}: {v}\n");
            buf.push_str(&entry);
        }
    }
}

struct Json;

impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('{');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{k}":"{v}"}}"#);
            buf.push_str(&entry);
            buf.push(',');
        }
        buf.pop();
        buf.push('}');
    }
}

fn main() {
    let mut result_buffer = String::from("");
    Report::generate(Text, &mut result_buffer);
    assert_eq!(result_buffer, "one: 1\ntwo: 2\n");

    result_buffer.clear();
    Report::generate(Json, &mut result_buffer);
    assert_eq!(result_buffer, r#"{{"one":"1"},{"two":"2"}}"#)
}
