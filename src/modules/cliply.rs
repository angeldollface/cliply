/*
CLIPLY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the environment API.
use std::env;

/// Importing this for getting
/// vector indexes.
use coutils::get_index;

/// Importing this for checking
/// vector indexes.
use coutils::has_index;

/// Importing this for checking
/// whether items exist in a vector.
use coutils::has_item;

/// Importing the "clean_split"
/// method from the "coutils"
/// module.
use coutils::clean_split;

/// Importing the struct to
/// represent errors.
use super::errors::CliplyError;

/// Importing the map API.
use std::collections::HashMap;

/// A public struct for your
/// awesome CLI app!
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct App{
    pub name: String,
    pub version: String,
    pub author: String,
    pub args: HashMap<String, Vec<String>>
}

/// Implementing all methods for
/// your app that give you the freedom
/// to choose what you'd like to do.
impl App{

    /// Instantiates the "App" struct.
    pub fn new(name: &str, version: &str, author: &str) -> App {
        let args: HashMap<String, Vec<String>> = HashMap::new();
        let instance: App = App {
            name: name.to_string(),
            version: version.to_string(),
            author: author.to_string(),
            args: args
        };
        return instance;
    }

    /// Checks if "-a" or "--arg" was used.
    pub fn arg_was_used(&self, arg: &str) -> bool {
        let mut result: bool = false;
        let args: Vec<String> = env::args().collect();
        let arg_first_letter: &String = &clean_split(&arg.to_string(), &String::from(""))[1];
        let minus_arg: String = format!("-{}", arg_first_letter);
        let minus_minus_arg: String = format!("--{}", arg);
        if args.contains(&minus_arg) || 
           args.contains(&minus_minus_arg)||
           args.contains(&arg.to_string())
        {
            result = true;
        }
        else {}
        return result;
    }

    /// Adds an argument to the argument pool.
    /// If you'd like to accept data for an argument,
    /// set the "data" flag to either "true" or "false".
    pub fn add_arg(
        &mut self, 
        name: &str, 
        help: &str, 
        data: &str,
    ) {
        let mut attribute_vec: Vec<String> = Vec::new();
        attribute_vec.push(help.to_string());
        attribute_vec.push(data.to_string());
        self.args.insert(name.to_string(), attribute_vec);
    }

    /// Retrieves the command line data for an argument.
    pub fn get_arg_data(&self, name: &str) -> Result<String, CliplyError> {
        let args: Vec<String> = env::args().collect();
        let mut result: &String = &String::from("");
        let arg_first_letter: &String = &clean_split(
            &name.to_string(), 
            &String::from("")
        )[1];
        let minus_arg: String = format!("-{}", arg_first_letter);
        let minus_minus_arg: String = format!("--{}", name);
        if &self.args[name][1] == "true" && args.contains(&minus_arg){
            let mut next_pos: usize = 0;
            if has_item(
                &args, 
                &minus_arg
            )
            {
                next_pos = match get_index(&args, &minus_arg){
                    Ok(next_pos) => next_pos + 1,
                    Err(e) => {
                        return Err::<String, CliplyError>(CliplyError::new(&e.to_string()));
                    }
                };
            }
            else {
                let err: String = format!("{} not found in arguments.", &minus_arg);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
            if has_index(&args, &next_pos){
                result = &args[next_pos];
            }
            else {
                let err: String = format!("No data supplied to \"{}\".", &minus_arg);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
        }
        else if &self.args[name][1] == "true" && args.contains(&minus_minus_arg) {
            let mut next_pos: usize = 0;
            if has_item(
                &args, 
                &minus_minus_arg
            )
            {
                next_pos = match get_index(&args, &minus_minus_arg){
                    Ok(next_pos) => next_pos + 1,
                    Err(e) => {
                        return Err::<String, CliplyError>(CliplyError::new(&e.to_string()));
                    }
                };
            }
            else {
                let err: String = format!("{} not found in arguments.", &minus_minus_arg);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
            if has_index(&args, &next_pos){
                result = &args[next_pos];
            }
            else {
                let err: String = format!("No data supplied to \"{}\".", &minus_minus_arg);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
        }
        else if &self.args[name][1] == "true" && args.contains(&name.to_owned()) {
            let mut next_pos: usize = 0;
            if has_item(
                &args, 
                &name.to_string()
            )
            {
                next_pos = match get_index(&args, &name.to_string()){
                    Ok(next_pos) => next_pos + 1,
                    Err(e) => {
                        return Err::<String, CliplyError>(CliplyError::new(&e.to_string()));
                    }
                };
            }
            else {
                let err: String = format!("\"{}\" not found in arguments.", &name);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
            if has_index(&args, &next_pos){
                result = &args[next_pos];
            }
            else {
                let err: String = format!("No data supplied to \"{}\".", &name);
                return Err::<String, CliplyError>(CliplyError::new(&err.to_string()));
            }
        }
        else {}
        Ok(result.to_string())
    }

    /// Returns a boolean to tell you whether version
    /// info was requested or not.
    pub fn version_is(&self) -> bool {
        let mut result: bool = false;
        let args: Vec<String> = env::args().collect();
        let arg_len = args.len();
        if arg_len == 2 {
            if args[1].clone() == "--version" || 
               args[1].clone() == "-v" ||  
               args[1].clone() == "version" 
            {
                result = true;
            }
            else {}
        }
        else {}
        return result;
    }

    /// Returns a boolean to tell you whether help
    /// info was requested or not.
    pub fn help_is(&self) -> bool {
        let mut result: bool = false;
        let args: Vec<String> = env::args().collect();
        let arg_len = args.len();
        if arg_len == 2 {
            if args[1].clone() == "--help" || 
               args[1].clone() == "-h" || 
               args[1].clone() == "help"
            {
                result = true;
            }
            else {}
        }
        else {}
        return result;
    }

    /// Returns a string with version info.
    pub fn version_info(&self) -> String {
        let version_string: String = format!(
            "{} v.{}\nby {}.", 
            &self.name, 
            &self.version, 
            &self.author
        );
        return version_string;
    }
    
    /// Returns a string with help info.
    pub fn help_info(&self) -> String {
        let mut help_string_vec: Vec<String> = Vec::new();
        let help_short: &String = &String::from("h");
        let help_long: &String = &String::from("help");
        let help_help_msg: &String = &String::from("displays this message");
        let version_short: &String = &String::from("v");
        let version_long: &String = &String::from("version");
        let version_help_msg: &String = &String::from("displays app info");
        for (key,value) in &self.args {
            if value[1].clone() == "true" {
                let first_letter: &String = &clean_split(
                    &key, 
                    &String::from("")
                )[1];
                let command_help: String = format!("-{} --{} {} DATA  {}", first_letter, &key, &key, value[0]);
                help_string_vec.push(command_help);
            }
            else {
                let first_letter: &String = &clean_split(
                    &key.to_string(), 
                    &String::from("")
                )[1];
                let command_help: String = format!("-{} --{} {}        {}", first_letter, &key, &key, value[0]);
                help_string_vec.push(command_help);
            }
        }
         help_string_vec.push(
            format!("-{} --{} {}           {}", help_short, &help_long, &help_long, help_help_msg)
        );
        help_string_vec.push(
            format!("-{} --{} {}     {}", version_short, &version_long, &version_long, version_help_msg)
        );
        let help_string = help_string_vec.join("\n");
        return help_string;
    }
}
