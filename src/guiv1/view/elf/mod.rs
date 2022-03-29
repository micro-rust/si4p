//! ELF Viewer.
//! GUI element that allows inspection of a ELF file's contents.


pub mod theme;

use self::theme::ElfViewTheme;


use crate::gui::message::{ AppMessage, ElfMessage };

use micro_elf::{ ElfTrait };

use iced::{
    Element, Column, Container, Row,

    Command,

    button::{ self, Button }, scrollable::{ self, Scrollable }, Text,
    
    Length, Align,
};


pub struct ELFView {
    /// Internal GUI state.
    state: ElfViewState,

    /// Them of the ELF view.
    theme: ElfViewTheme,

    /// ELF file header info.
    fhinfo: String,

    /// Sections of the ELF.
    /// Name, redux info, contents.
    sections: Vec<(String, String, String)>,

    /// Programs of the ELF.
    programs: Vec<(String, String, String)>,
}


impl ELFView {
    /// Creates a new ELF View.
    pub fn create() -> Self {
        Self {
            state: ElfViewState::new(),
            theme: ElfViewTheme::default(),
            fhinfo: String::new(),
            sections: Vec::new(),
            programs: Vec::new(),
        }
    }

    /// Loads the ELF from the given path.
    pub fn load(&mut self, elf: &Box<dyn ElfTrait>) {
        // Release memory first.
        self.fhinfo = String::new();
        self.sections = Vec::new();
        self.programs = Vec::new();

        // Load the File header information.
        self.fhinfo = elf.fileheader().prettyprint(String::new());

        // Load all the sections info.
        for section in elf.sections() {
            // Get name.
            let name = section.name();

            // Get reduced information.
            let info = section.info();

            // Get contents and format them.
            let rawcontent = section.content()
                .chunks(4)
                .map(|pair| match pair.len() {
                    1 => [(pair[0].0, pair[0].1), (         0,        0), (         0,        0), (         0,         0)],
                    2 => [(pair[0].0, pair[0].1), (pair[1].0, pair[1].1), (         0,        0), (         0,         0)],
                    3 => [(pair[0].0, pair[0].1), (pair[1].0, pair[1].1), (pair[2].0, pair[2].1), (         0,         0)],
                    4 => [(pair[0].0, pair[0].1), (pair[1].0, pair[1].1), (pair[2].0, pair[2].1), (pair[3].0, pair[3].1)],

                    s => panic!("Unexpected size in chunk of size 4: {}", s)
                })
                .map(|data| format!("0x{:08X}: 0x{:08X} 0x{:08X} 0x{:08X} 0x{:08X}\n", data[0].0, data[0].1, data[1].1, data[2].1, data[3].1))
                .fold(String::new(), |output, part| output + &part);

            self.sections.push((name, info, rawcontent))
        }

        self.state.setshsize(self.sections.len());
    }

    /// Update with GUI events.
    pub fn update(&mut self, message: ElfMessage) -> Command<AppMessage> {
        match message {
            ElfMessage::ViewFileHeaderInfo => self.state.togglefh(),
            ElfMessage::ViewAllProgramHeaderInfo => self.state.toggleph(),
            ElfMessage::ViewAllSectionHeaderInfo => self.state.togglesh(),

            ElfMessage::ViewProgramHeaderInfo(idx) => self.state.togglephsub(idx),
            ElfMessage::ViewSectionHeaderInfo(idx) => self.state.toggleshsub(idx),

            ElfMessage::ShowSectionContent(idx) => self.state.togglesection(idx),

            _ => return Command::none(),
        }

        Command::none()
    }

