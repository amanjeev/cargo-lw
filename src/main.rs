use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::Parser;
use console::pad_str_with;
use console::{Alignment, Term};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    What(Args),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about)]
struct Args {
    workspace_path: Option<PathBuf>,
}

fn main() {
    let CargoCli::What(args) = CargoCli::parse();

    // check if user passed a manifest path
    let workspace_path = match args.workspace_path {
        Some(path) => path,
        None => PathBuf::from("./"),
    };

    if !workspace_path.is_dir() {
        println!(
            "\n⚠️ It seems you have provided a path that is not a directory. Please provide path of directory.\n"
        );
        return (); // do not tell me this is bad. i know it is a crime.
    }

    // main manifest, top level only, which is provided by the `current_dir`.
    // for a workspace, no matter the members or any Cargo.toml of the workspace
    // the result is the same, if it is a workspace.
    let metadata = MetadataCommand::new()
        .current_dir(workspace_path)
        .features(CargoOpt::AllFeatures)
        .verbose(true)
        .exec()
        .expect("Metadata exec command failed.");

    let members_from_root_package = metadata.workspace_packages();
    let current_root = &metadata.workspace_root;

    let term = Term::stdout();

    // print greetings
    let _ = term.write_line("\n🤔 What! Listing your current workspace members\n");

    for package_member in members_from_root_package {
        let name = &package_member.name;
        let version = &package_member.version;
        let member_manifest_path = &package_member
            .manifest_path
            .strip_prefix(current_root) // todo: option for showing full path
            .expect("Could not strip the prefix for relative path");
        let member_initial_data = format!("{} ({})", name, version);
        let padded_member_initial_data =
            pad_str_with(member_initial_data.as_str(), 70, Alignment::Left, None, ' ');
        let member_manifest_path_appended =
            format!("{} {}", padded_member_initial_data, member_manifest_path);

        let output = member_manifest_path_appended;
        term.write_line(&output).unwrap();
    }
}
