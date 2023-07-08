/*
CLIPLY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the main
/// Cliply API struct.
use cliply::App;

/// Importing the error
/// struct to handle any
/// errors.
use cliply::CliplyError;

/// Main point of
/// entry for the 
/// Rust compiler.
pub fn main() -> () {

    // Instantiating the "App" struct with the required
    // data.
    let mut my_app: App = App::new(
        &"Test App",
        &"1.0.0",
        &"Alexander Abraham"
    );

    // Adding a greeting without data. Note the use of "false".
    my_app.add_arg(
        &"greet", 
        &" generic greeting for the user", 
        &"false"
    );

    // Adding a greeting with data. Note the use of "true".
    my_app.add_arg(
        &"cgreet", 
        &"custom greeting for the user", 
        &"true"
    );

    // Was the version flag used?
    if my_app.version_is() == true {
        println!("{}", my_app.version_info());
    }

    // Was the help flag used?
    else if my_app.help_is() == true {
        println!("{}", my_app.help_info());
    }

    // Was the "greet" flag used?
    else if my_app.arg_was_used(&"greet") == true {
        println!("Hello World!");
    }

    // Was the "cgreet" flag used? Note the use of the result!
    else if my_app.arg_was_used(&"cgreet") == true {
        let arg_data: Result<String, CliplyError> = my_app.get_arg_data(&"cgreet");
        match arg_data {
            Ok(x) => {
                println!("Hello, {}!", x);
            },
            Err(e) => {
                println!("{}", e.to_string());
            }
        }
    }

    // If user-supplied flags are invalid, show
    // them the app's help message.
    else {
        println!("{}", my_app.help_info());
    }
    
}