    /// Build the graphical view.
    pub fn view(&mut self) -> Element<AppMessage> {
        const NOFILE: &str = "No ELF file loaded";

        // Build the view title.
        let title = Text::new("ELF Viewer").size(50);

        let ElfViewState {
            ref mut fhbtnstate,
            ref mut phbtnstate,
            ref mut shbtnstate,
            ref mut leftscroll,
            ref mut rightscroll,

            fhinfodisplay,
            phinfodisplay,
            shinfodisplay,

            ref phdisplaying,
            ref shdisplaying,

            ref mut phsubbtn,
            ref mut shsubbtn,

            sectiondisplay
        } = self.state;


        // Create file header section.
        let fhsection = {
            // Create the file header view button.
            let btn = Button::new(fhbtnstate, Text::new("File Header").size(20))
                .on_press(AppMessage::ElfView(ElfMessage::ViewFileHeaderInfo))
                .height(Length::Units(30))
                .width(Length::Fill)
                .padding(5);

            let column = Column::new()
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(0)
                .push(btn);


            if fhinfodisplay {
                let info: &str = match self.fhinfo.len() {
                    0 => NOFILE,

                    _ => &self.fhinfo,
                };

                column.push( Text::new( info ).size(14) )
            } else {
                column  
            }
        };

        // Create program header section.
        let phsection = {
            // Create the program header viewer.
            let btn = Button::new( phbtnstate, Text::new("Program headers").size(20) )
                .on_press(AppMessage::ElfView(ElfMessage::ViewAllProgramHeaderInfo))
                .height(iced::Length::Units(30))
                .width(Length::Fill)
                .padding(5);

            // Check if FH info is displayed.
            if !phinfodisplay {
                Column::new()
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(0)
                    .push(btn)

            } else {
                Column::new()
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(0)
                    .push(btn)
            }
        };

        // Create section header section.
        let shsection = {
            // Create the sectin header viewer.
            let btn = Button::new( shbtnstate, Text::new("Section headers").size(20) )
                .on_press(AppMessage::ElfView(ElfMessage::ViewAllSectionHeaderInfo))
                .height(iced::Length::Units(30))
                .width(Length::Fill)
                .padding(5)
                .style(self.theme.mainbtn.clone());

            let btnstyle = self.theme.subbtn.clone();
            let rowstyle = self.theme.subrow.clone();
            let textstyle = self.theme.textinfo.clone();

            let column = Column::new()
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(0)
                .push(btn);


            // Check if SH info is displayed.
            if shinfodisplay {
                let info: Element<_> = match self.sections.len() {
                    0 => Column::new()
                        .width(Length::Fill)
                        .height(Length::Shrink)
                        .padding(15)
                        .push( Text::new( NOFILE ) )
                        .into(),

                    _ => {
                        let children: Vec<Element<_>> = self.sections.iter()
                            .zip(shsubbtn)
                            .enumerate()
                            .map(|(i, ((name, info, _), state))| {
                                // Create the info button.
                                let infobtn = Button::new(&mut state.0, Text::new("i").size(26))
                                    .on_press(AppMessage::ElfView(ElfMessage::ViewSectionHeaderInfo(i)))
                                    .height(Length::Units(30))
                                    .width(Length::Units(30))
                                    .padding(2)
                                    .style(btnstyle);

                                // Create the show button.
                                let showbtn = Button::new(&mut state.1, Text::new("Show").size(20))
                                    .on_press(AppMessage::ElfView(ElfMessage::ShowSectionContent(i)))
                                    .height(Length::Units(30))
                                    .width(Length::Shrink)
                                    .padding(5)
                                    .style(btnstyle);

                                // Create the row for the two buttons.
                                let row = Row::new()
                                    .height(Length::Fill)
                                    .width(Length::Fill)
                                    .padding(0)
                                    .spacing(0)
                                    .push(infobtn)
                                    .push(showbtn);


                                // Create the button row container.
                                let rowcontainer = Container::new(row)
                                    .height(Length::Fill)
                                    .width(Length::Shrink)
                                    .style(rowstyle)
                                    .center_y()
                                    .align_x(Align::End);

                                // Create the name tag.
                                let namecontainer = Container::new( Text::new( name ).size(20) )
                                    .height(Length::Fill)
                                    .width(Length::Fill)
                                    .style(rowstyle)
                                    .center_y()
                                    .align_x(Align::Start);

                                // Create the final row.
                                let row = Row::new()
                                    .height(Length::Fill)
                                    .width(Length::Fill)
                                    .push(namecontainer)
                                    .push(rowcontainer);

                                // Create final container.
                                let container = Container::new(row)
                                    .height(Length::Units(30))
                                    .width(Length::Fill)
                                    .style(rowstyle)
                                    .into();

                                if shdisplaying[i] {
                                    let innerinfo = Container::new( Text::new( info ).size(20) )
                                        .padding(3)
                                        .width(Length::Fill)
                                        .height(Length::Shrink)
                                        .center_x()
                                        .style(textstyle);

                                    Column::new()
                                        .width(Length::Fill)
                                        .height(Length::Shrink)
                                        .push(container)
                                        .push(innerinfo)
                                        .into()
                                } else {
                                    container
                                }

                            })
                            .collect();

                        Column::with_children(children)
                            .width(Length::Fill)
                            .height(Length::Shrink)
                            .padding(15)
                            .into()
                    }
                };

                column.push(info)
            } else {
                column
            }
        };

        // Create left scrollable section.
        let left = Scrollable::new(leftscroll)
            .padding(10)
            .spacing(0)
            .width(Length::FillPortion(30))
            .height(Length::Fill)
            .push(fhsection)
            .push(shsection)
            .push(phsection);


        // Display right section depending on whether a section is deisplayed.
        let righttext: Element<_> = match sectiondisplay {
            Some(idx) => {
                Column::new()
                    .padding(0)
                    .width(Length::FillPortion(40))
                    .height(Length::Fill)
                    .push( Text::new( format!("Contents of section {}", self.sections[idx].0) ).size(24) )
                    .push(
                        Scrollable::new(rightscroll)
                            .padding(5)
                            .spacing(0)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .push( Container::new(
                                    Text::new(&self.sections[idx].2).font(theme::MONO)
                                )
                                .width(Length::Fill)
                                .height(Length::Shrink)
                                .style(self.theme.textinfo.clone())
                            )
                    )
                    .into()
            },
            _ => Column::new().into(),
        };

        let right = Column::new()
            .width(Length::FillPortion(40))
            .height(Length::Fill)
            .push(righttext);

        // Create the main element.
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(title)
            .push(
                Row::new()
                    .push(left)
                    .push(right)
                    .padding(10)
            )
            .into()
    }
}



