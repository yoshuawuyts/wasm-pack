extern crate failure;
#[macro_use]
extern crate quicli;
extern crate wasm_pack;

use wasm_pack::{bindgen, build, manifest, readme};

use quicli::prelude::*;

/// 📦 ✨  pack and publish your wasm!
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
    /// Log all the things!
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "init")]
    /// Initialize a package.json based on your compiled wasm
    Init { path: Option<String> },
    #[structopt(name = "pack")]
    /// Create a tar of your npm package but don't publish! [NOT IMPLEMENTED]
    Pack {},
    #[structopt(name = "publish")]
    /// Pack up your npm package and publish! [NOT IMPLEMENTED]
    Publish {},
}

main!(|args: Cli, log_level: verbosity| match args.cmd {
    Command::Init { path } => {
        let crate_path = match path {
            Some(p) => p,
            None => ".".to_string(),
        };
        build::rustup_add_wasm_target();
        build::cargo_build_wasm(&crate_path);
        wasm_pack::create_pkg_dir(&crate_path)?;
        manifest::write_package_json(&crate_path)?;
        readme::copy_from_crate(&crate_path)?;
        bindgen::cargo_install_wasm_bindgen();
        let name = manifest::get_crate_name(&crate_path)?;
        bindgen::wasm_bindgen_build(&crate_path, &name);
    }
    Command::Pack { .. } => {
        println!("🙅‍♀️  whoops! this is not implemented yet! sorry!");
        //println!("🎒  packed up your package!");
    }
    Command::Publish { .. } => {
        println!("🙅‍♀️  whoops! this is not implemented yet! sorry!");
        //println!("💥  published your package!");
    }
});
