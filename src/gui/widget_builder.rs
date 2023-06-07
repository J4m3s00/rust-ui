use std::collections::HashMap;

use rust_graphics::{rect::Rect, vec::Vec2};

use crate::error::Result;

use super::widget::{SizePolicy, SizePolicy2D};

type WidgetNodeId = usize;
const ROOT_NODE_ID: WidgetNodeId = 1;

/// Represents a child widget. Used to group widgets together.
pub struct WidgetNode {
    pub id: WidgetNodeId,
    pub parent: Option<WidgetNodeId>,
    pub children: Vec<WidgetNodeId>,

    pub text: Option<String>,
    pub interaction: Option<WidgetInteractionType>,
    pub content_area: Rect,
}

pub struct WidgetNodeIterator<'a> {
    current: WidgetNodeId,
    builder: &'a WidgetBuilder,
}

impl<'a> Iterator for WidgetNodeIterator<'a> {
    type Item = &'a WidgetNode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.builder.has_children(self.current) {
            self.current = self.builder.get_node(self.current)?.children[0];
            Some(self.builder.get_node(self.current)?)
        } else {
            while let Some(parent) = self.builder.get_node(self.current)?.parent {
                let parent_node = self.builder.get_node(parent)?;
                if let Some(next_sibling) = parent_node
                    .children
                    .iter()
                    .skip_while(|&&x| x != self.current)
                    .nth(1)
                {
                    self.current = *next_sibling;
                    return Some(self.builder.get_node(self.current)?);
                } else {
                    self.current = parent;
                }
            }
            None
        }
    }
}

pub enum WidgetInteractionType {
    Click,
}
pub struct WidgetBuilder {
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
            text: None,
            interaction: None,
            id: ROOT_NODE_ID,
        };
        widget_nodes.insert(root_node.id, root_node);

        Self {
            cursor_pos: content_area.top_left(),
            widget_nodes,
            node_ptr: ROOT_NODE_ID,
        }
    }

    pub fn root_node(&self) -> &WidgetNode {
        self.get_node(ROOT_NODE_ID).expect("Root node not found!")
    }

    pub fn iter(&self) -> WidgetNodeIterator {
        WidgetNodeIterator {
            current: ROOT_NODE_ID,
            builder: self,
        }
    }
}

pub struct WidgetComposer<'a> {
    builder: &'a mut WidgetBuilder,
    current_node: WidgetNodeId,
}

impl<'a> WidgetComposer<'a> {
    pub fn new(builder: &'a mut WidgetBuilder, current_node: WidgetNodeId) -> Self {
        Self {
            builder,
            current_node,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.current_node().text = Some(text.into());
        self
    }

    pub fn interaction(mut self, interaction: WidgetInteractionType) -> Self {
        self.current_node().interaction = Some(interaction);
        self
    }

    pub fn widget<Widget>(self, widget: &Widget, size: SizePolicy2D) -> Self
    where
        Widget: super::widget::Widget + ?Sized,
    {
        widget.build(self.builder, size);
        self
    }

    pub fn current_node(&mut self) -> &mut WidgetNode {
        self.builder
            .get_node_mut(self.current_node)
            .expect("Failed to get current node!")
    }
}

impl Drop for WidgetComposer<'_> {
    fn drop(&mut self) {
        self.builder.pop_child();
    }
}

impl<'a> WidgetBuilder {
    pub fn child(&'a mut self, size: SizePolicy2D) -> WidgetComposer<'a> {
        let current_node = self
            .get_node(self.node_ptr)
            .expect("Failed to push child. The node pointer was not found!");
        // TODO: Add padding and margin
        let content_area = Rect::new_from_xy(
            self.cursor_pos.x,
            self.cursor_pos.y,
            match size.horizontal {
                SizePolicy::Percentage(percentage) => {
                    current_node.content_area.width() * percentage
                }
                SizePolicy::Fixed(width) => width,
            },
            match size.vertical {
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

        WidgetComposer::new(self, node_id)
    }

    pub fn pop_child(&mut self) {
        if let Some(parent) = self
            .get_node(self.node_ptr)
            .expect("Failed to pop child. The node pointer was not found!")
            .parent
        {
            self.node_ptr = parent;
            self.advance();
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
        self.cursor_pos.y += 32.;
    }
}

// Private functions
impl<'a> WidgetBuilder {
    fn new_node(&'a mut self, content_area: Rect, parent: Option<WidgetNodeId>) -> &'a WidgetNode {
        let node = WidgetNode {
            id: self.new_node_id(),
            parent: Some(parent.unwrap_or(ROOT_NODE_ID)),
            children: Vec::new(),
            content_area,
            text: None,
            interaction: None,
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

    fn get_node(&'a self, id: WidgetNodeId) -> Option<&'a WidgetNode> {
        self.widget_nodes.get(&id)
    }

    fn get_node_mut(&'a mut self, id: WidgetNodeId) -> Option<&'a mut WidgetNode> {
        self.widget_nodes.get_mut(&id)
    }

    fn has_children(&self, id: WidgetNodeId) -> bool {
        self.get_node(id)
            .map(|node| !node.children.is_empty())
            .unwrap_or(false)
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
