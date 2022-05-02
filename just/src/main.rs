use clap::Parser;
use toml::Value;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::path::Path;

fn read_config_toml(config_file: &str) -> std::io::Result<Value> {
    let content = std::fs::read_to_string(config_file)?;
    // Ok(toml::from_str(&content)?)
    Ok(content.parse::<Value>().unwrap())
    // Err(content.parse::<Value>().unwrap())
}
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    /// The pattern to look for
    #[clap(short, long)]
    pattern: String,
    /// The path to the file to read
    // #[clap(short, long)]
    path: String
    // path: std::path::PathBuf
}

// https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process
fn run_command<P: AsRef<Path>>(binary: P, args: Vec<&str>) {
    let mut cmd = Command::new(binary.as_ref())
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            // println!("Read: {:?}", line);
            println!("{}", line.unwrap());
        }
    }

    cmd.wait().unwrap();
}

#[test]
fn test_long_running_process() {
    // run_command("black", vec!("../just","--check"));
    // run_command("bash", vec!("../long_running_script.sh"));
    run_command("printenv", vec!());
}

fn main() {
    // println!("Hello, world!");
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    for line in content.lines() {
        // if line.contains(&args.pattern) {
            println!("{}", line);
        // }
    }
    
    let default_config_file = "./.just";
    let config = read_config_toml(default_config_file).expect("Could not parse config file");
    let command_string = config["lint"]["black"]["command"].as_str().unwrap();

    let command_vec: Vec<&str> = command_string.split_whitespace().collect();

    // println!("{:?}", &command_vec[1..]);

    run_command(command_vec[0], command_vec[1..].to_vec());
    // run_command("bash", vec!("../long_running_script.sh"));
}