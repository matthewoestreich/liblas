use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    errors::ParseError,
    parser::{KeyValueData, LasValue, Section, SectionEntry, SectionKind},
};

pub(crate) fn any_present<T>(items: &[&Option<T>]) -> bool {
    items.iter().any(|o| o.is_some())
}

pub(crate) fn write_kv_opt(f: &mut fmt::Formatter<'_>, kv: &Option<KeyValueData>) -> fmt::Result {
    if let Some(v) = kv {
        writeln!(f, "{v}")?;
    }
    Ok(())
}

pub(crate) fn write_comments(f: &mut fmt::Formatter<'_>, comments: &Option<Vec<String>>) -> fmt::Result {
    if let Some(cs) = comments {
        for c in cs {
            let fc = format!("# {c}").trim().to_string();
            writeln!(f, "{fc}")?;
        }
    }
    Ok(())
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
    pub(crate) line_number: usize,
    pub(crate) header: String,
}

impl fmt::Display for VersionInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        writeln!(f, "{}", self.version)?;
        writeln!(f, "{}", self.wrap)?;
        for addition in self.additional.iter() {
            writeln!(f, "{addition}")?;
        }
        Ok(())
    }
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

        version.header = format!("~{}", section.header.raw);
        version.comments = section.comments;
        version.line_number = section.line;
        Ok(version)
    }
}

// --------------------------------------------------------------------------------
// ------------------ OTHER INFORMATION -------------------------------------------
// --------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OtherInformationData {
    pub text: String,
    pub comments: Option<Vec<String>>,
}

impl fmt::Display for OtherInformationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.text)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OtherInformation {
    pub data: Vec<OtherInformationData>,
    pub comments: Option<Vec<String>>,
    pub(crate) line_number: usize,
    pub(crate) header: String,
}

impl fmt::Display for OtherInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        for info in self.data.iter() {
            write!(f, "{info}")?;
        }
        Ok(())
    }
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
            if let SectionEntry::Raw { text, comments } = entry {
                other.data.push(OtherInformationData { text, comments });
            }
        }

        other.header = format!("~{}", section.header.raw);
        other.comments = section.comments;
        other.line_number = section.line;
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
    pub(crate) line_number: usize,
    pub(crate) header: String,
}

impl fmt::Display for AsciiLogData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        for col in self.rows.iter() {
            for cell in col.iter() {
                write!(f, "{cell} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
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

        ascii_logs.header = format!("~{}", section.header.raw);
        ascii_logs.comments = section.comments;
        ascii_logs.line_number = section.line;
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
    pub(crate) line_number: usize,
    pub(crate) header: String,
}

impl fmt::Display for CurveInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        for curve in self.curves.iter() {
            writeln!(f, "{curve}")?;
        }
        Ok(())
    }
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

        curve.header = format!("~{}", section.header.raw);
        curve.comments = section.comments;
        curve.line_number = section.line;
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
    pub(crate) line_number: usize,
    pub(crate) header: String,
}

impl fmt::Display for ParameterInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        for param in self.parameters.iter() {
            writeln!(f, "{param}")?;
        }
        Ok(())
    }
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

        parameter.header = format!("~{}", section.header.raw);
        parameter.comments = section.comments;
        parameter.line_number = section.line;
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
    pub(crate) line_number: usize,
    pub(crate) header: String,
}

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
