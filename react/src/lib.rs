use std::collections::{HashMap, HashSet};

use uuid::Uuid;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(Uuid);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(Uuid);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(Uuid, ComputeCellId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type ComputeFn<T> = Box<dyn Fn(&[T]) -> T>;
type CallbackFn<'a, T> = Box<dyn FnMut(T) + 'a>;

struct Callback<'a, T> {
    id: CallbackId,
    callback_fn: CallbackFn<'a, T>,
    last_value: T,
}
#[derive(Default)]
pub struct Reactor<'a, T> {
    input_cells: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, ComputeFn<T>>,
    dependencies: HashMap<CellId, Vec<CellId>>,
    dependents: HashMap<CellId, Vec<ComputeCellId>>,
    callbacks: HashMap<ComputeCellId, Vec<Callback<'a, T>>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Default> Reactor<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(self.get_new_id());
        self.input_cells.insert(id, initial);
        id
    }

    fn get_new_id(&mut self) -> Uuid {
        Uuid::new_v4()
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        for d in dependencies {
            match d {
                CellId::Input(id) => {
                    if !self.input_cells.contains_key(id) {
                        return Err(*d);
                    }
                }
                CellId::Compute(id) => {
                    if !self.compute_cells.contains_key(id) {
                        return Err(*d);
                    }
                }
            }
        }
        let id = ComputeCellId(self.get_new_id());
        self.compute_cells.insert(id, Box::new(compute_func));
        self.dependencies
            .insert(CellId::Compute(id), dependencies.to_vec());
        for d in dependencies {
            self.dependents.entry(*d).or_default().push(id);
        }
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(iid) => self.input_cells.get(&iid).cloned(),
            CellId::Compute(cid) => {
                let f = self.compute_cells.get(&cid)?;
                let dependencies = self.dependencies.get(&id)?;
                let values = dependencies.iter().flat_map(|d| self.value(*d));
                Some(f(&values.collect::<Vec<_>>()))
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        match self.input_cells.get_mut(&id) {
            Some(val) => {
                if *val == new_value {
                    return true;
                }
                *val = new_value;
                let mut dependents = self
                    .dependents
                    .get(&CellId::Input(id))
                    .unwrap_or(&vec![])
                    .clone();
                let mut called = HashSet::new();
                while let Some(d) = dependents.pop() {
                    if called.insert(d) {
                        let value = self.value(CellId::Compute(d)).unwrap();
                        for Callback {
                            id: _,
                            last_value,
                            callback_fn,
                        } in self.callbacks.get_mut(&d).unwrap_or(&mut vec![])
                        {
                            if *last_value != value {
                                callback_fn(value);
                                *last_value = value;
                            }
                        }
                        dependents.extend(
                            self.dependents
                                .get(&CellId::Compute(d))
                                .unwrap_or(&vec![])
                                .iter(),
                        )
                    }
                }
                true
            }
            None => false,
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let cb = CallbackId(self.get_new_id(), id);
        self.compute_cells.get(&id)?;
        let value = self.value(CellId::Compute(id)).unwrap();
        self.callbacks.entry(id).or_default().push(Callback {
            id: cb,
            callback_fn: Box::new(callback),
            last_value: value,
        });

        Some(cb)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let cbs = self
            .callbacks
            .get_mut(&cell)
            .ok_or(RemoveCallbackError::NonexistentCell)?;
        let pos = cbs
            .iter()
            .position(|cb| cb.id == callback)
            .ok_or(RemoveCallbackError::NonexistentCallback)?;
        let _ = cbs.remove(pos);
        Ok(())
    }
}
