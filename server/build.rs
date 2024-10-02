use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use serde_json::Value;

fn main() {
    let out_dir = "src/function_types";
    let dest_path = Path::new(&out_dir).join("python_functions.rs");
    let mut f = File::create(dest_path).unwrap();

    let schema = include_str!("../functions/function_schemas.json");
    let schema: Value = serde_json::from_str(schema).unwrap();

    //delete "src/function_routes"
    let _ = std::fs::remove_dir_all("src/function_routes");

    // Create the routes directory if it doesn't exist
    create_dir_all("src/function_routes").unwrap();

    writeln!(f, "use super::python_util::PythonInterface;").unwrap();
    writeln!(f, "use serde_json::json;").unwrap();
    writeln!(f, "use futures::{{Stream, StreamExt}};").unwrap();
    writeln!(f, "use std::pin::Pin;").unwrap();

    // Add imports for all the types
    writeln!(f).unwrap();
    for (_, mut function_schema) in schema["properties"].as_object().unwrap() {
        function_schema = &function_schema["properties"];
        let args = function_schema["args"]["$ref"]
            .as_str()
            .unwrap()
            .split('/')
            .last()
            .unwrap();
        let return_type = function_schema["return"]["$ref"]
            .as_str()
            .unwrap()
            .split('/')
            .last()
            .unwrap();
        writeln!(f, "use super::types::{};", args).unwrap();
        writeln!(f, "use super::types::{};", return_type).unwrap();
    }

    writeln!(f).unwrap();
    writeln!(f, "pub struct Python {{").unwrap();
    writeln!(f, "    interface: PythonInterface,").unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "impl Python {{").unwrap();
    writeln!(f, "    pub fn new() -> std::io::Result<Self> {{").unwrap();
    writeln!(
        f,
        "        Ok(Self {{ interface: PythonInterface::new()? }})"
    )
    .unwrap();
    writeln!(f, "    }}").unwrap();

    // Create add_function_routes.rs file
    let add_routes_path = Path::new("src/function_routes").join("add_function_routes.rs");
    let mut add_routes_file = File::create(add_routes_path).unwrap();

    writeln!(
        add_routes_file,
        "use actix_web::dev::{{ServiceFactory, ServiceRequest}};"
    )
    .unwrap();
    writeln!(add_routes_file, "use paperclip::actix::App;").unwrap();
    writeln!(add_routes_file).unwrap();

    // Import all function routes
    for (function_name, mut function_schema) in schema["properties"].as_object().unwrap() {
        function_schema = &function_schema["properties"];

        let return_type = function_schema["return"]["$ref"]
            .as_str()
            .unwrap()
            .split('/')
            .last()
            .unwrap();
        let is_stream = schema["definitions"][return_type]["stream"]
            .as_bool()
            .unwrap_or(false);

        if is_stream {
            continue;
        }
        writeln!(
            add_routes_file,
            "use super::{}::{};",
            function_name, function_name
        )
        .unwrap();
    }

    writeln!(add_routes_file).unwrap();
    writeln!(add_routes_file, "pub trait AddFunctionRoutes {{").unwrap();
    writeln!(add_routes_file, "    fn add_function_routes(self) -> Self;").unwrap();
    writeln!(add_routes_file, "}}").unwrap();
    writeln!(add_routes_file).unwrap();
    writeln!(add_routes_file, "impl<T> AddFunctionRoutes for App<T>").unwrap();
    writeln!(add_routes_file, "where").unwrap();
    writeln!(add_routes_file, "    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,").unwrap();
    writeln!(add_routes_file, "{{").unwrap();
    writeln!(
        add_routes_file,
        "    fn add_function_routes(self) -> Self {{"
    )
    .unwrap();
    writeln!(add_routes_file, "        self").unwrap();

    // Create mod.rs file
    let mod_path = Path::new("src/function_routes").join("mod.rs");
    let mut mod_file = File::create(mod_path).unwrap();

    writeln!(mod_file, "pub mod add_function_routes;").unwrap();

    for (function_name, mut function_schema) in schema["properties"].as_object().unwrap() {
        function_schema = &function_schema["properties"];
        let args = function_schema["args"]["$ref"]
            .as_str()
            .unwrap()
            .split('/')
            .last()
            .unwrap();
        let return_type = function_schema["return"]["$ref"]
            .as_str()
            .unwrap()
            .split('/')
            .last()
            .unwrap();
        let is_stream = schema["definitions"][return_type]["stream"]
            .as_bool()
            .unwrap_or(false);

        if is_stream {
            continue;
        }

        writeln!(f).unwrap();
        if is_stream {
            writeln!(
                f,
                "    pub async fn {}(&self, args: {}) -> Pin<Box<dyn Stream<Item = std::io::Result<{}>> + Send>> {{",
                function_name, args, return_type
            )
            .unwrap();
            writeln!(
                f,
                "        let stream = self.interface.execute(\"{}\", json!(args)).await;",
                function_name
            )
            .unwrap();
            writeln!(f, "        Box::pin(stream.map(|response| {{").unwrap();
            writeln!(
                f,
                "            if let Some(error) = response.get(\"error\") {{"
            )
            .unwrap();
            writeln!(
                f,
                "                Err(std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))"
            )
            .unwrap();
            writeln!(f, "            }} else {{").unwrap();
            writeln!(
                f,
                "                Ok(serde_json::from_value(json!({{\"value\": response[\"result\"].clone()}})).unwrap())"
            )
            .unwrap();
            writeln!(f, "            }}").unwrap();
            writeln!(f, "        }}))").unwrap();
        } else {
            writeln!(
                f,
                "    pub async fn {}(&self, args: {}) -> std::io::Result<{}> {{",
                function_name, args, return_type
            )
            .unwrap();
            writeln!(
                f,
                "        let mut stream = self.interface.execute(\"{}\", json!(args)).await;",
                function_name
            )
            .unwrap();
            writeln!(f, "        let mut result = None;").unwrap();
            writeln!(
                f,
                "        while let Some(response) = stream.next().await {{"
            )
            .unwrap();
            writeln!(
                f,
                "            if let Some(error) = response.get(\"error\") {{"
            )
            .unwrap();
            writeln!(
                f,
                "                return Err(std::io::Error::new(std::io::ErrorKind::Other, error.to_string()));"
            )
            .unwrap();
            writeln!(f, "            }} else {{").unwrap();
            writeln!(
                f,
                "                result = Some(serde_json::from_value(json!({{\"value\": response[\"result\"].clone()}})).unwrap());"
            )
            .unwrap();
            writeln!(f, "            }}").unwrap();
            writeln!(
                f,
                "            if response[\"done\"].as_bool() == Some(true) {{"
            )
            .unwrap();
            writeln!(f, "                break;").unwrap();
            writeln!(f, "            }}").unwrap();
            writeln!(f, "        }}").unwrap();
            writeln!(
                f,
                "        result.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, \"No result received\"))"
            )
            .unwrap();
        }
        writeln!(f, "    }}").unwrap();

        // Create individual route files
        let route_path = Path::new("src/function_routes").join(format!("{}.rs", function_name));
        let mut route_file = File::create(route_path).unwrap();

        writeln!(route_file, "use actix_web::{{").unwrap();
        writeln!(route_file, "    web::{{Data, Json}},").unwrap();
        writeln!(route_file, "    Error,").unwrap();
        writeln!(route_file, "}};").unwrap();
        writeln!(
            route_file,
            "use paperclip::actix::{{api_v2_operation, post}};"
        )
        .unwrap();
        writeln!(route_file, "use crate::{{").unwrap();
        writeln!(
            route_file,
            "    function_types::types::{{{}, {}}},",
            args, return_type
        )
        .unwrap();
        writeln!(route_file, "    GlobalState,").unwrap();
        writeln!(route_file, "}};").unwrap();
        writeln!(route_file).unwrap();
        writeln!(route_file, "#[api_v2_operation]").unwrap();
        writeln!(route_file, "#[post(\"/api/{}\")]", function_name).unwrap();
        writeln!(route_file, "pub async fn {}(", function_name).unwrap();
        writeln!(route_file, "    gs: Data<GlobalState>,").unwrap();
        writeln!(route_file, "    body: Json<{}>,", args).unwrap();
        writeln!(route_file, ") -> Result<Json<{}>, Error> {{", return_type).unwrap();
        writeln!(
            route_file,
            "    Ok(Json(gs.python.{}(body.into_inner()).await?))",
            function_name
        )
        .unwrap();
        writeln!(route_file, "}}").unwrap();

        // Add service to add_function_routes.rs
        writeln!(add_routes_file, "            .service({})", function_name).unwrap();

        // Add module to mod.rs
        writeln!(mod_file, "pub mod {};", function_name).unwrap();
    }

    writeln!(f, "}}").unwrap();

    // Finish add_function_routes.rs
    writeln!(add_routes_file, "    }}").unwrap();
    writeln!(add_routes_file, "}}").unwrap();

    //apply migrations
    let output = Command::new("diesel")
        .arg("migration")
        .arg("run")
        .arg("--database-url=postgres:///maurice")
        .output()
        .unwrap();

    if output.status.success() {
        println!("cargo:warning=Successfully applied migrations");
    } else {
        println!(
            "cargo:warning=Failed to apply migrations. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    //run print-schema in cli
    let output = Command::new("diesel")
        .arg("print-schema")
        .arg("--database-url=postgres:///maurice")
        .output()
        .unwrap();

    if output.status.success() {
        let schema = String::from_utf8_lossy(&output.stdout);
        let mut file = File::create("src/schema.rs").unwrap();
        file.write_all(schema.as_bytes()).unwrap();
        println!("cargo:warning=Successfully generated schema in src/schema.rs");
    } else {
        println!(
            "cargo:warning=Failed to generate schema. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = Command::new("diesel_ext").output().unwrap();

    //write to src/model.rs

    if output.status.success() {
        let model = String::from_utf8_lossy(&output.stdout);
        let mut file = File::create("src/model.rs").unwrap();

        let model = model.replace(
            "use chrono::DateTime;",
            "use chrono::DateTime;\nuse diesel::{Identifiable, Queryable};\nuse serde::{Serialize, Deserialize};\nuse crate::schema::*;",
        );

        //replace all #[derive(Queryable, Debug)] with #[derive(Queryable, Debug, Serialize, Deserialize)]
        let model = model.replace(
            "#[derive(Queryable, Debug, Identifiable)]",
            "#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]",
        );

        let model = model.replace(
            "#[derive(Queryable, Debug)]",
            "#[derive(Queryable, Debug, Serialize, Deserialize)]",
        );

        file.write_all(model.as_bytes()).unwrap();
        println!("cargo:warning=Successfully generated model in src/model.rs");
    } else {
        println!(
            "cargo:warning=Failed to generate model. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rerun-if-changed=migrations");
}
