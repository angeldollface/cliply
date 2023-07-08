# CLIPLY

***Making command-line interfaces in Rust easy.***

![GitHub CI](https://github.com/angeldollface/cliply/actions/workflows/rust.yml/badge.svg)

## ABOUT

***Cliply*** is an alterantive to the popular `clap` library for parsing and processing command-line arguments. It is intended to be a little more friendly for people who are only just learning Rust. Enjoy!

## FEATURES

- Blazingly fast.
- Easy to use, no drama.
- Provides multiple options out of the box.
- Provides a `-h` or `--help` flag out of the box.
- Provides a `-v` or `--version` flag out of the box.

## INSTALLATION

To use ***Cliply*** in your Rust project add this line to your project's `Cargo.toml`'s `[dependencies]` section:

```TOML
cliply = "0.1.0"
```

To import the library into your project's code, use this line:

```Rust
use cliply::App;
```

To find out exactly how to use the library please read the section below.

## EXAMPLE

An example of how to use ***Cliply's*** APIs in a sample app can be viewed below:

```Rust
/*
CLIPLY by "Angel Dollface".
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
        &"Angel Dollface"
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
```

If you would like to read detailed documentation, you can do so by visiting [this link](https://docs.rs/cliply/0.1.0).

## CHANGELOG

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE

- *Cliply* by Alexander Abraham a.k.a. *"Angel Dollface"*
- Licensed under the MIT license.