pub struct ElfViewState {
    /// State for the file header button.
    pub(self) fhbtnstate: button::State,
    /// State for the program header button.
    pub(self) phbtnstate: button::State,
    /// State for the section header button.
    pub(self) shbtnstate: button::State,

    /// State for the left scrollable section.
    pub(self) leftscroll: scrollable::State,
    /// State for the right scrollable section.
    pub(self) rightscroll: scrollable::State,

    /// Indicates if File Header info shuold be displayed.
    fhinfodisplay: bool,
    /// Indicates if Program Header info shuold be displayed.
    phinfodisplay: bool,
    /// Indicates if Section Header info shuold be displayed.
    shinfodisplay: bool,

    /// Program headers displaying their information.
    phdisplaying: Vec<bool>,
    /// Section headers displaying their information.
    shdisplaying: Vec<bool>,

    /// State of buttons to display each program's info.
    phsubbtn: Vec<button::State>,
    /// State of buttons to display each section's info.
    shsubbtn: Vec<(button::State, button::State)>,

    /// The section currently displaying its contetns.
    sectiondisplay: Option<usize>,
}


#[allow(dead_code)]
impl ElfViewState {
    /// Creates a new state.
    pub fn new() -> Self {
        Self {
            fhbtnstate: button::State::new(),
            phbtnstate: button::State::new(),
            shbtnstate: button::State::new(),

            leftscroll: scrollable::State::new(),
            rightscroll: scrollable::State::new(),

            fhinfodisplay: false,
            phinfodisplay: false,
            shinfodisplay: false,

            phdisplaying: Vec::new(),
            shdisplaying: Vec::new(),

            phsubbtn: Vec::new(),
            shsubbtn: Vec::new(),

            sectiondisplay: None,
        }
    }

    /// Sets the size of the Program header vector.
    pub fn setphsize(&mut self, size: usize) {
        self.phdisplaying = Vec::with_capacity(size);
        for i in 0..size {
            self.phdisplaying[i] = false;
        }
    }

    /// Sets the size of the Section header vector.
    pub fn setshsize(&mut self, size: usize) {
        self.shdisplaying = Vec::with_capacity(size);

        for _ in 0..size {
            self.shdisplaying.push(false);
        }

        self.shsubbtn = Vec::with_capacity(size);

        for _ in 0..size {
            self.shsubbtn.push((button::State::new(), button::State::new()));
        }
    }

    /// Toggles the display of the given section.
    pub fn togglesection(&mut self, idx: usize) {
        match self.sectiondisplay {
            Some(x) => if x == idx { self.sectiondisplay = None; }
                else { self.sectiondisplay = Some(idx) },

            None => self.sectiondisplay = Some(idx),
        }
    }

    /// Toggles the display of the given Program header.
    #[inline(always)]
    pub fn togglephsub(&mut self, idx: usize) {
        self.phdisplaying[idx] ^= true
    }

    /// Toggles the display of the given Section header.
    #[inline(always)]
    pub fn toggleshsub(&mut self, idx: usize) {
        self.shdisplaying[idx] ^= true
    }

    /// Toggles the state of a display option.
    #[inline]
    pub fn fhdisplay(&mut self) -> bool {
        self.fhinfodisplay
    }

    /// Toggles the state of a display option.
    #[inline]
    pub fn phdisplay(&mut self) -> bool {
        self.phinfodisplay
    }

    /// Toggles the state of a display option.
    #[inline]
    pub fn shdisplay(&mut self) -> bool {
        self.shinfodisplay
    }

    /// Toggles the state of a display option.
    #[inline]
    pub fn togglefh(&mut self) {
        self.fhinfodisplay ^= true;
    }

    /// Toggles the state of a display option.
    #[inline]
    pub fn toggleph(&mut self) {
        self.phinfodisplay ^= true;
    }

    /// Toggles the state of a display option.
    #[inline]
    pub fn togglesh(&mut self) {
        self.shinfodisplay ^= true;
    }
}

