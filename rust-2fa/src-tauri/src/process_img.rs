use rusqlite::{Connection, Result};
use serde::Serialize;
use std::option::Option;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::command;
use totp_rs::{Algorithm, Secret, TOTP};
use url::Url;

#[derive(Debug, Serialize)]
pub struct TwoFactorAuthInfo {
    custom_name: String,
    username: String,
    company: String,
    totp_code: u32,
    remaining_time : u8,
    id: i32,
}
fn init_db() -> Result<Connection> {
    let conn = Connection::open("2fa_data.db")?;
    // let conn = Connection::open(":memory:")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS two_factor_auth (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            company TEXT,
            username TEXT,
            custom_name TEXT,
            period INTEGER,
            digits INTEGER,
            algorithm TEXT,
            secret TEXT,
            issued_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}
#[command]
pub fn save_2fa_data(name: Option<String>, otpauth: &str) -> String {
    //解析otp链接
    let url = Url::parse(otpauth).unwrap();
    //解析otp中的账户名和公司名
    let path_parts: Vec<&str> = url.path().splitn(2, ':').collect();
    let mut path_parts_iter =  path_parts.into_iter();
    let company_part = path_parts_iter.next();
    //去除totp/部分
    // 处理 Option<&str> 类型，取出 &str 后再调用 find 方法
    let company = if let Some(part) = company_part {
        if let Some(index) = part.find('/') {
            &part[index + 1..]
        } else {
            part
        }
    } else {
        "Company is not exist"
    };
    //处理自定义名称
    let custom_name= if let Some(cus_name) = name{
         cus_name
    }else{
        "".to_string()
    };
    let username = path_parts_iter.next().unwrap_or("");

    let period = url.query_pairs()
        .find(|(k, _)| k == "period")
        .and_then(|(_, v)| v.parse::<u32>().ok())
        .unwrap_or(30);

    let digits = url.query_pairs()
        .find(|(k, _)| k == "digits")
        .and_then(|(_, v)| v.parse::<u32>().ok())
        .unwrap_or(6);

    let algorithm = url.query_pairs()
        .find(|(k, _)| k == "algorithm")
        .map(|(_, v)| v.to_string())
        .unwrap_or("SHA1".to_string());

    let secret = url.query_pairs()
        .find(|(k, _)| k == "secret")
        .map(|(_, v)| v.to_string())
        .unwrap_or("".to_string());


    let conn = init_db().unwrap();
    let result = conn.execute(
        "INSERT INTO two_factor_auth (company, username, custom_name, period, digits, algorithm, secret) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        [company, username, &custom_name, &period.to_string(), &digits.to_string(), &algorithm,& secret],
    );
    if let Ok(_) = result {
        "保存成功".to_string()
    } else {
        "保存失败".to_string()
    }
}
#[command]
pub fn generate_totp() -> Result<Vec<TwoFactorAuthInfo>, String> {
    let conn = init_db().map_err(|e| e.to_string())?; // 转换init_db的错误
    let mut stmt = conn
        .prepare("SELECT custom_name, username, company, period, secret, algorithm, id FROM two_factor_auth")
        .map_err(|e| e.to_string())?; // 转换prepare的错误
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp_seconds = since_the_epoch.as_secs();

    let mut auth_info_list = Vec::new();
    let rows = stmt
        .query_map([], |row| {
            let custom_name: String = row.get(0)?;
            let username: String = row.get(1)?;
            let company: String = row.get(2)?;
            let period: u64 = row.get(3)?;
            let secret: String = row.get(4)?;
            let algorithm_str: String = row.get(5)?;
            let id : i32 = row.get(6)?;
            // 计算剩余有效时间
            let elapsed_seconds = timestamp_seconds % period as u64 ;
            let remaining_time = (period as u64 - elapsed_seconds) as u8;


            let algorithm = match algorithm_str.to_uppercase().as_str() {
                "SHA1" => Algorithm::SHA1,
                "SHA256" => Algorithm::SHA256,
                "SHA512" => Algorithm::SHA512,
                _ => Algorithm::SHA1,
            };
            // // Base32解码密钥
            // let secret_upper = secret.to_uppercase();
            // let secret_bytes = BASE32.decode(secret_upper.as_bytes())
            //     .map_err(|e|  e.to_string()).unwrap();
            // print!("{},{:?},{};;{:?}",&algorithm, &secret_bytes, &secret,Secret::Raw(secret.as_bytes().to_vec()).to_bytes().unwrap());
            // 正确参数顺序构造TOTP
            // let totp = TOTP::new(
            //     algorithm,
            //     digits as usize, // 使用数据库中的位数
            //     period as u64,   // 使用数据库中的周期
            //     1,               // 容差（通常为1）
            //     secret_bytes,    // 解码后的字节数组
            // ).map_err(|e| e.to_string())?;
            let totp = TOTP::new_unchecked(
                algorithm,
                6,
                1,
                period,
                Secret::Encoded(secret).to_bytes().unwrap(),
                // Secret::Raw("JBSWY3DPEHPK3PXP".as_bytes().to_vec()).to_bytes().unwrap(),
            );

            let totp_code = totp.generate_current().unwrap().parse::<u32>().unwrap();
            Ok(TwoFactorAuthInfo {
                custom_name,
                username,
                company,
                totp_code,
                remaining_time,
                id
            })
        })
        .map_err(|e| e.to_string())?; // 转换query_map的错误

    for row in rows {
        auth_info_list.push(row.map_err(|e| e.to_string())?); // 转换每个row的错误
    }

    Ok(auth_info_list)
}
#[command]
pub fn parse_2fa_data(otpauth: &str)->(String,String) {
    let url = Url::parse(otpauth).unwrap();
    //解析otp中的账户名和公司名
    let path_parts: Vec<&str> = url.path().splitn(2, ':').collect();
    let mut path_parts_iter =  path_parts.into_iter();
    let company_part = path_parts_iter.next();
    //去除totp/部分
    // 处理 Option<&str> 类型，取出 &str 后再调用 find 方法
    let company = if let Some(part) = company_part {
        if let Some(index) = part.find('/') {
            &part[index + 1..]
        } else {
            part
        }
    } else {
        "Company is not exist"
    };
    let username = path_parts_iter.next().unwrap_or("");

    (company.to_string(),username.to_string())
}
#[command]
pub fn delete_2fa_data(id: i32) -> Result<String, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    let result = conn.execute(
        "DELETE FROM two_factor_auth WHERE id =?1",
        [id],
    );
    if let Ok(_) = result {
        Ok("删除成功".to_string())
    } else {
        Err("删除失败".to_string())
    }
}