//! Implements the `iced` based GUI of the application.



mod commands;
mod common;
mod console;
mod message;
mod theme;
mod usbcfg;



use common::router::Router;

use crate::usb::Command as USBCommand;

use iced::{
    executor,

    Application as App, Command, Element, Theme,

    widget::{
        pane_grid::State as PaneGridState,
    },
};

pub use message::Message;

use std::sync::Arc;

use tokio::sync::{
    mpsc::{
        self,
        
        Sender,
    },
    
    Mutex,
};



pub struct Application {
    /// The console of the application.
    console: console::Console,

    /// The usbcfg of USB devices.
    usbcfg: usbcfg::USBConfiguration,

    /// The router for the console messages.
    router: Option<Arc<Mutex<Router<console::Entry, Message>>>>,

    /// Channel to send USB commands.
    usbcmd: Sender<USBCommand>,

    panes: PaneGridState<PaneGridView>,

    /// Application theme.
    /// Keep the theme alive until it is swapped.
    #[allow(dead_code)]
    theme: Arc<marcel::theme::Theme>,
}

impl Application {
    /// Starts the Probe subapplication.
    pub fn start() {
        use iced::{
            window::{
                PlatformSpecific, Position,
                Settings as Window,
            },
        };

        // Build the app settings.
        let settings: iced::Settings<()> = iced::Settings {
            window: Window {
                size: (1280, 720),
                position: Position::Centered,
                resizable: true,
                decorations: true,
                icon: None,
                min_size: Some((900, 900)),
                max_size: None,
                always_on_top: false,
                transparent: false,
                visible: true,
                platform_specific: PlatformSpecific,
            },

            id: None,
            text_multithreading: true,
            try_opengles_first: true,

            default_text_size: 17.0,
            exit_on_close_request: true,
            antialiasing: true,
            default_font: None,
            flags: (),
        };

        match Self::run(settings) {
            Err(e) => panic!("Failed to open the probe sub-application: {}", e),
            _ => (),
        }
    }
}

impl App for Application {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_: Self::Flags) -> (Self, Command<Message>) {
        use crate::usb::USBLogger;

        // Build the application default theme.
        let theme = {
            use marcel::theme::{ Theme, serial::Theme as Serial };

            // Deserialize the theme.
            let serial: Serial = ron::de::from_str( &theme::DARK ).expect( "Failed to deserialize prechecked theme" );

            // Create the new theme and parse it.
            let mut theme = Theme::new();
            let _ = theme.parse( &serial ).expect("Failed to parse prechecked theme");

            // Parse the theme.
            Arc::new( theme )
        };

        // Create the console.
        let console = console::Console::new( theme.clone() );

        // Create the MPSC pair.
        let (ctx, crx) = mpsc::channel(128);

        // Create the console router.
        let router = {

            // Create the map function.
            let map = |entry| Message::Console( console::Message::New(entry) );

            // Create the new router.
            let router = Router::create( Arc::new( map ), Message::USBThreadCrashed, crx );

            // Create the container.
            Some( Arc::new( Mutex::new( router ) ) )
        };

        // Create the USB logger.
        let (usb, usbcmd) = match USBLogger::new( ctx.clone() ) {
            Some((usb, cmd)) => (usb, cmd),
            _ => panic!("Failed to create USB context"),
        };        

        // Spawn the USB logger in a blocking thread.
        std::thread::spawn( move || { usb.run() } );

        // Create the USB usbcfg.
        let mut usbcfg = usbcfg::USBConfiguration::new();

        // Create the pane grid structure.
        let panes = Self::panegrid();

