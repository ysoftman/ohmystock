use chrono::prelude::*;
use chrono::Local;
use colored::*;
use select::document::Document;
use select::predicate::{Class, Name};
use std::collections::HashMap;
// use std::env;
// use std::fs::File;
// use std::io::prelude::*; // read_to_string()
use clap::Parser;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::thread;
use std::time::Duration;
mod stock_list;

#[derive(Debug, Default)]
struct StockInfo {
    name: String,                // 회사명
    code: String,                // 종목코드
    business_type: String,       // 업종
    product: String,             // 주요제품
    listed_date: String,         // 상장일
    settlement_date: String,     // 결산월
    representative_name: String, // 대표자명
    homepage: String,            // 홈페이지
    location: String,            // 지역
}

struct StockResult {
    price: String,
    up_down_same: String,
    compared_to_previous_day: String, // 전일대비 상승 또는 하락 변동값
}

#[derive(Parser, Debug)]
#[command(name = "ohmystock")]
#[command(version)] // clap 라이브러리에서 처리, Cargo.toml > version 사용 -V, --version Print version
struct Opt {
    // -f 또는 --follow 지원
    /// 1분 마다 갱신
    #[arg(short, long)]
    follow: bool,

    /// 회사 기본 정보
    #[arg(short, long)]
    company_info: bool,

    /// 종목명 검색 (키워드 없으면 전체 목록 출력)
    #[arg(short, long, num_args = 0..=1, default_missing_value = "")]
    list: Option<String>,

    // - 가 없는 단순 인자
    #[arg(default_value = "카카오")]
    // target: String,
    targets: Vec<String>,
}

fn main() {
    // let mut targets = String::from("카카오");
    // let args: Vec<String> = env::args().collect();
    // if args.len() > 1 {
    //     targets = args[1].to_uppercase().clone();
    // }
    // println!("args:{:?}", args);
    let mut opt = Opt::parse();
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

    // -l, --list 사용시 종목 검색
    if let Some(keyword) = &opt.list {
        search_stock_list(&stock_info_map, keyword);
        return;
    }

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

fn find_stock<'a>(
    stock_info_map: &'a HashMap<String, StockInfo>,
    keyword: &str,
) -> Option<&'a StockInfo> {
    // 정확히 일치하면 바로 반환
    if let Some(info) = stock_info_map.get(keyword) {
        return Some(info);
    }
    // fuzzy matching으로 가장 점수 높은 종목 반환
    let matcher = SkimMatcherV2::default();
    stock_info_map
        .values()
        .filter_map(|info| {
            matcher
                .fuzzy_match(&info.name, keyword)
                .map(|score| (info, score))
        })
        .max_by_key(|(_, score)| *score)
        .map(|(info, _)| info)
}

