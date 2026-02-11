use anyhow::{Context, Result};
use crate::models::PriceUpdate;
use super::DataSource;

/// 东方财富行情 API 适配器
pub struct EastmoneySource {
    client: reqwest::Client,
}

impl EastmoneySource {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }

    /// 将 sh/sz 前缀转换为东方财富的 secid 格式
    /// sh -> 1, sz -> 0
    fn to_secid(market: &str, code: &str) -> String {
        let market_id = match market {
            "sh" => "1",
            "sz" => "0",
            _ => "1",
        };
        format!("{}.{}", market_id, code)
    }
}

#[async_trait::async_trait]
impl DataSource for EastmoneySource {
    fn name(&self) -> &str {
        "eastmoney"
    }

    async fn fetch(&self, stocks: &[(String, String)]) -> Result<Vec<PriceUpdate>> {
        if stocks.is_empty() {
            return Ok(vec![]);
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut results = Vec::new();

        // 东方财富 API 需要逐个请求（或使用批量接口）
        for (market, code) in stocks {
            let secid = Self::to_secid(market, code);
            match self.fetch_single(&secid, market, code, now).await {
                Ok(Some(update)) => results.push(update),
                Ok(None) => {} // 数据不可用
                Err(e) => {
                    log::warn!("东财API获取 {} 失败: {}", code, e);
                }
            }
        }

        Ok(results)
    }
}

impl EastmoneySource {
    async fn fetch_single(
        &self,
        secid: &str,
        market: &str,
        code: &str,
        timestamp: u64,
    ) -> Result<Option<PriceUpdate>> {
        let url = format!(
            "http://push2.eastmoney.com/api/qt/stock/get?secid={}&fields=f43,f44,f45,f46,f57,f58,f60,f170",
            secid
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("东财API请求失败")?;

        let json: serde_json::Value = resp.json().await.context("解析东财API JSON失败")?;
        let data = json.get("data").and_then(|d| d.as_object());

        let data = match data {
            Some(d) => d,
            None => return Ok(None),
        };

        // f43: 现价(分), f44: 最高(分), f45: 最低(分), f46: 今开(分)
        // f57: 代码, f58: 名称, f60: 昨收(分), f170: 涨跌幅(百分比*100)
        let price = data.get("f43").and_then(|v| v.as_f64()).unwrap_or(0.0) / 100.0;
        let high = data.get("f44").and_then(|v| v.as_f64()).unwrap_or(0.0) / 100.0;
        let low = data.get("f45").and_then(|v| v.as_f64()).unwrap_or(0.0) / 100.0;
        let prev_close = data.get("f60").and_then(|v| v.as_f64()).unwrap_or(0.0) / 100.0;
        let name = data
            .get("f58")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if price == 0.0 || prev_close == 0.0 {
            return Ok(None);
        }

        let change = price - prev_close;
        let percent = change / prev_close;

        Ok(Some(PriceUpdate {
            id: format!("{}{}", market, code),
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
            source: "eastmoney".to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_eastmoney_fetch() {
        // use crate::sources::DataSource; // already imported by super::*? No, use super::DataSource.
        // Actually DataSouce is imported in line 3.
        
        // EastmoneySource implementation is separate.
        // Let's test fetch method.
        let source = EastmoneySource::new();
        // sh600519
        let stocks = vec![("sh".to_string(), "600519".to_string())];
        
        // We'll call fetch which calls fetch_single
        let result = source.fetch(&stocks).await; 
        
        if let Err(e) = &result {
             eprintln!("Eastmoney Error: {:?}", e);
        }
        
        assert!(result.is_ok());
        let updates = result.unwrap();
        // 東财 might return empty if fetch_single fails silently (logs warn)
        // But for 600519 it should work.
        if updates.is_empty() {
             panic!("Eastmoney returned empty results for specific stock");
        }
        
        let update = &updates[0];
        assert_eq!(update.code, "600519");
        println!("Eastmoney Fetch Result: {:?}", update);
        
        // Check price sanity (Moutai is around 1700)
        // If it is 17.00, then /100 was wrong.
        if update.price < 100.0 {
             panic!("Price seems too low: {}. Check /100.0 logic.", update.price);
        }
    }
}
