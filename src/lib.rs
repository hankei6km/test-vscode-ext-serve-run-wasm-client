use serde_json::de::IoRead;
use serde_json::{StreamDeserializer, Value};
use std::io::{BufReader, BufWriter, Read, Write};

fn output<S: Read, T: Write, U: Write>(
    stream: StreamDeserializer<'_, IoRead<BufReader<S>>, Value>,
    mut writer_out: BufWriter<T>,
    mut writer_err: BufWriter<U>,
) {
    for value in stream {
        let value = value.unwrap();
        if value["id"].as_i64().unwrap() % 2 == 0 {
            // print value with traiing "lf"  by using writerOut
            writer_out
                .write_all(format!("{}\n", value.to_string()).as_bytes())
                .unwrap();
        } else {
            writer_err
                .write_all(format!("{}\n", value.to_string()).as_bytes())
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        use std::io::{BufReader, Cursor};

        let input = Cursor::new(
            r#"{"id": 1, "name": "Alice"}
    {"id": 2, "name": "Bob"}
    {"id": 3, "name": "Charlie"}
    {"id": 4, "name": "Dave"}
    {"id": 5, "name": "Eve"}"#,
        );
        let reader = BufReader::new(input);
        let stream = serde_json::Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        output(
            stream,
            std::io::BufWriter::new(&mut stdout),
            std::io::BufWriter::new(&mut stderr),
        );

        assert_eq!(
            stdout,
            b"{\"id\":2,\"name\":\"Bob\"}\n{\"id\":4,\"name\":\"Dave\"}\n"
        );
        assert_eq!(
            stderr,
            b"{\"id\":1,\"name\":\"Alice\"}\n{\"id\":3,\"name\":\"Charlie\"}\n{\"id\":5,\"name\":\"Eve\"}\n"
        );
    }
}

pub mod run {
    use serde_json::{Deserializer, Value};
    use std::io::{BufReader, BufWriter};

    use crate::output;

    pub struct RunArgs {
        pub memory_initial: u32,
        pub memory_maximum: u32,
        pub memory_shared: bool,
        pub files: Vec<String>,
    }
    pub struct Run {
        memory_initial: u32,
        memory_maximum: u32,
        memory_shared: bool,
        files: Vec<String>,
    }

    impl Run {
        pub fn new(args: RunArgs) -> Self {
            Self {
                memory_initial: args.memory_initial,
                memory_maximum: args.memory_maximum,
                memory_shared: args.memory_shared,
                files: args.files,
            }
        }
        pub fn run(&self) {
            // let memory_initial = self.memory_initial;
            // let files = &self.files;
            // println!("memory_initial: {memory_initial:?}");
            // println!("files: {files:?}")
            let response = reqwest::blocking::get("http://httpbin.org/stream/10").unwrap();

            println!("Status: {}", response.status());

            let reader = BufReader::new(response);
            let stream = Deserializer::from_reader(reader).into_iter::<Value>();
            output(
                stream,
                BufWriter::new(std::io::stdout()),
                BufWriter::new(std::io::stderr()),
            );
            //for value in stream {
            //    println!("-{}", value.unwrap());
            //}
        }
    }
}
