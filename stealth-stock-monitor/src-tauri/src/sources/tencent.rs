use anyhow::{Context, Result};
use crate::models::PriceUpdate;
use super::DataSource;

/// 腾讯证券行情 API 适配器
pub struct TencentSource {
    client: reqwest::Client,
}

impl TencentSource {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }
}

#[async_trait::async_trait]
impl DataSource for TencentSource {
    fn name(&self) -> &str {
        "tencent"
    }

    async fn fetch(&self, stocks: &[(String, String)]) -> Result<Vec<PriceUpdate>> {
        if stocks.is_empty() {
            return Ok(vec![]);
        }

        let codes: Vec<String> = stocks
            .iter()
            .map(|(market, code)| format!("{}{}", market, code))
            .collect();
        let codes_str = codes.join(",");

        let url = format!("http://qt.gtimg.cn/q={}", codes_str);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("腾讯API请求失败")?;

        let text = resp.text().await.context("读取腾讯API响应失败")?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut results = Vec::new();

        for line in text.lines() {
            if let Some(update) = parse_tencent_line(line, now) {
                results.push(update);
            }
        }

        Ok(results)
    }
}

/// 解析腾讯 API 单行数据
/// 格式: v_sh600519="1~贵州茅台~600519~1755.00~1740.00~...";
/// 字段以 ~ 分隔:
/// [0]market_id [1]名称 [2]代码 [3]现价 [4]昨收 [5]今开 [6]成交量
/// [31]最高 [32]最低
fn parse_tencent_line(line: &str, timestamp: u64) -> Option<PriceUpdate> {
    // 提取 ID
    let id_start = line.find("v_")? + 2;
    let id_end = line[id_start..].find('=')?;
    let full_id = &line[id_start..id_start + id_end];

    // 提取引号内数据
    let data_start = line.find('"')? + 1;
    let data_end = line.rfind('"')?;
    if data_start >= data_end {
        return None;
    }
    let data = &line[data_start..data_end];
    let fields: Vec<&str> = data.split('~').collect();

    if fields.len() < 33 {
        return None;
    }

    let name = fields[1].to_string();
    let code = fields[2].to_string();
    let price: f64 = fields[3].parse().ok()?;
    let prev_close: f64 = fields[4].parse().ok()?;
    let high: f64 = fields[33].parse().unwrap_or(price);
    let low: f64 = fields[34].parse().unwrap_or(price);

    if price == 0.0 || prev_close == 0.0 {
        return None;
    }

    let change = price - prev_close;
    let percent = change / prev_close;

    let market = if full_id.starts_with("sh") {
        "sh"
    } else {
        "sz"
    };

    Some(PriceUpdate {
        id: full_id.to_string(),
        code,
        market: market.to_string(),
        name,
        price,
        prev_close,
        change,
        percent,
        high,
        low,
        timestamp,
        source: "tencent".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tencent_fetch() {
        let source = TencentSource::new();
        let stocks = vec![("sh".to_string(), "600519".to_string())];
        let result = source.fetch(&stocks).await;
        if let Err(e) = &result {
            eprintln!("Tencent Error: {:?}", e);
        }
        assert!(result.is_ok());
        let updates = result.unwrap();
        assert!(!updates.is_empty());
        let update = &updates[0];
        assert_eq!(update.code, "600519");
        println!("Tencent Fetch Result: {:?}", update);
    }
}
