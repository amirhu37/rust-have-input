

#[macro_export]
macro_rules! print_without_newline {
    ( $($arg:tt)* ) => {{
        print!($($arg)*);
    }};
}


#[macro_export]
macro_rules! input {
    ( $input_type:ty ) => {
        input!($input_type, "");
    };

    ( ) => {
        input!(String, "");
    };

    (  $msg:expr ) => {
        input!(String, $msg);
    };

    ( $input_type:ty, $msg:expr ) => {{
        use std::io::{self, Write};
        print!("{}", $msg  );
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        let trimmed_input = user_input.trim().to_string();
        match trimmed_input.parse::<$input_type>() {
            Ok(r) => r,
            Err(error) => panic!("Cannot parse: {:?}", error),
        }
    }};
}


