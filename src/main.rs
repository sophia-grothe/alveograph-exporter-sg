#![cfg_attr(not(debug_assertions),windows_subsystem = "windows")]
use std::{fs, path::PathBuf, time::{Duration, Instant}};

use alveograph_exporter_s::{config_store::{self, ConfigStore}, data::{self, Data}, process::{close_workbook, get_workbook, write_output_to_sheet}};
use gui::GUI;

mod gui;

fn main() {
    // setup gui
    let mut gui = GUI::initialize();
    let recv = gui.get_receiver();

    // get config information
    let config_name = "config";
    let mut config_path: Option<PathBuf> = None;
    let mut config_store: ConfigStore = ConfigStore::default();

    // make sure we get config information, update gui, walk user through fix if necessary
    ensure_config_valid(&mut gui, &mut config_store, &mut config_path, config_name);
    // update gui with given config store
    let _ = gui.set_config_store(&config_store);

    while gui.wait() {
        match recv.recv() {
            Some(gui::InterfaceMessage::AppClosing) => {
                if let Some(config_path_v) = config_path {
                    match gui.get_config_store() {
                        Err(msg) => gui.integrated_dialog_alert(&format!("Couldn't get save config store because:\n{}", msg)),
                        Ok(config) => {
                            if let Err(msg) = config_store::try_write_config(&config_path_v, &config) {
                                gui.integrated_dialog_alert(&format!("We weren't able to save the config file. Error message is:\n{}", msg));
                            }//end if writing is not successful
                        },
                    }//end matching whether or not we can get the config store
                    // move this back after we're done with it
                    config_path = Some(config_path_v);
                }//end if we have valid config_path
                GUI::quit();
            },
            Some(gui::InterfaceMessage::ConfigReset) => {
                if let Err(msg) = gui.set_config_store(&ConfigStore::default()) {
                    gui.integrated_dialog_alert(&format!("There was an issue resetting the config!:\n{}", msg));
                }//end if we had an error while trying to reset config store
            },
            Some(gui::InterfaceMessage::Process) => {
                // get input and output paths from gui/user
                let input_paths = gui.get_last_input_paths();
                let output_path = gui.get_last_output_paths();
                // make sure we have valid input and output paths
                let input_valid = validate_input_paths(&input_paths, &mut gui);
                let output_path = validate_output_path(output_path, &mut gui);
                if !input_valid || output_path.is_err() {continue;}
                let output_path = output_path.expect("We already checked it wasn't an error.");
                // grab configuration details from the gui
                config_store = gui.get_config_store().unwrap();
                // proceed with processing calls
                gui.start_wait();
                let start = Instant::now();
                let mut data_files: Vec<Data> = Vec::new();
                for (i,input_path) in input_paths.iter().enumerate() {
                    match fs::read_to_string(input_path) {
                        Err(msg) => gui.integrated_dialog_alert(&format!("There was an error reading from path \"{}\":\n{}",input_path.to_string_lossy(),msg)),
                        Ok(file_contents) => {
                            let filename = match input_path.file_name() {
                                Some(osstr) => osstr.to_string_lossy().into_owned(),
                                None => "UNKNOWN FILENAME".to_string(),
                            };
                            match data::read_data_from_file(&filename, &file_contents, &config_store) {
                                Err(msg) => {
                                    if i >= input_paths.len() - 1 {
                                        gui.integrated_dialog_alert(&format!("There was an issue reading from path \"{}\". The issue was:\n{}",input_path.to_string_lossy(),msg));
                                    }//end if this is last file
                                    else {
                                        if !gui.integrated_dialog_yes_no(&format!("There was an issue reading from path \"{}\". The issue will be displayed below.\n\tDo you want to continue processing?\n\n{}",input_path.to_string_lossy(),msg)) {
                                            break;} else {continue;}
                                    }//end else there are a bunch more files
                                },
                                Ok((data,errs)) => {
                                    if errs.len() > 0 {
                                        if !gui.integrated_dialog_yes_no(&format!("There were issue(s) parsing data from path {}. The issues will be displayed below.\n\tDo you still want to use output from this file?\n\n{}",input_path.to_string_lossy(),errs.join("\n"))) {
                                            if i >= input_paths.len() - 1 && !gui.integrated_dialog_yes_no(&format!("Since you said you don't want to use the current file, do you want to continue processing?")) {
                                                break;} else {continue;}
                                        }//end if user said they don't want to include current, potentially broken file
                                    }//end if there is at least one error
                                    data_files.push(data);
                                },
                            }//end matching whether we can read data from this file
                        },
                    }//end matching whether or not we can get a string from the input file
                }//end looping over each input file to read from

                let mut wb = get_workbook();
                let mut wrote_to_output = false;
                let mut closed_output = false;
                if let Err(err) = write_output_to_sheet(&mut wb, &data_files, "alveograph-exporter-output") {
                    gui.integrated_dialog_alert(&format!("There was an issue writing output data to the sheet:\n{}",err));
                }//end if there was an error writing to the sheet
                else {wrote_to_output = true;}
                if let Err(err) = close_workbook(&mut wb, &output_path) {
                    gui.integrated_dialog_alert(&format!("There was an issue closing the workbook \"{}\". \nIs it open? \n{}", output_path.to_string_lossy(),err));
                }//end if there was an error closing the workbook
                else {closed_output = true;}

                // perform cleanup after finishing processing
                gui.clear_last_input_paths();
                gui.clear_last_output_path();
                if wrote_to_output && closed_output {
                    eprintln!("Finished processing file(s).");
                    let total_duration = start.elapsed();
                    if gui.integrated_dialog_yes_no(&format!("Processing has completed successfully in {} miliseconds. Would you like to open the folder where the output file is located?", format_milliseconds(total_duration))) {
                        opener::reveal(output_path).unwrap_or_else(|e| eprintln!("Couldn't reveal output due to {}", e));
                    }//end if user want to open folder
                }//end if output file seems to be created ok
                gui.end_wait();
            },
            None => {},
        }//end matching message received
    }//end main application loop
}//end main function

