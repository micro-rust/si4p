//! Implements the `iced` based GUI of the application.



mod common;
mod console;
mod message;
mod selector;
mod theme;



use common::router::Router;

use crate::usb::Command as USBCommand;

use iced::{
    executor,

    Application as App, Command, Element, Theme,
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

    /// The selector of USB devices.
    selector: selector::USBSelector<Message>,

    /// The router for the console messages.
    router: Option<Arc<Mutex<Router<console::Entry, Message>>>>,

    /// Channel to send USB commands.
    usbcmd: Sender<USBCommand>,

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

        // Create the USB selector.
        let selector = selector::USBSelector::new( Self::selectormsg );

        // Spawn the USB logger in a blocking thread.
        std::thread::spawn( move || { usb.run() } );

        // Creates the new application.
        let app = Self {
            console,
            selector,
            router,
            usbcmd,
            theme,
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("defmt Host")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // A new message for the console.
            Message::Console( inner ) => return self.console.update(inner),

            // A message for the USB selector.
            Message::Selector( inner ) => return self.selector.update(inner),

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
            Message::SelectDefmtFile => return Command::perform( defmtfile(), |m| m ),

            // New defmt file.
            Message::NewDefmtFile( bytes ) => {
                println!("New defmt file bytes");

                self.usbcommand( USBCommand::SetDefmtFile( bytes ) );
            },

            _ =>(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Create the file button.
        let filebtn = iced::widget::Button::new( "Select defmt file" )
            .on_press( Message::SelectDefmtFile );


        iced::widget::Row::new()
            .push(self.console.view())
            .push(
                iced::widget::Column::new()
                    .width(iced::Length::FillPortion(20))
                    .push(filebtn)
                    .push(self.selector.view())
            )
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

        iced::Subscription::batch( subscriptions )
    }
}

impl Application {
    /// Default function to turn selector message into a message.
    pub fn selectormsg(msg: selector::Message) -> Message {
        Message::Selector( msg )
    }

    /// Sends an USB command.
    fn usbcommand(&mut self, cmd: USBCommand) {
        match self.usbcmd.try_send(cmd) {
            Err(_) => println!("Failed to send USB command"),
            Ok(_) => (),
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        // Make sure to send the drop signal to the USB thread.
        let _ = self.usbcmd.try_send( USBCommand::Quit );
    }
}



/// Async function to get a file and read it.
async fn defmtfile() -> Message {
    use rfd::AsyncFileDialog;
    use tokio::{
        fs::File,
        io::AsyncReadExt,
    };

    // Get the file.
    let maybe = AsyncFileDialog::new()
        .set_directory("/")
        .pick_file()
        .await;

    // Check if anything was chosen.
    let path = match maybe.as_ref() {
        None => return Message::None,
        Some(f) => f.path().clone(),
    };

    // Create the file.
    let file = match File::open(path).await {
        Err(_) => return Message::None,
        Ok(f) => f,
    };

    // Create the buffer.
    let mut data = Vec::new();

    // Create the reader.
    let mut reader = tokio::io::BufReader::new(file);

    // Read the file.
    match reader.read_to_end(&mut data).await {
        Err(_) => return Message::None,
        Ok(_) => (),
    }

    Message::NewDefmtFile( std::sync::Arc::from(data) )
}
