use std::collections::BTreeMap;

use bevy_ecs::prelude::{Component, Entity};

use crate::error::Error;

pub struct EntityTree {
    pub root: Option<Entity>,

    pub children: BTreeMap<Entity, Vec<Entity>>,
    pub parent: BTreeMap<Entity, Option<Entity>>,
}

impl Default for EntityTree {
    fn default() -> Self {
        Self {
            root: None,
            children: BTreeMap::new(),
            parent: BTreeMap::new(),
        }
    }
}

#[derive(Component)]
struct T;

impl EntityTree {
    pub fn new(root_entity: Entity) -> Self {
        let mut tree = Self::default();
        tree.insert_node(root_entity);
        tree.set_root(root_entity);
        tree
    }

    pub fn set_root(&mut self, entity: Entity) {
        self.root = Some(entity);
    }

    pub fn add_child(&mut self, parent: Entity, child: Entity) -> Result<(), Error> {
        if self.root.is_none() {
            return Err(Error::NoRootEntity);
        }

        if let Some(p) = self.children.get_mut(&parent) {
            p.push(child);
        } else if self.root.is_some() {
            if parent == self.root.unwrap() {
                return Err(Error::EntityNotFound(parent));
            }
            return self.add_child(self.root.unwrap(), child);
        } else {
            return Err(Error::EntityNotFound(parent));
        }
        self.parent.insert(child, Some(parent));
        self.children.insert(child, Vec::new());
        Ok(())
    }

    pub fn get_children(&self, entity: Entity) -> Result<&Vec<Entity>, Error> {
        self.children
            .get(&entity)
            .ok_or(Error::EntityNotFound(entity))
    }

    pub fn get_parent(&self, entity: Entity) -> Result<&Entity, Error> {
        self.parent
            .get(&entity)
            .and_then(|p| p.as_ref())
            .ok_or(Error::EntityNotFound(entity))
    }

    fn insert_node(&mut self, entity: Entity) {
        self.children.insert(entity, Vec::new());
        self.parent.insert(entity, None);
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn remove(&mut self, entity: Entity) -> Result<(), Error> {
        self.children.remove(&entity).ok_or(Error::EntityNotFound(entity))?;
        self.parent.remove(&entity).ok_or(Error::EntityNotFound(entity))?;
        Ok(())
    }
}

impl<'a> IntoIterator for &'a EntityTree {
    type Item = Entity;
    type IntoIter = EntityTreeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EntityTreeIterator {
            tree: self,
            current: None,
        }
    }
}

pub struct EntityTreeIterator<'a> {
    tree: &'a EntityTree,
    current: Option<Entity>,
}

impl<'a> Iterator for EntityTreeIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            if !self.tree.children.contains_key(&current) {
                panic!("EntityTreeIterator: {} not found in tree", current.index());
            }

            if !self.tree.children[&current].is_empty() {
                self.current = Some(self.tree.children[&current][0]);
                return self.current;
            } else {
                let mut iter_node = current;
                while let Some(parent) = self.tree.parent[&iter_node] {
                    let siblings = &self.tree.children[&parent];

                    let sibling_index = siblings.iter().position(|s| *s == iter_node).unwrap() + 1;
                    if sibling_index < siblings.len() {
                        self.current = Some(siblings[sibling_index]);
                        return self.current;
                    } else {
                        iter_node = parent;
                    }
                }
                return None;
            }
        }
        self.current = self.tree.root;
        self.current
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::Entity;

    #[test]
    fn test_tree() {
        let mut tree = super::EntityTree::default();

        let root = Entity::from_raw(1);
        let child1 = Entity::from_raw(2);
        let child2 = Entity::from_raw(3);
        let child3 = Entity::from_raw(4);

        tree.insert_node(root);
        tree.set_root(root);

        tree.add_child(root, child1).unwrap();
        tree.add_child(root, child2).unwrap();
        tree.add_child(child2, child3).unwrap();

        assert_eq!(tree.get_children(root).unwrap().len(), 2);
        assert_eq!(tree.get_children(child2).unwrap().len(), 1);
    }

    #[test]
    fn test_tree_iterator() {
        let mut tree = super::EntityTree::default();

        let root = super::Entity::from_raw(1);
        let child1 = super::Entity::from_raw(2);
        let child2 = super::Entity::from_raw(3);
        let child3 = super::Entity::from_raw(4);

        tree.insert_node(root);
        tree.set_root(root);

        tree.add_child(root, child1).unwrap();
        tree.add_child(root, child2).unwrap();
        tree.add_child(child2, child3).unwrap();

        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(root));
        assert_eq!(iter.next(), Some(child1));
        assert_eq!(iter.next(), Some(child2));
        assert_eq!(iter.next(), Some(child3));
        assert_eq!(iter.next(), None);
    }
}
