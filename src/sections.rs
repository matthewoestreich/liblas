use serde::{Deserialize, Serialize};

use crate::{
    errors::ParseError,
    parser::{KeyValueData, LasValue, Section, SectionEntry, SectionKind},
};

fn any_present<T>(items: &[&Option<T>]) -> bool {
    items.iter().any(|o| o.is_some())
}

// --------------------------------------------------------------------------------
// ------------------ VERSION INFORMATION -----------------------------------------
// --------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionInformation {
    #[serde(rename = "VERS")]
    pub version: KeyValueData,
    #[serde(rename = "WRAP")]
    pub wrap: KeyValueData,
    pub additional: Vec<KeyValueData>,
    pub comments: Option<Vec<String>>,
}

impl TryFrom<Section> for VersionInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Version {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Version,
                got: section.header.kind,
            });
        }

        let mut version = VersionInformation::default();
        let mut has_vers = false;
        let mut has_wrap = false;

        for entry in section.entries {
            if let SectionEntry::Delimited(kv) = entry {
                match kv.mnemonic.to_lowercase().as_str() {
                    "vers" => {
                        version.version = kv;
                        has_vers = true;
                    }
                    "wrap" => {
                        version.wrap = kv;
                        has_wrap = true;
                    }
                    _ => version.additional.push(kv),
                };
            }
        }

        if !has_vers || !has_wrap {
            return Err(ParseError::SectionMissingRequiredData {
                section: SectionKind::Version,
                one_of: vec!["VERS".to_string(), "WRAP".to_string()],
            });
        }

        version.comments = section.comments;
        Ok(version)
    }
}

// --------------------------------------------------------------------------------
// ------------------ OTHER INFORMATION -------------------------------------------
// --------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OtherInformation {
    pub text: String,
    pub comments: Option<Vec<String>>,
}

impl TryFrom<Section> for OtherInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Other {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Other,
                got: section.header.kind,
            });
        }

        let mut other = OtherInformation::default();

        for entry in section.entries {
            if let SectionEntry::Raw(s) = entry {
                other.text += format!("{s}\n").as_str();
            }
        }

        other.comments = section.comments;
        Ok(other)
    }
}

// --------------------------------------------------------------------------------
// ------------------ ASCII LOG DATA ----------------------------------------------
// --------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AsciiLogData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<f64>>,
    pub comments: Option<Vec<String>>,
}

impl TryFrom<Section> for AsciiLogData {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::AsciiLogData {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::AsciiLogData,
                got: section.header.kind,
            });
        }
        if section.ascii_headers.is_none() {
            return Err(ParseError::SectionMissingRequiredData {
                section: SectionKind::AsciiLogData,
                one_of: vec!["headers".to_string()],
            });
        }

        let mut ascii_logs = AsciiLogData::default();

        if let Some(headers) = section.ascii_headers {
            ascii_logs.headers = headers;
            ascii_logs.rows = section.ascii_rows;
        }

        ascii_logs.comments = section.comments;
        Ok(ascii_logs)
    }
}

// --------------------------------------------------------------------------------
// ------------------ CURVE INFORMATION -------------------------------------------
// --------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CurveInformation {
    pub curves: Vec<KeyValueData>,
    pub comments: Option<Vec<String>>,
}

impl TryFrom<Section> for CurveInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Curve {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Curve,
                got: section.header.kind,
            });
        }

        let mut curve = CurveInformation::default();

        for entry in section.entries {
            if let SectionEntry::Delimited(kv) = entry {
                curve.curves.push(kv);
            }
        }

        curve.comments = section.comments;
        Ok(curve)
    }
}

// --------------------------------------------------------------------------------
// ------------------ PARAMETER INFORMATION ---------------------------------------
// --------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParameterInformation {
    pub parameters: Vec<KeyValueData>,
    pub comments: Option<Vec<String>>,
}

impl TryFrom<Section> for ParameterInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Parameter {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Parameter,
                got: section.header.kind,
            });
        }

        let mut parameter = ParameterInformation::default();

        for entry in section.entries {
            if let SectionEntry::Delimited(kv) = entry {
                parameter.parameters.push(kv);
            }
        }

        parameter.comments = section.comments;
        Ok(parameter)
    }
}

// --------------------------------------------------------------------------------
// ------------------ WELL INFORMATION --------------------------------------------
// --------------------------------------------------------------------------------

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
}

impl WellInformation {
    pub fn validate(&self) -> Result<(), ParseError> {
        // ---- REQUIRED ----
        self.require_value(&self.strt, "STRT")?;
        self.require_value(&self.stop, "STOP")?;
        self.require_value(&self.step, "STEP")?;
        self.require_value(&self.null, "NULL")?;

        // ---- STRT / STOP / STEP must be numeric ----
        self.require_numeric(&self.strt, "STRT")?;
        self.require_numeric(&self.stop, "STOP")?;
        self.require_numeric(&self.step, "STEP")?;

        // ---- STEP consistency ----
        if let Some(LasValue::Float(step)) = &self.step.value
            && *step == 0.0
        {
            // allowed but special case
        }

        // ---- LOCATION: one-of ----
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

        // ---- IDENTITY: one-of ----
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

        well.comments = section.comments;

        well.validate()?;
        Ok(well)
    }
}
