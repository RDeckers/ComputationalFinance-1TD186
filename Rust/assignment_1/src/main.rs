extern crate rand;
use rand::distributions::{Normal, IndependentSample};
use rand::*;
use std::ops::{Add,Div,Mul};
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

pub fn simulate_option(
  initial_stock : f64,
  strikeout :f64,
  interest : f64,
  volatility : f64,
  gamma: f64,
  end_time : f64,
  delta_t : f64
  ) -> f64{
    let mut stock = initial_stock;
    let mut stock_2 = initial_stock;
    let mut t = 0.0;
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, delta_t.sqrt());
    while t < end_time {
      let delta_w = normal.ind_sample(&mut rng);
      //let StandardNormal(delta_w) = rng.gen();
      //println!("{}", delta_w);
      /*if stock < 0.0 {
        return 0.0; //discard when <0.0 because volatility_term becomes NaN.
      }
      let volatility_term = delta_w*volatility*(stock.powf(gamma));*/
      let volatility_term = {
        if stock < 0.0 {//this is how Matlab handles fractional exponentiation with negative bases.
          -delta_w*volatility*(stock.abs().powf(gamma))
          } else {
            delta_w*volatility*(stock.powf(gamma))
          }
        };
        stock = stock*(1.0+interest*delta_t)+volatility_term;
        stock_2 = stock*(1.0+interest*delta_t)-volatility_term;
        //println!("{}", stock);
        t = t + delta_t;
      }
      let mut profit_1 = (stock-strikeout)*((-interest*end_time).exp());
      if profit_1.is_sign_negative(){
        profit_1 = 0.0;
      }
      let mut profit_2 = (stock_2-strikeout)*((-interest*end_time).exp()); //cap these at 0 _before_ blending
      if profit_2.is_sign_negative(){
        profit_2 = 0.0;
      }
      0.5*profit_1+0.5*profit_2
    }

    fn main() {
      let total_runs = 10_000;
      let mut runs = Vec::<f64>::with_capacity(total_runs);
      for _ in 0..total_runs{
        let option = simulate_option(12.0, 15.0, 0.1, 0.25, 0.8, 0.5, 1.0/256.0);
        runs.push(option);
      }
      let boots = bootstrap(&runs, 5000);
      println!("Normal mean: {} +- {}", mean(&runs), variance(&runs));
      println!("Bootstrapped mean: {} +- {}", mean(&boots), variance(&boots)*(total_runs as f64));
    }