fn show_stock_info(opt: &Opt, stock_info_map: &HashMap<String, StockInfo>) {
    for v in &opt.targets {
        if let Some(stock_info) = find_stock(stock_info_map, v) {
            let reference_url = make_reference_url(stock_info);
            if opt.company_info {
                println!("회사명: {}", stock_info.name);
                println!("종목코드: {}", stock_info.code);
                println!("업종: {}", stock_info.business_type);
                println!("주요제품: {}", stock_info.product);
                println!("상장일: {}", stock_info.listed_date);
                println!("결산월: {}", stock_info.settlement_date);
                println!("대표자명: {}", stock_info.representative_name);
                println!("홈페이지: {}", stock_info.homepage);
                println!("지역: {}", stock_info.location);
                println!("reference => {reference_url}");
            }
            get_stock_price(&reference_url, stock_info);
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

fn get_stock_price(url: &str, stock_info: &StockInfo) {
    match get_url(url) {
        Ok(s) => match s.text() {
            Ok(content) => output(
                local_now().to_rfc3339_opts(SecondsFormat::Secs, false),
                stock_info,
                &parse_stock_result(&content),
            ),
            Err(e) => println!("error {e:?}"),
        },
        Err(e) => println!("error: {e:?}"),
    }
}

fn get_url(url: &str) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    // user-agent mozilla 없으면 에러 페이지로 응답된다.
    let resp = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "mozilla")
        .send()?;
    Ok(resp)
}

// async fn get_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let client = reqwest::Client::new();
//     let resp = client
//         .get(url)
//         .header(reqwest::header::USER_AGENT, "mozilla")
//         .send()
//         .await?;
//     Ok(resp)
// }

fn output(timestring: String, stock_info: &StockInfo, sr: &StockResult) {
    println!(
        "{} {} {} {}",
        timestring,
        stock_info.name.yellow().bold(),
        sr.price
            .on_truecolor(30, 30, 60)
            .truecolor(0, 230, 180)
            .bold(),
        if sr.up_down_same == "up" {
            format!("▲ {}", &sr.compared_to_previous_day)
                .red()
                .bold()
                .to_string()
        } else if sr.up_down_same == "down" {
            format!("▼ {}", &sr.compared_to_previous_day)
                .blue()
                .bold()
                .to_string()
        } else {
            // 0~255 의 고정된 터미널 컬러 사용
            format!("{} {}", &sr.up_down_same, &sr.compared_to_previous_day)
                .truecolor(188, 188, 188)
                .to_string()
        },
    );
}

fn parse_stock_result(resp_html: &str) -> StockResult {
    let mut sr = StockResult {
        price: "".to_string(),
        up_down_same: "-".to_string(),
        compared_to_previous_day: "".to_string(),
    };
    let document = Document::from(resp_html);
    let mut index = 1;
    for ele in document.find(Class("num")).take(2) {
        match index {
            1 => {
                if let Some(span) = ele.find(Name("span")).next() {
                    sr.price = span.text().trim().to_string();
                }
            }
            2 => {
                if let Some(span) = ele.find(Name("span")).nth(1) {
                    sr.compared_to_previous_day = span.text().trim().to_string();
                }
                // 전일대비 값 변동이 있다면 up, down 파악
                if sr.compared_to_previous_day != "0" {
                    for up_down_ele in ele.find(Name("em")).take(1) {
                        if up_down_ele
                            .attr("class")
                            .unwrap()
                            .to_string()
                            .contains("bu_pup")
                        {
                            sr.up_down_same = "up".to_string();
                        } else {
                            sr.up_down_same = "down".to_string();
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

fn search_stock_list(stock_info_map: &HashMap<String, StockInfo>, keyword: &str) {
    let keyword_upper = keyword.to_uppercase();
    let matcher = SkimMatcherV2::default();
    let mut results: Vec<(&StockInfo, i64)> = stock_info_map
        .values()
        .filter_map(|info| {
            if keyword_upper.is_empty() {
                return Some((info, 0));
            }
            let score = [&info.name, &info.code, &info.business_type]
                .iter()
                .filter_map(|field| matcher.fuzzy_match(field, &keyword_upper))
                .max();
            score.map(|s| (info, s))
        })
        .collect();
    results.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.name.cmp(&b.0.name)));

    if results.is_empty() {
        println!("\"{keyword}\" 에 해당하는 종목이 없습니다.");
        return;
    }

    println!(
        "{:<20} {:<10} {}",
        "종목명".yellow().bold(),
        "종목코드".green().bold(),
        "업종".truecolor(188, 188, 188)
    );
    println!("{}", "-".repeat(60));
    for (info, _) in &results {
        println!(
            "{:<20} {:<10} {}",
            info.name.yellow(),
            info.code.green(),
            info.business_type.truecolor(188, 188, 188)
        );
    }
    println!("\n총 {}개 종목", results.len());
}

fn load_stock_list_from_raw_string(contents: String) -> HashMap<String, StockInfo> {
    let document = Document::from(contents.as_str());

    // for node in
    //     document.find(Name("td").and(Attr("style", "mso-number-format:'@';text-align:center;")))
    let mut name = String::new();
    let mut stock_info_map = HashMap::new();
    for (i, node) in (0_u32..).zip(document.find(Name("td"))) {
        // println!("{}, {}", i, node.text());
        match i % 10 {
            0 => {
                name = node.text().clone();
                stock_info_map.insert(node.text().clone(), StockInfo::default());
                stock_info_map.get_mut(&name).unwrap().name = node.text().to_uppercase().clone();
            }
            1 => {} // 시장구분 (skip)
            2 => stock_info_map.get_mut(&name).unwrap().code = node.text(),
            3 => stock_info_map.get_mut(&name).unwrap().business_type = node.text(),
            4 => stock_info_map.get_mut(&name).unwrap().product = node.text(),
            5 => stock_info_map.get_mut(&name).unwrap().listed_date = node.text(),
            6 => stock_info_map.get_mut(&name).unwrap().settlement_date = node.text(),
            7 => stock_info_map.get_mut(&name).unwrap().representative_name = node.text(),
            8 => stock_info_map.get_mut(&name).unwrap().homepage = node.text().trim().to_string(),
            9 => stock_info_map.get_mut(&name).unwrap().location = node.text(),
            _ => (),
        }
    }
    println!();

    // for (k, v) in stock_info_map {
    //     println!("{} {:#?}", k, v);
    // }

    stock_info_map
}
