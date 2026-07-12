
/// Trait that defines how a menu will be printed by the painter
pub trait Menu: Send {

/// Menu to present suggestions in a columnar fashion
/// It presents a description of the suggestion if available
pub struct ColumnarMenu {
    /// Menu settings
    settings: MenuSettings,
    /// Columnar menu active status
    active: bool,
    /// Default column details that are set when creating the menu
    /// These values are the reference for the working details
    default_details: DefaultColumnDetails,
    /// Number of minimum rows that are displayed when
    /// the required lines is larger than the available lines
    min_rows: u16,
    /// Working column details keep changing based on the collected values
    working_details: ColumnDetails,
    /// Menu cached values
    values: Vec<Suggestion>,
    /// Cached display width of each suggestion in `values`
    display_widths: Vec<usize>,
    /// column position of the cursor. Starts from 0
    col_pos: u16,
    /// row position in the menu. Starts from 0
    row_pos: u16,
    /// Number of rows that are skipped when printing,
    /// depending on selected value and terminal height
    skip_rows: u16,
    /// Event sent to the menu
    event: Option<MenuEvent>,
    /// Longest suggestion found in the values
    longest_suggestion: usize,
    /// String collected after the menu is activated
    input: Option<String>,
}



/// Completion menu definition
pub struct DescriptionMenu {
    /// Menu settings
    settings: MenuSettings,
    /// Menu status
    active: bool,
    /// Default column details that are set when creating the menu
    /// These values are the reference for the working details
    default_details: DefaultMenuDetails,
    /// Number of minimum rows that are displayed when
    /// the required lines is larger than the available lines
    min_rows: u16,
    /// Working column details keep changing based on the collected values
    working_details: WorkingDetails,
    /// Menu cached values
    values: Vec<Suggestion>,
    /// column position of the cursor. Starts from 0
    col_pos: u16,
    /// row position in the menu. Starts from 0
    row_pos: u16,
    /// Event sent to the menu
    event: Option<MenuEvent>,
    /// String collected after the menu is activated
    input: Option<String>,
    /// Examples to select
    examples: Vec<String>,
    /// Example index
    example_index: Option<usize>,
    /// Examples may not be shown if there is not enough space in the screen
    show_examples: bool,
    /// Skipped description rows
    skipped_rows: usize,
}


/// Menu to present suggestions like similar to Ide completion menus
pub struct IdeMenu {
    /// Menu settings
    settings: MenuSettings,
    /// Ide menu active status
    active: bool,
    /// Default ide menu details that are set when creating the menu
    /// These values are the reference for the working details
    default_details: DefaultIdeMenuDetails,
    /// Working ide menu details keep changing based on the collected values
    working_details: IdeMenuDetails,
    /// Menu cached values
    values: Vec<Suggestion>,
    /// Cached display width of each suggestion in `values`
    display_widths: Vec<usize>,
    /// Selected value. Starts at 0
    selected: u16,
    /// Number of values that are skipped when printing,
    /// depending on selected value and terminal height
    skip_values: u16,
    /// Event sent to the menu
    event: Option<MenuEvent>,
    /// Longest suggestion found in the values
    longest_suggestion: usize,
    /// String collected after the menu is activated
    input: Option<String>,
}



/// Struct to store the menu style
/// Context menu definition
pub struct ListMenu {
    /// Menu settings
    settings: MenuSettings,
    /// Number of records pulled until page is full
    page_size: usize,
    /// Menu active status
    active: bool,
    /// Cached values collected when querying the completer.
    /// When collecting chronological values, the menu only caches at least
    /// page_size records.
    /// When performing a query to the completer, the cached values will
    /// be the result from such query
    values: Vec<Suggestion>,
    /// row position in the menu. Starts from 0
    row_position: u16,
    /// Max size of the suggestions when querying without a search buffer
    query_size: Option<usize>,
    /// Max number of lines that are shown with large suggestions entries
    max_lines: u16,
    /// Multiline marker
    multiline_marker: String,
    /// Registry of the number of entries per page that have been displayed
    pages: Vec<Page>,
    /// Page index
    page: usize,
    /// Event sent to the menu
    event: Option<MenuEvent>,
    /// String collected after the menu is activated
    input: Option<String>,
    /// Controls where the description is rendered relative to the completion value
    description_position: DescriptionPosition,
}
