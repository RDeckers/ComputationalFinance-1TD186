extern crate rand;
extern crate time;
use time::PreciseTime;
use rand::distributions::{Normal, IndependentSample};
use rand::*;
use std::ops::{Add,Div,Mul,Sub};
use std::marker::{Copy};
use std::num::{Zero};
use std::default::Default;

pub fn variance<T : Default + Add<T, Output = T> + Mul<T, Output = T> + Copy + Div<f64, Output = f64>>(data : &Vec<T>) -> f64{
  let mut sum = T::default(); //Should use Zero here, but not yet stable.
  let mut sum_sq = T::default(); //Should use Zero here, but not yet stable.
  let len = data.len();
  for i in 0..len{
    sum = sum + data[i];
    sum_sq = sum_sq + data[i]*data[i];
  }
  let mean = sum/(data.len() as f64);
  let mean_of_sq = sum_sq/(data.len() as f64);
  mean_of_sq - mean*mean
}

pub fn covariance<T : Default + Add<T, Output = T> + Sub<f64, Output = T> + Mul<T, Output = T> + Copy + Div<f64, Output = f64>>(x : &Vec<T>, y : &Vec<T>) -> f64{
  let mean_x = mean(x);
  let mean_y = mean(y);
  let mut sum = T::default(); //Should use Zero here, but not yet stable.
  let len = x.len(); //check that y is the same length
  for i in 0..len{
    sum = sum + (x[i]-mean_x)*(y[i]-mean_y);
  }
  sum/(x.len() as f64)
}

pub fn std_dev<T : Default + Add<T, Output = T> + Mul<T, Output = T> + Copy + Div<f64, Output = f64>>(data : &Vec<T>) -> f64{
  variance(data).sqrt()
}

pub fn mean<T : Default + Add<T, Output = T> + Copy + Div<f64, Output = f64>>(data : &Vec<T>) -> f64{
  let mut sum = T::default(); //Should use Zero here, but not yet stable.
  let len = data.len();
  for i in 0..len{
    sum = sum + data[i];
  }
  sum/(data.len() as f64)
}

pub fn bootstrap<T : Default + Add<T, Output = T> + Copy + Div<f64, Output = f64>>(data : &Vec<T>, samples : usize )-> Vec<f64>{
  let mut boots = Vec::<f64>::with_capacity(samples);
  let mut buff = Vec::<T>::with_capacity(data.len());
  let mut rng = thread_rng();
  for _ in 0..samples{//TODO: multithread
    buff.clear();
    for __ in 0..data.len(){
      let index = (rng.next_u32() as usize) % data.len();
      buff.push(data[index]);
    }
    boots.push(mean(&buff));//TODO: Don't use mean but pass a function as an argument.
  }
  boots
}

fn signed_power(x : f64, y : f64) -> f64{ //this is how matlabs deals with fratcional powrs of negative numbers.
  if x < 0.0 {
   -1.0*x.abs().powf(y)
  }else{
    x.abs().powf(y)
  }
}

fn step_stock_euler(stock : f64, volatility : f64, gamma : f64, interest: f64, delta_t : f64, delta_w : f64) -> f64{
  let volatility_term = delta_w*volatility*signed_power(stock, gamma);
  stock*(1.0+interest*delta_t)+volatility_term
}



fn step_stock_rk(stock : f64, volatility : f64, gamma : f64, interest: f64, delta_t : f64, delta_w : f64) -> f64{
  let stock_gamma = signed_power(stock, gamma);
  let stock_hat = stock + interest*stock*delta_t+delta_t.sqrt()*volatility*stock_gamma;
  let stock_hat_gamma = signed_power(stock_hat, gamma);
  stock + interest*delta_t*stock + delta_w*volatility*stock_gamma
    +1.0/(2.0*delta_t.sqrt())*(volatility*stock_hat_gamma-volatility*stock_gamma) * (delta_w*delta_w-delta_t)
}

fn step_stock_millstein(stock : f64, volatility : f64, gamma : f64, interest: f64, delta_t : f64, delta_w : f64) -> f64{
  step_stock_euler(stock, volatility, gamma, interest, delta_t, delta_w)
    + 0.5*gamma*volatility*volatility*signed_power(stock, 2.0*gamma-1.0)*(delta_w*delta_w - delta_t)
}

