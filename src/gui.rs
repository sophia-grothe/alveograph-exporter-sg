use std::{cell::RefCell, path::PathBuf, rc::Rc};

use alveograph_exporter_s::config_store::{ConfigStore, ReadRowMode, ReadStartMode};
use fltk::{app::{self, App, Receiver, Sender}, button::Button, dialog::{self, BeepType, FileDialogOptions, FileDialogType, NativeFileChooser}, enums::{Align, Color, Event, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Tile}, image::PngImage, input::IntInput, menu::Choice, misc::HelpView, prelude::{ButtonExt, DisplayExt, GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt}, text::{TextBuffer, TextDisplay, TextEditor, WrapMode}, window::{self, Window}};

/// Width in pixels of the main window
const WINDOW_WIDTH: i32 = 750;
/// Height in pixels of the main window
const WINDOW_HEIGHT: i32 = 465;

/// FrameType to use for all major groups of widgets
const GROUP_FRAME: FrameType = FrameType::GtkThinUpBox;
/// Background color (set_color()) for the major group of headers information
const HEADER_GROUP_COLOR: Color = Color::from_rgb(250,240,248);
/// The width in pixels of the header group. 
/// This will affect the sizes of other groups.
const HEADER_GROUP_WIDTH: i32 = 450;
/// The height in pixels of the header group. 
/// This will affect the sizes of other groups.
const HEADER_GROUP_HEIGHT: i32 = 90;
/// The height in pixels of the io_controls group
const IO_CONTROLS_GROUP_HEIGHT: i32 = 175;
/// Background color (set_color()) for the major group of io controls
const IO_CONTROLS_GROUP_COLOR: Color = Color::from_rgb(245,255,250);
/// Background color (set_color()) for the major group of config settings
const CONFIG_GROUP_COLOR: Color = Color::from_rgb(220,239,220);
/// Background color (set_color()) for the major group of integrated dialog
const DIALOG_GROUP_COLOR: Color = Color::from_rgb(255,248,220);

/// Alignment to use for labels in the header group
const HEADER_LABEL_ALIGN: Align = Align::Inside.union(Align::Center);
/// Color (set_label_color()) to use for labels in the header group
const HEADER_LABEL_COLOR: Color = Color::from_rgb(0,0,64);

/// The width in pixels of each fileIO button in the fileIO section.
const IO_BTN_WIDTH: i32 = 150;
/// The height in pixels of each fileIO button in the fileIO section.
const IO_BTN_HEIGHT: i32 = 30;
/// The amount of padding in pixels to put around each fileIO button in the fileIO section.
const IO_BTN_PADDING: i32 = 10;
/// The FrameType to use with each fileIO button in the fileIO section.
const IO_BTN_FRAME: FrameType = FrameType::GtkRoundUpFrame;
/// The Down FrameType to use with each fileIO button in the fileIO section. 
/// This is the Frame used when the button is pressed down.
const IO_BTN_DOWN_FRAME: FrameType = FrameType::GtkRoundDownFrame;
/// The color to use with each fileIO button in the fileIO section.
const IO_BTN_COLOR: Color = Color::from_rgb(248,248,255);
/// The down color to use with each fileIO button in the fileIO section. 
/// This is the color when the button is pressed down.
const IO_BTN_DOWN_COLOR: Color = Color::from_rgb(240,255,240);
/// The height in pixels of each TextBox in the fileIO section. 
/// The width is calculated based on the space available and the padding.
const IO_BOX_HEIGHT: i32 = 30;
/// The amount of padding in pixels to put around each TextBox in the fileIO section.
const IO_BOX_PADDING: i32 = 10;
/// The FrameType to use for each TextBox in the fileIO section.
const IO_BOX_FRAME: FrameType = FrameType::GtkDownFrame;
/// The amount of padding in pixels to use around the process button in the fileIO section.
const IO_PRC_BTN_PADDING: i32 = 10;
/// The width in pixels of the process button in the fileIO section. 
/// The height is calculated based on the space available and the padding.
const IO_PRC_BTN_WIDTH: i32 = 250;
/// The Color to use for the textbox for input files in the fileIO section. 
/// A gray color is recommended in order to indicate that it cannot be edited by the user.
const IO_INPUT_BOX_COLOR: Color = Color::from_rgb(240,240,240);

/// The padding in pixels to give to the dialog text box
const DIALOG_BOX_PADDING: i32 = 10;
/// The height of the section where buttons appear in the dialog section
const DIALOG_BTNS_HEIGHT: i32 = 40;
/// The color to use for text in the dialog box
const DIALOG_BOX_TEXT_COLOR: Color = Color::from_rgb(0,0,64);
/// The color to use for the background in the dialog box
const DIALOG_BOX_COLOR: Color = Color::from_rgb(250,240,248);
/// The frame to use for the dialog box
const DIALOG_BOX_FRAME: FrameType = FrameType::GtkThinDownFrame;
/// The wrap mode to use for the dialog box
const DIALOG_BOX_WRAP_MODE: WrapMode = WrapMode::AtBounds;
/// The work wrapping margin to use for the dialog box
const DIALOG_BOX_WRAP_MARGIN: i32 = 1;
/// The alignment to use for the scrollbar in the dialog box
const DIALOG_BOX_SCROLL_ALIGN: Align = Align::Right;
/// The size in pixels for the scrollbar in the dialog box
const DIALOG_BOX_SCROLL_SIZE: i32 = 10;
/// The text size to use for the dialog box
const DIALOG_BOX_TEXT_SIZE: i32 = 16;
/// The color to use for the background of the space that dialog buttons appear in
const DIALOG_BTNS_BACK_COLOR: Color = Color::from_rgb(255,248,220);
/// The frame to use for the space holding the dialog buttons
const DIALOG_BTNS_BACK_FRAME: FrameType = FrameType::FlatBox;
/// The frame to use for each dialog button.
const DIALOG_BTN_FRAME: FrameType = FrameType::GtkRoundDownFrame;
/// The down frame to use for each dialog button. 
/// This is the frame that's used when the button is pressed down.
const DIALOG_BTN_DOWN_FRAME: FrameType = FrameType::GtkRoundDownFrame;
/// The color used for each dialog button.
const DIALOG_BTN_COLOR: Color = Color::from_rgb(245,245,245);
/// The down color used for each dialog button. 
/// This is the color displayed when the button is pressed down.
const DIALOG_BTN_DOWN_COLOR: Color = Color::from_rgb(224,255,255);

