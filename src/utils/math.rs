use crate::errors::*;
/// 计算百分比值（用于风险价值等计算）
pub fn calc_percentage(data: &mut [f64], pct: f64) -> Result<f64> {
    if data.is_empty() {
        return Err(OptionError::InvalidParameter("Data is empty".to_string()));
    }
    if pct < 0.0 || pct > 1.0 {
        return Err(OptionError::InvalidParameter("Percentage must between 0 and 1.0".to_string()));
    }

    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = data.len() as f64;
    let rank = pct * (n - 1.0);
    let lower_rank = rank.floor() as usize;
    let upper_rank = rank.ceil() as usize;

    // 整数秩直接返回对应值
    if lower_rank == upper_rank {
        Ok(data[lower_rank])
    } else {
        // 线性插值计算
        let fraction = rank - lower_rank as f64;
        Ok(data[lower_rank] * (1.0 - fraction) + data[upper_rank] * fraction)
    }
}