//! 布朗运动及几何布朗运动模拟

use super::*;
use std::io;

/// 模拟标准布朗运动路径
pub fn simulating_brown_motion() ->io::Result<()>{
    println!("Enter the length of time(T):");
    let mut t_input = String::new();
    io::stdin().read_line(&mut t_input)?;
    let t: f64 = t_input.trim().parse().expect("please enter a valid number");

    println!("Enter the time perod(N)");
    let mut n_input= String::new();
    io::stdin().read_line(&mut n_input)?;
    let n: usize = n_input
        .trim()
        .parse()
        .expect("please enter a valid integer");

    let dt = t / n as f64;
    let sqrdt = dt.sqrt();

    let mut rng = rand::rng();
    let mut brownian_motion = 0.0;
    for i in 1..=n {
        let time = i as f64 * dt;

        let random_increment:f64 = rng.sample(StandardNormal);
        brownian_motion += random_increment * sqrdt;

        println!("{:<10.3} {:<10.6}", time, brownian_motion);
    }
    Ok(())
}

/// 模拟几何布朗运动路径
pub fn simulating_geo_brown_motion()->io::Result<()>{
    println!("Enter the time range(T):");
    let mut T_input=String::new();
    io::stdin().read_line(&mut T_input)?;
    let t:f64=T_input.trim().parse().expect("Please type a number!");

    println!("Enter the time period(N):");
    let mut N_input=String::new();
    io::stdin().read_line(&mut N_input)?;
    let n:usize=N_input.trim().parse().expect("Please type a integar!");

    println!("Enter the S0:");
    let mut S_input=String::new();
    io::stdin().read_line(&mut S_input)?;
    let s:f64=S_input.trim().parse().expect("Please type a number!");

    println!("Enter the mu:");
    let mut mu_input=String::new();
    io::stdin().read_line(&mut mu_input)?;
    let mu:f64=mu_input.trim().parse().expect("Please type a number!");

    println!("Enter the sigma:");
    let mut sigma_input=String::new();
    io::stdin().read_line(&mut sigma_input)?;
    let sigma:f64=sigma_input.trim().parse().expect("Please type a number!");

    let time=t / n as f64;
    let sqrtime=time.sqrt();
    let mut logS=s.ln();
    println!("{:<10.3} {:<10.6}", 0 as f64, s);
    let mut rng=rand::rng();
    for i in 1..=n{
        let cur=i as f64 *time;

        let r=rng.sample::<f64,StandardNormal>(StandardNormal);
        logS+=(mu-0.5*sigma*sigma)*time+sigma*sqrtime*r;
        println!("{:<10.3} {:<10.6}", cur, logS.exp());
    }

    Ok(())
}