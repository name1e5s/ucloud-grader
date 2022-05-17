use anyhow::anyhow;
use anyhow::Result;
use once_cell::sync::OnceCell;
use reqwest::header::ORIGIN;
use reqwest::header::REFERER;
use reqwest::header::USER_AGENT;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION};

static MY_AUTHORIZATION: OnceCell<String> = OnceCell::new();
static MY_BLADE_AUTH: OnceCell<String> = OnceCell::new();
static MY_IDENTITY: OnceCell<String> = OnceCell::new();

pub fn set_authorization(authorization: String) {
    let _ = MY_AUTHORIZATION.set(authorization);
}

pub fn set_blade_auth(blade_auth: String) {
    let _ = MY_BLADE_AUTH.set(blade_auth);
}

pub fn set_identity(identity: String) {
    let _ = MY_IDENTITY.set(identity);
}

pub fn get_header_map() -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert("authority", "apiucloud.bupt.edu.cn".parse()?);
    headers.insert(ACCEPT, "application/json".parse()?);
    headers.insert(
        "accept-language",
        "zh-CN,zh;q=0.9,en;q=0.8,zh-TW;q=0.7,ja;q=0.6".parse()?,
    );
    headers.insert(ORIGIN, "https://ucloud.bupt.edu.cn".parse()?);
    headers.insert(REFERER, "https://ucloud.bupt.edu.cn/".parse()?);
    headers.insert(
        USER_AGENT,
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36".parse()?,
    );
    headers.insert("tenant-id", "000000".parse()?);
    headers.insert(
        AUTHORIZATION,
        MY_AUTHORIZATION
            .get()
            .ok_or_else(|| anyhow!("get authority failed"))?
            .parse()?,
    );
    headers.insert(
        "blade-auth",
        MY_BLADE_AUTH
            .get()
            .ok_or_else(|| anyhow!("get blade-auth failed"))?
            .parse()?,
    );
    headers.insert(
        "identity",
        MY_IDENTITY
            .get()
            .ok_or_else(|| anyhow!("get identity failed"))?
            .parse()?,
    );
    Ok(headers)
}
