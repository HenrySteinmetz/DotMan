use clap::{builder::ValueParser, Arg, Command, ValueHint};

pub fn cli() -> Command {
    Command::new("dotman")
        .about("Your all-in-one dotfile managing solution")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("source")
                .about("Manage your source files")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("add")
                        .about("Adds selected file or directory to your dotfile storage")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .index(1)
                                .value_hint(ValueHint::AnyPath)
                                .value_parser(ValueParser::path_buf()),
                        ),
                )
                .subcommand(
                    Command::new("list")
                        .about("Lists all directories and files currently managed by dotman"),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Removes file or directory from dotman")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .index(1)
                                .value_hint(ValueHint::AnyPath)
                                .value_parser(ValueParser::path_buf()),
                        ),
                )
                .subcommand(
                    Command::new("link")
                        .about("Links managed file to an arbitrary one")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("source_path")
                                .required(true)
                                .index(1)
                                .value_hint(ValueHint::AnyPath)
                                .value_parser(ValueParser::path_buf()),
                        )
                        .arg(
                            Arg::new("destination_path")
                                .required(true)
                                .index(2)
                                .value_hint(ValueHint::AnyPath)
                                .value_parser(ValueParser::path_buf()),
                        ),
                )
                .subcommand(
                    Command::new("unlink")
                        .about("Unlinks managed file from any local files")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .index(1)
                                .value_hint(ValueHint::AnyPath)
                                .value_parser(ValueParser::path_buf()),
                        ),
                ),
        )
        .subcommand(
            Command::new("git")
                .about("Allows for easy git management in the CLI.")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("clone from")
                        .about("Clone a existing git repo from URL.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("url")
                                .required(true)
                                .index(1)
                                .value_hint(ValueHint::Url)
                                .value_parser(ValueParser::string()),
                        ),
                )
                .subcommand(
                    Command::new("init").about("Initialize repo in the DotMan home directory."),
                )
                .subcommand(
                    Command::new("commit")
                        .about("Commit changes with the commit message provided as an argument.")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("commit_message")
                                .required(true)
                                .index(1)
                                .value_hint(ValueHint::Other)
                                .value_parser(ValueParser::string()),
                        ),
                )
                .subcommand(
                    Command::new("set remote-url")
                        .about("Sets the remote url of the git repository")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("url")
                                .required(true)
                                .index(1)
                                .value_hint(ValueHint::Url)
                                .value_parser(ValueParser::string()),
                        ),
                )
                .subcommand(
                    Command::new("push").about("Pushes local commits to the remote repository."),
                )
                .subcommand(
                    Command::new("pull")
                        .about("Pulls remote changes and applies them to local repository."),
                ),
        )
        .subcommand(
            Command::new("set_home")
                .about("Sets the location for dotfiles to be stored e.g. a local git repo")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("path")
                        .required(true)
                        .index(1)
                        .value_hint(ValueHint::AnyPath)
                        .value_parser(ValueParser::path_buf()),
                ),
        )
}
