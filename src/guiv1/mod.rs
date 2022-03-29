//! Main GUI module.


pub mod message;
pub mod view;
pub mod theme;



use std::collections::HashMap;


use iced::{
	Application, Command, Settings, window::Settings as Window,

	Clipboard,

	Element, Column, Row, Text, Container, button::{ self, Button },

	HorizontalAlignment, VerticalAlignment, Length,
};


use self::message::AppMessage;
use self::view::AppViews;



pub struct MainApplication {
	/// Current state of the App.
	state: Option<AppState>,

	/// Current working project.
	current: usize,

	/// All loaded projects.
	projects: HashMap<usize, Project>,

	/// Active GUI theme.
	theme: theme::Theme,

	/// App Views.
	views: AppViews,
}


impl Application for MainApplication {
	type Executor = iced::executor::Default;
	type Message = AppMessage;
	type Flags = ();

	fn new(flags: ()) -> (Self, Command<AppMessage>) {
		let mut projects = HashMap::new();

		projects.insert(0, Project { name: String::from("Test project"), hwinfo: String::from("RP2040"), archinfo: String::from("ARMV6m"), codesize: 0 });

		// Create the app views.
		let mut views = view::AppViews::create();

		// Load the test ELF.
		views.loadelf(&std::env::current_dir().unwrap().join("test.elf"));


		(
			Self { state: None, current: 0, projects, theme: theme::Theme::default(), views },
			Command::perform(Self::initialize(), AppMessage::Initialized)
		)
	}

	fn title(&self) -> String {
		format!("Si4+ - {}", self.projects[&self.current].name)
	}

	fn update(&mut self, message: AppMessage, _: &mut Clipboard) -> Command<AppMessage> {
		match self.state {
			// App is not yet initialized, try to initialize.
			None => match message {
				AppMessage::Initialized(state) => {
					self.state = Some(state);
					Command::none()
				}

				_ => Command::none(),
			},

			// App is initialized.
			Some(state) => match message {
				// Update the ELF View.
				AppMessage::ElfView(m) => self.views.elf.update(m),

				// TODO : Manage state updates.
				_ => Command::none(),
			}
		}
	}

	fn view(&mut self) -> Element<AppMessage> {
		match self.state {
			// If not loaded show it.
			None => loading(),

			// If loaded, display.
			Some(AppState {
				ref mut newproject,
				ref mut loadprobe,
				view,
				..
			}) => {
				// Get current project info.
				let project: &Project = &self.projects[&self.current];

				// Create button bar.
				// Create 'New project button'
				let newprojectbtn = Button::new(newproject, Text::new("New project"))
					.on_press(AppMessage::NewProjectRequest)
					.padding(6)
					.style(self.theme.button.clone());

				// Create 'Load' button.
				let loadprobebtn = Button::new(loadprobe, Text::new("Load Probe"))
					.on_press(AppMessage::LoadProbe)
					.padding(6)
					.style(self.theme.button.clone());


				// Create button bar.
				let buttonbar = Row::new()
					.spacing(0)
					.push( newprojectbtn)
					.push(loadprobebtn );

				// Create content based on which view is selected.
				let content = match view {
					0 => self.views.elf.view(),
					_ => Text::new("No view").size(50).into(),
				};

				// Create viewed content.
				let column = Column::new()
					.width(Length::Fill)
					.height(Length::Fill)
					.spacing(20)
					.push(content);

				column.into()
			},
		}
	}
}


impl MainApplication {
	/// Starts running the application.
	pub fn start() {
		let settings: Settings<()> = Settings {
			window: Window {
				size: (900, 620),
				resizable: true,
				decorations: true,
				icon: None,
				..iced::window::Settings::default()
			},

			default_text_size: 17,
			exit_on_close_request: true,
			antialiasing: true,
			..iced::Settings::default()
		};

		MainApplication::run(settings).expect("Could not create main application")
	}

	/// Initialization method.
	pub async fn initialize() -> AppState {
		AppState { newproject: button::State::new(), loadprobe: button::State::new(), view: 0usize }
	}
}



pub struct Project {
	/// Project name.
	pub name: String,

	/// Project target hardware.
	pub hwinfo: String,

	/// Project target architecture (and topology).
	pub archinfo: String,

	/// Estimated code size.
	pub codesize: usize,
}



#[derive(Debug, Clone, Copy)]
pub struct AppState {

	/// New project Button State.
	newproject: button::State,

	/// Load probe Button State.
	loadprobe: button::State,

	/// Selected view.
	view: usize,
}



fn loading<'a>() -> Element<'a, AppMessage> {
	let title = Text::new("Loading app...")
		.horizontal_alignment(HorizontalAlignment::Center)
		.vertical_alignment(VerticalAlignment::Center)
		.size(20);

	Container::new( title )
		.width(Length::Fill)
		.height(Length::Fill)
		.center_x()
		.center_y()
		.into()
}

