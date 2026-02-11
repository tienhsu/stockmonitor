use anyhow::{Context, Result};
use crate::models::PriceUpdate;
use super::DataSource;

/// 新浪财经行情 API 适配器
pub struct SinaSource {
    client: reqwest::Client,
}

impl SinaSource {
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
impl DataSource for SinaSource {
    fn name(&self) -> &str {
        "sina"
    }

    /// 批量获取股票数据
    /// stocks: Vec<(market, code)> 如 [("sh", "600519"), ("sz", "000001")]
    async fn fetch(&self, stocks: &[(String, String)]) -> Result<Vec<PriceUpdate>> {
        if stocks.is_empty() {
            return Ok(vec![]);
        }

        // 拼接股票代码列表，如 "sh600519,sz000001"
        let codes: Vec<String> = stocks
            .iter()
            .map(|(market, code)| format!("{}{}", market, code))
            .collect();
        let codes_str = codes.join(",");

        let url = format!("https://hq.sinajs.cn/list={}", codes_str);
        let resp = self
            .client
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await
            .context("新浪API请求失败")?;

        let text = resp.text().await.context("读取新浪API响应失败")?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut results = Vec::new();

        for line in text.lines() {
            if let Some(update) = parse_sina_line(line, now) {
                results.push(update);
            }
        }

        Ok(results)
    }
}

/// 解析新浪 API 单行数据
/// 格式: var hq_str_sh600519="贵州茅台,1750.00,1740.00,1755.00,1760.00,1745.00,...";
fn parse_sina_line(line: &str, timestamp: u64) -> Option<PriceUpdate> {
    // 提取股票ID，如 "sh600519"
    let id_start = line.find("hq_str_")? + 7;
    let id_end = line[id_start..].find('=')?;
    let full_id = &line[id_start..id_start + id_end];

    // 提取引号内的数据
    let data_start = line.find('"')? + 1;
    let data_end = line.rfind('"')?;
    if data_start >= data_end {
        return None; // 空数据（停牌等）
    }
    let data = &line[data_start..data_end];
    let fields: Vec<&str> = data.split(',').collect();

    if fields.len() < 6 {
        return None;
    }

    let name = fields[0].to_string();
    let open: f64 = fields[1].parse().ok()?;
    let prev_close: f64 = fields[2].parse().ok()?;
    let price: f64 = fields[3].parse().ok()?;
    let high: f64 = fields[4].parse().ok()?;
    let low: f64 = fields[5].parse().ok()?;

    // 跳过价格为 0 的（未开盘/停牌）
    if price == 0.0 || prev_close == 0.0 {
        // 使用开盘价或昨收价
        let effective_price = if open > 0.0 { open } else { prev_close };
        if effective_price == 0.0 {
            return None;
        }
    }

    let change = price - prev_close;
    let percent = if prev_close > 0.0 {
        change / prev_close
    } else {
        0.0
    };

    // 从 full_id 提取 market 和 code
    let market = if full_id.starts_with("sh") {
        "sh"
    } else {
        "sz"
    };
    let code = &full_id[2..];

    Some(PriceUpdate {
        id: full_id.to_string(),
        code: code.to_string(),
        market: market.to_string(),
        name,
        price,
        prev_close,
        change,
        percent,
        high,
        low,
        timestamp,
        source: "sina".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sina_fetch() {
        let source = SinaSource::new();
        // sh600519 茅台
        let stocks = vec![("sh".to_string(), "600519".to_string())];
        let result = source.fetch(&stocks).await;
        if let Err(e) = &result {
            eprintln!("Sina Error: {:?}", e);
        }
        assert!(result.is_ok());
        let updates = result.unwrap();
        assert!(!updates.is_empty());
        let update = &updates[0];
        assert_eq!(update.code, "600519");
        println!("Sina Fetch Result: {:?}", update);
    }
}
