pub mod run {
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
            let memory_initial = self.memory_initial;
            let files = &self.files;
            println!("memory_initial: {memory_initial:?}");
            println!("files: {files:?}")
        }
    }
}
