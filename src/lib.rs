use run::RunArgs;
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
                .write_all(format!("{}\n", value["data"]).as_bytes())
                .unwrap();
        } else {
            writer_err
                .write_all(format!("{}\n", value["data"]).as_bytes())
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
            r#"{"id": 1, "data": "Alice"}
    {"id": 2, "data": "Bob"}
    {"id": 3, "data": "Charlie"}
    {"id": 4, "data": "Dave"}
    {"id": 5, "data": "Eve"}"#,
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
            b"\"Bob\"\n\"Dave\"\n"
        );
        assert_eq!(
            stderr,
            b"\"Alice\"\n\"Charlie\"\n\"Eve\"\n"
        );
    }
}

use serde::Serialize;
use url::Url;
#[derive(Serialize)]
struct ArgsArgs {
    args: Vec<String>,
}

fn build_url(run_args: RunArgs) -> Url {
    let mut url = Url::parse("http://localhost:3000/run").unwrap();

    let mut args: Vec<String> = vec![
        "--memory_initial".to_string(),
        run_args.memory_initial.to_string(),
        "--memory_maximum".to_string(),
        run_args.memory_maximum.to_string(),
        "--memory_shared".to_string(),
        run_args.memory_shared.to_string(),
        "--".to_string(),
    ];
    args.extend(run_args.files);
    let args_json_array = serde_json::to_string(&args).unwrap();
    url.query_pairs_mut()
        .append_pair("args", &args_json_array)
        .finish();

    url
}
#[cfg(test)]
mod test_build_url {
    use super::*;

    #[test]
    fn test_build_url() {
        let args = RunArgs {
            memory_initial: 1,
            memory_maximum: 2,
            memory_shared: true,
            files: vec![
                "test1.wasm".to_string(),
                "--foo".to_string(),
                "bar".to_string(),
            ],
        };
        let url = build_url(args);
        assert_eq!(
            url.as_str(),
            "http://localhost:3000/run?args=%5B%22--memory_initial%22%2C%221%22%2C%22--memory_maximum%22%2C%222%22%2C%22--memory_shared%22%2C%22true%22%2C%22--%22%2C%22test1.wasm%22%2C%22--foo%22%2C%22bar%22%5D"
        );
    }
}

pub mod run {
    use serde_json::{Deserializer, Value};
    use std::io::{BufReader, BufWriter};
    use url::Url;

    use crate::{build_url, output};

    pub struct RunArgs {
        pub memory_initial: u32,
        pub memory_maximum: u32,
        pub memory_shared: bool,
        pub files: Vec<String>,
    }
    pub struct Run {
        url: Url,
    }

    impl Run {
        pub fn new(run_args: RunArgs) -> Self {
            Self {
                url: build_url(run_args),
            }
        }
        pub fn run(&self) {
            // let memory_initial = self.memory_initial;
            // let files = &self.files;
            // println!("memory_initial: {memory_initial:?}");
            // println!("files: {files:?}")
            let response = reqwest::blocking::get(self.url.as_str()).unwrap();

            // println!("Status: {}", response.status());

            let reader = BufReader::new(response);
            let stream = Deserializer::from_reader(reader).into_iter::<Value>();
            output(
                stream,
                BufWriter::new(std::io::stdout()),
                BufWriter::new(std::io::stderr()),
            );
        }
    }
}
