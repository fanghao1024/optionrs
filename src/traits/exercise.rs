use std::any::Any;
/// define the interface for exercise rules
/// 定义行权规则接口
pub trait ExerciseRule:Send+Sync{
    /// Determine whether to exercise the right at a given time point <br>
    /// 判断在给定时点是否应行权
    /// # parameters
    /// + time: time until expiration(in year) 距离到期的时间（年）
    /// + spot: current underlying asset price 当前标的资产价格
    /// + intrinsic_value: intrisic value 内在价值
    /// + continuation_value: continue holding value 继续持有价值
    fn should_exercise(
        &self,
        _remaining_time:f64,
        _spot:f64,
        _intrinsic_value:f64,
        _continuation_value:f64
    ) ->bool{
        false
    }

    fn is_european(&self)->bool;

    fn as_any(&self)->&dyn Any;
    
}

/// European exercise rule <br>
/// 欧式行权规则
#[derive(Debug,Clone,Copy)]
pub struct EuropeanExercise;

impl EuropeanExercise{
    pub fn new()->Self{
        Self
    }
}
impl ExerciseRule for EuropeanExercise{
    fn should_exercise(
        &self,
        time: f64,
        _spot: f64,
        _intrinsic_value: f64,
        _continuation_value: f64
    ) -> bool {
        time<1e-9
    }

    fn is_european(&self) -> bool {
        true
    }

    fn as_any(&self)->&dyn Any{
        self
    }

}

/// American exercise rule <br>
/// 美式行权规则
pub struct AmericanExercise;

impl ExerciseRule for AmericanExercise{
    fn should_exercise(
        &self,
        _time: f64,
        _spot: f64,
        intrinsic_value: f64,
        continuation_value: f64
    ) -> bool {
        intrinsic_value>continuation_value
    }

    fn is_european(&self) -> bool {
        false
    }

    fn as_any(&self)->&dyn Any{
        self
    }

}
