use std::path::PathBuf;
use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, Workbook, XlsxError};
use crate::data::Data;

/// The border style to use for all the cells we write to.
const BORDER_FORMAT: FormatBorder = FormatBorder::Thin;
/// The font size for cells in the header row.
const FONT_SIZE_HEADER: i32 = 14;
/// The font size for cells with test name label.
const FONT_SIZE_TEST_NAME: i32 = 11;
/// The font size for cells with numeric data in them.
const FONT_SIZE_DATA: i32 = 11;
/// The row upon which the header starts.
/// This acts as a vertical offset for the whole output.
const HEADER_START_ROW: u32 = 2;

/// Creates an excel workbook, which can then be used in
/// further funtions.
pub fn get_workbook() -> Workbook {
    Workbook::new()
}//end get_workbook()

/// Should be called after done working with a workbook, for performance reasons.
pub fn close_workbook(workbook: &mut Workbook, output_path: &PathBuf) -> Result<(),XlsxError> {
    workbook.save(output_path)?;
    Ok(())
}//end close_workbook(workbook)

/// Writes output from another function to a workbook that has already
/// been created. After you're done calling this function (however many times),  
/// make sure to call process::close_workbook().
pub fn write_output_to_sheet(workbook: &mut Workbook, data: &Vec<Data>, sheet_name: &str) -> Result<(),XlsxError> {
    let sheet = workbook.add_worksheet();//workbook.create_sheet(sheet_name);
    sheet.set_name(sheet_name)?;
    if data.len() < 1 {return Ok(());}

    // write the header row
    let bold = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_border(BORDER_FORMAT)
        .set_font_size(FONT_SIZE_HEADER);
    sheet.write_with_format(HEADER_START_ROW,0,"Test Name", &bold)?;
    for (index, row) in data.first().expect("already checked").row_data.iter().enumerate() {
        let index = index as u16;
        sheet.write_with_format(HEADER_START_ROW,index + 1, row.header.clone(),&bold)?;
    }//end writing each row header

    let test_name_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(BORDER_FORMAT)
        .set_font_size(FONT_SIZE_TEST_NAME);
    let default_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(BORDER_FORMAT)
        .set_font_size(FONT_SIZE_DATA);
    let mut row_num = HEADER_START_ROW + 1;
    for data_file in data {
        sheet.write_with_format(row_num,0,data_file.test_name.clone(), &test_name_format)?;
        for (col_offset,row) in data_file.row_data.iter().enumerate() {
            let col_offset = col_offset as u16;
            sheet.write_number_with_format(row_num,1+col_offset,row.value, &default_format)?;
        }//end looping over each row of data to place in a column
        row_num += 1;
    }//end looping over each data file

    sheet.set_column_width(0, 14.5)?;

    Ok(())
}//end write_output_to_sheet()
