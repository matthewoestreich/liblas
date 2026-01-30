use liblas::*;

fn main() -> Result<(), LibLasErrorOld> {
    let version_info = VersionInformationOld::new(
        MnemonicOld::new(
            "VERS".into(),
            None,
            MnemonicData::Float(2.0),
            "CWLS LOG ASCII STANDARD -VERSION 2.0".into(),
        ),
        MnemonicOld::new(
            "WRAP".into(),
            None,
            MnemonicData::Text("NO".into()),
            "ONE LINE PER DEPTH STEP".into(),
        ),
        vec![],
        vec![],
    );

    let well_info = WellInformationOld::new(
        MnemonicOld::new(
            "STRT".into(),
            Some("M".into()),
            MnemonicData::Float(1670.0),
            "START DEPTH".into(),
        ),
        MnemonicOld::new(
            "STOP".into(),
            Some("M".into()),
            MnemonicData::Float(1669.7500),
            "STOP DEPTH".into(),
        ),
        MnemonicOld::new(
            "STEP".into(),
            Some("M".into()),
            MnemonicData::Float(-0.1250),
            "STEP".into(),
        ),
        MnemonicOld::new("NULL".into(), None, MnemonicData::Float(-999.25), "NULL VALUE".into()),
        MnemonicOld::new(
            "COMP".into(),
            None,
            MnemonicData::Text("ANY OIL COMPANY INC.".into()),
            "COMPANY".into(),
        ),
        MnemonicOld::new(
            "WELL".into(),
            None,
            MnemonicData::Text("ANY ET AL 12-34-12-34".into()),
            "WELL".into(),
        ),
        MnemonicOld::new("FLD".into(), None, MnemonicData::Text("WILDCAT".into()), "FIELD".into()),
        MnemonicOld::new(
            "LOC".into(),
            None,
            MnemonicData::Text("12-34-12-34W5M".into()),
            "LOCATION".into(),
        ),
        MnemonicOld::new(
            "PROV".into(),
            None,
            MnemonicData::Text("ALBERTA".into()),
            "PROVINCE".into(),
        ),
        MnemonicOld::new(
            "SRVC".into(),
            None,
            MnemonicData::Text("ANY LOGGING COMPANY INC.".into()),
            "SERVICE COMPANY".into(),
        ),
        MnemonicOld::new(
            "DATE".into(),
            None,
            MnemonicData::Text("13-DEC-86".into()),
            "LOG DATE".into(),
        ),
        MnemonicOld::new(
            "UWI".into(),
            None,
            MnemonicData::Text("100123401234W500".into()),
            "UNIQUE WELL ID".into(),
        ),
        MnemonicOld::default(),
        MnemonicOld::default(),
        MnemonicOld::default(),
        MnemonicOld::default(),
        vec![MnemonicOld::new(
            "LIC".into(),
            None,
            MnemonicData::Text("23412".into()),
            "ERCB LICENCE NUMB".into(),
        )],
        vec![],
    );

    let curve_info = CurveInformationOld::new(
        vec![
            MnemonicOld::new(
                "DEPT".into(),
                Some("M".into()),
                MnemonicData::Text("".into()),
                "1 DEPTH".into(),
            ),
            MnemonicOld::new(
                "DT".into(),
                Some("US/M".into()),
                MnemonicData::Text("60 520 32 00".into()),
                "2 SONIC TRANSIT TIME".into(),
            ),
            MnemonicOld::new(
                "RHOB".into(),
                Some("K/M3".into()),
                MnemonicData::Text("45 350 01 00".into()),
                "3 BULK DENSITY".into(),
            ),
            MnemonicOld::new(
                "NPHI".into(),
                Some("V/V".into()),
                MnemonicData::Text("42 890 00 00".into()),
                "4 NEUTRON POROSITY".into(),
            ),
            MnemonicOld::new(
                "SFLU".into(),
                Some("OHMM".into()),
                MnemonicData::Text("07 220 04 00".into()),
                "5 SHALLOW RESISTIVITY".into(),
            ),
            MnemonicOld::new(
                "SFLA".into(),
                Some("OHMM".into()),
                MnemonicData::Text("07 222 01 00".into()),
                "6 SHALLOW RESISTIVITY".into(),
            ),
            MnemonicOld::new(
                "ILM".into(),
                Some("OHMM".into()),
                MnemonicData::Text("07 120 44 00".into()),
                "7 MEDIUM RESISTIVITY".into(),
            ),
            MnemonicOld::new(
                "ILD".into(),
                Some("OHMM".into()),
                MnemonicData::Text("07 120 46 00".into()),
                "8 DEEP RESISTIVITY".into(),
            ),
        ],
        vec![],
    );

    let param_info = ParameterInformationOld::new(
        vec![
            MnemonicOld::new(
                "MUD".into(),
                None,
                MnemonicData::Text("GEL CHEM".into()),
                "MUD TYPE".into(),
            ),
            MnemonicOld::new(
                "BHT".into(),
                Some("DEGC".into()),
                MnemonicData::Float(35.5),
                "BOTTOM HOLE TEMPERATURE".into(),
            ),
            MnemonicOld::new(
                "CSGL".into(),
                Some("M".into()),
                MnemonicData::Float(124.6),
                "BASE OF CASING".into(),
            ),
            MnemonicOld::new(
                "MATR".into(),
                None,
                MnemonicData::Text("SAND".into()),
                "NEUTRON MATRIX".into(),
            ),
            MnemonicOld::new(
                "MDEN".into(),
                None,
                MnemonicData::Float(2710.0),
                "LOGGING MATRIX DENSITY".into(),
            ),
            MnemonicOld::new(
                "RMF".into(),
                Some("OHMM".into()),
                MnemonicData::Float(0.216),
                "MUD FILTRATE RESISTIVITY".into(),
            ),
            MnemonicOld::new(
                "DFD".into(),
                Some("K/M3".into()),
                MnemonicData::Float(1525.0),
                "DRILL FLUID DENSITY".into(),
            ),
        ],
        vec![],
    );

    let ascii_data = AsciiLogDataOld::new(
        vec![
            AsciiColumnOld::new("DEPTH".into(), vec![1670.0, 1669.875, 1669.750]),
            AsciiColumnOld::new("DT".into(), vec![123.450, 123.450, 123.450]),
            AsciiColumnOld::new("RHOB".into(), vec![2550.0, 2550.0, 2550.0]),
            AsciiColumnOld::new("NPHI".into(), vec![0.450, 0.450, 0.450]),
            AsciiColumnOld::new("SFLU".into(), vec![123.450, 123.450, 123.450]),
            AsciiColumnOld::new("SFLA".into(), vec![123.450, 123.450, 123.450]),
            AsciiColumnOld::new("ILM".into(), vec![110.200, 110.200, 110.200]),
            AsciiColumnOld::new("ILD".into(), vec![5.6, 5.6, 105.6]),
        ],
        vec![],
    );

    let other_info = OtherInformationOld::new(
    "Note: The logging tools became stuck at 625 metres causing the data between 625 metres and 615 metres to be invalid.".into(),
    vec![],
  );

    let las_file = LasFileOld::new(
        version_info,
        well_info,
        curve_info,
        ascii_data,
        Some(other_info),
        Some(param_info),
    );

    let json = las_file.to_json_str()?;

    println!("{json}");

    return Ok(());
}