        // Creates the new application.
        let app = Self {
            console,
            usbcfg,
            router,
            usbcmd,
            panes,
            theme,
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("defmt Host")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use common::Widget;

        match message {
            // A new message for the console.
            Message::Console( inner ) => return self.console.update(inner),

            // A message for the USB usbcfg.
            //Message::Selector( inner ) => return self.usbcfg.update(inner),
            Message::USBConfiguration( inner ) => return self.usbcfg.update( inner ),

            // A message for the USB handler.
            Message::USB( command ) => self.usbcommand( command ),

            // The USB thread crashed and the console router is closed.
            Message::USBThreadCrashed => if self.router.is_some() {
                // Remove the current router from the application.
                self.router = None;

                // Log this error.
                return self.console.update( console::Message::New( console::Entry::usbcrash() ) );
            },

            // Selects a new defmt file.
            Message::SelectELF( maybe ) => return Command::perform( commands::elf::selectELF(maybe), |m| m ),

            // Loads a defmt file.
            Message::LoadELF( path ) => return Command::perform( commands::elf::loadELF(path) , |m| m),

            // New defmt file.
            Message::NewELF( bytes, path ) => {
                // Send the USB command to parse the defmt file.
                self.usbcommand( USBCommand::SetDefmtFile( bytes ) );

                // Send the path to be reloaded.
                self.usbcfg.setpath( path );
            },

            // Message to rebuild the USB tree.
            Message::USBTreeRebuild => self.usbcfg.rebuild(),

            Message::PaneGridResize( event ) => self.panes.resize(&event.split, event.ratio),

            _ =>(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        use common::Widget;

        use iced::{
            Length,

            widget::{
                Container, Text,

                pane_grid::{
                    Content, PaneGrid,
                },
            },
        };

        // Build the pane grid.
        let panegrid = PaneGrid::new(&self.panes, |id, pane, maximized| match *pane {
            PaneGridView::Console => Content::new( self.console.view() ),

            PaneGridView::Configuration => Content::new( self.usbcfg.view() ),

            _ => Content::new( iced::widget::Column::new() ),
        })
        .height(Length::Fill)
        .width(Length::Fill)
        .spacing(2)
        .on_resize(10, |event| Message::PaneGridResize(event));

        Container::new( panegrid )
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        use iced::subscription::{
            unfold,
        };

        // Create the list of subscriptions.
        let mut subscriptions = Vec::new();

        // Create the console router.
        if let Some(container) = &self.router {
            subscriptions.push( unfold( 11, Arc::clone(container), Router::listen ) );
        }

        // Create the ticker for updating.
        let ticker = iced::time::every( core::time::Duration::from_millis(500) ).map(|_| Message::USBTreeRebuild);
        subscriptions.push( ticker );

        iced::Subscription::batch( subscriptions )
    }
}

impl Application {
    /// Sends an USB command.
    fn usbcommand(&mut self, cmd: USBCommand) {
        match self.usbcmd.try_send(cmd) {
            Err(e) => {
                self.console.update(
                    console::Message::New(
                        console::Entry::error( console::Source::Host, format!("Failed to send USB command : {}", e) )
                    )
                );
            },

            _ => (),
        }
    }

    /// Builds the panegrid structure.
    fn panegrid() -> PaneGridState<PaneGridView> {
        use iced::widget::pane_grid::{
            Axis, Configuration, Split,
        };

        // Build the configuration.
        let configuration = {
            // Bottom and top of the left sidebar.
            let left = Configuration::Split {
                axis: Axis::Horizontal,
                ratio: 0.5,
                // Top box for cores.
                a: Box::new( Configuration::Pane( PaneGridView::Cores ) ),
                // Bottom box for watch and vars.
                b: Box::new( Configuration::Pane( PaneGridView::WatchVars ) ),
            };

            // Main view and right pane.
            let right = Configuration::Split {
                axis: Axis::Vertical,
                ratio: 0.7,
                // Main view.
                a: Box::new( Configuration::Pane( PaneGridView::Main ) ),
                // Configuration view.
                b: Box::new( Configuration::Pane( PaneGridView::Configuration ) ),
            };

            // Main view and console.
            let center = Configuration::Split {
                axis: Axis::Horizontal,
                ratio: 0.6,
                // Main view.
                a: Box::new( right ),
                // Console.
                b: Box::new( Configuration::Pane( PaneGridView::Console ) ),
            };

            // Everything
            Configuration::Split {
                axis: Axis::Vertical,
                ratio: 0.2,
                // Left sidebar
                a: Box::new( left ),
                // The rest
                b: Box::new( center )
            }
        };

        PaneGridState::with_configuration(configuration)
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        // Make sure to send the drop signal to the USB thread.
        let _ = self.usbcmd.try_send( USBCommand::Quit );
    }
}


pub enum PaneGridView {
    Console,
    Main,
    Configuration,
    Cores,
    WatchVars,
}