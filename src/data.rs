use crate::config_store::ConfigStore;

/// Represents a single row with a single value and header.
#[derive(Clone,PartialEq,PartialOrd,Debug,Default)]
pub struct Row {
    pub header: String,
    pub value: f64,
}//end struct Row

impl Row {
    /// Creates a new Row with given header and value
    pub fn new(header: String, value: f64) -> Row {Row{header,value}}
}//end impl for Row

/// Represents all the data from a file.
#[derive(Clone,PartialEq,PartialOrd,Debug,Default)]
pub struct Data {
    pub test_name: String,
    pub row_data: Vec<Row>,
	pub curve_data1: Vec<Row>,
	pub curve_data2: Vec<Row>,
	pub curve_data3: Vec<Row>,
	pub curve_data4: Vec<Row>,
	pub curve_data5: Vec<Row>
}//end struct Data

impl Data {
    /// Creates a new Data struct with given test_name, empty row_data.
    pub fn new(test_name: String) -> Data {
		Data{
			test_name,
			row_data:Vec::new(),
			curve_data1:Vec::new(),
			curve_data2:Vec::new(),
			curve_data3:Vec::new(),
			curve_data4:Vec::new(),
			curve_data5:Vec::new(),
			
		}
	}
    /// Creates a new Data struct with given test_name and row_data.
    pub fn new1(test_name: String, row_data: Vec<Row>, curve_data1: Vec<Row>, curve_data2: Vec<Row>, curve_data3: Vec<Row>, curve_data4: Vec<Row>,curve_data5: Vec<Row> ) -> Data {
		Data{
			test_name,
			row_data,
			curve_data1,
			curve_data2,
			curve_data3,
			curve_data4,
			curve_data5,

		}
	}
}//end impl Data

/// Gets the test name, such as "24-PCF4001", from the lines of a file.
pub fn get_test_name_from_lines(lines: &Vec<String>, config: &ConfigStore) -> Option<String> {
    for line in lines.iter() {
        if line.starts_with(&config.read_test_name_prefix) {
            let test_name = line.replace(&config.read_test_name_prefix,"");
            return Some(test_name);
        }//end if we found the test_name_prefix
    }//end looking for test_name in each line
    return None;
}//end get_test_name_from_lines()

/// Gets the index of the header before the rows we want to read, such as "Standard\t : \tAverage".
pub fn get_header_idx_from_lines(filename: &str, lines: &Vec<String>, config: &ConfigStore) -> Result<usize,String> {
    match config.read_start_mode {
        crate::config_store::ReadStartMode::Index => Ok(config.read_start_idx as usize),
        crate::config_store::ReadStartMode::Header => {
            for (i,line) in lines.iter().enumerate() {
                if line.eq(&config.read_start_header) {
                    return Ok(i);
                }//end if we found the start_header
            }//end looking for start header in each line
            return Err(format!("Could not find the header str \"{}\" in file \"{}\"", config.read_start_header, filename));
        },
    }//end matching the read_start_mode
}//end get_header_idx_from_lines()

