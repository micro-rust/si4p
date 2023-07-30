//! Implements the `iced` based GUI of the application.



mod commands;
mod common;
pub mod console;
mod controller;
mod message;
mod theme;
//mod usbcfg;

mod right;



use common::router::Router;

use crate::usb::Command as USBCommand;

use iced::{
    executor,

    Application as App, Command, Element, Theme,

    widget::{
        Component,
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

    /// GUI Controller of the target.
    controller: controller::Controller,

    /// The usbcfg of USB devices.
    //usbcfg: usbcfg::USBConfiguration,

    /// The router for the console messages.
    router: Option<Arc<Mutex<Router<Message, Message>>>>,

    /// Channel to send USB commands.
    usbcmd: Sender<USBCommand>,

    panes: PaneGridState<PaneGridView>,

    /// Data library of the application.
    library: Arc<super::library::Library>,

    /// Right sidebar.
    right: right::RightSidebar,

    /// Application theme.
    /// Keep the theme alive until it is swapped.
    #[allow(dead_code)]
    theme: Arc<marcel::theme::Theme>,
}

impl Application {
    /// Starts the Probe subapplication.
    pub fn start() {
        use iced::window::{
            PlatformSpecific, Position,
            Settings as Window,
        };

        // Build the app settings.
        let settings: iced::Settings<()> = iced::Settings {
            window: Window {
                // Position and starting size.
                // Set a default screen size as the default window size.
                size: (1280, 720),
                position: Position::Centered,

                // Resizable and with normal decorations.
                resizable: true,
                decorations: true,

                // TODO : Include default icon.
                icon: None,

                // Minimum size to avoid everything looking weird.
                // No max size.
                min_size: Some((900, 900)),
                max_size: None,

                // Visible from start and non transparent window.
                transparent: false,
                visible: true,

                // Platform specific configuration.
                // Leave as is until we deal with Windows.
                platform_specific: PlatformSpecific { application_id: String::from("Si4+ instance") },

                // Normal application level (default behaviour).
                level: iced::window::Level::Normal,
            },

            id: Some( String::from("Si4+ instance") ),

            default_text_size: 14.0,
            exit_on_close_request: true,
            antialiasing: true,
            default_font: iced::Font::MONOSPACE,
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

        // Create the library.
        let library = Arc::new( super::library::Library::create() );

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
            let map = |msg| msg;

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
        //let usbcfg = usbcfg::USBConfiguration::new( library.clone() );

        // Create the pane grid structure.
        let panes = Self::panegrid();

        // Create the GUI controller of the target.
        let controller = controller::Controller::new();

        // Create the right sidebar.
        let right = right::RightSidebar::new(library.clone());

        // Creates the new application.
        let app = Self {
            console,
            controller,
            router,
            usbcmd,
            panes,
            library,
            theme,

            right,
        };

        // Create the library rebuild startup command.
        let library = {
            // Clone the reference outside the move block.
            let reference = Arc::clone( &app.library);
            Command::perform(async move { reference.rebuild().await }, |_| Message::None)
        };

        (app, Command::batch([library]))
    }

    fn title(&self) -> String {
        String::from("defmt Host")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use common::Widget;

        match message {
            // A new message for the console.
            Message::Console( message ) => return self.console.update( message ),

            Message::ConsoleEntry( entry ) => self.console.push( entry ),

            Message::Controller( event ) => return self.controller.update(event),

            Message::Right( event ) => return self.right.update(event),

            // A message for the USB usbcfg.
            //Message::Selector( inner ) => return self.usbcfg.update(inner),
            //Message::USBConfiguration( inner ) => return self.usbcfg.update( inner ),

            // A message for the USB handler.
            Message::USB( command ) => self.usbcommand( command ),

            // The USB thread crashed and the console router is closed.
            Message::USBThreadCrashed => if self.router.is_some() {
                // Log this error.
                self.console.push( console::Entry::usbcrash() );

                // Remove the current router from the application.
                self.router = None;
            },

            // Selects a new defmt file.
            Message::SelectELF( maybe ) => return Command::perform( commands::elf::selectELF(maybe), |m| m ),

            // Loads a defmt file.
            Message::LoadELF( path ) => return Command::perform( commands::elf::loadELF(path) , |m| m),

            // Reloads the library.
            Message::LibraryRebuild => {
                // Clone the ARC.
                let reference = Arc::clone( &self.library);

                return Command::perform(async move { reference.rebuild().await }, |_| Message::None)
            },

            Message::NewDebugSession => {
                // Rebuild the controller cores.
                self.controller.rebuild();
            }

            // New defmt file.
            Message::NewELF( bytes, path ) => {
                // Send the USB command to parse the defmt file.
                self.usbcommand( USBCommand::SetDefmtFile( bytes ) );

                // Send the path to be reloaded.
                //self.usbcfg.setpath( path );
                self.right.setpath( path.clone() );
            },

            // Set the current SVD in the peripheral selector.
            Message::NewSVD( peripherals, _ ) => {
                // Get the peripherals.
                //self.peripherals = peripherals.clone();

                // Update the controller data.
                self.controller.target(peripherals);
            },

            // Message to rebuild the USB tree.
            Message::USBTreeRebuild => {
                // Rebuild the device tree on the right sidebar.
                self.right.rebuild();
            },

            Message::PaneGridResize( event ) => self.panes.resize(&event.split, event.ratio),


            // Messages about debug probes.
            // ****************************************************************

            Message::SetDebugProbe( info ) => {
                // Notify the right sidebar.
                self.right.setprobe( info.clone() );
            },

            Message::ClearDebugProbe => {
                // Notify the right sidebar.
                self.right.clearprobe();
            },

            Message::SetDebugTarget( target ) => {
                // Mark the target as selected.
                self.right.select( target.clone() );

                // Clone the library reference.
                let library = Arc::clone( &self.library );

                // Parse the associated SVD.
                return Command::perform( commands::svd::loadSVD(target, library), |m| m );
            },

            Message::ClearDebugTarget => {
                // Mark the target as deselected.
                self.right.deselect();
            },

            _ =>(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        use common::Widget;

        use iced::{
            Length,

            widget::{
                Container,

                pane_grid::{
                    Content, PaneGrid,
                },
            },
        };

        // Build the pane grid.
        let panegrid = PaneGrid::new(&self.panes, |_, pane, _| match *pane {
            PaneGridView::Console => Content::new( self.console.view() ),

            PaneGridView::RightSidebar => Content::new( self.right.view() ),

            PaneGridView::Controller => Content::new( self.controller.view() ),

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
        use iced::subscription::unfold;

        // Create the list of subscriptions.
        let mut subscriptions = Vec::new();

        // Create the console router.
        if let Some(container) = &self.router {
            subscriptions.push( unfold( 11, Arc::clone(container), Router::listen ) );
        }

        // Create the ticker for updating the libraries.
        let libticker = iced::time::every( core::time::Duration::from_secs(10) ).map(|_| Message::LibraryRebuild);
        subscriptions.push( libticker );

        iced::Subscription::batch( subscriptions )
    }
}

impl Application {
    /// Sends an USB command.
    fn usbcommand(&mut self, cmd: USBCommand) {
        match self.usbcmd.try_send(cmd) {
            Err(e) => {
                self.console.push(
                    console::Entry::error( console::Source::Host, format!("Failed to send USB command : {}", e) )
                );
            },

            _ => (),
        }
    }

    /// Builds the panegrid structure.
    fn panegrid() -> PaneGridState<PaneGridView> {
        use iced::widget::pane_grid::{
            Axis, Configuration,
        };

        // Build the configuration.
        let configuration = {
            // Bottom and top of the left sidebar.
            let left = Configuration::Split {
                axis: Axis::Horizontal,
                ratio: 0.5,
                // Top box for cores.
                a: Box::new( Configuration::Pane( PaneGridView::Controller ) ),
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
                b: Box::new( Configuration::Pane( PaneGridView::RightSidebar ) ),
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


/// All possible views of the application.
pub enum PaneGridView {
    /// UI view of the console.
    Console,

    /// UI view of the main screen.
    Main,

    /// UI view of the right sidebar.
    RightSidebar,

    
    Controller,
    WatchVars,
}