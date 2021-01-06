extern crate chrono;
use ansi_term::Colour::Fixed;
use ansi_term::Colour::*;
use ansi_term::Style;
use chrono::prelude::*;
use chrono::Local;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::collections::HashMap;
// use std::env;
// use std::fs::File;
// use std::io::prelude::*; // read_to_string()
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod stock_list;

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

struct StockResult {
    price: String,
    up_down_same: String,
    compared_to_previous_day: String, // 전일대비 상승 또는 하락 변동값
}

#[derive(StructOpt, Debug)]
#[structopt(name = "ohmystock")]
struct Opt {
    // -f 또는 --follow 지원
    /// 1분 마다 갱신
    #[structopt(short, long)]
    follow: bool,

    /// 회사 기본 정보
    #[structopt(short, long)]
    company_info: bool,

    // - 가 없는 단순 인자
    #[structopt(default_value = "카카오")]
    // target: String,
    targets: Vec<String>,
}

fn main() {
    // let mut targets = String::from("카카오");
    // let args: Vec<String> = env::args().collect();
    // if args.len() > 1 {
    //     targets = args[1].to_uppercase().clone();
    // }
    // println!("ags:{:?}", args);
    let mut opt = Opt::from_args();
    // opt.target = opt.targets.to_uppercase();
    for v in &mut opt.targets {
        *v = v.to_uppercase()
    }
    // println!("{:#?}", opt);

    // 파일로 부터 읽는 경우(파일이 바이너리와 항상 같은 위치에 있어야 하기 때문에 지양)
    // let contents = load_stock_list_from_file(String::from("상장법인목록.xls"));
    // let stock_info_map = load_stock_list_from_raw_string(contents);
    // raw string 으로 부터 읽는 경우
    let stock_info_map = load_stock_list_from_raw_string(stock_list::STOCK_LIST.to_string());

    // -f , --follow 사용시
    if opt.follow {
        println!("enable follow mode...");
        loop {
            show_stock_info(&opt, &stock_info_map);
            thread::sleep(Duration::from_secs(60));
        }
    } else {
        show_stock_info(&opt, &stock_info_map);
    }
}

fn show_stock_info(opt: &Opt, stock_info_map: &HashMap<String, StockInfo>) {
    for v in &opt.targets {
        // let reference_url: String;
        // if let Some(stock_info) = stock_info_map.get(v) {
        //     get_stock_price(&reference_url, &stock_info);
        // }
        match stock_info_map.get(v) {
            Some(stock_info) => {
                let reference_url = make_reference_url(&stock_info);
                if opt.company_info {
                    println!("회사명: {}", stock_info.name);
                    println!("종목코드: {}", stock_info.code);
                    println!("업종: {}", stock_info.bussiness_type);
                    println!("주요제품: {}", stock_info.product);
                    println!("상장일: {}", stock_info.listed_date);
                    println!("결산월: {}", stock_info.settlement_date);
                    println!("대표자명: {}", stock_info.representative_name);
                    println!("홈페이지: {}", stock_info.homepage);
                    println!("지역: {}", stock_info.location);
                    println!("reference => {}", reference_url);
                }
                get_stock_price(&reference_url, &stock_info);
            }
            None => (),
        }
    }
}
fn local_now() -> DateTime<Local> {
    // let now = Local::now().timestamp();
    Local::now()
}

fn make_reference_url(stock_info: &StockInfo) -> String {
    let this_time = format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        local_now().year(),
        local_now().month(),
        local_now().day(),
        local_now().hour(),
        local_now().minute(),
        local_now().second(),
    );
    format!(
        "https://finance.naver.com/item/sise_time.nhn?thistime={}&code={}",
        this_time, &stock_info.code
    )
}

fn get_stock_price(url: &String, stock_info: &StockInfo) {
    match get_url(&url) {
        Ok(s) => match s.text() {
            Ok(content) => output(
                local_now().to_rfc3339_opts(SecondsFormat::Secs, false),
                &stock_info,
                &parse_stock_result(&content),
            ),
            Err(e) => println!("error {:?}", e),
        },
        Err(e) => println!("error: {:?} ", e),
    }
}

fn get_url(url: &str) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?;
    Ok(resp)
}

fn output(timestring: String, stock_info: &StockInfo, sr: &StockResult) {
    println!(
        "{} {} {} {}",
        timestring,
        Yellow.bold().paint(&stock_info.name).to_string(),
        Style::new()
            .on(Purple)
            .fg(Black)
            .underline()
            .paint(&sr.price),
        if sr.up_down_same == "up" {
            Red.bold()
                .paint("▲ ".to_string() + &sr.compared_to_previous_day)
                .to_string()
        } else if sr.up_down_same == "down" {
            Blue.bold()
                .paint("▼ ".to_string() + &sr.compared_to_previous_day)
                .to_string()
        } else {
            // 0~255 의 고정된 터미널 컬러 사용
            Fixed(250)
                .paint(sr.up_down_same.clone() + &" ".to_string() + &sr.compared_to_previous_day)
                .to_string()
        },
    );
}

fn parse_stock_result(resp_html: &str) -> StockResult {
    // println!("{}", resp_html)
    let mut sr = StockResult {
        price: "".to_string(),
        up_down_same: "-".to_string(),
        compared_to_previous_day: "".to_string(),
    };
    let document = Document::from(resp_html);
    let mut index = 1;
    for node in document.find(Class("num").child(Name("span"))).take(2) {
        // println!("node {}", node.text());
        match index {
            1 => sr.price = node.text().trim().to_string(),
            2 => {
                sr.compared_to_previous_day = node.text().trim().to_string();
                // 전일대비 값 변동이 있다면 up, down 파악
                if sr.compared_to_previous_day != "0" {
                    for img_node in document.find(Class("num").child(Name("img"))).take(1) {
                        if img_node.attr("src").unwrap().to_string().contains("down") {
                            sr.up_down_same = "down".to_string();
                        } else {
                            sr.up_down_same = "up".to_string();
                        }
                    }
                }
            }
            _ => (),
        }
        index += 1;
    }
    sr
}

// fn load_stock_list_from_file(filename: String) -> String {
//     let f = File::open(filename);
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => {
//             println!("error : {}", e);
//             return String::new();
//         }
//     };
//     let mut contents = String::new();
//     match f.read_to_string(&mut contents) {
//         Ok(_) => (),
//         Err(e) => {
//             println!("error : {}", e);
//             return String::new();
//         }
//     }
//     contents
// }

fn load_stock_list_from_raw_string(contents: String) -> HashMap<String, StockInfo> {
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