/// The amount of horizontal padding in pixels to apply to choices in the config section.
const CONF_CHOICE_HOR_PADDING: i32 = 5;
/// The amount of vertical padding in pixels to apply to choices in the config section.
const CONF_CHOICE_VER_PADDING: i32 = 20;
/// The height in pixels of each choice in the config section.
const CONF_CHOICE_HEIGHT: i32 = 20;
/// The alignment of the label for each choice and input in the config section.
const CONF_CHOICE_ALIGN: Align = Align::TopLeft;
/// The color of the drop down arrow in each choice in the config section.
const CONF_CHOICE_COLOR: Color = Color::Light1;
/// The color of selections in each choice in the config section.
const CONF_CHOICE_SELECTION_COLOR: Color = Color::from_rgb(248,248,255);
/// The size of text in each choice in the config section.
const CONF_CHOICE_TEXT_SIZE: i32 = 14;
/// The color of text in each choice in the config section.
const CONF_CHOICE_TEXT_COLOR: Color = Color::Black;
/// The frame for the whole menu in each choice in the config section.
const CONF_CHOICE_MENU_FRAME: FrameType = FrameType::FlatBox;
/// The frame for the selected item in each choice in the config section.
const CONF_CHOICE_SELECTION_FRAME: FrameType = FrameType::GtkThinUpBox;
/// The size of the label for each choice in the config section.
const CONF_CHOICE_LABEL_SIZE: i32 = 14;
/// The color of the label for each choice in the config section.
const CONF_CHOICE_LABEL_COLOR: Color = Color::Black;

/// The frame for input widgets in the config section.
const CONF_INPUT_FRAME: FrameType = FrameType::GleamRoundUpBox;
/// The size in pixels of the scrollbar for input widgets in the config section.
const CONF_INPUT_SCROLLBAR_SIZE: i32 = 5;
/// THe height in pixels of the multiline input flex in the config section.
const CONF_MULTI_INPUT_HEIGHT: i32 = 185;
/// The height in pixels of the button flex in the config section.
const CONF_BUTTON_HEIGHT: i32 = 30;
/// The text size in pixels to use for the line number in multiline inputs in the config group.
const CONF_MULTI_INPUT_LINENUMBER_SIZE: i32 = 10;
/// The width of the linenumber column to use in multiline inputs in the config group.
const CONF_MULTI_INPUT_LINENUMBER_WIDTH: i32 = 15;
/// The alignment to use for multi-line inputs in the config group.
const CONF_MULIT_INPUT_SCROLLBAR_ALIGN: Align = Align::Right;
/// The frame to use for buttons in the config section.
const CONF_BTN_FRAME: FrameType = FrameType::GleamRoundUpBox;
/// The down_frame to use for buttons in the config section.
const CONF_BTN_DOWN_FRAME: FrameType = FrameType::GleamRoundDownBox;

/// This enum is specifically intended for message passing from
/// the GUI to the main function. This is done with Sender and 
/// Receiver objects created in initialize().
#[derive(Clone,Copy,PartialEq,Debug)]
pub enum InterfaceMessage {
    /// Indicates that the user wants to process a selected input and output file
    Process,
    /// Indicates that the user wants to close the program
    AppClosing,
    /// Indicates that the user wants to reset the config to the default value
    ConfigReset
}//end enum InterfaceMessage

/// This struct holds together all the objects and functions for manipulating and using the GUI.
#[allow(dead_code)]
pub struct GUI {
    /// The main app object. Has some misc useful functions.
    app: App,
    /// The main window of the application.
    ux_main_window: Window,
    /// The sender used for sending messages back to main.
    msg_sender: Sender<InterfaceMessage>,
    /// The receiver handed to main in order to receive messages from the sender.
    msg_receiver: Receiver<InterfaceMessage>,
    /// A reference to the TextBox that shows the input files chosen by the user.
    ux_input_box: Rc<RefCell<TextDisplay>>,
    /// A reference to a vec containing the paths of any input files chosen by the user.
    last_input_paths: Rc<RefCell<Vec<PathBuf>>>,
    /// A reference to the TextBox that shows the output file chosen by the user.
    ux_output_box: Rc<RefCell<TextEditor>>,
    /// A reference to the path of a potential output path chosen by the user.
    last_output_path: Rc<RefCell<Option<PathBuf>>>,
    /// The group holding all the configuration controls.
    /// This is stored here in order to disable during dialog.
    ux_config_group: Group,
    /// The group holding all the input and output controls.
    /// This is stored her in order to disable during dialog.
    ux_io_controls_group: Group,
    /// The group holding the custom dialog controls.  
    /// This is stored here to enable during dialog.
    ux_dialog_group: Group,
    /// The display which shows dialog messages to the user.
    ux_dialog_box: TextDisplay,
    /// The flex which holds buttons corresponding to the 
    /// dialog choices available to a user.
    ux_dialog_btns_flx: Flex,
    /// The choice which displays options for the ReadStartMode.
    ux_cf_read_start_mode_choice: Choice,
    /// The choice which displays options for the ReadRowMode.
    ux_cf_read_row_mode_choice: Choice,
    /// The input box that displays setting for read_start_idx
    ux_cf_read_start_idx_input: Rc<RefCell<IntInput>>,
    /// The input box that displays setting for read_rows_max
    ux_cf_read_rows_max_input: Rc<RefCell<IntInput>>,
    /// The text editor that displays setting for read_start_header
    ux_cf_read_start_header_box: Rc<RefCell<TextEditor>>,
    /// The text editor that displays setting for read_row_headers
    ux_cf_read_row_headers_box: Rc<RefCell<TextEditor>>,
    /// The text editor that displays setting for row_order_pref
    ux_cf_row_order_pref_box: TextEditor,
    /// The text editor that displays setting for read_split_char
    ux_cf_split_char_box: TextEditor,
    /// THe text editor that displays setting for read_test_name_prefix
    ux_cf_test_name_prefix_box: TextEditor,
}//end struct GUI

impl GUI {
    /// Returns a clone of the receiver so you can
    /// react to messages sent by gui.
    pub fn get_receiver(&self) -> Receiver<InterfaceMessage> {
        return self.msg_receiver.clone();
    }//end get_receiver(self)

