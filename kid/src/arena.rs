#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId {
    pub index: u32,
    pub generation: u32,
}

struct Slot<T> {
    generation: u32,
    value: Option<T>,
    next_free: Option<u32>,
}

// Internal helper constructors can be added as needed.

pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    free_head: Option<u32>,
    len: usize,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_head: None,
            len: 0,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            slots: Vec::with_capacity(cap),
            free_head: None,
            len: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> EntityId {
        match self.free_head {
            Some(head) => {
                let idx = head as usize;
                let slot = &mut self.slots[idx];
                debug_assert!(slot.value.is_none());
                // Pop from free list
                self.free_head = slot.next_free;
                slot.next_free = None;
                slot.value = Some(value);
                self.len += 1;
                EntityId {
                    index: head,
                    generation: slot.generation,
                }
            }
            None => {
                let index = self.slots.len() as u32;
                self.slots.push(Slot {
                    generation: 0,
                    value: Some(value),
                    next_free: None,
                });
                self.len += 1;
                EntityId {
                    index,
                    generation: 0,
                }
            }
        }
    }

    pub fn get(&self, id: EntityId) -> Option<&T> {
        let idx = id.index as usize;
        let slot = self.slots.get(idx)?;
        if slot.generation == id.generation {
            slot.value.as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
        let idx = id.index as usize;
        let slot = self.slots.get_mut(idx)?;
        if slot.generation == id.generation {
            slot.value.as_mut()
        } else {
            None
        }
    }

    pub fn remove(&mut self, id: EntityId) -> Option<T> {
        let idx = id.index as usize;
        if idx >= self.slots.len() {
            return None;
        }
        let gen_matches;
        let is_occupied;
        {
            let slot = &self.slots[idx];
            gen_matches = slot.generation == id.generation;
            is_occupied = slot.value.is_some();
        }
        if !gen_matches || !is_occupied {
            return None;
        }

        // Now safe to take
        let slot = &mut self.slots[idx];
        let v = slot.value.take();
        debug_assert!(v.is_some());
        slot.generation = slot.generation.wrapping_add(1);
        slot.next_free = self.free_head;
        self.free_head = Some(id.index);
        self.len -= 1;
        v
    }

    pub fn contains(&self, id: EntityId) -> bool {
        self.get(id).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.slots.iter().enumerate().filter_map(|(i, slot)| {
            slot.value.as_ref().map(|v| {
                (
                    EntityId {
                        index: i as u32,
                        generation: slot.generation,
                    },
                    v,
                )
            })
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.slots.iter_mut().enumerate().filter_map(|(i, slot)| {
            slot.value.as_mut().map(|v| {
                (
                    EntityId {
                        index: i as u32,
                        generation: slot.generation,
                    },
                    v,
                )
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_capacity() {
        let a: Arena<i32> = Arena::new();
        assert_eq!(a.len(), 0);
        assert!(a.is_empty());

        let a: Arena<i32> = Arena::with_capacity(8);
        assert_eq!(a.len(), 0);
        assert!(a.is_empty());
    }

    #[test]
    fn insert_and_get() {
        let mut a = Arena::new();
        let id = a.insert(10);
        assert_eq!(a.get(id), Some(&10));
    }

    #[test]
    fn get_mut_updates_value() {
        let mut a = Arena::new();
        let id = a.insert(1);
        if let Some(v) = a.get_mut(id) {
            *v = 42;
        }
        assert_eq!(a.get(id), Some(&42));
    }

    #[test]
    fn remove_invalidates_old_id() {
        let mut a = Arena::new();
        let id = a.insert("A");
        assert_eq!(a.remove(id), Some("A"));
        assert!(a.get(id).is_none());
        assert!(a.remove(id).is_none());
    }

    #[test]
    fn remove_then_reinsert_reuses_index_and_changes_generation() {
        let mut a = Arena::new();
        let id1 = a.insert("A");
        let idx = id1.index;
        let _ = a.remove(id1).unwrap();
        let id2 = a.insert("B");
        assert_eq!(id2.index, idx, "free list should reuse index");
        assert_ne!(id2.generation, id1.generation, "generation should change");
        assert!(a.get(id1).is_none(), "old id must be invalid");
        assert_eq!(a.get(id2), Some(&"B"));
    }

    #[test]
    fn contains_and_len() {
        let mut a = Arena::new();
        let id1 = a.insert(1);
        let id2 = a.insert(2);
        assert_eq!(a.len(), 2);
        assert!(a.contains(id1));
        assert!(a.contains(id2));
        a.remove(id1);
        assert_eq!(a.len(), 1);
        assert!(!a.contains(id1));
        assert!(a.contains(id2));
    }

    #[test]
    fn out_of_bounds_and_generation_mismatch() {
        let mut a = Arena::new();
        let id = a.insert(5);
        // out of bounds index
        let bogus = EntityId {
            index: 9999,
            generation: 0,
        };
        assert!(a.get(bogus).is_none());
        // generation mismatch
        a.remove(id);
        assert!(a.get(id).is_none());
    }

    #[test]
    fn iter_and_iter_mut_visit_only_occupied() {
        let mut a = Arena::new();
        let id1 = a.insert(1);
        let id2 = a.insert(2);
        a.remove(id1);
        let items: Vec<(EntityId, i32)> = a.iter().map(|(id, v)| (id, *v)).collect();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].0.index, id2.index);
        assert_eq!(items[0].1, 2);

        for (_, v) in a.iter_mut() {
            *v *= 10;
        }
        assert_eq!(a.get(id2), Some(&20));
    }

    // 統合テスト: 生成→削除→再生成で古いIDが無効
    #[test]
    fn integration_recycle_id_invalidates_old() {
        let mut a = Arena::new();
        let id1 = a.insert("A");
        a.remove(id1);
        let id2 = a.insert("B");
        assert_ne!(id1, id2);
        assert!(a.get(id1).is_none());
        assert_eq!(a.get(id2), Some(&"B"));
    }
}
