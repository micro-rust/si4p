//! First loading screen view.



mod state;



use crate::gui::{ msg::Message };
use iced::{
    Element, Column,
    Text,
};
use tokio::sync::{ oneshot, watch };
use tracing::{ error, info, warn };

use self::state::ChipLoadingState;


pub struct LoadingView {
    /// A watcher of the chip load state.
    chipwatch: Option<watch::Receiver<(usize, usize)>>,

    /// A watcher for manufacturer load state.
    mnfswatch: Option<watch::Receiver<usize>>,

    /// A watcher for search engine load state.
    searchwatch: Option<watch::Receiver<usize>>,

    /// Gets something once the load completes.
    done: Option<oneshot::Receiver<bool>>,

    /// A loading state for chips.
    chips: ChipLoadingState,

    finished: bool,
}


impl LoadingView {
    /// Creates a new Loading view.
    pub const fn new() -> Self {
        LoadingView {
            chipwatch: None,
            mnfswatch: None,
            searchwatch: None,
            done: None,
            chips: ChipLoadingState::new(),
            finished: false,
        }
    }

    /// Updates with new watchers.
    pub fn watchers(&mut self,
        (r, (c, m, s)): (oneshot::Receiver<bool>, (watch::Receiver<(usize, usize)>, watch::Receiver<usize>, watch::Receiver<usize>))
    )
    {
        self.chipwatch = Some(c);
        self.mnfswatch = Some(m);
        self.searchwatch = Some(s);
        self.done = Some(r);
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Builds the first loading screen.
    pub fn screen(&mut self) -> (Element<Message>, bool) {
    // Create logo icon.

        // Create loading items.

        // Update the loading status.
        if let Some(ref mut watch) = &mut self.chipwatch {
            let chip = watch.borrow_and_update();
            self.chips.maxchip(chip.0);
            self.chips.loadedchip(chip.0);
        }

        if let Some(ref mut watch) = &mut self.mnfswatch {
            let mnfs = watch.borrow_and_update();
            self.chips.mnfs(*mnfs);
        }

        if let Some(ref mut watch) = &mut self.searchwatch {
            let search = watch.borrow_and_update();
            self.chips.search(*search);
        }

        // Create chips loading items.
        let chips = self.chips.view();

        // Check if the load completed.
        if let Some(ref mut oneshot) = &mut self.done {
            if let Ok(true) = oneshot.try_recv() {
                self.finished = true;

                warn!("Loading ended");

                return
                    (  
                        Column::new()
                            .push(chips)
                            .push(Text::new("DONE").size(20))
                            .into(),
                        true
                    )
            }
        }

        (
            Column::new()
                .push(chips)
                .into(),
            true
        )
    }

    /// Builds a GUI element depicting a loading progress.
    pub fn component(&mut self) {}
}
