use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use chrono::prelude::*;
use chrono_tz::Canada::Eastern;

#[get("/")]
async fn start_index() -> impl Responder {
    let content = "<!DOCTYPE html>
    <html>
    <head>
      <title>Page Title</title>
    </head>
    <body>
    <h1>Available Endpoints:</h1>
    <ul>
        <li><a href=\"/time\">/time</a>: Get the current time</li>
        <li><a href=\"/ip\">/ip</a>: Get the client's IP address</li>
        <li><a href=\"/add/2/3\">/add/{num1}/{num2}</a>: Add two numbers together</li>
        <li><a href=\"/roman/12\">/roman/{num1}</a>: Display integer as a roman numeral</li>
        <li><a href=\"/roman_clock\">/roman_clock</a>: Display time when GET request made as roman numerals.</li>
    </ul>
    </body>
    </html>";
    HttpResponse::Ok().body(content)
}

#[get("/time")]
async fn get_time() -> impl Responder {
    let now = chrono::Local::now();
    HttpResponse::Ok().body(format!("{}", now))
}

//#[get("/ip")]
// async fn get_ip(addr: actix_web::web::Addr<std::net::IpAddr>) -> impl Responder {
//     let ip = addr.get_ref().clone();
//     HttpResponse::Ok().body(format!("{}", ip))
// }

#[get("/add/{num1}/{num2}")]
async fn add_numbers(info: actix_web::web::Path<(i32, i32)>) -> impl Responder {
    let (num1, num2) = info.into_inner();
    let result = num1 + num2;
    HttpResponse::Ok().body(format!("{}", result))
}

#[get("/roman/{num}")]
async fn roman(info: actix_web::web::Path<u32>) -> impl Responder {
    let num = info.into_inner();
    let result = int_to_roman(num);
    HttpResponse::Ok().body(format!("{}", result))
}

#[get("/roman_clock")]
async fn roman_clock() -> impl Responder {
    let dt = Utc::now();
    let ottawa_time = Eastern.from_utc_datetime(&dt.naive_utc());
    //println!("Roman Clock: {}:{}:{}", roman_numeral(ottawa_time.hour()),roman_numeral(ottawa_time.minute()),roman_numeral(ottawa_time.second()));

    let hour = int_to_roman(ottawa_time.hour());
    let minute = int_to_roman(ottawa_time.minute());
    let second = int_to_roman(ottawa_time.second());
    
    HttpResponse::Ok().body(format!("{}:{}:{}", hour, minute, second))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bind the server to localhost:8080
    HttpServer::new(|| {
        let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
        App::new()
            .app_data(Data::new(ip))
            .service(start_index)
            .service(get_time)
            //.service(get_ip)
            .service(add_numbers)
            .service(roman)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

fn int_to_roman(number: u32) -> String {
    let mut result = String::new();

    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut remaining = number;
    for (value, numeral) in values.iter() {
        while remaining >= *value {
            result.push_str(numeral);
            remaining -= value;
        }
    }

    result
}

