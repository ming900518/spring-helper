extern crate tokio;
extern crate reqwest;

use std::fs::File;
use std::io::{Write};
use std::path::Path;

use structopt::StructOpt;

mod opt;
use self::opt::Opt;


fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

}

async fn init() {
    let target = "https://start.spring.io/starter.zip?type=gradle-project&language=java&bootVersion=2.7.1&baseDir=project&groupId=com.sourcecode&artifactId=project&name=project&description=Project%20for%20project&packageName=com.sourcecode.project&packaging=jar&javaVersion=17&dependencies=webflux,lombok,devtools,configuration-processor,data-r2dbc,postgresql";
    let path = Path::new("./project.zip");

    let content = match reqwest::get(target).await {
        Ok(byte) => byte.bytes().await.unwrap(),
        Err(why) => panic!("Response could not be found, reason: {}", why),
    };

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("File creation failed, reason: {}", why),
    };

    file.write_all(&*content).expect("Unable to write to file");
}
