use crate::{
    KeyValueData, LasValue, ParseError, Section, SectionEntry, SectionKind, any_present, write_comments, write_kv_opt,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WellInformation {
    #[serde(rename = "STRT")]
    pub strt: KeyValueData,
    #[serde(rename = "STOP")]
    pub stop: KeyValueData,
    #[serde(rename = "STEP")]
    pub step: KeyValueData,
    #[serde(rename = "NULL")]
    pub null: KeyValueData,

    #[serde(rename = "COMP")]
    pub comp: Option<KeyValueData>,
    #[serde(rename = "WELL")]
    pub well: Option<KeyValueData>,
    #[serde(rename = "FLD")]
    pub fld: Option<KeyValueData>,
    #[serde(rename = "LOC")]
    pub loc: Option<KeyValueData>,

    // location variants (one-of)
    #[serde(rename = "PROV")]
    pub prov: Option<KeyValueData>,
    #[serde(rename = "CNTY")]
    pub cnty: Option<KeyValueData>,
    #[serde(rename = "STAT")]
    pub stat: Option<KeyValueData>,
    #[serde(rename = "CTRY")]
    pub ctry: Option<KeyValueData>,

    #[serde(rename = "SRVC")]
    pub srvc: Option<KeyValueData>,
    #[serde(rename = "DATE")]
    pub date: Option<KeyValueData>,

    // identity (one-of)
    #[serde(rename = "UWI")]
    pub uwi: Option<KeyValueData>,
    #[serde(rename = "API")]
    pub api: Option<KeyValueData>,

    pub additional: Vec<KeyValueData>,
    pub comments: Option<Vec<String>>,

    pub header: String,

    #[serde(skip)]
    pub(crate) line_number: usize,
}

impl PartialEq for WellInformation {
    fn eq(&self, other: &Self) -> bool {
        self.strt == other.strt
            && self.stop == other.stop
            && self.step == other.step
            && self.null == other.null
            && self.comp == other.comp
            && self.well == other.well
            && self.fld == other.fld
            && self.loc == other.loc
            && self.prov == other.prov
            && self.cnty == other.cnty
            && self.stat == other.stat
            && self.ctry == other.ctry
            && self.srvc == other.srvc
            && self.date == other.date
            && self.uwi == other.uwi
            && self.api == other.api
            && self.additional == other.additional
            && self.comments == other.comments
            && self.header == other.header
    }
}

impl Eq for WellInformation {}

impl fmt::Display for WellInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        // Mandatory fields
        writeln!(f, "{}", self.strt)?;
        writeln!(f, "{}", self.stop)?;
        writeln!(f, "{}", self.step)?;
        writeln!(f, "{}", self.null)?;
        // Optional standard fields
        write_kv_opt(f, &self.comp)?;
        write_kv_opt(f, &self.well)?;
        write_kv_opt(f, &self.fld)?;
        write_kv_opt(f, &self.loc)?;
        // Location (one-of, but spec allows multiple lines syntactically)
        write_kv_opt(f, &self.prov)?;
        write_kv_opt(f, &self.cnty)?;
        write_kv_opt(f, &self.stat)?;
        write_kv_opt(f, &self.ctry)?;
        write_kv_opt(f, &self.srvc)?;
        write_kv_opt(f, &self.date)?;
        // Identity (one-of)
        write_kv_opt(f, &self.uwi)?;
        write_kv_opt(f, &self.api)?;
        // Additional user-defined lines
        for kv in &self.additional {
            writeln!(f, "{kv}")?;
        }
        Ok(())
    }
}

// TODO : maybe move this validation into the parser?
impl WellInformation {
    pub fn validate(&self) -> Result<(), ParseError> {
        // These data lines are required.
        self.require_value(&self.strt, "STRT")?;
        self.require_value(&self.stop, "STOP")?;
        self.require_value(&self.step, "STEP")?;
        self.require_value(&self.null, "NULL")?;

        // These must be numeric values.
        self.require_numeric(&self.strt, "STRT")?;
        self.require_numeric(&self.stop, "STOP")?;
        self.require_numeric(&self.step, "STEP")?;

        // TODO : Step consistency
        //
        //if let Some(LasValue::Float(step)) = &self.step.value
        //    && *step == 0.0
        //{
        // allowed but special case
        //}

        // "Location" must contain one of "PROV", "CNTY", "STAT" or "CTRY".
        if !any_present(&[&self.prov, &self.cnty, &self.stat, &self.ctry]) {
            return Err(ParseError::SectionMissingRequiredData {
                section: SectionKind::Well,
                one_of: vec![
                    "PROV".to_string(),
                    "CNTY".to_string(),
                    "STAT".to_string(),
                    "CTRY".to_string(),
                ],
            });
        }

        // "Identity" must contain one of "UWI" or "API".
        if !any_present(&[&self.uwi, &self.api]) {
            return Err(ParseError::SectionMissingRequiredData {
                section: SectionKind::Well,
                one_of: vec!["UWI".to_string(), "API".to_string()],
            });
        }

        Ok(())
    }

    fn require_value(&self, kv: &KeyValueData, name: &str) -> Result<(), ParseError> {
        if kv.value.is_none() {
            Err(ParseError::WellDataMissingRequiredValueForMnemonic {
                mnemonic: name.to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn require_numeric(&self, kv: &KeyValueData, name: &str) -> Result<(), ParseError> {
        match kv.value {
            Some(LasValue::Int(_)) | Some(LasValue::Float(_)) => Ok(()),
            _ => Err(ParseError::InvalidWellValue {
                mnemonic: name.to_string(),
                value: kv.value.clone(),
            }),
        }
    }
}

impl TryFrom<Section> for WellInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Well {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Well,
                got: section.header.kind,
            });
        }

        let mut well = WellInformation::default();

        for section_entry in section.entries {
            if let SectionEntry::Delimited(kv) = section_entry {
                match kv.mnemonic.as_str() {
                    "STRT" => well.strt = kv,
                    "STOP" => well.stop = kv,
                    "STEP" => well.step = kv,
                    "NULL" => well.null = kv,

                    "COMP" => well.comp = Some(kv),
                    "WELL" => well.well = Some(kv),
                    "FLD" => well.fld = Some(kv),
                    "LOC" => well.loc = Some(kv),

                    "PROV" => well.prov = Some(kv),
                    "CNTY" => well.cnty = Some(kv),
                    "STAT" => well.stat = Some(kv),
                    "CTRY" => well.ctry = Some(kv),

                    "SRVC" => well.srvc = Some(kv),
                    "DATE" => well.date = Some(kv),

                    "UWI" => well.uwi = Some(kv),
                    "API" => well.api = Some(kv),

                    _ => well.additional.push(kv),
                }
            }
        }

        well.header = format!("~{}", section.header.raw);
        well.comments = section.comments;
        well.line_number = section.line;

        well.validate()?;
        Ok(well)
    }
}
