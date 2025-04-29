use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
static POLYFILL_COUNTER: AtomicUsize = AtomicUsize::new(1);

use std::{cell::RefCell, rc::Rc};

use sticknodes_rs::DrawOrderIndex;
use wasm_bindgen::prelude::*;

use crate::{color::Color, stickfigure::Stickfigure};

#[wasm_bindgen]
pub struct Polyfill {
    id: usize,
    pub(crate) inner: Rc<RefCell<sticknodes_rs::Polyfill>>,
    pub(crate) stickfigure: Rc<Stickfigure>,
}

#[wasm_bindgen]
impl Polyfill {
    #[wasm_bindgen(getter)]
    pub fn stickfigure(&self) -> Stickfigure {
         self.stickfigure.deref().clone()
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> usize {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn anchor_node_draw_index(&self) -> i32 {
        self.inner.borrow().anchor_node_draw_index.0
    }

    /// Attempts to set polyfill anchor node to supplied draw index.
    /// Throws error if draw index is already occupied by a polyfill or is an invalid draw index.
    pub fn set_anchor_node_draw_index(&self, draw_index: i32) -> Result<(), JsError> {
        Ok(self.stickfigure.set_anchor_node_draw_index(Rc::clone(&self.inner), DrawOrderIndex(draw_index))?)

    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> Color {
        Color::from(self.inner.borrow().color)
    }

    #[wasm_bindgen(setter)]
    pub fn set_color(&self, color: Color) {
        self.inner.borrow_mut().color = sticknodes_rs::Color::from(color);
    }

    #[wasm_bindgen(getter)]
    pub fn use_polyfill_color(&self) -> bool {
        self.inner.borrow().use_polyfill_color
    }

    #[wasm_bindgen(setter)]
    pub fn set_use_polyfill_color(&self, use_polyfill_color: bool) {
        self.inner.borrow_mut().use_polyfill_color = use_polyfill_color;
    }

    #[wasm_bindgen(getter)]
    pub fn attached_node_draw_indices(&self) -> Vec<i32> {
        self.inner
            .borrow()
            .attached_node_draw_indices
            .iter()
            .map(|draw_order_index| draw_order_index.0)
            .collect()
    }

    /// Sets the attached node draw indices
    /// Throws error if any given node draw index is invalid
    pub fn set_attached_node_draw_indices(&self, draw_indices: Vec<i32>) -> Result<(), JsError> {
        let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
        Ok(self.stickfigure.set_attached_node_draw_indices(Rc::clone(&self.inner), draw_indices_mapped)?)
    }

    /// Inserts the given node draw indices after the given node draw index
    /// Throws error if any given node draw index is invalid
    pub fn insert_attached_node_draw_indices_after(
        &self,
        draw_indices: Vec<i32>,
        insert_after_draw_index: i32,
    ) -> Result<(), JsError> {
        let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
        Ok(self.stickfigure.insert_attached_node_draw_indices_after(Rc::clone(&self.inner), draw_indices_mapped, DrawOrderIndex(insert_after_draw_index))?)
    }

    /// Inserts the given node draw indices before the given node draw index
    /// Throws error if any given node draw index is invalid
    pub fn insert_attached_node_draw_indices_before(
        &self,
        draw_indices: Vec<i32>,
        insert_before_draw_index: i32,
    ) -> Result<(), JsError> {
        let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
        Ok(self.stickfigure.insert_attached_node_draw_indices_before(Rc::clone(&self.inner), draw_indices_mapped, DrawOrderIndex(insert_before_draw_index))?)
    }

    /// Removes the given node draw indices from the attached node draw indices.
    /// Throws error if any given node draw index is invalid
    pub fn remove_attached_node_draw_indices(&self, draw_indices: Vec<i32>) -> Result<(), JsError> {
        let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
        Ok(self.stickfigure.remove_attached_node_draw_indices(Rc::clone(&self.inner), draw_indices_mapped)?)

    }

    /// Sets the attached node draw indices, processing only valid indices.
    /// Returns an array of indices that were actually set.
    /// Unlike `set_attached_node_draw_indices`, this method ignores invalid indices instead of erroring.
    pub fn try_set_attached_node_draw_indices(&self, draw_indices: Vec<i32>) -> Vec<i32> {
        let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
        self.stickfigure.try_set_attached_node_draw_indices(Rc::clone(&self.inner), draw_indices_mapped).iter().map(|doi| doi.0).collect()
    }

    // below methods are commented out because I don't know if they'd be worth existing since there would have to be some kind of special return for if the provided index to insert after/before is invalid.. which kind of defeats the point of the method. At that point just handle the error of the non try versions lol
    // /// Inserts the given node draw indices after the given node draw index, processing only valid indices.
    // /// Returns an array of indices that were actually inserted.
    // /// Unlike `insert_attached_node_draw_indices_after`, this method ignores invalid indices instead of erroring.
    // pub fn try_insert_attached_node_draw_indices_after(
    //     &self,
    //     draw_indices: Vec<i32>,
    //     insert_after_draw_index: i32,
    // ) -> Vec<i32> {
    //     let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
    //     self.stickfigure.try_insert_attached_node_draw_indices_after(Rc::clone(&self.inner), draw_indices_mapped, DrawOrderIndex(insert_after_draw_index)).iter().map(|doi| doi.0).collect()
    // }

    // /// Inserts the given node draw indices before the given node draw index, processing only valid indices.
    // /// Returns an array of indices that were actually inserted.
    // /// Unlike `insert_attached_node_draw_indices_before`, this method ignores invalid indices instead of erroring.
    // pub fn try_insert_attached_node_draw_indices_before(
    //     &self,
    //     draw_indices: Vec<i32>,
    //     insert_before_draw_index: i32,
    // ) -> Vec<i32> {
    //     let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
    //     self.stickfigure.try_insert_attached_node_draw_indices_before(Rc::clone(&self.inner), draw_indices_mapped, DrawOrderIndex(insert_before_draw_index)).iter().map(|doi| doi.0).collect()
    // }

    /// Removes the given node draw indices from the attached node draw indices, processing only valid indices.
    /// Returns an array of indices that were actually removed.
    /// Unlike `remove_attached_node_draw_indices`, this method ignores invalid indices instead of erroring.
    pub fn try_remove_attached_node_draw_indices(&self, draw_indices: Vec<i32>) -> Vec<i32> {
        let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
        self.stickfigure.try_remove_attached_node_draw_indices(Rc::clone(&self.inner), draw_indices_mapped).iter().map(|doi| doi.0).collect()
    }
}

impl Polyfill {
    pub fn new(polyfill: Rc<RefCell<sticknodes_rs::Polyfill>>, stickfigure: Rc<Stickfigure>) -> Polyfill {
        let id = POLYFILL_COUNTER.fetch_add(1, Ordering::Relaxed);
        Polyfill {
            id,
            inner: polyfill,
            stickfigure,
        }
    }
}