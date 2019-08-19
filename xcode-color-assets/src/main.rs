use asset_catalog::write_asset_catalog;
use clap::{App, Arg, SubCommand};
use parser::parse_document_from_file;
use std::path::Path;
use swift_gen::gen_swift;

fn main() {
  let matches = App::new("xcode-color-assets")
    .version("0.2")
    .about("Create Xcode Asset Catalog with colors for light & dark mode.")
    .subcommand(
      SubCommand::with_name("gen-assets")
        .about("generates the Asset Catalog")
        .arg(
          Arg::with_name("output")
            .short("o")
            .help("Sets the output filename (e.g. Colors.xcassets)")
            .value_name("OUTPUT_FILE")
            .required(true),
        )
        .arg(
          Arg::with_name("input")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("force-overwrite")
            .short("f")
            .long("force")
            .help("Overwrite Asset catalog if it already exists"),
        ),
    )
    .subcommand(
      SubCommand::with_name("gen-swift")
        .about("generates Swift code")
        .arg(
          Arg::with_name("output")
            .short("o")
            .help("Sets the output filename (e.g. Colors.swift)")
            .value_name("OUTPUT_FILE")
            .required(true),
        )
        .arg(
          Arg::with_name("input")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required(true)
            .index(1),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    ("gen-assets", Some(m)) => {
      let input_file = m.value_of("input").unwrap();
      let output_path = m.value_of("output").unwrap();

      let doc = match parse_document_from_file(&input_file) {
        Ok(doc) => doc,
        Err(e) => {
          println!("{}", e);
          std::process::exit(0x0100);
        }
      };

      let overwrite_asset_catalog = m.is_present("force-overwrite");

      match write_asset_catalog(&doc, &Path::new(output_path), overwrite_asset_catalog) {
        Err(asset_catalog::Error::CatalogExists(_)) => {
          println!(
            "Asset catalog at {} already exists. Use -f to overwrite it.",
            output_path
          );
          std::process::exit(0x0100);
        }
        Err(e) => {
          println!("{}", e);
          std::process::exit(0x0100);
        }
        Ok(_) => println!("Generated Asset catalog at {}.", output_path),
      }
    }
    ("gen-swift", Some(m)) => {
      let input_file = m.value_of("input").unwrap();
      let output_path = m.value_of("output").unwrap();

      let doc = match parse_document_from_file(&input_file) {
        Ok(doc) => doc,
        Err(e) => {
          println!("{}", e);
          std::process::exit(0x0100);
        }
      };

      match gen_swift(&doc, &Path::new(output_path)) {
        Err(e) => {
          println!("{}", e);
          std::process::exit(0x0100);
        }
        Ok(_) => println!("Generated Swift file at {}.", output_path),
      }
    }
    (&_, _) => {}
  }
}