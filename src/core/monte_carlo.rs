//! The specific implementation of Monte Carlo Engine
//! 蒙特卡洛引擎的具体实现
use rand::{SeedableRng, rngs::StdRng, RngCore};
use std::any::Any;
use std::sync::Arc;
use crate::traits::engine::{PriceEngine, GreeksEngine, MonteCarloEngineExt};
use crate::traits::{payoff::Payoff,exercise::ExerciseRule,process::StochasticProcess};
use crate::params::common::CommonParams;
use crate::errors::*;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

/// Monte Carlo Engine
#[derive(Debug,Clone)]
pub struct MonteCarloEngine{
    num_simulations: usize,        //模拟次数
    time_steps: usize,             //时间步数
    process: Option<Arc<dyn StochasticProcess>>, //随机过程
    use_antithetic:bool,           //是否启用对偶
    use_parallel:bool,            //是否开启并行
    seed:u64,                   //随机数种子
}

impl MonteCarloEngine {
    pub fn new(
        num_simulations: usize,
        time_steps: usize,
        process: Option<Arc<dyn StochasticProcess>>,
        use_antithetic:bool,
        use_parallel:bool,
        seed:u64
    ) -> Result<Self>{
        if num_simulations<1000{
            return Err(OptionError::InvalidParameter("Simulation number cannot be below 1000".to_string()));
        }
        if time_steps<1{
            return Err(OptionError::InvalidParameter("Time steps must be over 0".to_string()));
        }
        let use_parallel = use_parallel && num_simulations>1000;
        Ok(Self{
            num_simulations,
            time_steps,
            process,
            use_antithetic,
            use_parallel,
            seed,
        })
    }

    fn create_rng(&self) -> Result<StdRng> {
        // 若要复现结果，用固定种子；否则用系统随机种子
        if self.seed!=0{
            Ok(StdRng::seed_from_u64(self.seed))
        }else{
            Ok(StdRng::from_os_rng())
        }
    }

    pub fn set_antithetic(&mut self, use_antithetic:bool){
        self.use_antithetic = use_antithetic;
    }

    fn simulate_single_path(
        &self,
        initial_price:f64,
        time_horizon:f64,
        rng:&mut StdRng
    )->Result<Vec<f64>>{
        let mut process=self.process
            .as_ref()
            .expect("Stochastic process must be initialized before simulation")
            .clone_box();
        // 为这个副本设置独立的种子，保证路径间的随机性
        process.init_rng_with_seed(rng.next_u64());
        process.simulate_path(initial_price, time_horizon,self.time_steps)
    }

    pub fn simulate_paths(
        &self,
        initial_price:f64,
        time_horizon:f64,
    )->Result<Vec<Vec<f64>>>{
        let mut master_rng=self.create_rng()?;
        let mut all_paths=Vec::with_capacity(self.num_simulations);

        let pb:ProgressBar;
        if self.use_antithetic{
            let num_pairs=self.num_simulations/2;
            pb=self.create_progress_bar(num_pairs as u64);
            for _ in 0..num_pairs{
                let mut process=self.process
                    .as_ref()
                    .expect("Stochastic process must be initialized before simulation")
                    .clone_box();
                process.init_rng_with_seed(master_rng.next_u64());
                let (path1,path2)=process.simulate_antithetic_path(initial_price,time_horizon,self.time_steps)?;
                all_paths.push(path1);
                all_paths.push(path2);
                pb.inc(1);
            }
            if self.num_simulations%2==1{
                let mut process=self.process.as_ref().unwrap().clone_box();
                process.init_rng_with_seed(master_rng.next_u64());
                let path=process.simulate_path(initial_price,time_horizon,self.time_steps)?;
                all_paths.push(path);
            }
        }else{
            pb=self.create_progress_bar(self.num_simulations as u64);
            for _ in 0..self.num_simulations{
                let path=self.simulate_single_path(initial_price,time_horizon,&mut master_rng)?;
                all_paths.push(path);
                pb.inc(1);
            }
        }
        pb.finish_with_message("Simulation finished");
        Ok(all_paths)
    }