    /// Gets a config store that represents the configuratino chosen
    /// by the user.
    pub fn get_config_store(&self) -> Result<ConfigStore,String> {
        let mut config = ConfigStore::default();
        
        match self.ux_cf_read_start_mode_choice.value() {
            0 => config.read_start_mode = ReadStartMode::Header,
            1 => config.read_start_mode = ReadStartMode::Index,
            _ => return Err(format!("Invalid read_start_mode menu choice {} !!!", self.ux_cf_read_start_mode_choice.value()))
        }//end matching from value to variant for read_start_mode
        match self.ux_cf_read_row_mode_choice.value() {
            0 => config.read_row_mode = ReadRowMode::Header,
            1 => config.read_row_mode = ReadRowMode::Max,
            _ => return Err(format!("Invalid read_row_mode menu choice {} !!!", self.ux_cf_read_row_mode_choice.value()))
        }// end matching from value to variant for read_row_mode

        {
            let read_start_idx_input_ref = (&self.ux_cf_read_start_idx_input).clone();
            let read_start_idx_input = read_start_idx_input_ref.as_ref().borrow();
            match read_start_idx_input.value().parse::<u16>() {
                Err(msg) => return Err(format!("Couldn't parse read_start_idx due to {:?}", msg)),
                Ok(read_start_idx) => config.read_start_idx = read_start_idx,
            }//end matching whether the parse fails (it really shouldn't) for read_start_idx
        }
        {
            let read_rows_max_input_ref = (&self.ux_cf_read_rows_max_input).clone();
            let read_rows_max_input = read_rows_max_input_ref.as_ref().borrow();
            match read_rows_max_input.value().parse::<u16>() {
                Err(msg) => return Err(format!("Couldn't parse read_rows_max due to {:?}", msg)),
                Ok(read_rows_max) => config.read_max_rows = read_rows_max,
            }//end matching whether the parse fails (it really shouldn't) for read_rows_max
        }

        {
            let read_start_header_box_ref = (&self.ux_cf_read_start_header_box).clone();
            let read_start_header_box = read_start_header_box_ref.as_ref().borrow();
            match read_start_header_box.buffer() {
                None => {},
                Some(buf) => config.read_start_header = buf.text(),
            }//end matching whether or not we can access buffer for read_start_header
        }
        {
            let read_row_headers_box_ref = (&self.ux_cf_read_row_headers_box).clone();
            let read_row_headers_box = read_row_headers_box_ref.as_ref().borrow();
            match read_row_headers_box.buffer() {
                None => {},
                Some(buf) => config.read_row_headers = buf.text().split("\n").map(|s| s.to_string()).collect(),
            }//end matching whether or not we can access buffer for read_row_headers
        }

        match self.ux_cf_row_order_pref_box.buffer() {
            None => {},
            Some(buf) => config.row_order_preference = buf.text().split("\n").map(|s| s.to_string()).collect(),
        }//end matching whether or not we can access buffer for row_order_preference

        match self.ux_cf_split_char_box.buffer() {
            None => {},
            Some(buf) => config.read_row_split_char = buf.text(),
        }//end matching whether or not we can access buffer for read_row_split_char

        match self.ux_cf_test_name_prefix_box.buffer() {
            None => {},
            Some(buf) => config.read_test_name_prefix = buf.text(),
        }//end matching whether or not we can access buffer for read_test_name_prefix

        Ok(config)
    }//end get_config_store()

    /// Updates the gui to show the given configuration settings
    pub fn set_config_store(&mut self, config: &ConfigStore) -> Result<(),String> {
        match config.read_start_mode {
            ReadStartMode::Header => {let _ = self.ux_cf_read_start_mode_choice.set_value(0);},
            ReadStartMode::Index => {let _ = self.ux_cf_read_start_mode_choice.set_value(1);},
        }
        match config.read_row_mode {
            ReadRowMode::Header => {let _ = self.ux_cf_read_row_mode_choice.set_value(0);},
            ReadRowMode::Max => {let _ = self.ux_cf_read_row_mode_choice.set_value(1);},
        }
        // hide and reshow choices to trigger event handling of change
        self.ux_cf_read_start_mode_choice.hide();
        self.ux_cf_read_row_mode_choice.hide();
        self.ux_cf_read_start_mode_choice.show();
        self.ux_cf_read_row_mode_choice.show();

        {
            let read_start_idx_input_ref = (&self.ux_cf_read_start_idx_input).clone();
            let mut read_start_idx_input = read_start_idx_input_ref.as_ref().borrow_mut();
            read_start_idx_input.set_value(&config.read_start_idx.to_string());
        }
        {
            let read_rows_max_input_ref = (&self.ux_cf_read_rows_max_input).clone();
            let mut read_rows_max_input = read_rows_max_input_ref.as_ref().borrow_mut();
            read_rows_max_input.set_value(&config.read_max_rows.to_string());
        }

        {
            let read_start_header_box_ref = (&self.ux_cf_read_start_header_box).clone();
            let mut read_start_header_box = read_start_header_box_ref.as_ref().borrow_mut();
            let mut buf = read_start_header_box.buffer().unwrap_or_else(|| TextBuffer::default());
            buf.set_text(&config.read_start_header);
            read_start_header_box.set_buffer(buf);
        }

        {
            let read_row_headers_box_ref = (&self.ux_cf_read_row_headers_box).clone();
            let mut read_row_headers_box = read_row_headers_box_ref.as_ref().borrow_mut();
            let mut buf = read_row_headers_box.buffer().unwrap_or_else(|| TextBuffer::default());
            buf.set_text(&config.read_row_headers.join("\n"));
            read_row_headers_box.set_buffer(buf);
        }

        let mut buf3 = self.ux_cf_row_order_pref_box.buffer().unwrap_or_else(|| TextBuffer::default());
        buf3.set_text(&config.row_order_preference.join("\n"));
        self.ux_cf_row_order_pref_box.set_buffer(buf3);

        let mut buf4 = self.ux_cf_split_char_box.buffer().unwrap_or_else(|| TextBuffer::default());
        buf4.set_text(&config.read_row_split_char);
        self.ux_cf_split_char_box.set_buffer(buf4);

        let mut buf5 = self.ux_cf_test_name_prefix_box.buffer().unwrap_or_else(|| TextBuffer::default());
        buf5.set_text(&config.read_test_name_prefix);
        self.ux_cf_test_name_prefix_box.set_buffer(buf5);

        Ok(())
    }//end set_config_store()

    /// Creates formatted strings holding the version number and date this
    /// application was compiled.
    /// 
    /// Used to build the header.
    fn header_version_day() -> (String,String) {
        let version = option_env!("CARGO_PKG_VERSION");
        let format_des = time::macros::format_description!("[month repr:long] [year]");
        let date = compile_time::date!();
        let date_str = date.format(format_des).unwrap_or_else(|_| String::from("unknown compile time"));
        let version_str = format!("{}", version.unwrap_or("unknown version"));
        return (version_str, date_str);
    }//end header_version_day

    /// Gets the last set of input file paths from the gui.  
    /// If there weren't any, it might be empty.  
    /// Uses clone to avoid references.
    pub fn get_last_input_paths(&self) -> Vec<PathBuf> {
        let last_input_paths_ref = (&self.last_input_paths).clone();
        let last_input_paths = last_input_paths_ref.as_ref().borrow();
        last_input_paths.to_vec()
    }//end get_last_input_paths()

    /// Gets the last output file path from the gui.  
    /// If there isn't anything, it might be None.  
    /// Uses clone to avoid references.
    pub fn get_last_output_paths(&self) -> Option<PathBuf> {
        let last_output_path_ref = (&self.last_output_path).clone();
        let last_output_path = last_output_path_ref.as_ref().borrow();
        last_output_path.clone()
    }//end get_last_output_paths()

    /// Gets the text from the box showing the output path/file.
    pub fn get_output_path_text(&self) -> String {
        let output_box_ref = (&self.ux_output_box).clone();
        let output_box = output_box_ref.as_ref().borrow();
        let output_buf = output_box.buffer().unwrap_or_else(|| TextBuffer::default());
        return output_buf.text();
    }//end get_output_path_text()

