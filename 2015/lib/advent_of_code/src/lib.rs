pub mod file {
    use std::io::prelude::*;
    use std::fs::File;
    use std::error::Error;
    use std::env;
    use std::path::Path;
    fn get_args() -> String  {
        let args: Vec<String> = env::args().collect();
        args[1].to_string()
    }

    fn load_file(fileinput: String) -> File {
        // Open file in ro mode
        let file = match File::open(Path::new(&fileinput)) {
            Err(why) => panic!("couldn't open exercise file cause: {}", why.description()),
            Ok(file) => file,
        };

        file
    }

    pub fn read() -> String {
        let path_arg = get_args();
        let mut file_handler = load_file(path_arg);
        // Init string var
        let mut s = String::new();
        match file_handler.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read cause: {}", why.description()),
            Ok(s) => s
        };
        s
    }
}
