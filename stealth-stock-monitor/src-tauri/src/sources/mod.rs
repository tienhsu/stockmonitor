pub mod sina;
pub mod tencent;
pub mod eastmoney;

use anyhow::Result;
use crate::models::PriceUpdate;

/// 数据源统一接口
#[async_trait::async_trait]
pub trait DataSource: Send + Sync {
    /// 数据源名称
    fn name(&self) -> &str;

    /// 批量获取股票数据
    async fn fetch(&self, stocks: &[(String, String)]) -> Result<Vec<PriceUpdate>>;
}

/// 根据股票代码自动识别市场
/// 规则：6 开头 -> sh, 0/3 开头 -> sz
pub fn detect_market(code: &str) -> &str {
    match code.chars().next() {
        Some('6') => "sh",
        Some('0') | Some('3') => "sz",
        _ => "sh",
    }
}

/// 生成股票 ID（market + code）
pub fn make_stock_id(market: &str, code: &str) -> String {
    format!("{}{}", market, code)
}
