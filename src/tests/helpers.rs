use plotters::style::full_palette::BLUE_A700;

use super::*;
use std::{fs::File, io::BufReader, ops::Range, path::PathBuf};

pub(crate) fn open_file(file_path: &str) -> BufReader<File> {
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file_path);
    let file = File::open(file_path).expect("open file error");
    BufReader::new(file)
}

pub(crate) fn parse_las_file(reader: BufReader<File>) -> Result<ParsedFile, ParseError> {
    let las_tokenizer = tokenizer::LasTokenizer::new(reader);
    let mut las_parser = parser::LasParser::new(las_tokenizer);
    las_parser.parse()
}

pub(crate) fn depths(las: &LasFile) -> Vec<f64> {
    let first_header_col = &las.ascii_log_data.headers[0];
    if !first_header_col.to_lowercase().starts_with("dept") {
        panic!("first header column is not depth! got {first_header_col}");
    }
    las.ascii_log_data.rows.iter().map(|row| row[0]).collect()
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
                        return Some(format!(
                            "{} : {}",
                            c.unit.clone().expect("is_some"),
                            c.description.clone().expect("is_some")
                        ));
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
    let null_value = match las.well_information.null.value {
        Some(LasValue::Float(v)) => v,
        Some(LasValue::Int(v)) => v as f64,
        _ => return None,
    };

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for row in &las.ascii_log_data.rows {
        let v = row[col_idx];
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

/// Plot LAS file to PNG
pub(crate) fn plot_las(las: &LasFile, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let depths = depths(las);
    let curves = plot_curves(las);

    if depths.is_empty() || curves.is_empty() {
        return Ok(());
    }

    let depth_min = *depths.first().unwrap();
    let depth_max = *depths.last().unwrap();

    let root = BitMapBackend::new(output, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    // One vertical track per curve
    let tracks = root.split_evenly((1, curves.len()));

    let null_value = match las.well_information.null.value {
        Some(LasValue::Float(v)) => v,
        Some(LasValue::Int(v)) => v as f64,
        _ => unreachable!(),
    };

    for (area, curve) in tracks.iter().zip(curves.iter()) {
        let x_range = match x_range_for_curve(las, curve.col_idx, 0.05) {
            Some(r) => r,
            None => continue,
        };

        let mut chart = ChartBuilder::on(area)
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption(
                format!(
                    "{}{}",
                    curve.mnemonic,
                    curve.unit.as_deref().map(|u| format!(" ({u})")).unwrap_or_default()
                ),
                ("sans-serif", 16),
            )
            .build_cartesian_2d(
                x_range,
                depth_max..depth_min, // depth increases downward
            )?;

        chart.configure_mesh().disable_mesh().x_labels(10).y_labels(20).draw()?;

        let series = las.ascii_log_data.rows.iter().filter_map(|row| {
            let v = row[curve.col_idx];
            if v == null_value { None } else { Some((v, row[0])) }
        });

        chart.draw_series(LineSeries::new(series, &BLUE_A700))?;
    }

    root.present()?;
    Ok(())
}

// Helper - put at bottom to not take up space
pub(crate) fn _print_parsed_las_file(parsed_file: &ParsedFile) {
    for section in &parsed_file.sections {
        println!("{:?}", section.header.kind);
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
