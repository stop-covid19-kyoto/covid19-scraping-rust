mod generates;
mod structs;
mod utils;

use regex::Regex;
use std::path::Path;
use clap::Parser;
use chrono::TimeZone;
use mail_parser::MimeHeaders;
use mail_parser::Message;
use crate::utils::date_format::convert_datetime_to_date_and_time;
use chrono::{DateTime, Local, Utc};
use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    basic::{
        BasicClient
    },
    reqwest::{
        http_client
    },
    RefreshToken,
    TokenResponse,
    TokenUrl
};
use std::io::{Write};
use std::fs::{File};

extern crate imap;
extern crate native_tls;

use crate::generates::{
    inspections_summary_generate::inspections_summary_generate,
    json::{
        jsonize_main_summary_generate::jsonize_main_summary_generate,
        jsonize_summary_generate::jsonize_summary_generate,
        jsonize_patients_generate::jsonize_patients_generate
    },
    main_summary_generate::main_summary_generate,
    news_generate::news_generate,
    patients_summary_generate::patients_summary_generate
};
use structs::{
    last_update::LastUpdate,
    main_summary::MainSummary,
    news::News,
    patient::Patient,
    sumdata::SumData,
    summary::Summary
};
use calamine::{Reader, Xlsx, open_workbook};
use generates::patients_generate::{patients_generate};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(long)]
    server: String,
    #[clap(long)]
    port: u16,
    #[clap(long)]
    account: String,
    #[clap(long)]
    auth_url: String,
    #[clap(long)]
    token_url: String,
    #[clap(long)]
    client_id: String,
    #[clap(long)]
    client_secret: String,
    #[clap(long)]
    refresh_token: String,
    #[clap(long)]
    query: String
}

struct OAuth2 {
    user: String,
    access_token: String,
}

