/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLHeadingElementBinding;
use dom::bindings::utils::DOMString;
use dom::document::AbstractDocument;
use dom::element::HTMLHeadingElementTypeId;
use dom::htmlelement::HTMLElement;
use dom::node::{AbstractNode, Node, ScriptView};

pub enum HeadingLevel {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
}

pub struct HTMLHeadingElement {
    htmlelement: HTMLElement,
    level: HeadingLevel,
}

impl HTMLHeadingElement {
    pub fn new_inherited(localName: ~str, document: AbstractDocument, level: HeadingLevel) -> HTMLHeadingElement {
        HTMLHeadingElement {
            htmlelement: HTMLElement::new_inherited(HTMLHeadingElementTypeId, localName, document),
            level: level,
        }
    }

    pub fn new(localName: ~str, document: AbstractDocument, level: HeadingLevel) -> AbstractNode<ScriptView> {
        let element = HTMLHeadingElement::new_inherited(localName, document, level);
        Node::reflect_node(@mut element, document, HTMLHeadingElementBinding::Wrap)
    }
}

impl HTMLHeadingElement {
    pub fn Align(&self) -> DOMString {
        None
    }

    pub fn SetAlign(&mut self, _align: &DOMString) {
    }
}
