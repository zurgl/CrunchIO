//struct.rs
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ModelDataCapture {
  run: i32,
  year: i32,
}

impl ModelDataCapture {
  pub fn new() -> Self {
    ModelDataCapture::default()
  }
  pub fn set_run(&mut self, run: i32) -> &mut ModelDataCapture {
    self.run = run;
    self
  }
  pub fn set_year(&mut self, year: i32) -> &mut ModelDataCapture {
    self.year = year;
    self
  }
}

fn main() {
  let data_capture = ModelDataCapture::new().set_run(0).set_year(1).to_owned();

  println!("here is the data capture {:?}", data_capture);
}
