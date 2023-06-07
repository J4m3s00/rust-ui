use std::{cell::RefCell, collections::HashMap};

use rust_graphics::{rect::Rect, vec::Vec2};

use crate::error::Result;

use super::widget::{SizePolicy, SizePolicy2D, Widget};

type WidgetNodeId = usize;
const ROOT_NODE_ID: WidgetNodeId = 1;

/// Represents a child widget. Used to group widgets together.
struct WidgetNode {
    pub(self) id: WidgetNodeId,
    pub(self) parent: Option<WidgetNodeId>,
    pub(self) children: Vec<WidgetNodeId>,

    pub(self) content_area: Rect,
}

pub enum WidgetInteractionType {
    Click,
}

pub struct WidgetInteraction {
    pub interaction_type: WidgetInteractionType,
    pub widget_id: WidgetNodeId,
}

pub struct WidgetBuilder {
    texts: Vec<(String, Vec2)>,
    interactions: Vec<WidgetInteraction>,
    cursor_pos: Vec2,
    widget_nodes: HashMap<WidgetNodeId, WidgetNode>,
    node_ptr: WidgetNodeId,
}

impl WidgetBuilder {
    pub fn new(content_area: Rect) -> Self {
        let mut widget_nodes = HashMap::new();
        let root_node = WidgetNode {
            children: Vec::new(),
            content_area,
            parent: None,
            id: ROOT_NODE_ID,
        };
        widget_nodes.insert(root_node.id, root_node);

        Self {
            texts: Vec::new(),
            interactions: Vec::new(),
            cursor_pos: content_area.top_left(),
            widget_nodes,
            node_ptr: ROOT_NODE_ID,
        }
    }
}

pub struct PushChild<'a>(RefCell<&'a mut WidgetBuilder>);

impl<'a> PushChild<'a> {
    pub fn new(builder: RefCell<&'a mut WidgetBuilder>) -> Self {
        Self(builder)
    }

    pub fn text(&self, text: impl Into<String>) -> &Self {
        let pos = self.0.borrow().cursor_pos;
        self.0.borrow_mut().texts.push((text.into(), pos));
        self
    }

    pub fn interaction(&self, interaction: WidgetInteractionType) -> &Self {
        let widget_id = self.0.borrow().node_ptr;
        self.0.borrow_mut().interactions.push(WidgetInteraction {
            interaction_type: interaction,
            widget_id,
        });
        self
    }

    pub fn widget<T>(&self, widget: &T, size: SizePolicy2D) -> &Self
    where
        T: Widget,
    {
        widget.build(&self, size);
        self
    }

    pub fn child<Func>(&'a self, content_area: SizePolicy2D, nest: Func) -> &Self
    where
        Func: FnOnce(&Self),
    {
        let mut builder = self.0.borrow_mut();
        let child = builder.push_child(content_area);

        //nest(&child);
        builder.pop_child();
        self
    }
}

impl<'a> Drop for PushChild<'a> {
    fn drop(&mut self) {
        self.0.borrow_mut().pop_child();
    }
}

impl<'a> WidgetBuilder {
    fn push_child(&'a mut self, content_area: SizePolicy2D) -> PushChild {
        let current_node = self
            .get_node(self.node_ptr)
            .expect("Failed to push child. The node pointer was not found!");
        // TODO: Add padding and margin
        let content_area = Rect::new_from_xy(
            self.cursor_pos.x,
            self.cursor_pos.y,
            match content_area.horizontal {
                SizePolicy::Fill => current_node.content_area.width(),
                SizePolicy::Percentage(percentage) => {
                    current_node.content_area.width() * percentage
                }
                SizePolicy::Fixed(width) => width,
            },
            match content_area.vertical {
                SizePolicy::Fill => current_node.content_area.height(),
                SizePolicy::Percentage(percentage) => {
                    current_node.content_area.height() * percentage
                }
                SizePolicy::Fixed(height) => height,
            },
        );
        let node_id = self.new_node(content_area, Some(self.node_ptr)).id;
        if let Err(err) = self.add_child(self.node_ptr, node_id) {
            println!("Failed to push child: {}", err);
        }
        self.node_ptr = node_id;
        PushChild::new(RefCell::new(self))
    }

    pub fn pop_child(&mut self) {
        if let Some(parent) = self
            .get_node(self.node_ptr)
            .expect("Failed to pop child. The node pointer was not found!")
            .parent
        {
            self.node_ptr = parent;
        }
    }

    pub fn child_content_area(&self, id: WidgetNodeId) -> Result<Rect> {
        let node = self.get_node(id).ok_or(format!(
            "Failed to get child content area. Node {} not found!",
            id
        ))?;
        Ok(node.content_area)
    }

    pub fn advance(&mut self) {
        self.cursor_pos.y += 20.;
    }

    pub fn texts(&self) -> &Vec<(String, Vec2)> {
        &self.texts
    }

    pub fn interactions(&self) -> &Vec<WidgetInteraction> {
        &self.interactions
    }
}

// Private functions
impl WidgetBuilder {
    fn new_node(&mut self, content_area: Rect, parent: Option<WidgetNodeId>) -> &WidgetNode {
        let node = WidgetNode {
            id: self.new_node_id(),
            parent: Some(parent.unwrap_or(ROOT_NODE_ID)),
            children: Vec::new(),
            content_area,
        };
        let id = node.id;
        self.widget_nodes.insert(id, node);

        self.widget_nodes
            .get(&id)
            .expect("Failed to insert new node!")
    }

    fn new_node_id(&self) -> WidgetNodeId {
        let mut id: WidgetNodeId;
        loop {
            id = rand::random();
            if !self.widget_nodes.contains_key(&id) {
                break id;
            }
        }
    }

    fn get_node(&self, id: WidgetNodeId) -> Option<&WidgetNode> {
        self.widget_nodes.get(&id)
    }

    fn get_node_mut(&mut self, id: WidgetNodeId) -> Option<&mut WidgetNode> {
        self.widget_nodes.get_mut(&id)
    }

    fn add_child(&mut self, parent: WidgetNodeId, child: WidgetNodeId) -> Result<()> {
        // Remove child from old parent
        if let Some(old_parent) = self.get_node_mut(
            self.get_node(child)
                .ok_or("Child node not found!")?
                .parent
                .unwrap_or(ROOT_NODE_ID),
        ) {
            old_parent.children.retain(|&x| x != child);
        }

        // Add child to new parent
        if let Some(new_parent) = self.get_node_mut(parent) {
            new_parent.children.push(child);
        }

        // Set child's parent
        if let Some(child_node) = self.get_node_mut(child) {
            child_node.parent = Some(parent);
        }
        Ok(())
    }
}
