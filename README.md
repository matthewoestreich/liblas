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

# Examples

## In-Depth Example

For this example, we will be using [this .las file](examples/from_readme/example.las):

Parse file:

```rust
let example_las = LasFile::parse("example.las".into())?;
```

Now we can convert it to JSON and save to a file if we wanted, etc..

[You can view our "example.las" file converted to JSON here](examples/from_readme/example.json).

```rust
let json = example_las.to_json_str()?;
```

**Version Information:**

There are two standardized mnemonics for the Version Information section, "VERS" and "WRAP". You can access these mnemonics directly on the object, all additional Version Information mnemonics will be stored within the `.additional` field.

```rust
let version_info = example_las.version_information;
let our_version = version_info.version;
let is_wrapped = version_info.wrap;
```

**Well Information:**

There are standardized 'mnemonics' for the Well Information section, you can acccess these as fields on the well information object. Anything outside of these standardized mnemonics can be access in the `.additional` field.  The `.additional` field is a vector of non-standard mnemonics.

Standardized Well Information Mnemonics

```
strt, stop, step, null, comp, well, fld, loc, prov, cnty, stat, ctry, srvc, date, uwi, api
```

Access Well Info

```rust
let well_info = example_las.well_information;
// Get 'strt'
let well_info_strt = well_info.strt;
```

**Curve Information:**

All data within the curve information can be accessed via the `.curves` field. The `.curves` field is a vector of mnemonics.

```rust
let curve_info = example_las.curve_information;
// Print curves
println!("{:#?}", curve_info.curves);
// Get depth for example
let curve_depth = curve_info.curves
  .iter()
  .find(|c| c.description.contains("DEPTH"));
println!("curve_depth={curve_depth:?}");
```

### Programmatically Build LAS File

If you wanted to reconstruct our example .las file, but programmatically, here is how you can accomplish that.