impl imap::Authenticator for OAuth2 {
    type Response = String;
    fn process(&self, _: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

fn main() {
    // 一時ディレクトリを作成
    let tmp_dir = "tmp";

    // コマンドライン引数をパース
    let args = Args::parse();

    println!("Fetching workbook from mail server...");

    let oauth2_client = BasicClient::new(
        ClientId::new(args.client_id.to_string()),
        Some(ClientSecret::new(args.client_secret.clone())),
        AuthUrl::new(args.auth_url).unwrap(),
        Some(TokenUrl::new(args.token_url.clone()).unwrap())
    );
    let token_result = oauth2_client
        .exchange_refresh_token(&RefreshToken::new(args.refresh_token.clone()))
        .request(http_client)
        .expect("Failed to request an access token.");

    // TLSコネクタを作成
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    // クライアントを作成
    let client = imap::connect((args.server.clone(), args.port), args.server.clone(), &tls)
        .unwrap();
    // IMAPサーバへログイン
    let auth = OAuth2 {
        user: args.account,
        access_token: token_result.access_token().secret().clone()
    };

    let mut imap_session = match client.authenticate("XOAUTH2", &auth) {
        Ok(c) => c,
        #[allow(unused_variables)]
        Err((e, unauthorized_client)) => {
            eprintln!("Authentication is failed: {}", e);
            return;
        }
    };

    //メールボックスを選択する
    imap_session.select("INBOX").unwrap();

    let mut found: bool = false;
    let mut filename: String = String::from("");
    let mut last_update: DateTime<Local> = Local::now();
    
    // メールボックスの内容を読み込む
    println!("{}", &last_update.format("%d-%b-%Y "));
    let result = imap_session.search(["SENTSINCE ", &last_update.format("\"%d-%b-%Y\" ").
        to_string(), &args.query]
        .concat())
        .unwrap();
    let mut result_vec: Vec<u32> = Vec::new();

    for res in result {
        result_vec.push(res);
    }
    result_vec.sort();
    result_vec.reverse();

    for res in result_vec {
        // メッセージを読み込む
        let messages = imap_session.fetch(res.to_string(), "RFC822")
        .unwrap();
        let regex = Regex::new("[0-9]{8}data.xlsx").unwrap();

        // メッセージの内容を読み込み、一時的に保存する
        for message in &messages {
            let body = message.body().unwrap();
            let parsed = Message::parse(body).unwrap();
            let mail_date = parsed.get_date().unwrap();

            filename = parsed.get_attachment(0)
                .unwrap()
                .unwrap_binary()
                .get_attachment_name()
                .unwrap()
                .to_string();

            if regex.is_match(&filename) {
                last_update = Local.ymd(mail_date.year as i32, mail_date.month, mail_date.day)
                    .and_hms(mail_date.hour, mail_date.minute, mail_date.second);
                
                if !Path::new(tmp_dir).is_dir() {
                    std::fs::create_dir(tmp_dir).unwrap();
                }

                let attach = parsed.get_attachment(0)
                    .unwrap()
                    .unwrap_binary()
                    .get_body();
        
                let mut file = File::create([tmp_dir.clone(), "/", &filename].concat()).unwrap();
                file.write_all(attach).unwrap();

                found = true;
                break; 
            }
        }

        if found == true {
            break;
        }
    }

    imap_session.logout().unwrap();

    // ワークブックを読み出す
    println!("Loading a workbook...");
    let worksheets_name: [&str; 3] = ["陽性者の属性", "PCR検査件数", "最新の情報"];
    let mut workbook: Xlsx<_> = open_workbook([tmp_dir.clone(), "/", &filename].concat())
        .expect("Failed to open workbook.");

    let mut patients: Vec<Patient>;
    let mut patients_summary: Summary;
    let mut inspections_summary: Summary;
    let mut main_summary: MainSummary;
    let mut news: News;

    // 陽性者の属性ワークシートを読み込む
    for worksheet in worksheets_name {
        
        if let Some(Ok(range)) = workbook.worksheet_range(worksheet) {

            if worksheet == "陽性者の属性" {

                println!("Last update date is {}.", last_update);
            
                println!("Generating patients data...");
                patients = patients_generate(range.clone());
    
                println!("Generating jsonized patients data...");
                let jsonize_patients: String = jsonize_patients_generate(patients.clone(), last_update);

                let mut patients_date: Vec<DateTime<Utc>> = Vec::new();
    
                println!("Generating summary...");
                for i in 0..patients.len() {
                    patients_date.push(patients[i].clone().release_date.unwrap());
                }
    
                patients_summary = patients_summary_generate(
                        patients_date.clone(),
                        patients_date[0].clone(),
                        patients_date[patients_date.len() -1].clone(),
                        last_update.clone())
                    .unwrap();                
                let jsonize_patients_summary: String = jsonize_summary_generate(
                        patients_summary.clone(),
                        last_update
                    ).clone();
    
                // シリアライズした陽性者の属性を書き込む
                let mut file = File::create("data/patients.json").unwrap();
                file.write_all(jsonize_patients.as_bytes()).expect("Failed to output json file.");

                let mut file = File::create("data/patients_summary.json").unwrap();
                file.write_all(jsonize_patients_summary.as_bytes()).expect("Failed to output json file.");

            }

            if worksheet == "PCR検査件数" {
                
                inspections_summary = inspections_summary_generate(range.clone(), last_update);
                main_summary = main_summary_generate(range.clone(), last_update);

                let jsonize_inspections_summary: String = jsonize_summary_generate(
                        inspections_summary.clone(),
                        last_update)
                    .clone();
                let jsonize_main_summary: String = jsonize_main_summary_generate(main_summary, last_update);

                let mut file = File::create("data/inspections_summary.json").unwrap();
                file.write_all(jsonize_inspections_summary.as_bytes()).expect("Failed to output json file.");

                file = File::create("data/main_summary.json").unwrap();
                file.write_all(jsonize_main_summary.as_bytes()).expect("Failed to output json file.");

            }

            if worksheet == "最新の情報" {

                news = news_generate(&range);
                let jsonize_news: String = serde_json::to_string_pretty(&news).unwrap();

                let mut file = File::create("data/news.json").unwrap();
                file.write_all(jsonize_news.as_bytes()).expect("Failed to output json file.");

            }
    
        }

    }

    let update: LastUpdate = LastUpdate {
        last_update: convert_datetime_to_date_and_time(last_update)
    };

    let mut file = File::create("data/last_update.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&update).unwrap().as_bytes())
        .expect("Failed to output json file.");

    // 一時ディレクトリを削除
    println!("Remove temporary directory...");
    std::fs::remove_dir_all(tmp_dir).unwrap();

    println!("Done!");
}
