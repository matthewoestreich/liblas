# liblas

[![Crates.io](https://img.shields.io/crates/v/liblas.svg)](https://crates.io/crates/liblas) [![docs.rs](https://img.shields.io/docsrs/liblas?style=flat-square)](https://docs.rs/liblas/latest/liblas/)

Parse .las Files in Rust

[Canadian Well Logging Society LAS File 2.0 Specification](https://github.com/matthewoestreich/liblas/blob/7dcfca33c32ed2bc97a5fc721f5c5676f287c872/spec/CWLS_LAS_2_0_SPEC.pdf)

# Features

- Parse .las files into JSON, YAML, or YML formats
- Uses streaming to parse efficiently
- Has a CLI
- Fast - parses, and writes, a 50mb .las file in 0.2 - 0.7 seconds
- Ability to use parsed data in [plots](https://github.com/matthewoestreich/liblas/tree/1efe2c7167de743b0cce60ea96996113df6112f0/plots)
  - [See example code here](https://github.com/matthewoestreich/liblas/blob/1efe2c7167de743b0cce60ea96996113df6112f0/src/tests/helpers.rs#L18-L233)

# Installation

```bash
cargo add liblas
```

```bash
cargo install liblas
```

# Usage

```rust
// Parse (stream) directly into anything that implements the Write trait
liblas::parse_into("/some/file.las", your_writer, OutputFormat::JSON)?;

// Parse into LasFile struct
let my_las_file = liblas::parse("/some/file.las")?;
// To json string?
let json_str = my_las_file.to_json_str()?;
// To yaml/yml string?
let yaml_str = my_las_file.to_yaml_str()?;
// Back to raw las?
let raw_las_str = my_las_file.to_las_str();
```

## Command Line Interface

Export as JSON.

```sh
liblas --las las_files/_good_sample_1.las --out output/_good_sample_1.json --out-type json --force
```

Export as YAML/YML

```sh
liblas --las las_files/_good_sample_1.las --out output/_good_sample_1.yaml --out-type yaml --force
liblas --las las_files/_good_sample_1.las --out output/_good_sample_1.yml --out-type yml --force
```

# Example

For this example, we will be using the following .las file (also located at `las_files/_good_sample_1.las`).

<details>
  <summary>Click to show raw .las file</summary>

```
~VERSION INFORMATION
VERS. 2.0 : CWLS LOG ASCII STANDARD -VERSION 2.0
WRAP. NO : ONE LINE PER DEPTH STEP
CREA.   02-08-2006               :LAS File Creation Date (MM-DD-YYYY)
~WELL INFORMATION
#MNEM.UNIT DATA DESCRIPTION
#----- ----- ---------- -----------------
STRT .M 1670.0000 :START DEPTH
STOP .M 1669.7500 :STOP DEPTH
STEP .M -0.1250 :STEP
NULL . -999.25 :NULL VALUE
COMP . ANY OIL COMPANY INC. :COMPANY
WELL . ANY ET AL 12-34-12-34 :WELL
FLD . WILDCAT :FIELD
LOC . 12-34-12-34W5M :LOCATION
PROV . ALBERTA :PROVINCE
SRVC . ANY LOGGING COMPANY INC. :SERVICE COMPANY
DATE . 13-DEC-86 :LOG DATE
UWI . 100123401234W500 :UNIQUE WELL ID
LIC . 23412 :ERCB LICENCE NUMB
~CURVE INFORMATION
#MNEM.UNIT API CODES CURVE DESCRIPTION
#------------------ ------------ -------------------
DEPT .M : 1 DEPTH
DT .US/M 60 520 32 00 : 2 SONIC TRANSIT TIME
RHOB .K/M3 45 350 01 00 : 3 BULK DENSITY
NPHI .V/V 42 890 00 00 : 4 NEUTRON POROSITY
SFLU .OHMM 07 220 04 00 : 5 SHALLOW RESISTIVITY
SFLA .OHMM 07 222 01 00 : 6 SHALLOW RESISTIVITY
ILM .OHMM 07 120 44 00 : 7 MEDIUM RESISTIVITY
ILD .OHMM 07 120 46 00 : 8 DEEP RESISTIVITY
~PARAMETER INFORMATION
#MNEM.UNIT VALUE DESCRIPTION
#-------------- ---------------- ------------------------
MUD . GEL CHEM : MUD TYPE
BHT .DEGC 35.5000 : BOTTOM HOLE TEMPERATURE
CSGL .M 124.6 : BASE OF CASING
MATR . SAND : NEUTRON MATRIX
MDEN . 2710.0000 : LOGGING MATRIX DENSITY
RMF .OHMM 0.2160 : MUD FILTRATE RESISTIVITY
DFD .K/M3 1525.0000 : DRILL FLUID DENSITY
# This is a comment above other
~OTHER
# First comment in other
Note: The logging tools became stuck at 625 metres causing the
# Second comment in other
# Second line in second comment in other
data between 625 metres and 615 metres to be invalid.
# First comment above ~A
# Second comment above ~A
~A DEPTH DT RHOB NPHI SFLU SFLA ILM ILD
1670.000 123.450 2550.000 0.450 123.450 123.450 110.200 05.600
1669.875 123.450 2550.000 0.450 123.450 123.450 110.200 05.600
1669.750 123.450 2550.000 0.450 123.450 123.450 110.200 105.600
```

</details>

## Parse LAS File

```rust
let parsed_file = liblas::parse("las_files/_good_sample_1.las")?;
```

## Convert Parsed LAS File

### To JSON

```rust
let json_string = parsed_file.to_json_str()?;
```

<details>
  <summary>Click to show LAS as JSON</summary>

```json
{
  "VersionInformation": {
    "VERS": {
      "mnemonic": "VERS",
      "unit": null,
      "value": "2.0",
      "description": "CWLS LOG ASCII STANDARD -VERSION 2.0",
      "comments": null
    },
    "WRAP": {
      "mnemonic": "WRAP",
      "unit": null,
      "value": "NO",
      "description": "ONE LINE PER DEPTH STEP",
      "comments": null
    },
    "additional": [
      {
        "mnemonic": "CREA",
        "unit": null,
        "value": "02-08-2006",
        "description": "LAS File Creation Date (MM-DD-YYYY)",
        "comments": null
      }
    ],
    "comments": ["Comment before version info"],
    "header": "~VERSION INFORMATION"
  },
  "WellInformation": {
    "STRT": {
      "mnemonic": "STRT",
      "unit": "M",
      "value": "1670.0000",
      "description": "START DEPTH",
      "comments": [
        "MNEM.UNIT DATA DESCRIPTION",
        "----- ----- ---------- -----------------"
      ]
    },
    "STOP": {
      "mnemonic": "STOP",
      "unit": "M",
      "value": "1669.7500",
      "description": "STOP DEPTH",
      "comments": null
    },
    "STEP": {
      "mnemonic": "STEP",
      "unit": "M",
      "value": "-0.1250",
      "description": "STEP",
      "comments": null
    },
    "NULL": {
      "mnemonic": "NULL",
      "unit": null,
      "value": "-999.25",
      "description": "NULL VALUE",
      "comments": null
    },
    "COMP": {
      "mnemonic": "COMP",
      "unit": null,
      "value": "ANY OIL COMPANY INC.",
      "description": "COMPANY",
      "comments": null
    },
    "WELL": {
      "mnemonic": "WELL",
      "unit": null,
      "value": "ANY ET AL 12-34-12-34",
      "description": "WELL",
      "comments": null
    },
    "FLD": {
      "mnemonic": "FLD",
      "unit": null,
      "value": "WILDCAT",
      "description": "FIELD",
      "comments": null
    },
    "LOC": {
      "mnemonic": "LOC",
      "unit": null,
      "value": "12-34-12-34W5M",
      "description": "LOCATION",
      "comments": null
    },
    "PROV": {
      "mnemonic": "PROV",
      "unit": null,
      "value": "ALBERTA",
      "description": "PROVINCE",
      "comments": null
    },
    "CNTY": null,
    "STAT": null,
    "CTRY": null,
    "SRVC": {
      "mnemonic": "SRVC",
      "unit": null,
      "value": "ANY LOGGING COMPANY INC.",
      "description": "SERVICE COMPANY",
      "comments": null
    },
    "DATE": {
      "mnemonic": "DATE",
      "unit": null,
      "value": "13-DEC-86",
      "description": "LOG DATE",
      "comments": null
    },
    "UWI": {
      "mnemonic": "UWI",
      "unit": null,
      "value": "100123401234W500",
      "description": "UNIQUE WELL ID",
      "comments": null
    },
    "API": null,
    "additional": [
      {
        "mnemonic": "LIC",
        "unit": null,
        "value": 23412,
        "description": "ERCB LICENCE NUMB",
        "comments": null
      }
    ],
    "comments": null,
    "header": "~WELL INFORMATION"
  },
  "AsciiLogData": {
    "headers": ["DEPT", "DT", "RHOB", "NPHI", "SFLU", "SFLA", "ILM", "ILD"],
    "rows": [
      [
        "1670.000",
        "123.450",
        "2550.000",
        "0.450",
        "123.450",
        "123.450",
        "110.200",
        "05.600"
      ],
      [
        "1669.875",
        "123.450",
        "2550.000",
        "0.450",
        "123.450",
        "123.450",
        "110.200",
        "05.600"
      ],
      [
        "1669.750",
        "123.450",
        "2550.000",
        "0.450",
        "123.450",
        "123.450",
        "110.200",
        "105.600"
      ]
    ],
    "comments": ["First comment above ~A", "Second comment above ~A"],
    "header": "~A DEPTH DT RHOB NPHI SFLU SFLA ILM ILD"
  },
  "CurveInformation": {
    "curves": [
      {
        "mnemonic": "DEPT",
        "unit": "M",
        "value": null,
        "description": "1 DEPTH",
        "comments": [
          "MNEM.UNIT API CODES CURVE DESCRIPTION",
          "------------------ ------------ -------------------"
        ]
      },
      {
        "mnemonic": "DT",
        "unit": "US/M",
        "value": "60 520 32 00",
        "description": "2 SONIC TRANSIT TIME",
        "comments": null
      },
      {
        "mnemonic": "RHOB",
        "unit": "K/M3",
        "value": "45 350 01 00",
        "description": "3 BULK DENSITY",
        "comments": null
      },
      {
        "mnemonic": "NPHI",
        "unit": "V/V",
        "value": "42 890 00 00",
        "description": "4 NEUTRON POROSITY",
        "comments": null
      },
      {
        "mnemonic": "SFLU",
        "unit": "OHMM",
        "value": "07 220 04 00",
        "description": "5 SHALLOW RESISTIVITY",
        "comments": null
      },
      {
        "mnemonic": "SFLA",
        "unit": "OHMM",
        "value": "07 222 01 00",
        "description": "6 SHALLOW RESISTIVITY",
        "comments": null
      },
      {
        "mnemonic": "ILM",
        "unit": "OHMM",
        "value": "07 120 44 00",
        "description": "7 MEDIUM RESISTIVITY",
        "comments": null
      },
      {
        "mnemonic": "ILD",
        "unit": "OHMM",
        "value": "07 120 46 00",
        "description": "8 DEEP RESISTIVITY",
        "comments": null
      }
    ],
    "comments": null,
    "header": "~CURVE INFORMATION"
  },
  "OtherInformation": {
    "data": [
      {
        "text": "Note: The logging tools became stuck at 625 metres causing the",
        "comments": ["First comment in other"]
      },
      {
        "text": "data between 625 metres and 615 metres to be invalid.",
        "comments": [
          "Second comment in other",
          "Second line in second comment in other"
        ]
      }
    ],
    "comments": ["This is a comment above other"],
    "header": "~OTHER"
  },
  "ParameterInformation": {
    "parameters": [
      {
        "mnemonic": "MUD",
        "unit": null,
        "value": "GEL CHEM",
        "description": "MUD TYPE",
        "comments": [
          "MNEM.UNIT VALUE DESCRIPTION",
          "-------------- ---------------- ------------------------"
        ]
      },
      {
        "mnemonic": "BHT",
        "unit": "DEGC",
        "value": "35.5000",
        "description": "BOTTOM HOLE TEMPERATURE",
        "comments": null
      },
      {
        "mnemonic": "CSGL",
        "unit": "M",
        "value": "124.6",
        "description": "BASE OF CASING",
        "comments": null
      },
      {
        "mnemonic": "MATR",
        "unit": null,
        "value": "SAND",
        "description": "NEUTRON MATRIX",
        "comments": null
      },
      {
        "mnemonic": "MDEN",
        "unit": null,
        "value": "2710.0000",
        "description": "LOGGING MATRIX DENSITY",
        "comments": null
      },
      {
        "mnemonic": "RMF",
        "unit": "OHMM",
        "value": "0.2160",
        "description": "MUD FILTRATE RESISTIVITY",
        "comments": null
      },
      {
        "mnemonic": "DFD",
        "unit": "K/M3",
        "value": "1525.0000",
        "description": "DRILL FLUID DENSITY",
        "comments": null
      }
    ],
    "comments": null,
    "header": "~PARAMETER INFORMATION"
  }
}
```

</details>

### To YAML

```rust
let yaml_string = parsed_file.to_yaml_str()?;
```

<details>
  <summary>Click to show LAS as YAML</summary>

```yaml
VersionInformation:
  VERS:
    mnemonic: VERS
    unit: null
    value: "2.0"
    description: CWLS LOG ASCII STANDARD -VERSION 2.0
    comments: null
  WRAP:
    mnemonic: WRAP
    unit: null
    value: NO
    description: ONE LINE PER DEPTH STEP
    comments: null
  additional:
    - mnemonic: CREA
      unit: null
      value: 02-08-2006
      description: LAS File Creation Date (MM-DD-YYYY)
      comments: null
  comments:
    - Comment before version info
  header: ~VERSION INFORMATION
WellInformation:
  STRT:
    mnemonic: STRT
    unit: M
    value: "1670.0000"
    description: START DEPTH
    comments:
      - MNEM.UNIT DATA DESCRIPTION
      - "----- ----- ---------- -----------------"
  STOP:
    mnemonic: STOP
    unit: M
    value: "1669.7500"
    description: STOP DEPTH
    comments: null
  STEP:
    mnemonic: STEP
    unit: M
    value: "-0.1250"
    description: STEP
    comments: null
  "NULL":
    mnemonic: "NULL"
    unit: null
    value: "-999.25"
    description: NULL VALUE
    comments: null
  COMP:
    mnemonic: COMP
    unit: null
    value: ANY OIL COMPANY INC.
    description: COMPANY
    comments: null
  WELL:
    mnemonic: WELL
    unit: null
    value: ANY ET AL 12-34-12-34
    description: WELL
    comments: null
  FLD:
    mnemonic: FLD
    unit: null
    value: WILDCAT
    description: FIELD
    comments: null
  LOC:
    mnemonic: LOC
    unit: null
    value: 12-34-12-34W5M
    description: LOCATION
    comments: null
  PROV:
    mnemonic: PROV
    unit: null
    value: ALBERTA
    description: PROVINCE
    comments: null
  CNTY: null
  STAT: null
  CTRY: null
  SRVC:
    mnemonic: SRVC
    unit: null
    value: ANY LOGGING COMPANY INC.
    description: SERVICE COMPANY
    comments: null
  DATE:
    mnemonic: DATE
    unit: null
    value: 13-DEC-86
    description: LOG DATE
    comments: null
  UWI:
    mnemonic: UWI
    unit: null
    value: 100123401234W500
    description: UNIQUE WELL ID
    comments: null
  API: null
  additional:
    - mnemonic: LIC
      unit: null
      value: 23412
      description: ERCB LICENCE NUMB
      comments: null
  comments: null
  header: ~WELL INFORMATION
AsciiLogData:
  headers:
    - DEPT
    - DT
    - RHOB
    - NPHI
    - SFLU
    - SFLA
    - ILM
    - ILD
  rows:
    - - "1670.000"
      - "123.450"
      - "2550.000"
      - "0.450"
      - "123.450"
      - "123.450"
      - "110.200"
      - "05.600"
    - - "1669.875"
      - "123.450"
      - "2550.000"
      - "0.450"
      - "123.450"
      - "123.450"
      - "110.200"
      - "05.600"
    - - "1669.750"
      - "123.450"
      - "2550.000"
      - "0.450"
      - "123.450"
      - "123.450"
      - "110.200"
      - "105.600"
  comments:
    - First comment above ~A
    - Second comment above ~A
  header: ~A DEPTH DT RHOB NPHI SFLU SFLA ILM ILD
CurveInformation:
  curves:
    - mnemonic: DEPT
      unit: M
      value: null
      description: 1 DEPTH
      comments:
        - MNEM.UNIT API CODES CURVE DESCRIPTION
        - "------------------ ------------ -------------------"
    - mnemonic: DT
      unit: US/M
      value: 60 520 32 00
      description: 2 SONIC TRANSIT TIME
      comments: null
    - mnemonic: RHOB
      unit: K/M3
      value: 45 350 01 00
      description: 3 BULK DENSITY
      comments: null
    - mnemonic: NPHI
      unit: V/V
      value: 42 890 00 00
      description: 4 NEUTRON POROSITY
      comments: null
    - mnemonic: SFLU
      unit: OHMM
      value: 07 220 04 00
      description: 5 SHALLOW RESISTIVITY
      comments: null
    - mnemonic: SFLA
      unit: OHMM
      value: 07 222 01 00
      description: 6 SHALLOW RESISTIVITY
      comments: null
    - mnemonic: ILM
      unit: OHMM
      value: 07 120 44 00
      description: 7 MEDIUM RESISTIVITY
      comments: null
    - mnemonic: ILD
      unit: OHMM
      value: 07 120 46 00
      description: 8 DEEP RESISTIVITY
      comments: null
  comments: null
  header: ~CURVE INFORMATION
OtherInformation:
  data:
    - text: "Note: The logging tools became stuck at 625 metres causing the"
      comments:
        - First comment in other
    - text: data between 625 metres and 615 metres to be invalid.
      comments:
        - Second comment in other
        - Second line in second comment in other
  comments:
    - This is a comment above other
  header: ~OTHER
ParameterInformation:
  parameters:
    - mnemonic: MUD
      unit: null
      value: GEL CHEM
      description: MUD TYPE
      comments:
        - MNEM.UNIT VALUE DESCRIPTION
        - "-------------- ---------------- ------------------------"
    - mnemonic: BHT
      unit: DEGC
      value: "35.5000"
      description: BOTTOM HOLE TEMPERATURE
      comments: null
    - mnemonic: CSGL
      unit: M
      value: "124.6"
      description: BASE OF CASING
      comments: null
    - mnemonic: MATR
      unit: null
      value: SAND
      description: NEUTRON MATRIX
      comments: null
    - mnemonic: MDEN
      unit: null
      value: "2710.0000"
      description: LOGGING MATRIX DENSITY
      comments: null
    - mnemonic: RMF
      unit: OHMM
      value: "0.2160"
      description: MUD FILTRATE RESISTIVITY
      comments: null
    - mnemonic: DFD
      unit: K/M3
      value: "1525.0000"
      description: DRILL FLUID DENSITY
      comments: null
  comments: null
  header: ~PARAMETER INFORMATION
```

</details>

# Programmatically Create

You can programmatically build .las fles.

<details>
  <summary>Click to view the code to rebuild our example .las file</summary>

```rust
use liblas::{
    DataLine, LasFile, LasValue, ParseError,
    sections::{
        AsciiLogData, AsciiLogDataParams, CurveInformation, CurveInformationParams, OtherInformation,
        OtherInformationData, OtherInformationParams, ParameterInformation, ParameterInformationParams,
        VersionInformation, VersionInformationParams, WellInformation, WellInformationParams,
    },
};

fn main() -> Result<(), ParseError> {
    let version_info = VersionInformation::new(VersionInformationParams {
        version: DataLine {
            mnemonic: "VERS".to_string(),
            unit: None,
            value: LasValue::new("2.0"),
            description: Some("CWLS LOG ASCII STANDARD -VERSION 2.0".to_string()),
            comments: None,
        },
        wrap: DataLine {
            mnemonic: "WRAP".to_string(),
            unit: None,
            value: LasValue::new("NO"),
            description: Some("ONE LINE PER DEPTH STEP".to_string()),
            comments: None,
        },
        additional: vec![DataLine {
            mnemonic: "CREA".to_string(),
            unit: None,
            value: LasValue::new("02-08-2006"),
            description: Some("LAS File Creation Date (MM-DD-YYYY)".to_string()),
            comments: None,
        }],
        comments: Some(vec!["Comment before version info".to_string()]),
        header: "~VERSION INFORMATION".to_string(),
    });

    let well_info = WellInformation::new(WellInformationParams {
        cnty: None,
        stat: None,
        ctry: None,
        api: None,
        strt: DataLine {
            mnemonic: "STRT".to_string(),
            unit: Some("M".to_string()),
            value: LasValue::new("1670.0000"),
            description: Some("START DEPTH".to_string()),
            comments: Some(vec![
                "MNEM.UNIT DATA DESCRIPTION".to_string(),
                "----- ----- ---------- -----------------".to_string(),
            ]),
        },
        stop: DataLine {
            mnemonic: "STOP".to_string(),
            unit: Some("M".to_string()),
            value: LasValue::new("1669.7500"),
            description: Some("STOP DEPTH".to_string()),
            comments: None,
        },
        step: DataLine {
            mnemonic: "STEP".to_string(),
            unit: Some("M".to_string()),
            value: LasValue::new("-0.1250"),
            description: Some("STEP".to_string()),
            comments: None,
        },
        null: DataLine {
            mnemonic: "NULL".to_string(),
            unit: None,
            value: LasValue::new("-999.25"),
            description: Some("NULL VALUE".to_string()),
            comments: None,
        },
        comp: Some(DataLine {
            mnemonic: "COMP".to_string(),
            unit: None,
            value: LasValue::new("ANY OIL COMPANY INC."),
            description: Some("COMPANY".to_string()),
            comments: None,
        }),
        well: Some(DataLine {
            mnemonic: "WELL".to_string(),
            unit: None,
            value: LasValue::new("ANY ET AL 12-34-12-34"),
            description: Some("WELL".to_string()),
            comments: None,
        }),
        fld: Some(DataLine {
            mnemonic: "FLD".to_string(),
            unit: None,
            value: LasValue::new("WILDCAT"),
            description: Some("FIELD".to_string()),
            comments: None,
        }),
        loc: Some(DataLine {
            mnemonic: "LOC".to_string(),
            unit: None,
            value: LasValue::new("12-34-12-34W5M"),
            description: Some("LOCATION".to_string()),
            comments: None,
        }),
        prov: Some(DataLine {
            mnemonic: "PROV".to_string(),
            unit: None,
            value: LasValue::new("ALBERTA"),
            description: Some("PROVINCE".to_string()),
            comments: None,
        }),
        srvc: Some(DataLine {
            mnemonic: "SRVC".to_string(),
            unit: None,
            value: LasValue::new("ANY LOGGING COMPANY INC."),
            description: Some("SERVICE COMPANY".to_string()),
            comments: None,
        }),
        date: Some(DataLine {
            mnemonic: "DATE".to_string(),
            unit: None,
            value: LasValue::new("13-DEC-86"),
            description: Some("LOG DATE".to_string()),
            comments: None,
        }),
        uwi: Some(DataLine {
            mnemonic: "UWI".to_string(),
            unit: None,
            value: LasValue::new("100123401234W500"),
            description: Some("UNIQUE WELL ID".to_string()),
            comments: None,
        }),
        additional: vec![DataLine {
            mnemonic: "LIC".to_string(),
            unit: None,
            value: LasValue::new("23412"),
            description: Some("ERCB LICENCE NUMB".to_string()),
            comments: None,
        }],
        comments: None,
        header: "~WELL INFORMATION".to_string(),
    });

    //
    // IMPORTANT : THE ORDER OF CURVES MATTERS!!
    //
    let curve_info = CurveInformation::new(CurveInformationParams {
        // ORDER MATTERS HERE!!!!
        curves: vec![
            DataLine {
                mnemonic: "DEPT".to_string(),
                unit: Some("M".to_string()),
                value: None,
                description: Some("1 DEPTH".to_string()),
                comments: Some(vec![
                    "MNEM.UNIT API CODES CURVE DESCRIPTION".to_string(),
                    "------------------ ------------ -------------------".to_string(),
                ]),
            },
            DataLine {
                mnemonic: "DT".to_string(),
                unit: Some("US/M".to_string()),
                value: None,
                description: Some("2 SONIC TRANSIT TIME".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "RHOB".to_string(),
                unit: Some("K/M3".to_string()),
                value: None,
                description: Some("3 BULK DENSITY".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "NPHI".to_string(),
                unit: Some("V/V".to_string()),
                value: None,
                description: Some("4 NEUTRON POROSITY".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "SFLU".to_string(),
                unit: Some("OHMM".to_string()),
                value: None,
                description: Some("5 SHALLOW RESISTIVITY".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "SFLA".to_string(),
                unit: Some("OHMM".to_string()),
                value: None,
                description: Some("6 SHALLOW RESISTIVITY".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "ILM".to_string(),
                unit: Some("OHMM".to_string()),
                value: None,
                description: Some("7 MEDIUM RESISTIVITY".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "ILD".to_string(),
                unit: Some("OHMM".to_string()),
                value: None,
                description: Some("8 DEEP RESISTIVITY".to_string()),
                comments: None,
            },
        ],
        comments: None,
        header: "~CURVE INFORMATION".to_string(),
    });

    let param_info = ParameterInformation::new(ParameterInformationParams {
        parameters: vec![
            DataLine {
                mnemonic: "MUD".to_string(),
                unit: None,
                value: LasValue::new("GEL CHEM"),
                description: Some("MUD TYPE".to_string()),
                comments: Some(vec![
                    "MNEM.UNIT VALUE DESCRIPTION".to_string(),
                    "-------------- ---------------- ------------------------".to_string(),
                ]),
            },
            DataLine {
                mnemonic: "BHT".to_string(),
                unit: Some("DEGC".to_string()),
                value: LasValue::new("35.5000"),
                description: Some("BOTTOM HOLE TEMPERATURE".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "CSGL".to_string(),
                unit: Some("M".to_string()),
                value: LasValue::new("124.6"),
                description: Some("BASE OF CASING".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "MATR".to_string(),
                unit: None,
                value: LasValue::new("SAND"),
                description: Some("NEUTRON MATRIX".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "MDEN".to_string(),
                unit: None,
                value: LasValue::new("2710.0000"),
                description: Some("LOGGING MATRIX DENSITY".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "RMF".to_string(),
                unit: Some("OHMM".to_string()),
                value: LasValue::new("0.2160"),
                description: Some("MUD FILTRATE RESISTIVITY".to_string()),
                comments: None,
            },
            DataLine {
                mnemonic: "DFD".to_string(),
                unit: Some("K/M3".to_string()),
                value: LasValue::new("1525.0000"),
                description: Some("DRILL FLUID DENSITY".to_string()),
                comments: None,
            },
        ],
        comments: None,
        header: "~PARAMETER INFORMATION".to_string(),
    });

    let other_info = OtherInformation::new(OtherInformationParams {
        data: vec![
            OtherInformationData {
                text: "Note: The logging tools became stuck at 625 metres causing the".to_string(),
                comments: Some(vec!["First comment in other".to_string()]),
            },
            OtherInformationData {
                text: "data between 625 metres and 615 metres to be invalid.".to_string(),
                comments: Some(vec![
                    "Second comment in other".to_string(),
                    "Second line in second comment in other".to_string(),
                ]),
            },
        ],
        comments: Some(vec!["This is a comment above other".to_string()]),
        header: "~OTHER".to_string(),
    });

    let ascii_data = AsciiLogData::new(AsciiLogDataParams {
        // SHOULD MATCH THE SAME ORDER OF CURVES!
        headers: vec![
            "DEPTH".to_string(),
            "DT".to_string(),
            "RHOB".to_string(),
            "NPHI".to_string(),
            "SFLU".to_string(),
            "SFLA".to_string(),
            "ILM".to_string(),
            "ILD".to_string(),
        ],
        rows: vec![
            vec![
                "1670.000".to_string(),
                "123.450".to_string(),
                "2550.000".to_string(),
                "0.450".to_string(),
                "123.450".to_string(),
                "123.450".to_string(),
                "110.200".to_string(),
                "05.600".to_string(),
            ],
            vec![
                "1669.875".to_string(),
                "123.450".to_string(),
                "2550.000".to_string(),
                "0.450".to_string(),
                "123.450".to_string(),
                "123.450".to_string(),
                "110.200".to_string(),
                "05.600".to_string(),
            ],
            vec![
                "1669.750".to_string(),
                "123.450".to_string(),
                "2550.000".to_string(),
                "0.450".to_string(),
                "123.450".to_string(),
                "123.450".to_string(),
                "110.200".to_string(),
                "105.600".to_string(),
            ],
        ],
        comments: Some(vec![
            "First comment above ~A".to_string(),
            "Second comment above ~A".to_string(),
        ]),
        header: "~A DEPTH DT RHOB NPHI SFLU SFLA ILM ILD".to_string(),
    });

    let las_file = LasFile::new(
        version_info,
        well_info,
        curve_info,
        ascii_data,
        Some(other_info),
        Some(param_info),
    );

    println!("{las_file}");

    Ok(())
}
```

</details>
