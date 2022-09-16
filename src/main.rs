use dialoguer::{ Select, theme::ColorfulTheme };
use console::Term;

struct DataLine(f32, f32, i32); // [x1, x2, t]
static DATA_SET: [DataLine; 4] = [
  DataLine(-0.3, 0.6, 0),
  DataLine(0.3, -0.6, 0),
  DataLine(1.2, -1.2, 1),
  DataLine(1.2, 1.2, 1),
];

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
    Some(0) => println!("Vartotojas pasirinko: 0 {}", DATA_SET[0].0),
    Some(1) => println!("Vartotojas pasirinko: 1"),
    Some(i) => println!("Neregistruotas pasirinkimas {}", i),
    None => println!("User did not select anything")
  }

  Ok(())
}