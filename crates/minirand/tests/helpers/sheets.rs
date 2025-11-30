use std::fs::File;
use std::io::{Result, Write, Read};

use std::path::PathBuf;
use std::fs::{self, OpenOptions};
use crate::helpers::DISTRIBUTION_LENGTH;

/// Write the average distribution to a CSV file
pub fn write_csv<T: ToString + std::fmt::Debug>(
    path: &str, 
    average: &[T; DISTRIBUTION_LENGTH]
) -> Result<()> {
    let file_path = PathBuf::from(path);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directory");
    }
    let mut file = File::create(file_path)?;

    // Write header
    writeln!(file, "Term,Count")?;

    // Write each value
    for i in 0..DISTRIBUTION_LENGTH {
        writeln!(file, "{},{:#?}", i + 1, average[i])?;
    }

    Ok(())
}



/// Creates a Microsoft Excel 2003 XML Spreadsheet (.xml)
/// that contains multiple sheets.
///
/// `sheets` is a slice where:
///   sheets[i] = Vec<Vec<String>>
///   Each Vec<String> is a row.
///   Each String is a cell.
///
/// The resulting XML file opens perfectly in Excel with multiple worksheets.
const COLS: usize = 2;
pub fn write_excel_xml<T: ToString>(
    path: &str,
    sheet_name: &str,
    data: &[T; DISTRIBUTION_LENGTH]
) -> Result<()> {
    
    let xml_sheet = build_sheet_xml(sheet_name, data);

    // Case 1 — File doesn't exist → create new workbook
    if !std::path::Path::new(path).exists() {
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;

        // XML header + workbook start
        writeln!(file, r#"<?xml version="1.0"?>"#)?;
        writeln!(
            file,
            r#"<Workbook xmlns="urn:schemas-microsoft-com:office:spreadsheet"
    xmlns:o="urn:schemas-microsoft-com:office:office"
    xmlns:x="urn:schemas-microsoft-com:office:excel"
    xmlns:ss="urn:schemas-microsoft-com:office:spreadsheet"
    xmlns:html="http://www.w3.org/TR/REC-html40">"#
        )?;

        file.write_all(xml_sheet.as_bytes())?;

        // Close workbook
        writeln!(file, "</Workbook>")?;

        return Ok(());
    }

    // Case 2 — File exists → append sheet before </Workbook>
    let mut contents = String::new();
    fs::File::open(path)?.read_to_string(&mut contents)?;

    // Find closing tag
    let end_tag = "</Workbook>";
    if let Some(index) = contents.rfind(end_tag) {
        let (start, end) = contents.split_at(index);

        // Rewrite file with new sheet inserted
        let mut file = fs::File::create(path)?;
        write!(file, "{}{}{}", start, xml_sheet, end)?;

        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Workbook: missing </Workbook>",
        ))
    }
}

/// Builds the XML for a single worksheet from a 1D fixed array.
fn build_sheet_xml<T: ToString>(
    sheet_name: &str,
    data: &[T; DISTRIBUTION_LENGTH]
) -> String {
    let mut out = String::new();

    out.push_str(&format!(r#"  <Worksheet ss:Name="{}">"#, escape_xml(sheet_name)));
    out.push_str("\n    <Table>\n");

    let mut col_index = 0;
    out.push_str("      <Row>\n");

    for item in data {
        let value = escape_xml(&item.to_string());
        out.push_str(&format!(
            r#"        <Cell><Data ss:Type="String">{}</Data></Cell>"#,
            value
        ));
        out.push('\n');

        col_index += 1;

        // If row filled → begin next row
        if col_index == COLS {
            col_index = 0;
            out.push_str("      </Row>\n      <Row>\n");
        }
    }

    out.push_str("      </Row>\n");
    out.push_str("    </Table>\n");
    out.push_str("  </Worksheet>\n");

    out
}

/// Minimal XML escaping
fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
     .replace("<", "&lt;")
     .replace(">", "&gt;")
     .replace("\"", "&quot;")
     .replace("'", "&apos;")
}