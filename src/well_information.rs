use crate::Mnemonic;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct WellInformation {
  pub start: Mnemonic,
  pub stop: Mnemonic,
  pub step: Mnemonic,
  pub null: Mnemonic,
  pub comp: Mnemonic,
  pub well: Mnemonic,
  pub fld: Mnemonic,
  pub loc: Mnemonic,
  pub prov: Mnemonic,
  pub cnty: Mnemonic,
  pub stat: Mnemonic,
  pub ctry: Mnemonic,
  pub srvc: Mnemonic,
  pub date: Mnemonic,
  pub uwi: Mnemonic,
  pub api: Mnemonic,
  pub extra: HashMap<String, Mnemonic>,
}
