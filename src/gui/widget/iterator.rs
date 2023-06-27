use std::{mem, slice::from_ref};

use crate::prelude::WidgetInstance;

pub struct WidgetIter<'a> {
    children: &'a [WidgetInstance],
    parent: Option<Box<WidgetIter<'a>>>,
}

impl<'a> WidgetIter<'a> {
    pub fn new(root: &'a WidgetInstance) -> Self {
        Self {
            children: from_ref(root),
            parent: None,
        }
    }
}

impl Default for WidgetIter<'_> {
    fn default() -> Self {
        Self {
            children: &[],
            parent: None,
        }
    }
}

impl<'a> Iterator for WidgetIter<'a> {
    type Item = &'a WidgetInstance;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(item) => {
                self.children = &self.children[1..];

                let children = item.widget().children();
                if children.len() != 0 {
                    // start iterating the child trees
                    *self = WidgetIter {
                        children,
                        parent: Some(Box::new(mem::take(self))),
                    };
                }
                Some(item)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::{Label, VStack};

    #[test]
    fn test_iterator() {
        let widget = VStack::new(vec![
            VStack::new(vec![Label::new("")]),
            VStack::new(vec![Label::new(""), Label::new("")]),
            VStack::new(vec![]),
            Label::new(""),
        ]);

        let mut label_count = 0;
        for item in widget.iter() {
            if item.type_name() == "rust_ui::gui::widget::impls::label::Label" {
                label_count += 1;
            }
        }
        assert_eq!(label_count, 4);
    }
}