pub fn simulate_option(
  initial_stock : f64,
  strikeout :f64,
  interest : f64,
  volatility : f64,
  gamma: f64,
  end_time : f64,
  delta_t : f64,
  n_runs: usize
  ) -> Vec<f64>{
    //let mut runs_1 = Vec::<f64>::with_capacity(n_runs);
    //let mut runs_2 = Vec::<f64>::with_capacity(n_runs);
    let mut runs_combined = Vec::<f64>::with_capacity(n_runs);
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0);
    for _ in 0..n_runs{
      let mut t = 0.0;
      let mut stock = initial_stock;
      let mut stock_2 = initial_stock;
      while t < end_time {
        let delta_w = normal.ind_sample(&mut rng)*delta_t.sqrt();
        let delta_w_2 = -delta_w;//normal.ind_sample(&mut rng)*delta_t.sqrt(); //idd just as good as negative?!
        //println!("{}", delta_w/delta_w_2);
        stock = step_stock_millstein(stock, volatility, gamma, interest, delta_t, delta_w);
        stock_2 = step_stock_millstein(stock_2, volatility, gamma, interest, delta_t, delta_w_2); //no better then taking a new idd delta_w?
        t = t + delta_t;
      }
      //println!("{}", correlation);
      let mut profit_1 = (stock-strikeout)*((-interest*end_time).exp());
      if profit_1.is_sign_negative(){
        profit_1 = 0.0;
      }
      let mut profit_2 = (stock_2-strikeout)*((-interest*end_time).exp());
      if profit_2.is_sign_negative(){
        profit_2 = 0.0;
      }
      //runs_1.push(1.0*profit_1+0.0*profit_2); //Antithetic variates, see http://www.columbia.edu/~ks20/4703-Sigman/4703-07-Notes-ATV.pdf , section 1.4
      //runs_2.push(0.0*profit_1+1.0*profit_2); //Antithetic variates, see http://www.columbia.edu/~ks20/4703-Sigman/4703-07-Notes-ATV.pdf , section 1.4
      runs_combined.push(0.5*profit_1+0.5*profit_2); //Antithetic variates, see http://www.columbia.edu/~ks20/4703-Sigman/4703-07-Notes-ATV.pdf , section 1.4
    }
    runs_combined
  }

  fn main() {
    let start = PreciseTime::now();
    let mut total_runs = 100_000;
    let exact = 0.199889972822117;
    println!("Exact solution: {}\n\n", exact);
    //let variance_1 = variance(&runs_1);
    //let variance_2 = variance(&runs_2);
    //let covariance = covariance(&runs_1, &runs_2);
    //println!("Normal mean  1 : {} +- {}", mean(&runs_1), variance_1);
    //println!("Normal mean  2 : {} +- {}", mean(&runs_2), variance_2);
    //println!("covariance : {}", covariance);
    //println!("Expected variance : {}", 0.25*variance_1+0.25*variance_2+0.5*covariance);
    let mut dt = 0.04;
    while dt > 0.000099{
    //while total_runs <= 100_000{
      let mut runs_combined = simulate_option(12.0, 15.0, 0.1, 0.25, 1.0, 0.5, dt, total_runs);
      let mean_x = mean(&runs_combined);
      println!("{},{}", dt, (mean_x-exact).abs());
      dt *= 0.85
      // let var = variance(&runs_combined);
      // println!("var\t{}", var);
      // for i in 0..runs_combined.len(){
        // runs_combined[i] = (runs_combined[i]-exact)*(runs_combined[i]-exact);
      // }
      // let mse = mean(&runs_combined);
      // println!("mse \t{}", mse);
      // println!("bias \t{}", (mse-var).sqrt());
      //println!("{{{}, {}}},", dt, ((mean(&runs_combined)-exact).powf(2.0)-variance(&runs_combined)).sqrt());
      //dt *= 0.9;
    // }/
    //let boots = bootstrap(&runs, 500);
    //println!("Bootstrapped mean: {} +- {}", mean(&boots), variance(&boots)*(total_runs as f64));
  }
    let end = PreciseTime::now();
    println!("Completed in {}s", start.to(end));
  }
