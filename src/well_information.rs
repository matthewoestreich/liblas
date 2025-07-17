use crate::{LibLasError, Mnemonic, PeekableFileReader, Token, errors::LibLasError::*};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};

#[derive(Default, Debug, Deserialize)]
pub struct WellInformation {
  #[serde(rename = "STRT")]
  pub strt: Mnemonic,
  #[serde(rename = "STOP")]
  pub stop: Mnemonic,
  #[serde(rename = "STEP")]
  pub step: Mnemonic,
  #[serde(rename = "NULL")]
  pub null: Mnemonic,
  #[serde(rename = "COMP")]
  pub comp: Mnemonic,
  #[serde(rename = "WELL")]
  pub well: Mnemonic,
  #[serde(rename = "FLD")]
  pub fld: Mnemonic,
  #[serde(rename = "LOC")]
  pub loc: Mnemonic,
  #[serde(rename = "PROV")]
  pub prov: Mnemonic,
  #[serde(rename = "CNTY")]
  pub cnty: Mnemonic,
  #[serde(rename = "STAT")]
  pub stat: Mnemonic,
  #[serde(rename = "CTRY")]
  pub ctry: Mnemonic,
  #[serde(rename = "SRVC")]
  pub srvc: Mnemonic,
  #[serde(rename = "DATE")]
  pub date: Mnemonic,
  #[serde(rename = "UWI")]
  pub uwi: Mnemonic,
  #[serde(rename = "API")]
  pub api: Mnemonic,
  pub additional: Vec<Mnemonic>,
  pub comments: Vec<String>,
  #[serde(skip)]
  pub(crate) is_parsed: bool,
}