    /// Clears all memory or display of currently stored input paths.
    pub fn clear_last_input_paths(&mut self) {
        let last_input_paths_ref = (&self.last_input_paths).clone();
        let mut last_input_paths = last_input_paths_ref.as_ref().borrow_mut();
        let input_box_ref = (&self.ux_input_box).clone();
        let mut input_box = input_box_ref.as_ref().borrow_mut();
        let mut input_buf = input_box.buffer().unwrap_or_else(|| TextBuffer::default());
        input_buf.set_text("");
        input_box.set_buffer(input_buf);
        last_input_paths.clear();
    }//end clear_last_input_paths()

    /// Clears all memory or display of currently stored output path.
    pub fn clear_last_output_path(&mut self) {
        let last_output_path_ref = (&self.last_output_path).clone();
        let mut last_output_path = last_output_path_ref.as_ref().borrow_mut();
        let output_box_ref = (&self.ux_output_box).clone();
        let mut output_box = output_box_ref.as_ref().borrow_mut();
        let mut output_buf = output_box.buffer().unwrap_or_else(|| TextBuffer::default());
        output_buf.set_text("");
        output_box.set_buffer(output_buf);
        *last_output_path = None;
    }//end clear_last_output_path()

    /// Gives a small visual indication that the program is doing something in the background.
    pub fn start_wait(&mut self) {
        self.ux_main_window.set_cursor(fltk::enums::Cursor::Wait);
    }//end start_wait(self)

    /// Clears the visual indication from start_wait()
    pub fn end_wait(&mut self) {
        self.ux_main_window.set_cursor(fltk::enums::Cursor::Default);
    }//end end_wait(self)

    /// Closes the application.
    pub fn quit() {
        app::App::default().quit();
    }//end show(self)

    /// Wraps app.wait().  
    /// To run main app loop, use while(gui.wait()){}.
    pub fn wait(&self) -> bool {
        self.app.wait()
    }//end wait(&self)

    /// Resets group activations to ensure user can
    /// interact with gui after dialog has eneded.
    pub fn clear_integrated_dialog(&mut self) {
        self.ux_io_controls_group.activate();
        self.ux_config_group.activate();
        self.ux_dialog_group.deactivate();
        self.ux_dialog_box.buffer().unwrap_or_else(|| TextBuffer::default()).set_text("");
        self.ux_dialog_btns_flx.clear();
        self.ux_dialog_btns_flx.redraw();
    }//end clear_integrated_dialog()

    /// Deactivates most of the gui so that user
    /// is forced to interact with dialog
    fn activate_dialog(&mut self) {
        self.ux_io_controls_group.deactivate();
        self.ux_config_group.deactivate();
        self.ux_dialog_group.activate();
    }//end activate_dialog()

    /// Creates a modal dialog message that is integrated into
    /// the main window of the application.
    pub fn integrated_dialog_message(&mut self, txt: &str) {
        self.integrated_dialog_message_choice(txt, vec!["Ok"]);
    }//end integrated_dialog_message()

    /// Creates a modal error message that is integrated into the
    /// main window of the application.
    pub fn integrated_dialog_alert(&mut self, txt: &str) {
        dialog::beep(BeepType::Error);
        self.integrated_dialog_message(txt);
    }//end integrated_dialog_alert()

    /// Creates a modal dialog message which forces the user
    /// to ask a yes or no question.
    pub fn integrated_dialog_yes_no(&mut self, txt: &str) -> bool {
        match self.integrated_dialog_message_choice(txt, vec!["yes","no"]) {
            Some(idx) => idx == 0,
            None => false,
        }//end matching whether selection was yes or no
    }//end integrated_dialog_yes_no()

    /// Creates a modal dialog message which forces the user to choose
    /// between the options specified.  
    /// The buttons for options have auto-generated sizes, so if there are too
    /// many options, or they are too wordy, text might not be readable.  
    /// If this function is passed an empty vec for options, it will immediately
    /// return None. Without any options to end dialog, the user wouldn't be able
    /// to continue.
    pub fn integrated_dialog_message_choice(&mut self, txt: &str, options: Vec<&str>) -> Option<usize> {
        self.activate_dialog();
        // input validation for options being empty
        if options.len() == 0 {return None;}
        // update text based on parameter
        let mut dialog_buffer = self.ux_dialog_box.buffer().unwrap_or_else(|| TextBuffer::default());
        dialog_buffer.set_text(txt);
        self.ux_dialog_box.set_buffer(dialog_buffer);
        // update buttons based on type
        let button_pressed_index = Rc::from(RefCell::from(None));

        self.ux_dialog_btns_flx.clear();
        for (idx, option) in options.iter().enumerate() {
            let mut button = Button::default().with_label(option);
            button.set_frame(DIALOG_BTN_FRAME);
            button.set_down_frame(DIALOG_BTN_DOWN_FRAME);
            button.set_color(DIALOG_BTN_COLOR);
            button.set_selection_color(DIALOG_BTN_DOWN_COLOR);
            button.set_callback({
                let button_index_ref = (&button_pressed_index).clone();
                move |_| {
                    let mut button_index = button_index_ref.borrow_mut();
                    *button_index = Some(idx);
                }//end closure
            });
            self.ux_dialog_btns_flx.add(&button);
        }//end creating each button and handler
        self.ux_dialog_btns_flx.redraw();

        // wait for user to click a button
        let button_pressed_index_ref = (&button_pressed_index).clone();
        let mut button_index_to_return = None;
        while self.app.wait() {
            if let Ok(pushed_index) = button_pressed_index_ref.try_borrow() {
                if pushed_index.is_some() {button_index_to_return = pushed_index.clone(); break;}
            }
        }//end continuing application while we wait for button to be pressed

        self.clear_integrated_dialog();
        return button_index_to_return;
    }//end integrated_dialog_message(self, txt)

