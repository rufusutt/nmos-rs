use std::{env, fs, path::Path, process::Command};

use convert_case::{Case, Casing};
use serde_json::{Map, Value};

fn file_name_to_key(file_path: &str) -> String {
    let file_name = file_path.split('/').last().unwrap();
    let file_stem = file_name.split_once(".json").unwrap().0;

    // Fix: Some IS-05 v1.0.x schemas have the prefix "v1.0-" or "v1.0_"
    let file_stem = file_stem.replace("v1.0-", "");
    let file_stem = file_stem.replace("v1.0_", "");

    // Definition key is file name without ".json" in camel case
    file_stem.to_case(Case::Camel)
}

fn crawl(value: &mut Value, definitions: &mut Map<String, Value>, current_path: &Path) {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        Value::Array(array) => array
            .iter_mut()
            .for_each(|v| crawl(v, definitions, current_path)),
        Value::Object(object) => object.iter_mut().for_each(|(k, v)| {
            if k == "$ref" {
                // Reference must be a string
                let ref_path = v.as_str().expect("Ref is not a string");

                // Update relative paths within a file
                // I.e. #/definitions/...
                if ref_path.starts_with('#') {
                    let current_path = current_path.to_str().unwrap();

                    let definitions_key = file_name_to_key(current_path);
                    let ref_rel_path = ref_path.split_once('#').unwrap().1;

                    let definition_path =
                        format!("#/definitions/{}{}", definitions_key, ref_rel_path);
                    *v = Value::String(definition_path);

                    return;
                }

                // Check if the reference is a path inside another file
                // I.e. contraint-schema.json#/definitions/...
                let (ref_file_path, ref_rel_path) =
                    if let Some((ref_file_path, ref_rel_path)) = ref_path.split_once('#') {
                        (ref_file_path, ref_rel_path)
                    } else {
                        (ref_path, "")
                    };

                // Turn referenced file path into definitions key
                let definitions_key = file_name_to_key(ref_file_path);

                // Construct path to current dir from current_path
                let mut current_dir = current_path.to_path_buf();
                current_dir.pop();

                // Update referenced file path to be relative to current dir
                current_dir.push(ref_file_path);
                let ref_file_path = current_dir;

                // Replace referenced path
                let definition_path = format!("#/definitions/{}{}", definitions_key, ref_rel_path);
                *v = Value::String(definition_path);

                if !definitions.contains_key(&definitions_key) {
                    // Open ref schema
                    let ref_file = fs::File::open(&ref_file_path).unwrap_or_else(|_| {
                        panic!("Referenced schema does not exist: {:?}", &ref_file_path)
                    });
                    let mut ref_schema = serde_json::from_reader::<_, Value>(&ref_file).unwrap();

                    // Crawl file
                    crawl(&mut ref_schema, definitions, &ref_file_path);

                    // Add file to definitions
                    definitions.insert(definitions_key, ref_schema);
                }
            } else {
                crawl(v, definitions, current_path);
            }
        }),
    }
}

fn flatten_schema(root_schema: &mut Value) {
    // Create new object for definitions
    let mut definitions = Map::new();

    crawl(root_schema, &mut definitions, Path::new(""));

    // Get root as object
    let root_object = root_schema.as_object_mut().expect("Root is not an object");

    let root_definitions = root_object
        .entry("definitions")
        .or_insert(Value::Object(Map::new()))
        .as_object_mut()
        .expect("Definitions not an object");

    // Add new definitions to root schema
    *root_definitions = definitions;
}

fn create_root_schema<P: AsRef<Path>>(path: P) -> Value {
    let read_dir = fs::read_dir(path).expect("Cannot read schemas dir");

    let definitions = read_dir
        .flat_map(|entry| -> Result<(String, Value), std::io::Error> {
            let entry = entry?;

            let value = Value::Object(Map::from_iter([(
                String::from("$ref"),
                Value::String(String::from(entry.path().to_str().unwrap())),
            )]));

            let definitions_key = file_name_to_key(entry.path().to_str().unwrap());

            Ok((definitions_key, value))
        })
        .fold(Map::new(), |mut m, (k, v)| {
            m.insert(k, v);
            m
        });

    let mut root_schema = Value::Object(Map::from_iter([
        (
            String::from("$schema"),
            Value::String(String::from("http://json-schema.org/draft-04/schema#")),
        ),
        (String::from("definitions"), Value::Object(definitions)),
    ]));

    flatten_schema(&mut root_schema);

    root_schema
}

fn main() {
    // Fetch schemas from AMWA repository via submodules
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .expect("Failed to update submodules");

    // Get manifest dir from env
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Create schema output dirs
    fs::create_dir_all(manifest_dir.clone() + "/schemas/is_04")
        .expect("Could not create schemas output dir");
    fs::create_dir_all(manifest_dir.clone() + "/schemas/is_05")
        .expect("Could not create schemas output dir");

    // Create flattened schemas
    let is_04_v1_0_x = create_root_schema("vendor/is_04/v1_0_x/APIs/schemas");
    let is_04_v1_1_x = create_root_schema("vendor/is_04/v1_1_x/APIs/schemas");
    let is_04_v1_2_x = create_root_schema("vendor/is_04/v1_2_x/APIs/schemas");
    let is_04_v1_3_x = create_root_schema("vendor/is_04/v1_3_x/APIs/schemas");
    let is_05_v1_0_x = create_root_schema("vendor/is_05/v1_0_x/APIs/schemas");
    let is_05_v1_1_x = create_root_schema("vendor/is_05/v1_1_x/APIs/schemas");

    let write_schema = |schema: Value, path: &str| {
        fs::write(
            manifest_dir.clone() + "/" + path,
            serde_json::to_string(&schema).expect("Could not serialise schema"),
        )
        .expect("Could not write schema");
    };

    // Write to schemas dir
    write_schema(is_04_v1_0_x, "schemas/is_04/v1_0_x.json");
    write_schema(is_04_v1_1_x, "schemas/is_04/v1_1_x.json");
    write_schema(is_04_v1_2_x, "schemas/is_04/v1_2_x.json");
    write_schema(is_04_v1_3_x, "schemas/is_04/v1_3_x.json");
    write_schema(is_05_v1_0_x, "schemas/is_05/v1_0_x.json");
    write_schema(is_05_v1_1_x, "schemas/is_05/v1_1_x.json");
}