impl WellInformation {
  pub fn parse(reader: &mut PeekableFileReader, current_comments: &mut Vec<String>) -> Result<Self, LibLasError> {
    let mut this = Self::default();

    // Comments were above the "~Well Information" section
    if !current_comments.is_empty() {
      this.comments = current_comments.to_vec();
      // Clear comments because any additional comments may be intended for a mnemonic or a diff section entirely.
      current_comments.clear();
    }

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.trim().to_string().starts_with(&Token::Tilde()) {
        break;
      }

      let line = reader.next().ok_or(ReadingNextLine)??.trim().to_string();

      if line.starts_with(&Token::Comment()) {
        current_comments.push(line.clone());
        continue;
      }

      if line.starts_with("STRT") {
        this.strt = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("STOP") {
        this.stop = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("STEP") {
        this.step = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("NULL") {
        this.null = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("COMP") {
        this.comp = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("WELL") {
        this.well = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("FLD") {
        this.fld = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("LOC") {
        this.loc = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("PROV") {
        this.prov = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("CNTY") {
        this.cnty = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("STAT") {
        this.stat = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("CTRY") {
        this.ctry = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("SRVC") {
        this.srvc = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("DATE") {
        this.date = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("UWI") {
        this.uwi = Mnemonic::from_str(&line, current_comments)?;
      } else if line.starts_with("API") {
        this.api = Mnemonic::from_str(&line, current_comments)?;
      } else {
        let x = Mnemonic::from_str(&line, current_comments)?;
        this.additional.push(x);
      }
    }

    // Validate required fields
    let required = [
      ("STRT", &this.strt),
      ("STOP", &this.stop),
      ("STEP", &this.step),
      ("NULL", &this.null),
      ("COMP", &this.comp),
      ("WELL", &this.well),
      ("FLD", &this.fld),
      ("LOC", &this.loc),
      ("SRVC", &this.srvc),
      ("DATE", &this.date),
    ];

    for (field_name, mnemonic) in required.iter() {
      if mnemonic.name.trim().is_empty() {
        let mut e = "[~Well Information] -> ".to_owned();
        e.push_str(field_name);
        return Err(MissingRequiredMnemonicField(e));
      }
    }

    let one_of_prov_cnty_ctry_state_must_exist = [(
      ("PROV", &this.prov),
      ("CTRY", &this.ctry),
      ("CNTY", &this.cnty),
      ("STAT", &this.stat),
    )];

    for (pair_a, pair_b, pair_c, pair_d) in one_of_prov_cnty_ctry_state_must_exist.iter() {
      if pair_a.1.name.trim().is_empty()
        && pair_b.1.name.trim().is_empty()
        && pair_c.1.name.trim().is_empty()
        && pair_d.1.name.trim().is_empty()
      {
        let e = "[~Well Information] Must have one of PROV, CNTY, CTRY, STAT! ->".to_string();
        return Err(InvalidLasFile(e));
      }
    }

    if this.uwi.name.trim().is_empty() && this.api.name.trim().is_empty() {
      let e = "[~Well Information] Must have one of API or UWI! ->".to_string();
      return Err(InvalidLasFile(e));
    }

    this.is_parsed = true;
    return Ok(this);
  }

  pub fn to_str(&self) -> String {
    let mut output = "~Well Information".to_string();
    if !self.comments.is_empty() {
      output = format!("{}\n{output}", self.comments.join(" "));
    }
    if !self.strt.name.is_empty() {
      output = format!("{output}\n{}", self.strt.to_str());
    }
    if !self.stop.name.is_empty() {
      output = format!("{output}\n{}", self.stop.to_str());
    }
    if !self.step.name.is_empty() {
      output = format!("{output}\n{}", self.step.to_str());
    }
    if !self.null.name.is_empty() {
      output = format!("{output}\n{}", self.null.to_str());
    }
    if !self.comp.name.is_empty() {
      output = format!("{output}\n{}", self.comp.to_str());
    }
    if !self.well.name.is_empty() {
      output = format!("{output}\n{}", self.well.to_str());
    }
    if !self.fld.name.is_empty() {
      output = format!("{output}\n{}", self.fld.to_str());
    }
    if !self.loc.name.is_empty() {
      output = format!("{output}\n{}", self.loc.to_str());
    }
    if !self.prov.name.is_empty() {
      output = format!("{output}\n{}", self.prov.to_str());
    }
    if !self.cnty.name.is_empty() {
      output = format!("{output}\n{}", self.cnty.to_str());
    }
    if !self.stat.name.is_empty() {
      output = format!("{output}\n{}", self.stat.to_str());
    }
    if !self.ctry.name.is_empty() {
      output = format!("{output}\n{}", self.ctry.to_str());
    }
    if !self.srvc.name.is_empty() {
      output = format!("{output}\n{}", self.srvc.to_str());
    }
    if !self.date.name.is_empty() {
      output = format!("{output}\n{}", self.date.to_str());
    }
    if !self.uwi.name.is_empty() {
      output = format!("{output}\n{}", self.uwi.to_str());
    }
    if !self.api.name.is_empty() {
      output = format!("{output}\n{}", self.api.to_str());
    }
    if !self.additional.is_empty() {
      self
        .additional
        .iter()
        .for_each(|a| output = format!("{output}\n{}", a.to_str()));
    }
    return output;
  }

  #[allow(clippy::too_many_arguments)]
  pub fn new(
    strt: Mnemonic,
    stop: Mnemonic,
    step: Mnemonic,
    null: Mnemonic,
    comp: Mnemonic,
    well: Mnemonic,
    fld: Mnemonic,
    loc: Mnemonic,
    prov: Mnemonic,
    cnty: Mnemonic,
    stat: Mnemonic,
    ctry: Mnemonic,
    srvc: Mnemonic,
    date: Mnemonic,
    uwi: Mnemonic,
    api: Mnemonic,
    additional: Vec<Mnemonic>,
    comments: Vec<String>,
  ) -> Self {
    return Self {
      strt,
      stop,
      step,
      null,
      comp,
      well,
      fld,
      loc,
      prov,
      cnty,
      stat,
      ctry,
      srvc,
      date,
      uwi,
      api,
      additional,
      comments,
      is_parsed: false,
    };
  }
}

impl Serialize for WellInformation {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    // Estimate number of fields: 17 known + additional + comments
    let mut map = serializer.serialize_map(Some(17 + self.additional.len() + 1))?;

    macro_rules! serialize_field {
      ($map:ident, $field:expr, $name:expr) => {
        $map.serialize_entry($name, &$field)?
      };
    }

    serialize_field!(map, self.strt, "STRT");
    serialize_field!(map, self.stop, "STOP");
    serialize_field!(map, self.step, "STEP");
    serialize_field!(map, self.null, "NULL");
    serialize_field!(map, self.comp, "COMP");
    serialize_field!(map, self.well, "WELL");
    serialize_field!(map, self.fld, "FLD");
    serialize_field!(map, self.loc, "LOC");
    serialize_field!(map, self.prov, "PROV");
    serialize_field!(map, self.cnty, "CNTY");
    serialize_field!(map, self.stat, "STAT");
    serialize_field!(map, self.ctry, "CTRY");
    serialize_field!(map, self.srvc, "SRVC");
    serialize_field!(map, self.date, "DATE");
    serialize_field!(map, self.uwi, "UWI");
    serialize_field!(map, self.api, "API");

    for mnemonic in &self.additional {
      map.serialize_entry(&mnemonic.name, mnemonic)?;
    }

    map.serialize_entry("comments", &self.comments)?;
    return map.end();
  }
}
