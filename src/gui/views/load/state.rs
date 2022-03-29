//! State of the initial loading.


use crate::gui::Message;
use iced::{
    Element, Column, Row,
    Align, Length,
    Text,
};



#[derive(Debug, Clone, Copy)]
pub struct ChipLoadingState {
    /// Number of chips loaded.
    chips: (usize, usize),

    /// Number of manufacturers entries generated.
    mnfs: usize,

    /// Number of families entries generated.
    fmls: usize,

    /// Number of search engine entries generated.
    search: usize,
}

impl ChipLoadingState {
    /// Creates an empty `ChipLoadingState`.
    pub const fn new() -> Self {
        ChipLoadingState {
            chips: (0, 0),
            mnfs: 0,
            fmls: 0,
            search: 0,
        }
    }

    /// Sets the max number of chips.
    #[inline]
    pub fn maxchip(&mut self, max: usize) {
        self.chips.1 = max;
    }

    /// Sets the current number of chips loaded.
    #[inline]
    pub fn loadedchip(&mut self, l: usize) {
        self.chips.0 = l;
    }

    /// Sets the number of manufacturers entries loaded.
    #[inline]
    pub fn mnfs(&mut self, mnfs: usize) {
        self.mnfs = mnfs;
    }

    /// Sets the number of families entries loaded.
    #[inline]
    pub fn fmls(&mut self, fmls: usize) {
        self.fmls = fmls;
    }

    /// Sets the number of search entries loaded.
    #[inline]
    pub fn search(&mut self, search: usize) {
        self.search = search;
    }

    /// Creates a GUI element displaying its information.
    pub fn view(&self) -> Element<Message> {
        // Create chip section.
        let chips = {
            let desc = Text::new("Chip data").size(12);

            let frac = Text::new(&format!("{} / {}", self.chips.0, self.chips.1)).size(12);

            Row::new()
                .push(
                    Column::new()
                        .push(desc)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .padding(0)
                ).push(
                    Column::new()
                        .push(frac)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .padding(0)
                        .align_items(Align::End)
                )
        };

        // Create relational database section.
        let rela = {
            let desc = Text::new("Building relational database").size(12);

            let manufacturers = {
                let desc = Text::new("Manufacturers").size(10);

                let frac = Text::new(&format!("{} entries", self.mnfs)).size(10);

                Row::new()
                    .push(
                        Column::new()
                            .push(desc)
                            .width(Length::Shrink)
                            .height(Length::Shrink)
                            .padding(0)
                    ).push(
                        Column::new()
                            .push(frac)
                            .width(Length::Fill)
                            .height(Length::Shrink)
                            .padding(0)
                            .align_items(Align::End)
                    )
            };

            let families = {
                let desc = Text::new("Families").size(10);

                let frac = Text::new(&format!("{} entries", self.fmls)).size(10);

                Row::new()
                    .push(
                        Column::new()
                            .push(desc)
                            .width(Length::Shrink)
                            .height(Length::Shrink)
                            .padding(0)
                    ).push(
                        Column::new()
                            .push(frac)
                            .width(Length::Fill)
                            .height(Length::Shrink)
                            .padding(0)
                            .align_items(Align::End)
                    )
            };

            let container = Column::new()
                .push(manufacturers)
                .push(families)
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(5);

            Column::new()
                .push(desc)
                .push(container)
                .width(Length::Fill)
                .height(Length::Shrink)
        };

        // Create search section.
        let search = {
            let desc = Text::new("Building search engine...").size(12);

            let frac = Text::new(&format!("{} entries", self.search)).size(12);

            Row::new()
                .push(
                    Column::new()
                        .push(desc)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .padding(0)
                ).push(
                    Column::new()
                        .push(frac)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .padding(0)
                        .align_items(Align::End)
                )
        };


        Column::new()
            .push(chips)
            .push(rela)
            .push(search)
            .into()
    }
}