```rust
let version_info = VersionInformation::new(
  Mnemonic::new("VERS".into(), None, MnemonicData::Float(2.0), "CWLS LOG ASCII STANDARD -VERSION 2.0".into()),
  Mnemonic::new("WRAP".into(), None, MnemonicData::Text("NO".into()), "ONE LINE PER DEPTH STEP".into()),
  vec![],
  vec![]
);

let well_info = WellInformation::new(
  Mnemonic::new("STRT".into(), Some("M".into()), MnemonicData::Float(1670.0), "START DEPTH".into()),
  Mnemonic::new("STOP".into(), Some("M".into()), MnemonicData::Float(1669.7500), "STOP DEPTH".into()),
  Mnemonic::new("STEP".into(), Some("M".into()), MnemonicData::Float(-0.1250), "STEP".into()),
  Mnemonic::new("NULL".into(), None, MnemonicData::Float(-999.25), "NULL VALUE".into()),
  Mnemonic::new("COMP".into(), None, MnemonicData::Text("ANY OIL COMPANY INC.".into()), "COMPANY".into()),
  Mnemonic::new("WELL".into(), None, MnemonicData::Text("ANY ET AL 12-34-12-34".into()), "WELL".into()),
  Mnemonic::new("FLD".into(), None, MnemonicData::Text("WILDCAT".into()), "FIELD".into()),
  Mnemonic::new("LOC".into(), None, MnemonicData::Text("12-34-12-34W5M".into()), "LOCATION".into()),
  Mnemonic::new("PROV".into(), None, MnemonicData::Text("ALBERTA".into()), "PROVINCE".into()),
  Mnemonic::new("SRVC".into(), None, MnemonicData::Text("ANY LOGGING COMPANY INC.".into()), "SERVICE COMPANY".into()),
  Mnemonic::new("DATE".into(), None, MnemonicData::Text("13-DEC-86".into()), "LOG DATE".into()),
  Mnemonic::new("UWI".into(), None, MnemonicData::Text("100123401234W500".into()), "UNIQUE WELL ID".into()),
  Mnemonic::default(),
  Mnemonic::default(),
  Mnemonic::default(),
  Mnemonic::default(),
  vec![Mnemonic::new("LIC".into(), None, MnemonicData::Text("23412".into()), "ERCB LICENCE NUMB".into())],
  vec![],
);

let curve_info = CurveInformation::new(
  vec![
    Mnemonic::new("DEPT".into(), Some("M".into()), MnemonicData::Text("".into()), "1 DEPTH".into()),
    Mnemonic::new("DT".into(), Some("US/M".into()), MnemonicData::Text("60 520 32 00".into()), "2 SONIC TRANSIT TIME".into()),
    Mnemonic::new("RHOB".into(), Some("K/M3".into()), MnemonicData::Text("45 350 01 00".into()), "3 BULK DENSITY".into()),
    Mnemonic::new("NPHI".into(), Some("V/V".into()), MnemonicData::Text("42 890 00 00".into()), "4 NEUTRON POROSITY".into()),
    Mnemonic::new("SFLU".into(), Some("OHMM".into()), MnemonicData::Text("07 220 04 00".into()), "5 SHALLOW RESISTIVITY".into()),
    Mnemonic::new("SFLA".into(), Some("OHMM".into()), MnemonicData::Text("07 222 01 00".into()), "6 SHALLOW RESISTIVITY".into()),
    Mnemonic::new("ILM".into(), Some("OHMM".into()), MnemonicData::Text("07 120 44 00".into()), "7 MEDIUM RESISTIVITY".into()),
    Mnemonic::new("ILD".into(), Some("OHMM".into()), MnemonicData::Text("07 120 46 00".into()), "8 DEEP RESISTIVITY".into()),
  ],
  vec![],
);

let param_info = ParameterInformation::new(
  vec![
    Mnemonic::new("MUD".into(), None, MnemonicData::Text("GEL CHEM".into()), "MUD TYPE".into()),
    Mnemonic::new("BHT".into(), Some("DEGC".into()), MnemonicData::Float(35.5), "BOTTOM HOLE TEMPERATURE".into()),
    Mnemonic::new("CSGL".into(), Some("M".into()), MnemonicData::Float(124.6), "BASE OF CASING".into()),
    Mnemonic::new("MATR".into(), None, MnemonicData::Text("SAND".into()), "NEUTRON MATRIX".into()),
    Mnemonic::new("MDEN".into(), None, MnemonicData::Float(2710.0), "LOGGING MATRIX DENSITY".into()),
    Mnemonic::new("RMF".into(), Some("OHMM".into()), MnemonicData::Float(0.216), "MUD FILTRATE RESISTIVITY".into()),
    Mnemonic::new("DFD".into(), Some("K/M3".into()), MnemonicData::Float(1525.0), "DRILL FLUID DENSITY".into()),
  ],
  vec![],
);

let ascii_data = AsciiLogData::new(
  vec![
    AsciiColumn::new("DEPTH".into(), vec![1670.0, 1669.875, 1669.750]),
    AsciiColumn::new("DT".into(), vec![123.450, 123.450, 123.450]),
    AsciiColumn::new("RHOB".into(), vec![2550.0, 2550.0, 2550.0]),
    AsciiColumn::new("NPHI".into(), vec![0.450, 0.450, 0.450]),
    AsciiColumn::new("SFLU".into(), vec![123.450, 123.450, 123.450]),
    AsciiColumn::new("SFLA".into(), vec![123.450, 123.450, 123.450]),
    AsciiColumn::new("ILM".into(), vec![110.200, 110.200, 110.200]),
    AsciiColumn::new("ILD".into(), vec![5.6, 5.6, 105.6]),
  ],
  vec![],
);

let other_info = OtherInformation::new(
  "Note: The logging tools became stuck at 625 metres causing the data between 625 metres and 615 metres to be invalid.".into(),
  vec![],
);

let las_file = LasFile::new(version_info, well_info, curve_info, ascii_data, Some(other_info), Some(param_info));

let json = las_file.to_json_str()?;
```