    pub fn simulate_paths_parallel(
        &self,
        initial_price:f64,
        time_horizon:f64,
    )->Result<Vec<Vec<f64>>>{
        // 需要设置主种子来确保可复现
        let mut master_rng=StdRng::seed_from_u64(self.seed);
        let num_seeds=if self.use_antithetic{self.num_simulations/2}else{self.num_simulations};
        // 预先生成所有子种子
        // 并行计算中不能共享同一个master_rng,所以先在主线程批量生成种子
        let seeds:Vec<u64>=(0..num_seeds).map(|_| master_rng.next_u64()).collect();
        let pb=self.create_progress_bar(num_seeds as u64);
        let paths=if self.use_antithetic{
                let results:Vec<Vec<Vec<f64>>>=seeds.into_par_iter().map(|seed|{
                    let mut process=self.process
                        .as_ref()
                        .expect("Process missing")
                        .clone_box();
                    let (p1,p2)=process.simulate_antithetic_path(initial_price,time_horizon,self.time_steps).expect("Simulating antithet error");
                    pb.inc(1);
                    vec![p1,p2]
                }).collect();
                Ok(results.into_iter().flatten().collect())
            }else{
                seeds.into_par_iter().map(|seed|{
                    let mut process=self.process
                        .as_ref()
                        .expect("Process missing")
                        .clone_box();
                    process.init_rng_with_seed(seed);
                    pb.inc(1);
                    process.simulate_path(initial_price,time_horizon,self.time_steps)
                }).collect()
            };
        pb.finish_with_message("Simulation finished");
        paths
    }

    fn calculate_total_payoff_serial(
        &self,
        s0:f64,
        t:f64,
        payoff:&dyn Payoff,
    )->Result<f64>{
        let mut rng=self.create_rng()?;
        let mut total_payoff=0.0f64;
        let iters=if self.use_antithetic{self.num_simulations/2}else{self.num_simulations};

        let pb=self.create_progress_bar(iters as u64);

        for _ in 0..iters{
            let mut process=self.process.as_ref().unwrap().clone_box();
            process.init_rng_with_seed(rng.next_u64());


            if self.use_antithetic{
                let (path1,path2)=process.simulate_antithetic_path(s0,t,self.time_steps)?;
                total_payoff+=payoff.path_dependent_payoff(&path1)+payoff.path_dependent_payoff(&path2);
            }else{
                let path=process.simulate_path(s0,t,self.time_steps)?;
                total_payoff+=payoff.path_dependent_payoff(&path);
            }
            pb.inc(1);
        }
        pb.finish_with_message("Simulation finished");
        Ok(total_payoff)
    }

    fn calculate_total_payoff_parallel(
        &self,
        s0:f64,
        t:f64,
        payoff:&dyn Payoff,
    )->Result<f64>{
        let mut master_rng=self.create_rng()?;
        let num_seeds=if self.use_antithetic{self.num_simulations/2}else{self.num_simulations};
        let seeds:Vec<u64>=(0..num_seeds).map(|_| master_rng.next_u64()).collect();

        let pb=self.create_progress_bar(num_seeds as u64);
        // 使用rayon并行处理
        let total_payoff:f64=seeds.into_par_iter().map(|seed|{
            let mut process=self.process.as_ref().unwrap().clone_box();
            process.init_rng_with_seed(seed);

            let val=if self.use_antithetic{
                process.simulate_antithetic_path(s0,t,self.time_steps)
                    .map(|(path1,path2)| payoff.path_dependent_payoff(&path1)+payoff.path_dependent_payoff(&path2))
                    .unwrap_or(0.0)
            }else{
                process.simulate_path(s0,t,self.time_steps)
                    .map(|path| payoff.path_dependent_payoff(&path))
                    .unwrap_or(0.0)
            };
            pb.inc(1);
            val
        }).sum();
        pb.finish_with_message("Simulation finished");
        Ok(total_payoff)

    }

    fn create_progress_bar(&self,len:u64)->ProgressBar{
        let pb=ProgressBar::new(len as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green}[{elasped_precies}][{bar:40.cyan/blue}]{pos}/{len}({eta})")
            .unwrap()
            .progress_chars("#>-"));
        pb
    }
}

impl MonteCarloEngineExt for MonteCarloEngine {
    fn set_process(&mut self, process: Arc<dyn StochasticProcess>) {
        self.process=Some(process);
    }

    fn set_num_simulation(&mut self, num: usize) -> Result<()> {
        if num==0{
            return Err(OptionError::InvalidParameter("Simulation number must be greater than 0".to_string()));
        }
        self.num_simulations=num;
        Ok(())
    }

    fn set_time_steps(&mut self, time_steps: usize) -> Result<()> {
        if time_steps==0{
            return Err(OptionError::InvalidParameter("Time steps must be greater than 0".to_string()));
        }
        self.time_steps=time_steps;
        Ok(())
    }
}


impl PriceEngine for MonteCarloEngine {
    fn calculate_price(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        _exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {

        if self.process.is_none(){
            return Err(OptionError::NotSet("Process not set".to_string()));
        }

        let s0=params.spot();
        let t=params.time_to_maturity();

        let total_payoff=if self.use_parallel{
            self.calculate_total_payoff_parallel(s0,t,payoff)?
        }else{
            self.calculate_total_payoff_serial(s0,t,payoff)?
        };

        let avg_payoff=total_payoff/self.num_simulations as f64;
        let discount=(-params.risk_free_rate()*t).exp();
        Ok(avg_payoff*discount)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GreeksEngine for MonteCarloEngine {}