/// Reads data from a given file.  
/// If the process fails, a string will be returned, holding a message about the failure.  
/// If any issues occur that don't prevent completion, they will be returned as well, as strings.
pub fn read_data_from_file(filename: &str, file_contents: &str, config: &ConfigStore) -> Result<(Data,Vec<String>),String> {
    // init error message list
    let mut errs = Vec::new();
    // split up contents into lines
    let lines: Vec<&str> = file_contents.split(['\n']).collect();
    // clean out any carriage returns and convert to string
    let lines: Vec<String> = lines.iter().map(|s| s.trim_matches('\r').to_string()).collect();
    // find the test_name
    let test_name = get_test_name_from_lines(&lines, config).unwrap_or_else(|| format!("Unknown test name from {:?}", filename));
    // find the line with the header
    let header_idx = match get_header_idx_from_lines(filename, &lines, config) {
        Ok(h) => h,
        Err(err) => return Err(err),
    };
    // start reading rows after the header idx
    let mut row_data = Vec::new();
    match config.read_row_mode {
        crate::config_store::ReadRowMode::Max => {
            for line in &lines[(header_idx+1)..=(header_idx+(config.read_max_rows as usize))] {
                let split_row: Vec<&str> = line.split(&config.read_row_split_char).collect();
                if split_row.len() < 2 {errs.push(format!("Couldn't find a proper split for \"{:?}\", len < 2", split_row));}
                else {
                    let row_header = split_row[0].to_string();
                    let row_value = split_row[1].trim().parse::<f64>();
                    match row_value {
                        Ok(row_value) => row_data.push(Row::new(row_header, row_value)),
                        Err(msg) => errs.push(format!("Failed to parse \"{}\" in line \"{}\" as f64:\n{}",split_row[1],line,msg)),
                    }//end matching whether we can parse the raw value
                }//end else we can get split stuff find
            }//end looping over each line specified
        },
        crate::config_store::ReadRowMode::Header => {
            let mut header_offset = 0;
            for line in &lines[(header_idx+1)..] {
                if config.read_row_headers.len() <= header_offset {break;}
                let this_row_header = config.read_row_headers.get(header_offset).expect("Already checked.");
                if line.starts_with(this_row_header) {
                    let split_row: Vec<&str> = line.split(&config.read_row_split_char).collect();
                    if split_row.len() < 2 {errs.push(format!("Couldn't find a proper split for \"{:?}\", len < 2", split_row));}
                    else {
                        let row_header = split_row[0].to_string();
                        let row_value = split_row[1].trim().parse::<f64>();
                        match row_value {
                            Ok(row_value) => row_data.push(Row::new(row_header, row_value)),
                            Err(msg) => errs.push(format!("Failed to parse \"{}\" in line \"{}\" as f64:\n{}",split_row[1],line,msg)),
                        }//end matching whether we can parse the row value
                    }//end else we can get split stuff find
                } else {errs.push(format!("{} breaks row pattern", line)); break;}
                header_offset += 1;
            }//end looping over each line specified
        },
    }//end matching the row read method

    // sort the row_data based off config
    row_data = sort_row_data(row_data, config);

	let mut curve1 = Vec::new();
	for line in &lines[41..=51] {
		let split_row: Vec<&str> = line.split("\t").collect();
		if split_row.len() < 2 {errs.push(format!("Couldn't find a proper split for \"{:?}\", len < 2", split_row));}
		else {
			let row_header = split_row[0].to_string();
			let row_value = split_row[1].trim().parse::<f64>();
			match row_value {
				Ok(row_value) => curve1.push(Row::new(row_header, row_value)),
				Err(msg) => errs.push(format!("Failed to parse \"{}\" in line \"{}\" as f64:\n{}",split_row[1],line,msg)),
			}//end matching whether we can parse the raw value
		}//end else we can get split stuff find
	}

	let mut curve2 = Vec::new();
	for line in &lines[54..=64] {
		let split_row: Vec<&str> = line.split("\t").collect();
		if split_row.len() < 2 {errs.push(format!("Couldn't find a proper split for \"{:?}\", len < 2", split_row));}
		else {
			let row_header = split_row[0].to_string();
			let row_value = split_row[1].trim().parse::<f64>();
			match row_value {
				Ok(row_value) => curve2.push(Row::new(row_header, row_value)),
				Err(msg) => errs.push(format!("Failed to parse \"{}\" in line \"{}\" as f64:\n{}",split_row[1],line,msg)),
			}//end matching whether we can parse the raw value
		}//end else we can get split stuff find
	}

	let mut curve3 = Vec::new();
	for line in &lines[67..=77] {
		let split_row: Vec<&str> = line.split("\t").collect();
		if split_row.len() < 2 {errs.push(format!("Couldn't find a proper split for \"{:?}\", len < 2", split_row));}
		else {
			let row_header = split_row[0].to_string();
			let row_value = split_row[1].trim().parse::<f64>();
			match row_value {
				Ok(row_value) => curve3.push(Row::new(row_header, row_value)),
				Err(msg) => errs.push(format!("Failed to parse \"{}\" in line \"{}\" as f64:\n{}",split_row[1],line,msg)),
			}//end matching whether we can parse the raw value
		}//end else we can get split stuff find
	}

	let mut curve4 = Vec::new();
	for line in &lines[80..=90] {
		let split_row: Vec<&str> = line.split("\t").collect();
		if split_row.len() < 2 {errs.push(format!("Couldn't find a proper split for \"{:?}\", len < 2", split_row));}
		else {
			let row_header = split_row[0].to_string();
			let row_value = split_row[1].trim().parse::<f64>();
			match row_value {
				Ok(row_value) => curve4.push(Row::new(row_header, row_value)),
				Err(msg) => errs.push(format!("Failed to parse \"{}\" in line \"{}\" as f64:\n{}",split_row[1],line,msg)),
			}//end matching whether we can parse the raw value
		}//end else we can get split stuff find
	}

	let mut curve5 = Vec::new();
	for line in &lines[93..=103] {
		let split_row: Vec<&str> = line.split("\t").collect();
		if split_row.len() < 2 {errs.push(format!("Couldn't find a proper split for \"{:?}\", len < 2", split_row));}
		else {
			let row_header = split_row[0].to_string();
			let row_value = split_row[1].trim().parse::<f64>();
			match row_value {
				Ok(row_value) => curve5.push(Row::new(row_header, row_value)),
				Err(msg) => errs.push(format!("Failed to parse \"{}\" in line \"{}\" as f64:\n{}",split_row[1],line,msg)),
			}//end matching whether we can parse the raw value
		}//end else we can get split stuff find
	}

    Ok((Data::new1(
		test_name,
		row_data,
		curve1,
		curve2,
		curve3,
		curve4,
		curve5,
	),errs))
}//end read_data_from_file()

/// Sorts the Vec of Rows based off of config row order pref.  
/// No rows will be removed or added, simply rearranged, with specified rows
/// in front of unspecified rows.  
/// Note: The sorting doesn't have great O(n) for speed or space, but n is small
/// enough for the expected input that it shouldn't matter.
pub fn sort_row_data(row_data: Vec<Row>, config: &ConfigStore) -> Vec<Row> {
    let mut new_row_data = Vec::new();
    let mut row_data_taken: Vec<bool> = vec![false; row_data.len()];
    for header_template in config.row_order_preference.iter() {
        for i in 0..row_data.len() {
            if row_data[i].header.eq(header_template) {
                new_row_data.push(row_data[i].clone());
                row_data_taken[i] = true;
            }//end if we found a match
        }//end searching for position of matching header
    }//end finding all the sorted headers we can

	// add any non-sorted values to new_row_data
	for i in 0..row_data.len() {
		if !row_data_taken[i] {
			new_row_data.push(row_data[i].clone());
		}//end if this element hasn't been moved already
	}//end adding non-sorted values to return vec

    return new_row_data;
}//end sort_row_data()
