use plotters::style::full_palette::BLUE_A700;

use super::*;
use std::{
    io::{Cursor, Seek},
    ops::Range,
};

pub(crate) fn generate_temp_las(size_in_mb: usize) -> std::io::Result<Cursor<Vec<u8>>> {
    let target_size_bytes = size_in_mb * 1024 * 1024; // 250 MB

    let buffer = Vec::with_capacity(target_size_bytes);
    let mut writer = Cursor::new(buffer);

    write!(
        writer,
        r#"~Version Information
VERS.                      2.0: CWLS Log ASCII Standard - VERSION 2.0
WRAP.                       NO: One line per depth step
~Well Information Block
STRT.FT               -14.0000: START DEPTH
STOP.FT              9405.0000: STOP DEPTH
STEP.FT                 0.5000: STEP
NULL.                -999.2500: NULL VALUE
COMP.          Foo Energy: COMPANY
WELL.          Brand 11 - 2 -3 4Z: WELL
FLD.                     Dyess: FIELD
LOC.           Lat: 30.111111--Long: -60.222222----: LOCATION
CNTY.                MakeBelieve: COUNTY
SRVC.      Oil Well Service Co: SERVICE COMPANY
DATE.          Mon Nov 21 10-28-03 2016: LOG DATE
UWI.              11-222-33333: UNIQUE WELL ID
STAT.                       OH: STATE
~Curve Information Block
DEPT.FT            0 000 00 00: Depth
GR.GAPI                       : Gamma Ray
CCL.                          : Casing Collar Locator
AMP3FT.MV                     : AMP3FT Amplitude
TT3FT.USEC                    : 3FT Travel Time
AMPS1.                        : AMPS1 Amplitude
AMPS2.                        : AMPS2 Amplitude
AMPS3.                        : AMPS3 Amplitude
AMPS4.                        : AMPS4 Amplitude
AMPS5.                        : AMPS5 Amplitude
AMPS6.                        : AMPS6 Amplitude
AMPS7.                        : AMPS7 Amplitude
AMPS8.                        : AMPS8 Amplitude
Temp.DEGF                     : Integral Temperature
LTEN.LB                       : Surface Line Tension
ADPTH.FT                      : Actual Depth
~Parameter Information Block
TDEPTH.FT               0.0000: Total Depth
~A  Depth        GR         CCL        AMP3FT      TT3FT       AMPS1       AMPS2       AMPS3       AMPS4       AMPS5       AMPS6       AMPS7       AMPS8        Temp        LTEN       ADPTH 
"#
    )?;

    writer.flush()?; // ensure header is written

    //let mut file = writer.into_inner()?;
    //let mut writer = BufWriter::new(&mut file);

    let mut depth = -14.0;

    while writer.get_ref().len() < target_size_bytes {
        writeln!(
            writer,
            "{:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3} {:8.3}",
            depth, 80.0, 1.0, 120.0, 55.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 180.0, 200.0, depth
        )?;

        depth += 0.5;
    }

    writer.flush()?;

    println!("Finished generating LAS file.");
    println!("Size: {} MB", writer.get_ref().len() / 1024 / 1024);

    writer.seek(std::io::SeekFrom::Start(0))?;

    Ok(writer)
}

pub(crate) fn depths(las: &LasFile) -> Vec<f64> {
    let first_header_col = &las.ascii_log_data.headers[0];
    if !first_header_col.to_lowercase().starts_with("dept") {
        panic!("first header column is not depth! got {first_header_col}");
    }
    las.ascii_log_data
        .rows
        .iter()
        .map(|row| row[0].parse::<f64>().unwrap())
        .collect()
}

/// One plotted curve (column)
pub(crate) struct PlotCurve {
    mnemonic: String,
    unit: Option<String>,
    col_idx: usize,
}

/// Build curves from LAS metadata (skip depth/index column)
pub(crate) fn plot_curves(las: &LasFile) -> Vec<PlotCurve> {
    las.ascii_log_data
        .headers
        .iter()
        .enumerate()
        .skip(1)
        .map(|(col_idx, mnemonic)| {
            let unit = las
                .curve_information
                .curves
                .iter()
                .find(|c| c.mnemonic == *mnemonic)
                .and_then(|c| {
                    if c.unit.is_some() && c.description.is_some() {
                        let s = format!(
                            "{} : {}",
                            c.unit.clone().expect("is_some"),
                            c.description.clone().expect("is_some")
                        );
                        let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
                        return Some(s);
                    }
                    None
                });

            PlotCurve {
                mnemonic: mnemonic.clone(),
                unit,
                col_idx,
            }
        })
        .collect()
}

