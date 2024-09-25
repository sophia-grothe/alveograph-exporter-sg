use serde::{Deserialize, Serialize};
use std::{env, fs::{self, File}, io::Write, path::PathBuf};

/// An enum to represent different ways of finding the header in a file.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Deserialize, Serialize)]
pub enum ReadStartMode {
    /// If this ReadStartMode is selected, then input will look for
    /// an exact header in the file, read lines under it, then exit.
    Header,
    /// If this ReadStartMode is selected, then input will look for
    /// a line index in the file and assume that's the header. It will
    /// then read lines underneath that one, ignoring aditional lines 
    /// after reading finishes.
    Index,
}

impl ReadStartMode {
    /// Returns string representation of variant.
    pub fn to_string(&self) -> String {
        match self {
            ReadStartMode::Header => "Header".to_string(),
            ReadStartMode::Index => "Index".to_string(),
        }//end matching self
    }//end to_string()

    /// Attempts to match label to variant.
    pub fn from_str(str: &str) -> Option<ReadStartMode> {
        match str {
            "Header" => Some(ReadStartMode::Header),
            "Index" => Some(ReadStartMode::Index),
            _ => None,
        }//end matching str
    }//end from_str()
}//end impl for ReadStartMode

/// An enum to represent different ways of reading rows after the header.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Deserialize, Serialize)]
pub enum ReadRowMode {
    /// If this ReadRowMode is selected, then input will only read
    /// continuous rows under the Start Header that match specified
    /// row headers before exiting.
    Header,
    /// If this ReadRowMode is selected, then input will simply read
    /// a specified amount of rows after the Start Header before exiting.
    Max,
}//end enum ReadRowMode

impl ReadRowMode {
    /// Returns string representation of variant.
    pub fn to_string(&self) -> String {
        match self {
            ReadRowMode::Header => "Header".to_string(),
            ReadRowMode::Max => "Max".to_string(),
        }//end matching self
    }//end to_string()

    /// Attempts to match label to variant.
    pub fn from_str(str: &str) -> Option<ReadRowMode> {
        match str {
            "Header" => Some(ReadRowMode::Header),
            "Max" => Some(ReadRowMode::Max),
            _ => None,
        }//end matching str
    }//end from_str()
}//end impl for ReadRowMode

/// This struct is meant to store configuration information
/// in a way that is not reliant on a specific ui implementation,
/// such that it can be passed around easily.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Deserialize, Serialize)]
pub struct ConfigStore {
    /// The exact string header above where the data rows start.
    pub read_start_header: String,
    /// An optional alternative to read_start_header. If the correct ReadMode is
    /// enabled, then the program will treat the line with this index 
    /// as read_start_header.
    pub read_start_idx: u16,
    /// A list of exact string row header/starts that are
    /// directly below the read start header.
    pub read_row_headers: Vec<String>,
    /// The maximum number of rows to read after the read_start_header, assuming
    /// that all headers are valid.
    pub read_max_rows: u16,
    /// The method to use for finding the start of rows to read.
    pub read_start_mode: ReadStartMode,
    /// The method to use for finding rows under the start to read.
    pub read_row_mode: ReadRowMode,
    /// An optional list of specific rows that should be put in a particular order.
    /// Any rows starting with an element of this vector will be placed at the
    /// beginning of the output in an order matching the one here. Any rows found
    /// which do not match an element of this vector will be placed in output in
    /// the order they are found.
    pub row_order_preference: Vec<String>,
    /// The character (or string) to split on when separating the header
    /// from the data in a row.
    pub read_row_split_char: String,
    /// The string directly in front of the test-name, which is used to label
    /// which file data came from.
    pub read_test_name_prefix: String,
}//end struct ConfigStore

impl Default for ConfigStore {
    fn default() -> Self {
        let read_row_headers = vec![
                "P","L","G","W","P/L","Ie","K",
                "SH","Dmin","Dmax","H2O",
            ].iter().map(|str| str.to_string()).collect();
        let row_order_preference = vec![
                "P","L","G","W","P/L","Ie","K",
                "SH","Dmin","Dmax","H2O",
            ].iter().map(|str| str.to_string()).collect();

        Self {
            read_start_header: "Standard\t : \tAverage".to_string(),
            read_start_idx: 105,
            read_row_headers,
            read_max_rows: 11,
            read_start_mode: ReadStartMode::Header,
            read_row_mode: ReadRowMode::Header,
            row_order_preference,
            read_row_split_char: "\t".to_string(),
            read_test_name_prefix: "Test name\t:\t".to_string(),
        }//end struct construction
    }//end default()
}//end impl Default for ConfigStore

/// Attempts to determine the path to the config file.  
/// Assumes that config file has filename of config_name and extension of .config.  
/// If create_if_missing is true, and the file at path does not exist, then it will be created with default values.  
/// If create_if_missing is false, then this function does not check whether or not the filepath exists.
pub fn try_read_config_path(config_name: &str, create_if_missing: bool) -> Result<PathBuf, String> {
    // directory which contains exe this program runs from
    let exe_path = {
        match env::current_exe() {
            Ok(exe_path) => exe_path,
            Err(error) => return Err(error.to_string()),
        }//end matching whether we could get the current exe path
    };

    // set config path to be same parent as exe_path, but config_name
    let config_path = {
        let mut config_path = exe_path.clone();
        config_path.set_file_name(config_name);
        config_path.set_extension("json");
        config_path
    };

    // depending on parameter, ensure config file exists
    if !config_path.exists() && create_if_missing {
        match File::create(config_path.clone()) {
            Ok(mut file) => {
                let default_config = ConfigStore::default();
                match serde_json::to_string_pretty(&default_config) {
                    Ok(serialized_config) => {
                        match file.write_all(serialized_config.as_bytes()) {
                            Ok(_) => (),
                            Err(error) => return Err(error.to_string()),
                        }//end matching whether file write was successful
                    },
                    Err(error) => return Err(error.to_string()),
                }//end matching whether or not serde serialization worked
            },
            Err(error) => return Err(error.to_string()),
        }//end matching if file was created
    }//end if config_path does not exist

    Ok(config_path)
}//end try_read_config_path()

/// Attempts to read contents of file at path and deserialize into ConfigStore object.
pub fn try_read_config(config_path: &PathBuf) -> Result<ConfigStore,String> {
    match fs::read_to_string(config_path) {
        Ok(file_contents) => {
            match serde_json::from_str(&file_contents) {
                Ok(config_store) => Ok(config_store),
                Err(error) => Err(error.to_string()),
            }//end matching whether we can deserialize config
        },
        Err(error) => Err(error.to_string())
    }//end matching whether we could read string from file
}//end try_read_config()

/// Attempts to write given config_store to the given path.
pub fn try_write_config(config_path: &PathBuf, config_store: &ConfigStore) -> Result<(),String> {
    match File::create(config_path) {
        Ok(mut file) => {
            match serde_json::to_string_pretty(config_store) {
                Ok(config_serial) => {
                    match file.write_all(config_serial.as_bytes()) {
                        Ok(_) => Ok(()),
                        Err(error) => Err(error.to_string()),
                    }//end matching whether or not write succeeded
                },
                Err(error) => Err(error.to_string()),
            }//end matching whether we could serialize config
        },
        Err(error) => Err(error.to_string()),
    }//end matching whether we can see the file
}//end try_write_config()
