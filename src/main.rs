use dialoguer::{ Select, theme::ColorfulTheme };
use console::Term;
use rand::distributions::{Distribution, Uniform};
use std::f32;

struct Result(f32,f32,f32, i32); // [b, x1, x2, iteration Count]
struct DataLine(f32, f32, i32); // [x1, x2, t]
static DATA_SET: [DataLine; 4] = [
  DataLine(-0.3, 0.6, 0),
  DataLine(0.3, -0.6, 0),
  DataLine(1.2, -1.2, 1),
  DataLine(1.2, 1.2, 1),
];

fn step_activation(a: f32) -> i32 {
  return if a >= 0.0 { 1 } else { 0 } 
} 

fn sigmoid_activation(a: f32) -> i32 {
  let e = std::f32::consts::E;
  let result = 1.0 / (1.0 + e.powf(-a));
  return if result >= 0.5 { 1 } else { 0 }
}

fn main() -> std::io::Result<()> {
  let items = vec!["Slenkstine funkcija", "Sigmoidine funkcija"];
  
  let term = Term::stdout();
  print!("{}[2J", 27 as char);
  term.write_line("Kokia aktyvacijos funkcija norite naudoti?")?;
  
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&items)
    .default(0)
    .interact_on_opt(&Term::stderr())?;

  match selection {
    Some(0) => calculate_wheights(step_activation),
    Some(1) => calculate_wheights(sigmoid_activation),
    Some(i) => println!("Neregistruotas pasirinkimas {}", i),
    None => println!("User did not select anything")
  }

  Ok(())
}

fn calculate_wheights(activation_f: fn(f32) -> i32) {
  let mut solutions: Vec<Result> = Vec::new();
  let mut rng = rand::thread_rng();
  let w_set = Uniform::from(-5.0..5.0);
  let mut iter_count = 0;

  loop {
    let b = w_set.sample(&mut rng);
    let w1 = w_set.sample(&mut rng);
    let w2 = w_set.sample(&mut rng);
    let mut is_valid = true;


    for data_line in &DATA_SET {
      let a = w1*data_line.0 + w2*data_line.1 + b;
      let class = activation_f(a);
      iter_count += 1;
      if class != data_line.2 {
        is_valid = false;
        break; 
      }
    }

    if is_valid {
      solutions.push(Result(b, w1, w2, iter_count));
      iter_count = 0;
      if solutions.len() == 100000 {
        println!("Surasti 5 skirtingi sprendiniai!");
        for solution in &solutions {
          println!("Bias: {}, w1: {}, w2: {}, surasta per {} iteraciju", solution.0, solution.1, solution.2, solution.3)
        }
        break;
      }
    }  
  } 
}
