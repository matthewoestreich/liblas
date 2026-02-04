# liblas

Parse .las Files in Rust

[Canadian Well Logging Society LAS File 2.0 Specification](https://www.cwls.org/wp-content/uploads/2017/02/Las2_Update_Feb2017.pdf)

# Installation

**To use programmatically**

```bash
cargo add liblas
```

**To use CLI globally**

```bash
cargo install liblas
```

# Usage

```rust
let my_las_file = LasFile::parse("/some/file.las".into())?;
// To json string?
let json_str = my_las_file.to_json_str()?;
```

# Example

For this example, we will be using the following .las file (also located at `las_files/_good_sample_1.las`).

<details>
  <summary><h3>Click to show raw .las file</h3></summary>

```
#  Comment before version info
~VERSION INFORMATION
VERS. 2.0 : CWLS LOG ASCII STANDARD -VERSION 2.0
WRAP. NO : ONE LINE PER DEPTH STEP
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
#
~A DEPTH DT RHOB NPHI SFLU SFLA ILM ILD
1670.000 123.450 2550.000 0.450 123.450 123.450 110.200 05.600
1669.875 123.450 2550.000 0.450 123.450 123.450 110.200 05.600
1669.750 123.450 2550.000 0.450 123.450 123.450 110.200 105.600
```

</details>

## Parse LAS File

I will be using `.expect()` to simplify the example code.

```rust
use liblas::LasFile;

fn main() {
  let parsed_file = LasFile::parse("las_files/_good_sample_1.las").expect("las file");
}
```

### Convert Parsed LAS File

**Into JSON**

```rust
let json_string = parsed_file.to_json_str().expect("json");
```

<details>
  <summary><h3>Click to view LAS as JSON</h3></summary>

```json
{
  "VersionInformation": {
    "VERS": {
      "mnemonic": "VERS",
      "unit": null,
      "value": 2.0,
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
    "additional": [],
    "comments": ["Comment before version info"],
    "line_number": 2,
    "header": "~VERSION INFORMATION"
  },
  "WellInformation": {
    "STRT": {
      "mnemonic": "STRT",
      "unit": "M",
      "value": 1670.0,
      "description": "START DEPTH",
      "comments": [
        "MNEM.UNIT DATA DESCRIPTION",
        "----- ----- ---------- -----------------"
      ]
    },
    "STOP": {
      "mnemonic": "STOP",
      "unit": "M",
      "value": 1669.75,
      "description": "STOP DEPTH",
      "comments": null
    },
    "STEP": {
      "mnemonic": "STEP",
      "unit": "M",
      "value": -0.125,
      "description": "STEP",
      "comments": null
    },
    "NULL": {
      "mnemonic": "NULL",
      "unit": null,
      "value": -999.25,
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
    "line_number": 5,
    "header": "~WELL INFORMATION"
  },
  "AsciiLogData": {
    "headers": ["DEPT", "DT", "RHOB", "NPHI", "SFLU", "SFLA", "ILM", "ILD"],
    "rows": [
      [1670.0, 123.45, 2550.0, 0.45, 123.45, 123.45, 110.2, 5.6],
      [1669.875, 123.45, 2550.0, 0.45, 123.45, 123.45, 110.2, 5.6],
      [1669.75, 123.45, 2550.0, 0.45, 123.45, 123.45, 110.2, 105.6]
    ],
    "comments": [""],
    "line_number": 50,
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
    "line_number": 21,
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
    "line_number": 43,
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
        "value": 35.5,
        "description": "BOTTOM HOLE TEMPERATURE",
        "comments": null
      },
      {
        "mnemonic": "CSGL",
        "unit": "M",
        "value": 124.6,
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
        "value": 2710.0,
        "description": "LOGGING MATRIX DENSITY",
        "comments": null
      },
      {
        "mnemonic": "RMF",
        "unit": "OHMM",
        "value": 0.216,
        "description": "MUD FILTRATE RESISTIVITY",
        "comments": null
      },
      {
        "mnemonic": "DFD",
        "unit": "K/M3",
        "value": 1525.0,
        "description": "DRILL FLUID DENSITY",
        "comments": null
      }
    ],
    "comments": null,
    "line_number": 32,
    "header": "~PARAMETER INFORMATION"
  }
}
```

</details>

**Into YAML**

```rust
let yaml_string = parsed_file.to_yaml_str().expect("yaml");
```

<details>
  <summary><h3>Click to show LAS as YAML</h3></summary>

```yaml
VersionInformation:
  VERS:
    mnemonic: VERS
    unit: null
    value: 2.0
    description: CWLS LOG ASCII STANDARD -VERSION 2.0
    comments: null
  WRAP:
    mnemonic: WRAP
    unit: null
    value: NO
    description: ONE LINE PER DEPTH STEP
    comments: null
  additional: []
  comments:
    - Comment before version info
  line_number: 2
  header: ~VERSION INFORMATION
WellInformation:
  STRT:
    mnemonic: STRT
    unit: M
    value: 1670.0
    description: START DEPTH
    comments:
      - MNEM.UNIT DATA DESCRIPTION
      - "----- ----- ---------- -----------------"
  STOP:
    mnemonic: STOP
    unit: M
    value: 1669.75
    description: STOP DEPTH
    comments: null
  STEP:
    mnemonic: STEP
    unit: M
    value: -0.125
    description: STEP
    comments: null
  "NULL":
    mnemonic: "NULL"
    unit: null
    value: -999.25
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
  line_number: 5
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
    - - 1670.0
      - 123.45
      - 2550.0
      - 0.45
      - 123.45
      - 123.45
      - 110.2
      - 5.6
    - - 1669.875
      - 123.45
      - 2550.0
      - 0.45
      - 123.45
      - 123.45
      - 110.2
      - 5.6
    - - 1669.75
      - 123.45
      - 2550.0
      - 0.45
      - 123.45
      - 123.45
      - 110.2
      - 105.6
  comments:
    - ""
  line_number: 50
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
  line_number: 21
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
  line_number: 43
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
      value: 35.5
      description: BOTTOM HOLE TEMPERATURE
      comments: null
    - mnemonic: CSGL
      unit: M
      value: 124.6
      description: BASE OF CASING
      comments: null
    - mnemonic: MATR
      unit: null
      value: SAND
      description: NEUTRON MATRIX
      comments: null
    - mnemonic: MDEN
      unit: null
      value: 2710.0
      description: LOGGING MATRIX DENSITY
      comments: null
    - mnemonic: RMF
      unit: OHMM
      value: 0.216
      description: MUD FILTRATE RESISTIVITY
      comments: null
    - mnemonic: DFD
      unit: K/M3
      value: 1525.0
      description: DRILL FLUID DENSITY
      comments: null
  comments: null
  line_number: 32
  header: ~PARAMETER INFORMATION
```

</details>
