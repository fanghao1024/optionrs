//! 通用数学工具函数

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

/// 一维线性插值
///
/// # parameter
/// - `x`: 待插值的x作标
/// - `x_min`: 网格起始坐标
/// - `dx`: 网格步长
/// - `grid`: 已知网格值（假设均匀分布）
///
/// # return
/// 插值结果 `y=y_left + (y_right - y_left) * (x - x_left) / dx`
///
/// # 边界处理
/// - x超出左边界：返回第一个网格值
/// - x超出右边界：返回最后一个网格值
///
/// # example:
/// ```rust
/// use assert_approx_eq::assert_approx_eq;
/// use optionrs::utils::math::linear_interpolate;
///
/// let grid=vec![10.0,20.0,30.0];
/// let y=linear_interpolate(1.5,1.0,1.0,&grid)?;
/// assert_approx_eq!(y,15.0);
/// ```
pub fn linear_interpolate(x: f64, x_min: f64, dx: f64,grid: &[f64])->Result<f64>{
    if grid.is_empty() {
        return Err(OptionError::InvalidParameter("Grid is empty".to_string()));
    }

    let i_float=(x-x_min)/dx;
    let i_floor=i_float.floor() as usize;
    let i_ceil=i_floor+1;

    // 边界保护
    if i_floor>=grid.len()-1{
        return Ok(grid[grid.len()-1]);
    }
    if i_float<0.0{
        return Ok(grid[0]);
    }

    let weight=i_float-i_floor as f64;
    Ok(grid[i_floor]*(1.0-weight)+grid[i_ceil]*weight)
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_linear_interpolate_inside() ->Result<()> {
        let grid = vec![10.0, 20.0, 30.0];
        assert_approx_eq!(linear_interpolate(1.5, 1.0, 1.0, &grid)?, 15.0);
        assert_approx_eq!(linear_interpolate(-1.0, 1.0, 1.0, &grid)?, 10.0);
        assert_approx_eq!(linear_interpolate(5.0, 1.0, 1.0, &grid)?, 30.0);
        assert_approx_eq!(linear_interpolate(2.0, 1.0, 1.0, &grid)?, 20.0);
        Ok(())
    }

}
