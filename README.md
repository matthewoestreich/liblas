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
