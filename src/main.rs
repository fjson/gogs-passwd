use anyhow::{Ok, Result};
use clap::Parser;
use regex::bytes::Regex;
use reqwest::Response;
use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    time::Duration,
};
use tokio::time::sleep;

static HOST: &str = "http://10.32.108.93:3000";

struct Auth {
    i_like_gogs: String,
    _csrf: String,
}

#[derive(Parser, Debug, Clone)]
#[command(author = "jason xing", version, about, long_about = None)]
pub struct CliArgs {
    /// username
    #[arg(long, short)]
    username: String,

    /// passwd
    #[arg(long, short)]
    passwd: String,

    /// temp passwd
    #[arg(long, short)]
    temp_passwd: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut count = 5;
    loop {
        if let Err(err) = start().await {
            count -= 1;
            if count <= 0 {
                return Err(err);
            }
            sleep(Duration::from_secs(5)).await;
            continue;
        }
        break Ok(());
    }
}

async fn start() -> Result<()> {
    let mut args = CliArgs::parse();
    if let Some(real_passwd) = read_real_passwd(&args.username) {
        if args.temp_passwd == real_passwd {
            let temp = args.passwd;
            args.passwd = args.temp_passwd;
            args.temp_passwd = temp;
        }
    }
    let auth = get_auth_from_login().await?;
    let auth = login(&auth, &args.username, &args.passwd).await?;
    change_passwd(&auth, &args.passwd, &args.temp_passwd).await?;
    writ_real_passwd(&args.username, &args.temp_passwd);
    println!(
        "username: {}, passwd: {}",
        &args.username, &args.temp_passwd
    );
    Ok(())
}

/// 模拟请求登录页，获取 i_like_gogs _csrf
async fn get_auth_from_login() -> Result<Auth> {
    let resp = reqwest::get(format!("{HOST}/user/login")).await?;
    Ok(get_auth_from(&resp))
}

/// 登录完成后返回 i_like_gogs _csrf
async fn login(auth: &Auth, username: &str, passwd: &str) -> Result<Auth> {
    let mut params = HashMap::new();
    params.insert("user_name", username);
    params.insert("password", passwd);
    params.insert("_csrf", &auth._csrf);
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{HOST}/user/login"))
        .form(&params)
        .header(
            "Cookie",
            format!(
                "lang=zh-CN; i_like_gogs={}; _csrf={}; redirect_to=%252F",
                auth.i_like_gogs, auth._csrf
            ),
        )
        .send()
        .await?;
    let mut new_auth = get_auth_from(&resp);
    new_auth.i_like_gogs = auth.i_like_gogs.to_owned();
    Ok(new_auth)
}

/// 修改密码
async fn change_passwd(auth: &Auth, old_passwd: &str, new_passwd: &str) -> Result<String> {
    let mut params = HashMap::new();
    params.insert("old_password", old_passwd);
    params.insert("password", new_passwd);
    params.insert("retype", new_passwd);
    params.insert("_csrf", &auth._csrf);
    let client = reqwest::Client::new();
    client
        .post(format!("{HOST}/user/settings/password"))
        .form(&params)
        .header(
            "Cookie",
            format!(
                "lang=zh-CN; i_like_gogs={}; _csrf={};",
                auth.i_like_gogs, auth._csrf
            ),
        )
        .send()
        .await?;
    Ok(new_passwd.to_string())
}

/// 解析cookie字符串
fn parse_cookies(v: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let re = Regex::new(r"(?i)([^=;]+)=([^;]*)").unwrap();
    for cap in re.captures_iter(v.as_bytes()) {
        let key = &cap[1];
        let value = &cap[2];
        map.insert(
            String::from_utf8(key.to_vec()).unwrap(),
            String::from_utf8(value.to_vec()).unwrap(),
        );
    }
    map
}

/// 解析响应体中的cookie， 获取i_like_gogs _csrf
fn get_auth_from(resp: &Response) -> Auth {
    let mut auth = Auth {
        i_like_gogs: String::from(""),
        _csrf: String::from(""),
    };
    resp.headers().get_all("set-cookie").iter().for_each(|v| {
        let cookie = parse_cookies(v.to_str().unwrap());
        let i_like_gogs = cookie.get("i_like_gogs");
        let _csrf = cookie.get("_csrf");
        if let Some(i_like_gogs) = i_like_gogs {
            auth.i_like_gogs = i_like_gogs.to_owned();
        }
        if let Some(_csrf) = _csrf {
            auth._csrf = _csrf.to_owned();
        }
    });
    auth
}

fn writ_real_passwd(username: &str, passwd: &str) {
    let temp_dir = get_temp_dir();
    let f_buf = temp_dir.join(username);
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(f_buf)
        .unwrap();
    f.write_all(passwd.as_bytes()).unwrap();
}

fn read_real_passwd(username: &str) -> Option<String> {
    let temp_dir = get_temp_dir();
    let f_buf = temp_dir.join(username);
    if f_buf.exists() {
        if let std::result::Result::Ok(res) = fs::read_to_string(f_buf) {
            return Some(res);
        }
    }
    None
}

fn get_temp_dir() -> PathBuf {
    let current_exe = env::current_exe().unwrap();
    let temp_dir = current_exe.parent().unwrap().join(".passwd_temp");
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).unwrap();
    }
    temp_dir
}
