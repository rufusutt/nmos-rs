use std::{env, fs, path::Path, process::Command};

use convert_case::{Case, Casing};
use serde_json::{Map, Value};

fn crawl(value: &mut Value, definitions: &mut Map<String, Value>, root_path: &Path) {
    match value {
        Value::Null => {}
        Value::Bool(_) => {}
        Value::Number(_) => {}
        Value::String(_) => {}
        Value::Array(array) => array
            .iter_mut()
            .for_each(|v| crawl(v, definitions, root_path)),
        Value::Object(object) => object.iter_mut().for_each(|(k, v)| {
            if k == "$ref" {
                let ref_path = v.as_str().expect("Ref is not a string");

                // Ignore relative paths
                if ref_path.starts_with("#") {
                    return;
                }

                // Construct new path
                let mut path_buf = root_path.to_owned();
                path_buf.push(ref_path);

                // Open ref schema
                let ref_file = fs::File::open(&path_buf).unwrap_or_else(|_| {
                    panic!("Referenced schema does not exist: {:?}", &path_buf)
                });
                let mut ref_schema = serde_json::from_reader::<_, Value>(&ref_file).unwrap();

                // Form key
                let definition_key = path_buf.file_stem().unwrap().to_string_lossy();
                let definition_key = definition_key.to_case(Case::Camel);

                // Crawl ref schema for nested references
                path_buf.pop();
                crawl(&mut ref_schema, definitions, &path_buf);

                // Remove schema header key
                if let Some(obj) = ref_schema.as_object_mut() {
                    obj.remove("$schema");
                }

                // Add to definitions
                definitions.insert(definition_key.to_string(), ref_schema);

                // Replace ref path with definition path
                *v = Value::String(format!("#/definitions/{}", definition_key));
            } else {
                crawl(v, definitions, root_path);
            }
        }),
    }
}

fn flatten_schema(root_schema: &mut Value) {
    // Create new object for definitions
    let mut definitions = Map::new();

    crawl(root_schema, &mut definitions, &Path::new(""));

    // Get root as object
    let root_object = root_schema.as_object_mut().expect("Root is not an object");

    let root_definitions = root_object
        .entry("definitions")
        .or_insert(Value::Object(Map::new()))
        .as_object_mut()
        .expect("Definitions not an object");

    // Add new definitions to root schema
    root_definitions.extend(definitions.into_iter());
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

            let name = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_case(Case::Camel);

            Ok((name, value))
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
    let _ = Command::new("git")
        .args(&["submodule", "update", "--init"])
        .status();

    // Get manifest dir from env
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Create schemas dir
    fs::create_dir_all(manifest_dir.clone() + "/schemas")
        .expect("Could not create schemas output dir");

    // Create flattened schemas
    let root_schema_v1_0_x = create_root_schema("vendor/is_04/v1_0_x/APIs/schemas");
    let root_schema_v1_1_x = create_root_schema("vendor/is_04/v1_1_x/APIs/schemas");
    let root_schema_v1_2_x = create_root_schema("vendor/is_04/v1_2_x/APIs/schemas");
    let root_schema_v1_3_x = create_root_schema("vendor/is_04/v1_3_x/APIs/schemas");

    // Write to schemas dir
    fs::write(
        manifest_dir.clone() + "/schemas/v1_0_x.json",
        serde_json::to_string(&root_schema_v1_0_x).unwrap(),
    )
    .expect("Could not write schema");
    fs::write(
        manifest_dir.clone() + "/schemas/v1_1_x.json",
        serde_json::to_string(&root_schema_v1_1_x).unwrap(),
    )
    .expect("Could not write schema");
    fs::write(
        manifest_dir.clone() + "/schemas/v1_2_x.json",
        serde_json::to_string(&root_schema_v1_2_x).unwrap(),
    )
    .expect("Could not write schema");
    fs::write(
        manifest_dir + "/schemas/v1_3_x.json",
        serde_json::to_string(&root_schema_v1_3_x).unwrap(),
    )
    .expect("Could not write schema");
}
