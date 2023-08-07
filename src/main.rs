use regex::Regex;
use std::env;
use std::process::Command;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    wrap_git_args(&mut args);

    disable_commit_signing(&mut args);

    let args: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    cmd("wsl.exe", args)
}

fn wrap_git_args(args: &mut Vec<String>) {
    args.remove(0);
    args.insert(0, "git".to_string());
}

fn disable_commit_signing(args: &mut Vec<String>) {
    let mut i = 0;
    for arg in args.iter() {
        if arg == "-c" {
            args.insert(i + 1, "commit.gpgsign=false".to_string());
            args.insert(i + 2, "-c".to_string());
            return;
        }
        i += 1;
    }
}

fn cmd(program: &str, args: Vec<&str>) {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    let output = command.output().expect("Failed to execute process!");
    let status = command.status().expect("Failed to execute process!");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !status.success() {
        eprint!("{}", translate_paths(stderr));
        std::process::exit(status.code().unwrap_or(1));
    }

    print!("{}", translate_paths(stdout));
    std::process::exit(0);
}

fn translate_paths(input: String) -> String {
    let re = Regex::new(r"/(.)/(.+?)[\n\s]").unwrap();

    let Some(found) = re.captures(&input) else { return input };
    let translated = found.get(2).unwrap().as_str().replace("/", "\\");
    let drive = found.get(1).unwrap().as_str();

    re.replace_all(&input, format!("{}:\\{}\n", drive, translated))
        .to_string()
}
