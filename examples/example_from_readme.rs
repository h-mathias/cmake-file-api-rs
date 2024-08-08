use cmake_file_api::{objects, query, reply};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_dir = std::path::Path::new(".");
    let build_dir = std::path::Path::new(".");

    // write query for codemodel-v2
    query::Writer::default()
        .request_object::<objects::CodeModelV2>()
        .write_stateless(build_dir)?;

    // run cmake
    assert!(std::process::Command::new("cmake")
        .arg("-S")
        .arg(source_dir)
        .arg("-B")
        .arg(build_dir)
        .status()?
        .success());

    // parse cmake-file-api
    let reader = reply::Reader::from_build_dir(build_dir)?;

    // read and print codemodel-v2
    let codemodel: objects::CodeModelV2 = reader.read_object()?;
    codemodel.configurations.iter().for_each(|config| {
        config.targets.iter().for_each(|target| {
            println!("{}", target.name);
            println!("{:#?}", target.sources);
        });
    });

    Ok(())
}