/// Compute per-curve X axis range ignoring NULLs
pub(crate) fn x_range_for_curve(las: &LasFile, col_idx: usize, pad_frac: f64) -> Option<Range<f64>> {
    let null_value = match las.well_information.null.value.as_ref() {
        Some(LasValue::Text(v)) => v.parse().unwrap(), //v.value,
        Some(LasValue::Int(v)) => *v as f64,
        _ => return None,
    };

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for row in &las.ascii_log_data.rows {
        let val = &row[col_idx];
        let v = val.parse::<f64>().unwrap(); //&row[col_idx];
        if v == null_value {
            continue;
        }
        min = min.min(v);
        max = max.max(v);
    }

    if !min.is_finite() || min == max {
        return None;
    }

    let pad = (max - min) * pad_frac;
    Some((min - pad)..(max + pad))
}

pub(crate) fn plot_las(las: &LasFile, output: &str, curves_per_row: usize) -> Result<(), Box<dyn std::error::Error>> {
    let depths = depths(las);
    let curves = plot_curves(las);

    if depths.is_empty() || curves.is_empty() {
        return Ok(());
    }

    let depth_min = *depths.first().unwrap();
    let depth_max = *depths.last().unwrap();

    let root = BitMapBackend::new(output, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let num_rows = curves.len().div_ceil(curves_per_row);

    // Split vertically into rows
    let row_areas = root.split_evenly((num_rows, 1));

    let null_value = match las.well_information.null.value.as_ref() {
        Some(LasValue::Text(v)) => v.parse().unwrap(), //v.value,
        Some(LasValue::Int(v)) => *v as f64,
        _ => unreachable!(),
    };

    for (row_idx, row_area) in row_areas.into_iter().enumerate() {
        let start = row_idx * curves_per_row;
        let end = (start + curves_per_row).min(curves.len());
        let row_curves = &curves[start..end];

        // IMPORTANT: split this row into EXACTLY the number of curves it has
        let track_areas = row_area.split_evenly((1, row_curves.len()));

        for (area, curve) in track_areas.into_iter().zip(row_curves.iter()) {
            let x_range = match x_range_for_curve(las, curve.col_idx, 0.05) {
                Some(r) => r,
                None => continue,
            };

            let mut chart = ChartBuilder::on(&area)
                .margin(10)
                .set_label_area_size(LabelAreaPosition::Left, 60)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(
                    format!(
                        "{}{}",
                        curve.mnemonic,
                        curve.unit.as_deref().map(|u| format!(" ({u})")).unwrap_or_default()
                    ),
                    ("sans-serif", 14),
                )
                .build_cartesian_2d(
                    x_range,
                    depth_max..depth_min, // depth increases downward
                )?;

            chart.configure_mesh().disable_mesh().x_labels(6).y_labels(15).draw()?;

            let series = las.ascii_log_data.rows.iter().filter_map(|row| {
                let val = &row[curve.col_idx];
                let v = val.parse::<f64>().unwrap(); //&row[curve.col_idx];
                if v == null_value {
                    None
                } else {
                    Some((v, row[0].parse::<f64>().unwrap()))
                }
            });

            chart.draw_series(LineSeries::new(
                series,
                ShapeStyle {
                    color: BLUE_A700.to_rgba(),
                    filled: false,
                    stroke_width: 1,
                },
            ))?;
        }
    }

    root.present()?;
    Ok(())
}

// Helper - put at bottom to not take up space
pub(crate) fn _print_parsed_las_file(parsed_file: &ParsedLasFile) {
    for section in &parsed_file.sections {
        println!("{:?}", section.header.kind);
        if let Some(comments) = section.comments.as_ref() {
            for comment in comments {
                println!("\t{comment}");
            }
        }
        for entry in &section.entries {
            println!("\t{entry:?}");
        }

        let Some(headers) = &section.ascii_headers else {
            continue;
        };

        if !section.ascii_rows.is_empty() {
            let n_cols = headers.len();
            let mut col_widths = vec![0; n_cols];

            // Convert rows to strings with fixed decimal precision
            let string_table: Vec<Vec<String>> = section
                .ascii_rows
                .iter()
                .map(|row| row.iter().map(|v| format!("{:.3}", v)).collect())
                .collect();

            // First, compute max width per column, considering headers too
            for (i, h) in headers.iter().enumerate() {
                col_widths[i] = h.len();
            }
            for row in &string_table {
                for (i, cell) in row.iter().enumerate() {
                    col_widths[i] = col_widths[i].max(cell.len() + 3);
                }
            }

            // Print headers
            print!("\t");
            for (i, h) in headers.iter().enumerate() {
                print!("{:<width$} ", h, width = col_widths[i]); // Left-align headers
            }
            println!();

            // Print rows
            for row in &string_table {
                print!("\t");
                for (i, cell) in row.iter().enumerate() {
                    print!("{:<width$} ", cell, width = col_widths[i]);
                }
                println!();
            }
        }
    }
}
