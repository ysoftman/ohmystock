extern crate chrono;
use ansi_term::Colour::Fixed;
use ansi_term::Colour::*;
use ansi_term::Style;
use chrono::prelude::*;
use chrono::{DateTime, Local, TimeZone, Utc};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*; // read_to_string()
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
struct StockInfo {
    name: String,                // 회사명
    code: String,                // 종목코드
    bussiness_type: String,      // 업종
    product: String,             // 주요제품
    listed_date: String,         // 상장일
    settlement_date: String,     // 결산월
    representative_name: String, // 대표자명
    homepage: String,            // 홈페이지
    location: String,            // 지역
}

impl Default for StockInfo {
    fn default() -> StockInfo {
        StockInfo {
            name: String::from(""),
            code: String::from(""),
            bussiness_type: String::from(""),
            product: String::from(""),
            listed_date: String::from(""),
            settlement_date: String::from(""),
            representative_name: String::from(""),
            homepage: String::from(""),
            location: String::from(""),
        }
    }
}

fn main() {
    let mut target = String::from("카카오");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        target = args[1].to_uppercase().clone();
        println!("target:{}", target);
    }
    let stock_info_map = load_stock_code_from_file(String::from("상장법인목록.xls"));

    match stock_info_map.get(&target) {
        Some(stock_info) => {
            println!("stock_info : {:?}", stock_info);
            get_stock_price(&stock_info);
        }
        None => (),
    }
}

fn get_stock_price(stock_info: &StockInfo) -> i32 {
    // let now = Local::now().timestamp();
    let now = Local::now();
    let this_time = format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    );
    let url = format!(
        "https://finance.naver.com/item/sise_time.nhn?thistime={}&code={}",
        this_time, stock_info.code
    );
    println!("URL {}", url);
    match get_url(&url) {
        Ok(s) => match s.text() {
            Ok(content) => output(&stock_info, &parse_stock_result(&content)),
            Err(e) => println!("error {:?}", e),
        },
        Err(e) => println!("error: {:?} ", e),
    }
    100
}

fn get_url(url: &str) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?;
    Ok(resp)
}

fn output(stock_info: &StockInfo, price: &str) {
    // let str1 = "red";
    // println!(" {}", Red.paint(str1));
    println!(
        "종목명: {}",
        Yellow.bold().paint(&stock_info.name).to_string()
    );
    // println!("bold {}", Cyan.bold().paint("bold style"));
    println!(
        "현재가: {}",
        Style::new().on(Purple).fg(Black).underline().paint(price)
    );
    // println!("fixed {}", Fixed(127).paint("fixed test"));
}

fn parse_stock_result(resp_html: &str) -> String {
    // println!("{}", resp_html)
    let mut price = String::new();
    let document = Document::from(resp_html);
    for node in document.find(Class("num").child(Name("span"))) {
        // println!("node {}", node.text());
        price = node.text().clone();
        break;
    }
    price
}

// fn load_stock_code_from_file(filename: String) -> Result<(), Box<dyn std::error::Error>> {
//     let mut f = File::open(filename)?;
//     let mut contents = String::new();
//     f.read_to_string(&mut contents)?;
//     println!("{}", contents)
//     Ok(())
// }

fn load_stock_code_from_file(filename: String) -> HashMap<String, StockInfo> {
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => {
            println!("error : {}", e);
            return HashMap::new();
        }
    };
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            println!("error : {}", e);
            return HashMap::new();
        }
    }
    let document = Document::from(contents.as_str());

    // for node in
    //     document.find(Name("td").and(Attr("style", "mso-number-format:'@';text-align:center;")))
    let mut cnt: u32 = 0;
    let mut name = String::new();
    let mut stock_info_map = HashMap::new();
    for node in document.find(Name("td")) {
        // println!("cnt {}, {}", cnt, node.text());
        match cnt % 9 {
            0 => {
                name = node.text().clone();
                stock_info_map.insert(node.text().clone(), StockInfo::default());
                stock_info_map.get_mut(&name).unwrap().name = node.text().to_uppercase().clone();
            }
            1 => stock_info_map.get_mut(&name).unwrap().code = node.text(),
            2 => stock_info_map.get_mut(&name).unwrap().bussiness_type = node.text(),
            3 => stock_info_map.get_mut(&name).unwrap().product = node.text(),
            4 => stock_info_map.get_mut(&name).unwrap().listed_date = node.text(),
            5 => stock_info_map.get_mut(&name).unwrap().settlement_date = node.text(),
            6 => stock_info_map.get_mut(&name).unwrap().representative_name = node.text(),
            7 => stock_info_map.get_mut(&name).unwrap().homepage = node.text().trim().to_string(),
            8 => stock_info_map.get_mut(&name).unwrap().location = node.text(),
            _ => (),
        }
        cnt += 1;
    }
    println!("");

    // for (k, v) in stock_info_map {
    //     println!("{} {:#?}", k, v);
    // }

    stock_info_map
}
