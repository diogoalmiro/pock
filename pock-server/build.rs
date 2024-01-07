use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

pub fn main() {
    println!("cargo:rerun-if-changed=src/entities/");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let mut entities_itens = Vec::<String>::new();
    let mut mount_contents = Vec::<String>::new();
    fs::read_dir("src/entities/")
        .unwrap()
        .filter_map(|entry| {
            let file_name = entry.unwrap().file_name().into_string().unwrap();
            Path::new(
                format!(
                    "{0}/src/entities/{1}/routes.rs",
                    env!("CARGO_MANIFEST_DIR"),
                    file_name
                )
                .as_str(),
            )
            .exists()
            .then_some(file_name)
        })
        .for_each(|entity| {
            let functions = public_funcs(&entity);
            mount_contents.push(format!(
                ".mount(\"/{}/\", routes![{}])",
                entity,
                functions.join(", ")
            ));
            entities_itens.push(format!(
                r#"#[path = "{0}/src/entities/{1}/routes.rs"]
pub mod {1};
"#,
                env!("CARGO_MANIFEST_DIR"),
                entity
            ));
        });

    let generated = format!(
        r#"use rocket::{{Rocket, Build}};
use std::env;

{}

pub fn setup_routes(builder: Rocket<Build>) -> Rocket<Build> {{
    builder
        {}
}}"#,
        entities_itens.join("\n"),
        mount_contents.join("\n\t\t")
    );

    fs::write(dest_path, generated).unwrap();
}

fn public_funcs(entity: &str) -> Vec<String> {
    let function_name_regex = Regex::new(r"pub fn (\w+)").unwrap();
    function_name_regex
        .captures_iter(
            fs::read_to_string(format!("src/entities/{}/routes.rs", entity))
                .unwrap_or(String::new())
                .as_str(),
        )
        .map(|m| format!("{}::{}", entity, m.get(1).unwrap().as_str()))
        .collect()
}
