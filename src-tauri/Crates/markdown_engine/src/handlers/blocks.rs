use crate::ElementHandler;
// use html5ever::Attribute;
use crate::MarkdownConverter;
use markup5ever_rcdom::{Handle, NodeData};

pub struct DivHandler;
impl ElementHandler for DivHandler {
	fn handle(
		&self,
		converter: &MarkdownConverter,
		node: &Handle,
		attrs: &[html5ever::Attribute],
		output: &mut String,
		depth: usize,
	) {
		// Check if the parent node is a list item
		let is_within_list_item = if let Some(parent) = node.parent.take() {
			if let NodeData::Element { name, .. } = &parent.upgrade().unwrap().data {
				name.local.as_ref() == "li"
			} else {
				false
			}
		} else {
			false
		};

		if is_within_list_item {
			// If within a list item, avoid adding extra newlines
			converter.walk_children(node, output, depth);
		} else {
			// Standard paragraph handling with newlines

			// Check if it's an editor-line div
			let is_editor_line = attrs.iter().any(|attr| {
				attr.name.local.as_ref() == "class" && attr.value.as_ref().contains("editor-line")
			});

			if is_editor_line {
				output.push('\n');
			}

			converter.walk_children(node, output, depth);

			// if is_editor_line {
			//     output.push('\n');
			// }
		}
	}
}

pub struct ParagraphHandler;
impl ElementHandler for ParagraphHandler {
	fn handle(
		&self,
		converter: &MarkdownConverter,
		node: &Handle,
		_attrs: &[html5ever::Attribute],
		output: &mut String,
		depth: usize,
	) {
		// Check if the parent node is a list item
		let is_within_list_item = if let Some(parent) = node.parent.take() {
			if let NodeData::Element { name, .. } = &parent.upgrade().unwrap().data {
				name.local.as_ref() == "li"
			} else {
				false
			}
		} else {
			false
		};

		if is_within_list_item {
			// If within a list item, avoid adding extra newlines
			converter.walk_children(node, output, depth);
		} else {
			// Standard paragraph handling with newlines
			output.push_str("\n");
			converter.walk_children(node, output, depth);
			output.push_str("\n");
		}
	}
}

pub struct BlockquoteHandler;
impl ElementHandler for BlockquoteHandler {
	fn handle(
		&self,
		converter: &MarkdownConverter,
		node: &Handle,
		_attrs: &[html5ever::Attribute],
		output: &mut String,
		depth: usize,
	) {
		// Add newline before blockquote if not at start
		// if !output.is_empty() && !output.ends_with('\n') {
		output.push('\n');
		// }

		// Create a temporary buffer for the quote content
		let mut quote_content = String::new();
		converter.walk_children(node, &mut quote_content, depth + 1);

		// Process the quote content line by line
		for line in quote_content.trim().lines() {
			output.push_str("> ");
			output.push_str(line);
			output.push('\n');
		}

		// Add extra newline after blockquote(optional)
		// output.push('\n');
	}
}

pub struct CodeBlockHandler;
impl ElementHandler for CodeBlockHandler {
	fn handle(
		&self,
		converter: &MarkdownConverter,
		node: &Handle,
		_attrs: &[html5ever::Attribute],
		output: &mut String,
		depth: usize,
	) {
		// Get the code element which should be the child
		if let Some(code_node) = node.children.borrow().first() {
			if let NodeData::Element { attrs, .. } = &code_node.data {
				output.push_str("\n```");

				// Extract language if present
				for attr in attrs.borrow().iter() {
					if attr.name.local.as_ref() == "class" {
						if let Some(lang) = attr.value.as_ref().strip_prefix("language-") {
							output.push_str(lang);
							break;
						}
					}
				}
				output.push('\n');

				// Process the actual code content
				converter.walk_children(code_node, output, depth);
				output.push_str("\n```\n");
			}
		}
	}
}
