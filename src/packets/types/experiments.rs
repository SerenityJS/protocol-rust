use protocol_derive::packet;

#[packet]
pub struct Experiment {
  pub name: String,
  pub enabled: bool,
}
