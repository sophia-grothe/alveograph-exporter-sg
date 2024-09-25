
/// This module contains code for loading
/// data from a file and storing that data.
pub mod data;

/// This module contains code for storing,
/// loading, and saving configuration information.
pub mod config_store;

/// This module contains code for processing
/// data from the data module and exporting
/// that data to a file.
pub mod process;

/// This module contains automated testing for
/// various functions in other modules
#[cfg(test)]
pub mod test;
