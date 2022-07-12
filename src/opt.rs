use structopt_derive::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "spring-helper", about = "A CLI helper for Spring Framework projects.", author = "Ming Chang (mail@mingchang.tw)")]
pub struct Opt {
    #[structopt(subcommand)]
    #[allow(dead_code)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "init", about = "Initialize a new Spring project.", author = "Ming Chang (mail@mingchang.tw)")]
    Init {
        #[structopt(help = "The package name of the project, e.g. tw.mingchang.project.")]
        package_name: String,
        #[structopt(help = "The package type of the project, e.g. JAR, WAR.")]
        package_type: String,
        #[structopt(help = "The Java version of the project, e.g. 18, 17, 11.")]
        java_version: i32,
        #[structopt(help = "The project type of the project, e.g. maven, gradle.")]
        project_type: String,
        #[structopt(help = "Specify the file name, if not specified, project.zip will be used as the file name.")]
        file_name: Option<String>
    },
    #[structopt(name = "qs", about = "Add basic CRUD APIs for every database schema, only PostgreSQL is supported right now.")]
    QuickStart {
        ip: String,
        port: i32,
        user: String,
        password: String,
    }

}