/// Given a duration, gives a string of a float representation of the number
/// of milliseconds. If the parse fails, it will return the whole
/// number of milliseconds as a string.
fn format_milliseconds(duration: Duration) -> String {
	match format!("{}",duration.as_micros()).parse::<f64>() {
		Err(_) => format!("{}",duration.as_millis()),
		Ok(micros) => format!("{0:.2}", micros / 1000.),
	}//end matching whether we can parse float-micros
}//end format_milliseconds(duration)

/// Returns true if the input paths are more than 0 and valid for processing.  
/// If invalid, shows dialog message about issue.
fn validate_input_paths(input_paths: &Vec<PathBuf>, gui: &mut GUI) -> bool {
    if input_paths.len() > 0 {true}
    else {
        gui.integrated_dialog_alert("There are no input files selected. Please select one before processing.");
        false
    }
}//end validate_input_paths()

/// Returns true if the output_path given is valid for processing.  
/// If invalid, shows dialog message about issue.
fn validate_output_path(output_path: Option<PathBuf>, gui: &mut GUI) -> Result<PathBuf,()> {
    let output_txt = gui.get_output_path_text();
    if output_txt.len() == 0 {
        gui.integrated_dialog_alert("No output path selected. Please select one before processing.");
        return Err(());
    }//end if no selected file OR user deleted selection
    else if output_path.is_some() {
        return Ok(output_path.expect("Already checked that output_path is_some()"));
    }//end else case that both txt and path are valid, all seems good
    else {
        let input_paths = gui.get_last_input_paths();
        let input_dir = match input_paths.first() {
            Some(first_input_path) => match first_input_path.parent() {
                Some(parent_path) => parent_path.to_string_lossy().to_string(),
                None => "".to_string(),
            },
            None => "".to_string(),
        };
        if input_dir != "" {
            let mut output_pathbuf = PathBuf::new();
            output_pathbuf.push(input_dir);
            output_pathbuf.push(output_txt);
            output_pathbuf.set_extension("xlsx");
            if !output_pathbuf.exists() || gui.integrated_dialog_yes_no("The output file you specified already exists. Are you sure you want to overwrite it?") {
                return Ok(output_pathbuf);
            } else {return Err(());}
        } else {
            gui.integrated_dialog_alert("Couldn't use input paths to determine output path for typed name. Please select valid input files.");
            return Err(());
        }//end else we couldn't figure out input dir
    }//end else case that txt is valid, but path is not, must generate path
}//end validate_output_path()

/// Gets the config information from the config file.
/// If we encounter issues with that, lets the user know through the gui.
fn ensure_config_valid(
    gui: &mut GUI,
    config_store: &mut ConfigStore,
    config_path: &mut Option<PathBuf>,
    config_name: &str
) {
    *config_store = ConfigStore::default();
    *config_path = None;

    match config_store::try_read_config_path(config_name, false) {
        Ok(config_path_tmp) => {
            if !config_path_tmp.exists() {
                match config_store::try_write_config(&config_path_tmp, &config_store) {
                    Ok(_) => {
                        *config_path = Some(config_path_tmp);
                    },
                    Err(msg) => gui.integrated_dialog_alert(&format!("I couldn't find an exisitng configuration file, so I tried creating one, but that also failed...\nYou can use the default config, but it won't be saved when you exit.\nIf you contine seeing this message, please contact the developer. Error message below:\n{}", msg)),
                }//end matching whether we can write the default config
            }//end if the config file does not already exist
            else {
                match config_store::try_read_config(&config_path_tmp) {
                    Ok(config_store_tmp) => *config_store = config_store_tmp,
                    Err(msg) => {
                        gui.integrated_dialog_alert(&format!("I found a config file, but I couldn't read it. Things like this can happen during version changes or if the file is edited incorrectly. I'm going to go ahead and create a new file with the default settings for you. Here's the error message:\n{}",msg));
                        match config_store::try_write_config(&config_path_tmp, config_store) {
                            Ok(_) => {},
                            Err(msg) => gui.integrated_dialog_alert(&format!("Ok, so I tried writing a new config file, but I wasn't able to. Was it open? Either way, if you keep seeing messages like this, please contact the developer. You can still use the program with the default config and even edit the settings while you use it, but I can't keep track of those changes after you close the program. Error message below:\n{}", msg)),
                        }
                    },
                }//end matching whether or not we can read from the config file we have
                *config_path = Some(config_path_tmp);
            }//end else we do have a config file to read
        },
        Err(msg) => gui.integrated_dialog_alert(&format!("Could not determine the path to a config file:\n{}", msg)),
    }//end matching whether or not we can get config path
}//end ensure_config_valid()
