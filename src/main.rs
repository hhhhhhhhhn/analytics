#[macro_use] extern crate rocket;
use rocket::http::ContentType;
use rocket::config::Config;
use rocket_client_addr::ClientRealAddr;
use std::io::*;
use std::fs::OpenOptions;
use std::env;
use json;

struct Log {
    page: String,
    platform: String,
    addr: String,
}

fn write(log: Log) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let mut json = json::JsonValue::new_object();
    json["page"] = log.page.into();
    json["platform"] = log.platform.into();
    json["addr"] = log.addr.into();

    writeln!(file, "{}", json).ok();
}

#[get("/<page>/<platform>/image.svg")]
fn endpoint(addr: ClientRealAddr, page: &str, platform: &str) -> (ContentType, &'static str) {
    write(Log{addr: addr.get_ipv6().to_string(), page: page.to_string(), platform: platform.to_string()});
    return (ContentType::SVG, "<svg xmlns=\"http://www.w3.org/2000/svg\"/>")
}

#[catch(404)]
fn error() -> &'static str {
    println!("Error 404");
    return "Error 404: Not Found"
}

#[launch]
fn rocket() -> _ {
    let mut conf = Config::figment().extract::<Config>().unwrap();
    conf.port = env::var("PORT").unwrap().parse().unwrap();
    println!("Serving in port {}", conf.port);
    rocket::build()
        .configure(conf)
        .mount("/", routes![endpoint])
        .register("/", catchers![error])
}
