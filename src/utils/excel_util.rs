use excel::*;

pub fn export_excel_workbook_with_image(full_file_path:String,sheet_name:String, headers:Vec<String>,
	contents:Vec<Vec<String>>, is_images:Vec<bool>){
	let mut wb = Workbook::create(&full_file_path);
	let mut sheet = wb.create_sheet(&sheet_name);

	sheet.add_column(Column { width: 10.0 });
	for _item in headers.clone() {
		sheet.add_column(Column { width: 20.0 });
	}

	wb.write_sheet(&mut sheet, |sheet_writer| {
		let sw = sheet_writer;
		let _empty_head_row = Row::new();
		sw.append_blank_rows(1);
		let mut head_row = Row::new();
		head_row.add_cell("".to_string());
		for header in headers.clone() {
			head_row.add_cell(header);
		}
		let _ = sw.append_row(head_row);
		for content in contents.clone(){
			let mut content_row = Row::new();
			content_row.add_cell("".to_string());
			let mut col_index = 0;
			for item in content.clone() {
				let is_image = is_images.get(col_index);
				match is_image {
					Some(is_image) => {
						if *is_image {
							content_row.add_cell("".to_string());
						}else {
							content_row.add_cell(item);
						}
					},
					None => content_row.add_cell(item),
				}
				col_index += 1;
			}
			let _ = sw.append_row(content_row);
		}
		Ok(())
	}).expect("write excel error!");
	wb.close().expect("close excel error!");
}
