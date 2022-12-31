#[macro_use] extern crate rocket;
use rocket::http::ContentType;
use rocket_client_addr::ClientRealAddr;
use std::io::*;
use std::fs::OpenOptions;
use std::env;
use std::time;
use json;

struct Log {
    timestamp: u64,
    page: String,
    platform: String,
    addr: String,
}

fn write(log: Log) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.jsonl")
        .unwrap();

    let mut json = json::JsonValue::new_object();
    json["page"] = log.page.into();
    json["platform"] = log.platform.into();
    json["addr"] = log.addr.into();
    json["timestamp"] = log.timestamp.into();

    writeln!(file, "{}", json).ok();
}

#[get("/<page>/<platform>/image.svg")]
fn endpoint(addr: ClientRealAddr, page: &str, platform: &str) -> (ContentType, &'static str) {
    let timestamp = time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Could not read time");
    write(Log{
        addr: addr.get_ipv6().to_string(),
        page: page.to_string(),
        platform: platform.to_string(),
        timestamp: timestamp.as_secs()
    });
    return (ContentType::SVG, "<svg xmlns=\"http://www.w3.org/2000/svg\"/>")
}

#[post("/logs", data="<key>")]
fn get_logs(key: String) -> String {
    if key != env::var("KEY").expect("Please set KEY environment variable") {
        return "Unauthorized".to_string()
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open("log.jsonl")
        .unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).ok();

    return content
}

#[catch(404)]
fn error() -> &'static str {
    println!("Error 404");
    return "Error 404: Not Found"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![endpoint, get_logs])
        .register("/", catchers![error])
}