    /// Sets up all the properties and appearances of
    /// various widgets and UI settings.
    pub fn initialize() -> GUI {
        let alveo_app = app::App::default();
        let mut main_window = window::Window::default()
            .with_size(WINDOW_WIDTH,WINDOW_HEIGHT)
            .with_label("USDA Alveograph Exporter");
        main_window.make_resizable(true);
        main_window.end();
        match PngImage::load("icon.png") {
            Ok(icon) => main_window.set_icon(Some(icon)),
            Err(err) => eprintln!("Couldn't load icon image because of {}",err),
        }//end matching whether we could load the icon image alright

        let (s,r) = app::channel();

        let mut tile_group = Tile::default()
            .with_pos(0,0)
            .with_size(main_window.w(), main_window.h());
        tile_group.end();
        main_window.add(&tile_group);

        // set up header information
        let mut header_group = Flex::default()
            .with_pos(0,0)
            .with_size(HEADER_GROUP_WIDTH, HEADER_GROUP_HEIGHT);
        header_group.end();
        header_group.set_frame(GROUP_FRAME);
        header_group.set_color(HEADER_GROUP_COLOR);
        header_group.set_type(FlexType::Column);
        tile_group.add(&header_group);

        let mut header_label1 = Frame::default()
            .with_label(&format!("USDA Alveograph Exporter\tv{}\t{}", GUI::header_version_day().0, GUI::header_version_day().1))
            .with_align(HEADER_LABEL_ALIGN);
        header_label1.set_label_size(18);
        header_label1.set_label_type(fltk::enums::LabelType::Embossed);
        header_label1.set_label_color(HEADER_LABEL_COLOR);
        header_group.add(&header_label1);
        let mut header_label2 = Frame::default()
            .with_label("Processes txt files from the Alveograph Machine\nSophia Grothe\tUSDA-ARS Manhattan,KS")
            .with_align(HEADER_LABEL_ALIGN);
        header_label2.set_label_color(HEADER_LABEL_COLOR);
        header_group.add(&header_label2);

        // set up group with input and output controls, processing stuff
        let mut io_controls_group = Group::default()
            .with_pos(0, header_group.y() + header_group.h())
            .with_size(header_group.width(), IO_CONTROLS_GROUP_HEIGHT);
        io_controls_group.end();
        io_controls_group.set_frame(GROUP_FRAME);
        io_controls_group.set_color(IO_CONTROLS_GROUP_COLOR);
        tile_group.add(&io_controls_group);

        let mut io_controls_label = Frame::default()
            .with_pos(io_controls_group.x(), io_controls_group.y() + 10)
            .with_size(io_controls_group.w(), 20)
            .with_label("Input and Output Controls")
            .with_align(Align::Center);
        io_controls_label.set_label_size(16);
        io_controls_group.add(&io_controls_label);

        let mut input_btn = Button::default()
            .with_pos(io_controls_label.x() + IO_BTN_PADDING, io_controls_label.y() + io_controls_label.h() + IO_BTN_PADDING)
            .with_size(IO_BTN_WIDTH, IO_BTN_HEIGHT)
            .with_label("Select Input File(s)");
        input_btn.set_frame(IO_BTN_FRAME);
        input_btn.set_down_frame(IO_BTN_DOWN_FRAME);
        input_btn.set_tooltip("Click this button to choose an input file.");
        input_btn.clear_visible_focus();
        input_btn.set_color(IO_BTN_COLOR);
        input_btn.set_selection_color(IO_BTN_DOWN_COLOR);
        io_controls_group.add(&input_btn);

        let input_buf = TextBuffer::default();
        let mut input_box = TextDisplay::default()
            .with_pos(input_btn.x() + input_btn.w() + IO_BOX_PADDING, input_btn.y())
            .with_size(io_controls_group.w() - (input_btn.w() + (3 * IO_BOX_PADDING)), IO_BOX_HEIGHT);
        input_box.set_frame(IO_BOX_FRAME);
        input_box.set_scrollbar_align(Align::Bottom);
        input_box.set_scrollbar_size(7);
        input_box.set_color(IO_INPUT_BOX_COLOR);
        input_box.set_buffer(input_buf);
        input_box.set_tooltip("This box shows all the input files you currently have selected.");
        io_controls_group.add_resizable(&input_box);

        let mut output_btn = Button::default()
            .with_pos(input_btn.x(), input_btn.y() + input_btn.h() + IO_BTN_PADDING)
            .with_size(IO_BTN_WIDTH, IO_BTN_HEIGHT)
            .with_label("Select Output File");
        output_btn.set_frame(IO_BTN_FRAME);
        output_btn.set_down_frame(IO_BTN_DOWN_FRAME);
        output_btn.set_tooltip("Click this button to set where the output file will be located.\nOr, just type a name in the box to the right.");
        output_btn.clear_visible_focus();
        output_btn.set_color(IO_BTN_COLOR);
        output_btn.set_selection_color(IO_BTN_DOWN_COLOR);
        io_controls_group.add(&output_btn);

        let output_buf = TextBuffer::default();
        let mut output_box = TextEditor::default()
            .with_pos(output_btn.x() + output_btn.w() + IO_BOX_PADDING, output_btn.y())
            .with_size(io_controls_group.w() - (output_btn.w() + (3 * IO_BOX_PADDING)), IO_BOX_HEIGHT);
        output_box.set_frame(IO_BOX_FRAME);
        output_box.set_scrollbar_align(Align::Bottom);
        output_box.set_scrollbar_size(7);
        output_box.set_buffer(output_buf);
        output_box.set_tooltip("This box shows the output file you have selected.");
        io_controls_group.add_resizable(&output_box);

        let mut process_btn = Button::default()
            .with_pos(io_controls_group.x() + (io_controls_group.w() / 2) - (IO_PRC_BTN_WIDTH / 2), output_btn.y() + output_btn.h() + IO_PRC_BTN_PADDING)
            .with_size(IO_PRC_BTN_WIDTH,(io_controls_group.y() + io_controls_group.h()) - (output_btn.y() + output_btn.h()) - (2 * IO_PRC_BTN_PADDING))
            .with_label("Process Data");
        process_btn.emit(s, InterfaceMessage::Process);
        process_btn.set_frame(IO_BTN_FRAME);
        process_btn.set_down_frame(IO_BTN_DOWN_FRAME);
        process_btn.clear_visible_focus();
        process_btn.set_color(IO_BTN_COLOR);
        process_btn.set_selection_color(IO_BTN_DOWN_COLOR);
        process_btn.set_tooltip("Once you've selected an input and output, click this to process your files.");
        io_controls_group.add_resizable(&process_btn);

        // set up group with configuration options
        let mut config_group = Group::default()
            .with_pos(io_controls_group.x() + io_controls_group.w(), 0)
            .with_size(tile_group.width() - io_controls_group.width(), tile_group.height());
        config_group.end();
        config_group.set_frame(GROUP_FRAME);
        config_group.set_color(CONFIG_GROUP_COLOR);
        config_group.set_tooltip("Click the \"Help\" button for more information on any configuration setting.");
        tile_group.add(&config_group);

        let mut config_group_label = Frame::default()
            .with_pos(config_group.x(), config_group.y() + 10)
            .with_size(config_group.width(), 20)
            .with_align(Align::Inside.union(Align::Left))
            .with_label("Configuration Settings");
        config_group_label.set_label_size(16);
        config_group.add(&config_group_label);

        let mut read_start_mode_choice = Choice::default()
            .with_pos(config_group.x() + CONF_CHOICE_HOR_PADDING, config_group_label.y() + config_group_label.h() + CONF_CHOICE_VER_PADDING)
            .with_size((config_group.width() / 2) - (CONF_CHOICE_HOR_PADDING * 2), CONF_CHOICE_HEIGHT)
            .with_align(CONF_CHOICE_ALIGN)
            .with_label("Read Start Mode");
        read_start_mode_choice.add_choice("Header|Index");
        read_start_mode_choice.set_color(CONF_CHOICE_COLOR);
        read_start_mode_choice.set_selection_color(CONF_CHOICE_SELECTION_COLOR);
        read_start_mode_choice.set_text_color(CONF_CHOICE_TEXT_COLOR);
        read_start_mode_choice.set_frame(CONF_CHOICE_MENU_FRAME);
        read_start_mode_choice.set_down_frame(CONF_CHOICE_SELECTION_FRAME);
        read_start_mode_choice.set_label_size(CONF_CHOICE_LABEL_SIZE);
        read_start_mode_choice.set_label_color(CONF_CHOICE_LABEL_COLOR);
        read_start_mode_choice.set_text_size(CONF_CHOICE_TEXT_SIZE);
        read_start_mode_choice.clear_visible_focus();
        read_start_mode_choice.set_value(0);
        read_start_mode_choice.set_tooltip("The method by which the program finds the header above the rows of data in the file. See Help for details.");
        config_group.add(&read_start_mode_choice);

        let mut read_row_mode_choice = Choice::default()
            .with_pos(config_group.x() + (config_group.w() / 2) + CONF_CHOICE_HOR_PADDING, config_group_label.y() + config_group_label.h() + CONF_CHOICE_VER_PADDING)
            .with_size(read_start_mode_choice.width(), CONF_CHOICE_HEIGHT)
            .with_align(CONF_CHOICE_ALIGN)
            .with_label("Read Row Mode");
        read_row_mode_choice.add_choice("Header|Max");
        read_row_mode_choice.set_color(CONF_CHOICE_COLOR);
        read_row_mode_choice.set_selection_color(CONF_CHOICE_SELECTION_COLOR);
        read_row_mode_choice.set_text_color(CONF_CHOICE_TEXT_COLOR);
        read_row_mode_choice.set_frame(CONF_CHOICE_MENU_FRAME);
        read_row_mode_choice.set_down_frame(CONF_CHOICE_SELECTION_FRAME);
        read_row_mode_choice.set_label_size(CONF_CHOICE_LABEL_SIZE);
        read_row_mode_choice.set_label_color(CONF_CHOICE_LABEL_COLOR);
        read_row_mode_choice.set_text_size(CONF_CHOICE_TEXT_SIZE);
        read_row_mode_choice.clear_visible_focus();
        read_row_mode_choice.set_value(0);
        read_row_mode_choice.set_tooltip("The method by which the program finds rows of data under the start header in the file. See Help for details.");
        config_group.add(&read_row_mode_choice);

        let mut read_start_idx_input = IntInput::default()
            .with_pos(read_start_mode_choice.x(), read_start_mode_choice.y() + read_start_mode_choice.h() + CONF_CHOICE_VER_PADDING)
            .with_size(read_start_mode_choice.w(), read_start_mode_choice.h())
            .with_align(CONF_CHOICE_ALIGN)
            .with_label("Read Start Idx");
        read_start_idx_input.set_frame(CONF_INPUT_FRAME);
        read_start_idx_input.set_tooltip("If using Read Start Mode of Index, sets the 0-based index where the start header is located. See Help for details.");
        config_group.add(&read_start_idx_input);

        let mut read_rows_max_input = IntInput::default()
            .with_pos(read_row_mode_choice.x(), read_start_idx_input.y())
            .with_size(read_row_mode_choice.w(), read_row_mode_choice.h())
            .with_align(CONF_CHOICE_ALIGN)
            .with_label("Read Rows Max");
        read_rows_max_input.set_frame(CONF_INPUT_FRAME);
        read_rows_max_input.set_tooltip("If using Read Rows Mode of Max, sets the number of rows after the start index to read. See Help for details.");
        config_group.add(&read_rows_max_input);

        let read_start_header_buf = TextBuffer::default();
        let mut read_start_header_box = TextEditor::default()
            .with_pos(read_start_idx_input.x(), read_start_idx_input.y() + read_start_idx_input.h() + CONF_CHOICE_VER_PADDING)
            .with_size(read_rows_max_input.x() - read_start_idx_input.x() + read_rows_max_input.w(), read_start_mode_choice.h() + CONF_INPUT_SCROLLBAR_SIZE)
            .with_align(CONF_CHOICE_ALIGN)
            .with_label("Read Start Header");
        read_start_header_box.set_frame(CONF_INPUT_FRAME);
        read_start_header_box.set_scrollbar_align(Align::Bottom);
        read_start_header_box.set_scrollbar_size(CONF_INPUT_SCROLLBAR_SIZE);
        read_start_header_box.set_buffer(read_start_header_buf);
        read_start_header_box.set_cursor_style(fltk::text::Cursor::Simple);
        read_start_header_box.set_tooltip("If using Read Start Mode of Header, sets the exact value of header to look for as the start header. See Help for details.");
        config_group.add(&read_start_header_box);

        let mut cf_multiline_flex = Flex::default()
            .with_pos(read_start_header_box.x(), read_start_header_box.y() + read_start_header_box.h() + CONF_CHOICE_VER_PADDING)
            .with_size(read_start_header_box.w(), CONF_MULTI_INPUT_HEIGHT)
            .with_type(FlexType::Row);
        config_group.add_resizable(&cf_multiline_flex);

        let read_row_headers_buf = TextBuffer::default();
        let mut read_row_headers_box = TextEditor::default()
            .with_align(CONF_CHOICE_ALIGN)
            .with_label("Read Row Headers");
        read_row_headers_box.set_frame(CONF_INPUT_FRAME);
        read_row_headers_box.set_buffer(read_row_headers_buf);
        read_row_headers_box.set_linenumber_size(CONF_MULTI_INPUT_LINENUMBER_SIZE);
        read_row_headers_box.set_linenumber_width(CONF_MULTI_INPUT_LINENUMBER_WIDTH);
        read_row_headers_box.set_scrollbar_align(CONF_MULIT_INPUT_SCROLLBAR_ALIGN);
        read_row_headers_box.set_scrollbar_size(CONF_INPUT_SCROLLBAR_SIZE);
        read_row_headers_box.set_cursor_style(fltk::text::Cursor::Simple);
        read_row_headers_box.set_tooltip("If using Read Rows Mode of Header, sets the exact headers (and ordering of those headers) to look for after the start header. Very sensitive, see Help for details.");
        cf_multiline_flex.add(&read_row_headers_box);

        let row_order_pref_buf = TextBuffer::default();
        let mut row_order_pref_box = TextEditor::default()
            .with_align(CONF_CHOICE_ALIGN)
            .with_label("Row Order Pref.");
        row_order_pref_box.set_frame(CONF_INPUT_FRAME);
        row_order_pref_box.set_buffer(row_order_pref_buf);
        row_order_pref_box.set_linenumber_size(CONF_MULTI_INPUT_LINENUMBER_SIZE);
        row_order_pref_box.set_linenumber_width(CONF_MULTI_INPUT_LINENUMBER_WIDTH);
        row_order_pref_box.set_scrollbar_align(CONF_MULIT_INPUT_SCROLLBAR_ALIGN);
        row_order_pref_box.set_scrollbar_size(CONF_INPUT_SCROLLBAR_SIZE);
        row_order_pref_box.set_cursor_style(fltk::text::Cursor::Simple);
        row_order_pref_box.set_tooltip("Sets a custom output order for various data headers. This setting will never add or remove data; see Help for details.");
        cf_multiline_flex.add(&row_order_pref_box);

        // add box for split character
        // add box for test name prefix
        let split_char_buf = TextBuffer::default();
        let mut split_char_box = TextEditor::default()
            .with_pos(row_order_pref_box.x(), row_order_pref_box.y() + row_order_pref_box.h() + CONF_CHOICE_HOR_PADDING)
            .with_size(row_order_pref_box.w(), read_start_header_box.h())
            .with_align(Align::LeftTop)
            .with_label("Row Split Character");
        split_char_box.set_buffer(split_char_buf);
        split_char_box.set_scrollbar_align(Align::Bottom);
        split_char_box.set_scrollbar_size(CONF_INPUT_SCROLLBAR_SIZE);
        split_char_box.set_frame(CONF_INPUT_FRAME);
        split_char_box.set_cursor_style(fltk::text::Cursor::Simple);
        split_char_box.set_tooltip("Sets the split character between header and value in data rows. See Help for details.");
        config_group.add(&split_char_box);

        let test_name_prefix_buf = TextBuffer::default();
        let mut test_name_prefix_box = TextEditor::default()
            .with_pos(read_start_header_box.x(), split_char_box.y() + split_char_box.h() + (CONF_CHOICE_VER_PADDING / 2))
            .with_size(read_start_header_box.w(), read_start_header_box.h())
            .with_align(CONF_CHOICE_ALIGN)
            .with_label("Test Name Prefix");
        test_name_prefix_box.set_buffer(test_name_prefix_buf);
        test_name_prefix_box.set_frame(CONF_INPUT_FRAME);
        test_name_prefix_box.set_cursor_style(fltk::text::Cursor::Simple);
        test_name_prefix_box.set_scrollbar_align(Align::Bottom);
        test_name_prefix_box.set_scrollbar_size(CONF_INPUT_SCROLLBAR_SIZE);
        test_name_prefix_box.set_tooltip("Sets the prefix to the test name to look for in each file. See Help for details.");
        config_group.add(&test_name_prefix_box);

        let mut cf_button_flex = Flex::default()
            .with_pos(cf_multiline_flex.x(), config_group.y() + config_group.h() - CONF_BUTTON_HEIGHT)
            .with_size(cf_multiline_flex.w(),CONF_BUTTON_HEIGHT)
            .with_type(FlexType::Row);
        cf_button_flex.set_margins(0,CONF_CHOICE_HOR_PADDING,0,CONF_CHOICE_HOR_PADDING);
        config_group.add(&cf_button_flex);

        let mut cf_reset_btn = Button::default()
            .with_label("Config Reset");
        cf_reset_btn.set_frame(CONF_BTN_FRAME);
        cf_reset_btn.set_down_frame(CONF_BTN_DOWN_FRAME);
        cf_reset_btn.clear_visible_focus();
        cf_reset_btn.emit(s, InterfaceMessage::ConfigReset);
        cf_reset_btn.set_tooltip("Resets all configuration settings to the default values.");
        cf_button_flex.add(&cf_reset_btn);

        let mut cf_help_btn = Button::default()
            .with_label("Help");
        cf_help_btn.set_frame(CONF_BTN_FRAME);
        cf_help_btn.set_down_frame(CONF_BTN_DOWN_FRAME);
        cf_help_btn.clear_visible_focus();
        cf_help_btn.set_tooltip("Provides detailed help information on using and configuring the program.");
        cf_button_flex.add(&cf_help_btn);
        cf_help_btn.set_callback({
            move |_| {
                let mut dialog_window = Window::default()
                    .with_size(550,300)
                    .with_label("Help Dialog");
                match PngImage::load("icon.png") {
                    Ok(icon) => dialog_window.set_icon(Some(icon)),
                    Err(err) => eprintln!("Couldn't load icon image because of {}",err),
                }//end matching whether we could load the icon image alright
                dialog_window.make_resizable(true);
                let mut help_box = HelpView::default_fill();
                if let Err(err) = help_box.load("help.html") {
                    dialog::message_default(&format!("Couldn't find help.html and encountered an error: {}",err));}
                help_box.set_text_size(16);
                dialog_window.end();
                dialog_window.show();
            }
        });

        // set up group for integrated dialog
        let mut dialog_group = Group::default()
            .with_pos(io_controls_group.x(), io_controls_group.y() + io_controls_group.h())
            .with_size(io_controls_group.w(), tile_group.h() - (io_controls_group.y() + io_controls_group.h()));
        dialog_group.end();
        dialog_group.set_frame(GROUP_FRAME);
        dialog_group.set_color(DIALOG_GROUP_COLOR);
        dialog_group.deactivate();
        tile_group.add(&dialog_group);

        let mut dialog_buf = TextBuffer::default();
        let mut dialog_box = TextDisplay::default()
            .with_pos(dialog_group.x() + (DIALOG_BOX_PADDING / 2), dialog_group.y() + (DIALOG_BOX_PADDING / 2))
            .with_size(dialog_group.w() - DIALOG_BOX_PADDING, dialog_group.height() - DIALOG_BOX_PADDING - DIALOG_BTNS_HEIGHT)
            .with_align(Align::Inside);
        dialog_box.set_text_color(DIALOG_BOX_TEXT_COLOR);
        dialog_box.set_color(DIALOG_BOX_COLOR);
        dialog_box.set_frame(DIALOG_BOX_FRAME);
        dialog_box.wrap_mode(DIALOG_BOX_WRAP_MODE, DIALOG_BOX_WRAP_MARGIN);
        dialog_box.set_scrollbar_align(DIALOG_BOX_SCROLL_ALIGN);
        dialog_box.set_scrollbar_size(DIALOG_BOX_SCROLL_SIZE);
        dialog_box.set_text_size(DIALOG_BOX_TEXT_SIZE);
        dialog_buf.set_text("");
        dialog_box.set_buffer(dialog_buf);
        dialog_group.add(&dialog_box);

        let mut dialog_btns = Flex::default()
            .with_pos(dialog_box.x(), dialog_box.y() + dialog_box.h() + (DIALOG_BOX_PADDING / 2))
            .with_size(dialog_box.w(), dialog_group.h() - dialog_box.h() - DIALOG_BOX_PADDING)
            .with_align(Align::Right)
            .with_type(FlexType::Row);
        dialog_btns.end();
        dialog_btns.set_color(DIALOG_BTNS_BACK_COLOR);
        dialog_btns.set_frame(DIALOG_BTNS_BACK_FRAME);
        dialog_group.add(&dialog_btns);

        // set up callbacks and reference stuff
        let input_box_ref = Rc::from(RefCell::from(input_box));
        let last_input_path_ref = Rc::from(RefCell::from(Vec::new()));
        let output_box_ref = Rc::from(RefCell::from(output_box));
        let last_output_path_ref = Rc::from(RefCell::from(None));

        input_btn.set_callback({
            let input_box_ref = (&input_box_ref).clone();
            let last_input_path_ref = (&last_input_path_ref).clone();
            move |_| {
                // get valid references to everything we need from outside
                let mut input_box = input_box_ref.as_ref().borrow_mut();
                let mut last_input_path = last_input_path_ref.as_ref().borrow_mut();
                let mut input_buf = input_box.buffer().unwrap_or_else(|| TextBuffer::default());
                // create a dialog to show
                let mut dialog = NativeFileChooser::new(FileDialogType::BrowseMultiFile);
                dialog.set_option(FileDialogOptions::UseFilterExt);
                dialog.set_filter("*.txt");
                dialog.set_title("Please Select an Input File");
                dialog.show();
                let dialog_error = dialog.error_message().unwrap_or_else(|| "".to_string()).replace("No error","");
                if dialog_error != "" {println!("We encountered a dialog error while getting input file:\n{}", dialog_error)}
                *last_input_path = dialog.filenames();
                let mut name_vec = Vec::new();
                for path in last_input_path.iter() {
                    match path.file_name() {
                        None => name_vec.push("FilenameInvalid".to_string()),
                        Some(name) => name_vec.push(name.to_string_lossy().to_string()),
                    }//end matching whether we can get the filename
                }//end putting filename of each file in the input_box buf
                input_buf.set_text(&name_vec.join(", "));
                drop(dialog);
                // make sure we still have our buffer
                input_box.set_buffer(input_buf);
            }//end closure
        });

        output_btn.set_callback({
            let output_box_ref = (&output_box_ref).clone();
            let last_output_path_ref = (&last_output_path_ref).clone();
            move |_| {
                // get valid references to everything we need from outside
                let mut output_box = output_box_ref.as_ref().borrow_mut();
                let mut last_output_path = last_output_path_ref.as_ref().borrow_mut();
                let mut output_buf = output_box.buffer().unwrap_or_else(|| TextBuffer::default());
                // create a dialog to show
                let mut dialog = NativeFileChooser::new(FileDialogType::BrowseSaveFile);
                dialog.set_option(FileDialogOptions::SaveAsConfirm);
                dialog.set_filter("*.xlsx");
                dialog.set_title("Please select a path for the output file.");
                dialog.show();
                let dialog_error = dialog.error_message().unwrap_or_else(|| "".to_string()).replace("No error", "");
                if dialog_error != "" {
                    println!("We encountered a dialog error while getting the output file path:\n{}", dialog_error);
                    *last_output_path = None;
                    return;
                }//end if we cauldn't get dialog
                *last_output_path = Some(dialog.filename());
                match dialog.filename().file_name() {
                    Some(name) => output_buf.set_text(&name.to_string_lossy().to_string()),
                    None => output_buf.set_text("Invalid output filename"),
                }//end matching whether we can get the filename and update buffer
                // make sure we still have our buffer
                output_box.set_buffer(output_buf);
            }//end closure
        });

        let read_start_idx_input_ref = Rc::from(RefCell::from(read_start_idx_input));
        let read_start_header_box_ref = Rc::from(RefCell::from(read_start_header_box));

        read_start_mode_choice.handle({
            let read_start_idx_input_ref_clone = (&read_start_idx_input_ref).clone();
            let read_start_header_box_ref_clone = (&read_start_header_box_ref).clone();
            move |c,ev| {
                match ev {
                    Event::KeyUp | Event::Push | Event::Show => {
                        let mut read_start_idx_input = read_start_idx_input_ref_clone.as_ref().borrow_mut();
                        let mut read_start_header_box = read_start_header_box_ref_clone.as_ref().borrow_mut();
                        match c.value() {
                            0 => {
                                read_start_idx_input.deactivate();
                                read_start_header_box.activate();
                            },
                            1 => {
                                read_start_idx_input.activate();
                                read_start_header_box.deactivate();
                            },
                            _ => eprintln!("Unknown menu value {} for read_start_mode_choice!!!", c.value()),
                        }//end matching value of choice
                        true
                    },
                    _ => false,
                }//end matching event
            }//end closure
        });

        let read_rows_max_input_ref = Rc::from(RefCell::from(read_rows_max_input));
        let read_row_headers_box_ref = Rc::from(RefCell::from(read_row_headers_box));

        read_row_mode_choice.handle({
            let read_rows_max_input_ref_clone = (&read_rows_max_input_ref).clone();
            let read_row_headers_box_ref_clone = (&read_row_headers_box_ref).clone();
            move |c,ev| {
                match ev {
                    Event::KeyUp | Event::Push | Event::Show => {
                        let mut read_rows_max_input = read_rows_max_input_ref_clone.as_ref().borrow_mut();
                        let mut read_row_headers_box = read_row_headers_box_ref_clone.as_ref().borrow_mut();
                        match c.value() {
                            0 => {
                                read_rows_max_input.deactivate();
                                read_row_headers_box.activate();
                            },
                            1 => {
                                read_rows_max_input.activate();
                                read_row_headers_box.deactivate();
                            },
                            _ => eprintln!("Unkown menu value {} for read_row_mode_choice!!!", c.value()),
                        }
                        true
                    },
                    _ => false,
                }
            }//end closure
        });

        main_window.show();
        main_window.emit(s, InterfaceMessage::AppClosing);
        GUI {
            app: alveo_app,
            ux_main_window: main_window,
            msg_sender: s,
            msg_receiver: r,
            ux_input_box: input_box_ref,
            last_input_paths: last_input_path_ref,
            ux_output_box: output_box_ref,
            last_output_path: last_output_path_ref,
            ux_config_group: config_group,
            ux_io_controls_group: io_controls_group,
            ux_dialog_group: dialog_group,
            ux_dialog_box: dialog_box,
            ux_dialog_btns_flx: dialog_btns,
            ux_cf_read_start_mode_choice: read_start_mode_choice,
            ux_cf_read_row_mode_choice: read_row_mode_choice,
            ux_cf_read_start_idx_input: read_start_idx_input_ref,
            ux_cf_read_rows_max_input: read_rows_max_input_ref,
            ux_cf_read_start_header_box: read_start_header_box_ref,
            ux_cf_read_row_headers_box: read_row_headers_box_ref,
            ux_cf_row_order_pref_box: row_order_pref_box,
            ux_cf_split_char_box: split_char_box,
            ux_cf_test_name_prefix_box: test_name_prefix_box,
        }//end struct construction
    }//end initialize()
}//end impl for GUI