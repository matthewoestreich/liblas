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

/*
#  Comment before version info
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
*/
