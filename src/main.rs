extern crate tokio;
extern crate reqwest;

use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IOWrite;
use std::path::Path;

use structopt::StructOpt;
use structopt::clap::AppSettings;
use crate::Command::{Init, Model};

#[derive(StructOpt, Debug)]
#[structopt(
name = "spring-helper",
about = "A CLI helper for Spring Web projects.",
author = "Ming Chang (mail@mingchang.tw)",
long_about = "\nA CLI helper for Spring Web projects.",
global_settings = & [AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp],
)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(
    about = "Initialize a new Spring project with Spring WebFlux, Spring Data R2DBC and PostgreSQL Driver.",
    author = "Ming Chang (mail@mingchang.tw)",
    long_about = "\nA CLI helper for Spring Web projects.\n\n'init' subcommand will initialize a new Spring project with Spring WebFlux, Spring Data R2DBC and PostgreSQL Driver."
    )]
    Init {
        #[structopt(help = "Package name, e.g. tw.mingchang.project.")]
        package_name: String,
        #[structopt(help = "Package type.\n- JAR\n- WAR")]
        package_type: String,
        #[structopt(help = "The Java version of the project.\n- 18\n- 17\n- 11")]
        java_version: i32,
        #[structopt(help = "Project type.\n- maven\n- gradle")]
        project_type: String,
        #[structopt(help = "Specify the file name.\nIf not specified, 'project.zip' will be used as the file name.")]
        file_name: Option<String>,
    },

    #[structopt(
    about = "Create the model from JSON directly.",
    author = "Ming Chang (mail@mingchang.tw)",
    long_about = "\nA CLI helper for Spring Web projects.\n\n'model' subcommand will create the model from JSON directly."
    )]
    Model {
        #[structopt(help = "Model name.")]
        model_name: String,
        #[structopt(help = "Package name, e.g. tw.mingchang.project.")]
        package_name: String
    },
}

#[tokio::main]
async fn main() {
    match Opt::from_args().cmd {
        Init { package_name, package_type, java_version, project_type, file_name } => {
            init(package_name, package_type, java_version, project_type, file_name).await;
        }
        Model { model_name, package_name } => {
            model(model_name, package_name).await;
        }
    }
}

async fn init(package_name: String, package_type: String, java_version: i32, project_type: String, file_name: Option<String>) {
    let file_name = file_name.unwrap_or_else(|| "project.zip".to_string());
    let path = Path::new(&file_name);

    // split package name into parts
    let mut package_name_array = package_name.split('.').collect::<Vec<&str>>();

    match package_name_array.len().cmp(&3) {
        std::cmp::Ordering::Greater => {
            println!("Package name structure is longer than excepted, string parts after index 1 will all be defined as artifactId.");
            for i in 2..package_name_array.len() {
                package_name_array[i] = package_name_array[i - 1];
            }
        }
        std::cmp::Ordering::Less => {
            println!("Package name structure is too short.");
            return;
        }
        std::cmp::Ordering::Equal => {}
    }

    // define url as mutable
    let mut url = String::from("https://start.spring.io/starter.zip?");

    // push project type into url
    match project_type.as_str() {
        "gradle" => {
            url.push_str("type=gradle-project&");
        }
        "maven" => {
            url.push_str("type=maven-project&");
        }
        _ => {
            println!("Invalid project type.");
            return;
        }
    }

    url.push_str("language=java&bootVersion=2.7.1&");

    // set baseDir
    url.push_str("baseDir=");
    url.push_str(package_name_array[2]);
    url.push('&');

    // set groupId
    url.push_str("groupId=");
    url.push_str(package_name_array[0]);
    url.push('.');
    url.push_str(package_name_array[1]);
    url.push('&');

    // set artifactId
    url.push_str("artifactId=");
    url.push_str(package_name_array[2]);
    url.push('&');

    // set name
    url.push_str("name=");
    url.push_str(package_name_array[2]);
    url.push('&');

    // set description
    url.push_str("description=");
    url.push_str(package_name_array[2]);
    url.push('&');

    // set packageName
    url.push_str("packageName=");
    url.push_str(&package_name);
    url.push('&');

    // set packageType
    match package_type.to_lowercase().as_str() {
        "jar" => {
            url.push_str("packaging=jar&");
        }
        "war" => {
            url.push_str("packaging=war&");
        }
        _ => {
            println!("Invalid package type.");
            return;
        }
    }

    // set javaVersion
    match java_version.to_string().as_str() {
        "18" => {
            url.push_str("javaVersion=18&");
        }
        "17" => {
            url.push_str("javaVersion=17&");
        }
        "11" => {
            url.push_str("javaVersion=11&");
        }
        _ => {
            println!("Invalid Java version.");
            return;
        }
    }

    url.push_str("dependencies=webflux,lombok,devtools,configuration-processor,data-r2dbc,postgresql");

    println!("Downloading Spring project zip file from Spring Initalizr.");
    println!("Please wait...\n");

    let content = match reqwest::get(url).await {
        Ok(byte) => byte.bytes().await.unwrap(),
        Err(why) => panic!("Response could not be found, reason: {}", why),
    };
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("File creation failed, reason: {}", why),
    };
    file.write_all(&*content).expect("Unable to write to file");
    println!("Project downloaded successfully as {}.", file_name);
}

async fn model(model_name: String, package_name: String) {
    let file_name = format!("{}.java", model_name);
    let path = Path::new(&file_name);

    println!("Please paste the JSON below. (Newline is not allowed. Press Enter/Return to continue, Ctrl+C to cancel)\n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let head = format!("package {}.{};\n\n@Data\n@AllArgsConstructor\n@NoArgsConstructor\npublic class {} {{", package_name, model_name, model_name);

    let mut body = String::new();

    let value = serde_json::from_str::<HashMap<String, String>>(input.as_str());
    match value {
        Ok(x) => {
            println!("\nJSON parsed successfully, will create model class with following fields:");
            x.iter().for_each(|a| {
                println!("{}, Java type: {}", a.0, a.1);
                body.write_str(&format!("\n\tprivate {} {};", a.1, a.0)).expect("Unable to write to string");
            });
            println!()
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("File creation failed, reason: {}", why),
    };
    file.write_all(format!("{}\n{}\n\n}}", head, body).as_ref()).expect("Unable to write to file");
    println!("Model created successfully as {}.", file_name);
}
