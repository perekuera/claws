use std::{env, process::exit};

use codecommit::codecommit::{clone_repository, list_repositories};


pub mod codecommit;
pub mod codebuild;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_help();
        exit(1);
    }

    let command = args[1].to_lowercase();

    match command.as_str() {
        "codecommit" | "cc" | "-c" => {
            if args.len() < 3 {
                show_help();
                exit(1);
            }

            let subcommand = args[2].to_lowercase();
            match subcommand.as_str() {
                "list" | "l" | "-l" => {
                    let query = if args.len() > 3 {
                        format!("repositories[?contains(repositoryName, '{}')]", &args[3])
                    } else {
                        String::from("repositories[]")
                    };
                    list_repositories(&query);
                }
                "clone" | "c" | "-c" => {
                    let name = if args.len() > 3 {
                        args[3].clone()
                    } else {
                        println!("Required repository name");
                        exit(1);
                    };
                    clone_repository(&name);
                }
                _ => {
                    println!("Invalid subcommand: {}", subcommand);
                    exit(1);
                }
            }
        }
        "codebuild" | "cb" | "-b" => {
            println!("Not implemented yet!");
            exit(0);
        }
        "help" | "h" => {
            show_help();
        }
        _ => {
            println!("Invalid command: {}", command);
            exit(1);
        }
    }
}

fn show_help() {
    println!(
        r#"
Usage: 
    waws COMMAND [SUBCOMMAND] [OPTIONS|ARGUMENTS]

Commands/subcommands:
    codecommit             cc  -c      CodeCommit operations
    
        list [QUERY]       l   -l      List repositories
        clone REPO_NAME    c   -c      Clone repository

    codebuild              cb  -b      CodeBuild operations

    help                   h   -h      This help
"#
    );
}