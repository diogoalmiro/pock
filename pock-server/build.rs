use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

pub fn main() {
    println!("cargo:warning=Building pock-server");
    println!("cargo:rerun-if-changed=src/entities/");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let mut entities_itens = Vec::<String>::new();
    let mut mount_contents = Vec::<String>::new();
    fs::read_dir("src/entities/")
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .for_each(|entity| {
            let functions = public_funcs(&entity);
            mount_contents.push(format!(
                ".mount(\"/{}/\", routes![{}])",
                entity,
                functions.join(", ")
            ));
            entities_itens.push(format!(
                r#"pub mod {} {{ #[path = "{}/src/entities/{}/routes.rs"] pub mod routes; }}"#,
                entity,
                env!("CARGO_MANIFEST_DIR"),
                entity.to_string()
            ));
        });

    let generated = format!(
        r#"use rocket::{{Rocket, Build}};
use std::env;

mod entities {{
    {}
}}
pub fn setup_routes(builder: Rocket<Build>) -> Rocket<Build> {{
    builder
        {}
}}"#,
        entities_itens.join("\n\t"),
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
        .map(|m| {
            format!(
                "entities::{}::routes::{}",
                entity,
                m.get(1).unwrap().as_str()
            )
        })
        .collect()
}
