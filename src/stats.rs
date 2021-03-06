use std::sync::Mutex;

lazy_static! {
  pub static ref DIVES: Mutex<DiveStats> = Mutex::new(DiveStats{total: 0, duplicates: 0});
}

pub struct DiveStats {
  total: usize,
  duplicates: usize,
}

impl DiveStats {
  pub fn record(&mut self, total: usize, duplicates: usize) {
    let new = DiveStats{total, duplicates};
    info!("{:?}", new);
    self.merge(&new);
  }

  fn merge(&mut self, other: &Self) {
    self.total += other.total;
    self.duplicates += other.duplicates;
  }

  pub fn clear(&mut self) {
    self.total = 0;
    self.duplicates = 0;
  }
}

use std::fmt;
impl fmt::Debug for DiveStats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
      "DiveStats {{ total: {}, duplicates={}, dupe_rate={:.2} }}",
      self.total,
      self.duplicates,
      (self.duplicates as f64) / (self.total as f64)
    )
  }
}


// misfit functions


impl DiveStats {
  // exists for dumb type coercion reasons
  pub fn format(&self) -> String {
    format!("{:?}", self)
  }
}
