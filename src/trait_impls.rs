use crate::convert::*;
use crate::dom_impl::{get_implementation, Implementation};
use crate::error::{Error, Result};
use crate::name::Name;
use crate::node_impl::*;
use crate::syntax::*;
use crate::traits::*;
use crate::{
    MSG_INDEX_ERROR, MSG_INVALID_EXTENSION, MSG_INVALID_NAME, MSG_INVALID_NODE_TYPE,
    MSG_NO_PARENT_NODE,
};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Attribute for RefNode {}

// ------------------------------------------------------------------------------------------------

impl CDataSection for RefNode {}

// ------------------------------------------------------------------------------------------------

impl CharacterData for RefNode {
    fn substring(&self, offset: usize, count: usize) -> Result<String> {
        if offset + count == offset {
            return Ok(String::new());
        }
        let ref_self = self.borrow();
        match &ref_self.i_value {
            None => {
                warn!("{}", MSG_INDEX_ERROR);
                Err(Error::IndexSize)
            }
            Some(data) => {
                if offset >= data.len() {
                    warn!("{}", MSG_INDEX_ERROR);
                    Err(Error::IndexSize)
                } else {
                    if offset + count >= data.len() {
                        Ok(data[offset..].to_string())
                    } else {
                        Ok(data[offset..offset + count].to_string())
                    }
                }
            }
        }
    }

    fn append(&mut self, new_data: &str) -> Result<()> {
        if new_data.is_empty() {
            return Ok(());
        }
        let mut mut_self = self.borrow_mut();
        Ok(match &mut_self.i_value {
            None => mut_self.i_value = Some(new_data.to_string()),
            Some(old_data) => mut_self.i_value = Some(format!("{}{}", old_data, new_data)),
        })
    }

    fn insert(&mut self, offset: usize, new_data: &str) -> Result<()> {
        if new_data.is_empty() {
            return Ok(());
        }
        self.replace(offset, 0, new_data)
    }

    fn delete(&mut self, offset: usize, count: usize) -> Result<()> {
        if offset + count == offset {
            return Ok(());
        }
        const NOTHING: &str = "";
        self.replace(offset, count, NOTHING)
    }

    fn replace(&mut self, offset: usize, count: usize, replace_data: &str) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        match &mut_self.i_value {
            None => {
                if offset + count != 0 {
                    warn!("{}", MSG_INDEX_ERROR);
                    Err(Error::IndexSize)
                } else {
                    mut_self.i_value = Some(replace_data.to_string());
                    Ok(())
                }
            }
            Some(old_data) => {
                if offset >= old_data.len() {
                    warn!("{}", MSG_INDEX_ERROR);
                    Err(Error::IndexSize)
                } else {
                    let mut new_data = old_data.clone();
                    if offset + count >= old_data.len() {
                        new_data.replace_range(offset.., replace_data);
                    } else {
                        new_data.replace_range(offset..offset + count, replace_data);
                    }
                    mut_self.i_value = Some(new_data);
                    Ok(())
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Comment for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Document for RefNode {
    fn doc_type(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        if let Extension::Document {
            i_document_type, ..
        } = &ref_self.i_extension
        {
            i_document_type.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn document_element(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        if let Extension::Document {
            i_document_element, ..
        } = &ref_self.i_extension
        {
            i_document_element.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn implementation(&self) -> &dyn DOMImplementation<NodeRef = RefNode> {
        get_implementation()
    }

    fn create_attribute(&self, name: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_attribute(self.clone().downgrade(), name, None);
        Ok(RefNode::new(node_impl))
    }

    fn create_attribute_with(&self, name: &str, value: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_attribute(self.clone().downgrade(), name, Some(value));
        Ok(RefNode::new(node_impl))
    }

    fn create_attribute_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_attribute(self.clone().downgrade(), name, None);
        Ok(RefNode::new(node_impl))
    }

    fn create_cdata_section(&self, data: &str) -> Result<RefNode> {
        let node_impl = NodeImpl::new_cdata(self.clone().downgrade(), data);
        Ok(RefNode::new(node_impl))
    }

    fn create_document_fragment(&self) -> Result<RefNode> {
        let node_impl = NodeImpl::new_document_fragment(self.clone().downgrade());
        Ok(RefNode::new(node_impl))
    }

    fn create_entity_reference(&self, name: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_entity_reference(self.clone().downgrade(), name);
        Ok(RefNode::new(node_impl))
    }

    fn create_comment(&self, data: &str) -> RefNode {
        let node_impl = NodeImpl::new_comment(self.clone().downgrade(), data);
        RefNode::new(node_impl)
    }

    fn create_element(&self, tag_name: &str) -> Result<RefNode> {
        let name = Name::from_str(tag_name)?;
        let node_impl = NodeImpl::new_element(self.clone().downgrade(), name);
        Ok(RefNode::new(node_impl))
    }

    fn create_element_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_element(self.clone().downgrade(), name);
        Ok(RefNode::new(node_impl))
    }

    fn create_processing_instruction(&self, target: &str, data: Option<&str>) -> Result<RefNode> {
        let target = Name::from_str(target)?;
        let node_impl =
            NodeImpl::new_processing_instruction(self.clone().downgrade(), target, data);
        Ok(RefNode::new(node_impl))
    }

    fn create_text_node(&self, data: &str) -> RefNode {
        let node_impl = NodeImpl::new_text(self.clone().downgrade(), data);
        RefNode::new(node_impl)
    }

    fn get_element_by_id(&self, _id: &str) -> Option<RefNode> {
        //
        // Until we are schema/data-type aware we do not know which attributes are actually XML
        // identifiers. A common, but erroneous, implementation is to look for attributes named
        // "id", we aren't going to do that. It may be possible to make that a flag on
        // implementation for those that want the lax behavior.
        //
        None
    }

    fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<RefNode> {
        //
        // Delegate this call to the document element
        //
        let ref_self = self.borrow();
        if let Extension::Document {
            i_document_element, ..
        } = &ref_self.i_extension
        {
            match i_document_element {
                None => Vec::default(),
                Some(root_node) => {
                    let root_element = as_element(root_node).expect("invalid node type");
                    root_element.get_elements_by_tag_name(tag_name)
                }
            }
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            Vec::default()
        }
    }

    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, local_name: &str) -> Vec<RefNode> {
        //
        // Delegate this call to the document element
        //
        let ref_self = self.borrow();
        if let Extension::Document {
            i_document_element, ..
        } = &ref_self.i_extension
        {
            match i_document_element {
                None => Vec::default(),
                Some(root_node) => {
                    let root_element = as_element(root_node).expect("invalid node type");
                    root_element.get_elements_by_tag_name_ns(namespace_uri, local_name)
                }
            }
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            Vec::default()
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl DocumentFragment for RefNode {}

// ------------------------------------------------------------------------------------------------

impl DocumentType for RefNode {
    fn entities(&self) -> HashMap<Name, Self::NodeRef, RandomState> {
        let ref_self = self.borrow();
        if let Extension::DocumentType { i_entities, .. } = &ref_self.i_extension {
            i_entities.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            HashMap::default()
        }
    }

    fn notations(&self) -> HashMap<Name, Self::NodeRef, RandomState> {
        let ref_self = self.borrow();
        if let Extension::DocumentType { i_notations, .. } = &ref_self.i_extension {
            i_notations.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            HashMap::default()
        }
    }

    fn public_id(&self) -> Option<String> {
        let ref_self = self.borrow();
        if let Extension::DocumentType { i_public_id, .. } = &ref_self.i_extension {
            i_public_id.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn system_id(&self) -> Option<String> {
        let ref_self = self.borrow();
        if let Extension::DocumentType { i_system_id, .. } = &ref_self.i_extension {
            i_system_id.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn internal_subset(&self) -> Option<String> {
        let ref_self = self.borrow();
        if let Extension::DocumentType {
            i_internal_subset, ..
        } = &ref_self.i_extension
        {
            i_internal_subset.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl DOMImplementation for Implementation {
    type NodeRef = RefNode;

    fn create_document(
        &self,
        namespace_uri: &str,
        qualified_name: &str,
        doc_type: Option<RefNode>,
    ) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_document(name, doc_type);
        let mut document_node = RefNode::new(node_impl);

        let element = {
            let document =
                as_document_mut(&mut document_node).expect("could not cast node to Document");
            document.create_element_ns(namespace_uri, qualified_name)?
        };

        {
            let mut mut_document = document_node.borrow_mut();
            if let Extension::Document {
                i_document_element, ..
            } = &mut mut_document.i_extension
            {
                *i_document_element = Some(element);
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                return Err(Error::InvalidState);
            }
        }

        Ok(document_node)
    }

    fn create_document_type(
        &self,
        qualified_name: &str,
        public_id: Option<&str>,
        system_id: Option<&str>,
    ) -> Result<RefNode> {
        let name = Name::from_str(qualified_name)?;
        let node_impl = NodeImpl::new_document_type(None, name, public_id, system_id);
        Ok(RefNode::new(node_impl))
    }

    fn has_feature(&self, feature: &str, version: &str) -> bool {
        ((feature == XML_FEATURE_CORE || feature == XML_FEATURE_XML) && (version == XML_FEATURE_V1)
            || (feature == XML_FEATURE_CORE && version == XML_FEATURE_V2))
    }
}

// ------------------------------------------------------------------------------------------------

impl Element for RefNode {
    fn get_attribute(&self, name: &str) -> Option<String> {
        if !is_element(self) {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            None
        } else {
            match Name::from_str(name) {
                Ok(attr_name) => {
                    let ref_self = self.borrow();
                    if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
                        match i_attributes.get(&attr_name) {
                            None => None,
                            Some(attr_node) => {
                                let attribute = attr_node.borrow();
                                match &attribute.i_value {
                                    None => None,
                                    Some(value) => Some(value.clone()),
                                }
                            }
                        }
                    } else {
                        warn!("{}", MSG_INVALID_EXTENSION);
                        None
                    }
                }
                Err(_) => {
                    warn!("{}: '{}'", MSG_INVALID_NAME, name);
                    None
                }
            }
        }
    }

    fn set_attribute(&mut self, name: &str, value: &str) -> Result<()> {
        let attr_name = Name::from_str(name)?;
        let attr_node = {
            let ref_self = &self.borrow_mut();
            let document = ref_self.i_owner_document.as_ref().unwrap();
            NodeImpl::new_attribute(document.clone(), attr_name, Some(value))
        };
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute(&mut self, _name: &str) -> Result<()> {
        if !is_element(self) {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            Ok(())
        } else {
            // TODO: deal with namespaces
            unimplemented!()
        }
    }

    fn get_attribute_node(&self, _name: &str) -> Option<RefNode> {
        if !is_element(self) {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            None
        } else {
            unimplemented!()
        }
    }

    fn set_attribute_node(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        // ENSURE: You can ONLY add attributes to an element
        if !is_element(self) || !is_attribute(&new_attribute) {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            Err(Error::InvalidState)
        } else {
            let name: Name = new_attribute.name();
            if name.is_namespace_attribute() {
                let attribute = as_attribute(&new_attribute).unwrap();
                let namespace_uri = attribute.value().unwrap();

                let as_namespaced = as_element_namespaced_mut(self).unwrap();
                let _ignore = match &name.prefix() {
                    None => as_namespaced.insert_mapping(None, &namespace_uri),
                    Some(prefix) => as_namespaced.insert_mapping(Some(prefix), &namespace_uri),
                }?;
            }
            let mut mut_self = self.borrow_mut();
            if let Extension::Element { i_attributes, .. } = &mut mut_self.i_extension {
                let _safe_to_ignore =
                    i_attributes.insert(new_attribute.name(), new_attribute.clone());
                Ok(new_attribute)
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                Err(Error::Syntax)
            }
        }
    }

    fn remove_attribute_node(&mut self, _old_attribute: RefNode) -> Result<RefNode> {
        unimplemented!()
    }

    fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<RefNode> {
        let mut results = Vec::default();
        if is_element(self) {
            let tag_name = tag_name.to_string();
            let ref_self = self.borrow();
            if tag_name_match(&ref_self.i_name.to_string(), &tag_name) {
                results.push(self.clone());
            }
            for child_node in &ref_self.i_child_nodes {
                match as_element(child_node) {
                    Ok(ref_child) => results.extend(ref_child.get_elements_by_tag_name(&tag_name)),
                    Err(_) => {
                        warn!("{}", MSG_INVALID_NODE_TYPE);
                        ()
                    }
                }
            }
        }
        results
    }

    fn get_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<String> {
        if !is_element(self) {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            None
        } else {
            unimplemented!()
        }
    }

    fn set_attribute_ns(
        &mut self,
        namespace_uri: &str,
        qualified_name: &str,
        value: &str,
    ) -> Result<()> {
        let attr_name = Name::new_ns(namespace_uri, qualified_name)?;
        let attr_node = {
            let ref_self = &self.borrow_mut();
            let document = ref_self.i_owner_document.as_ref().unwrap();
            NodeImpl::new_attribute(document.clone(), attr_name, Some(value))
        };
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute_ns(&mut self, _namespace_uri: &str, _local_name: &str) -> Result<()> {
        unimplemented!()
    }

    fn get_attribute_node_ns(&self, namespace_uri: &str, local_name: &str) -> Option<RefNode> {
        if is_element(self) {
            match Name::new_ns(namespace_uri, local_name) {
                Ok(name) => {
                    let ref_self = self.borrow();
                    if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
                        i_attributes.get(&name).map(|n| n.clone())
                    } else {
                        warn!("{}", MSG_INVALID_EXTENSION);
                        None
                    }
                }
                Err(_) => {
                    warn!("{}: '{}'", MSG_INVALID_NAME, local_name);
                    None
                }
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            None
        }
    }

    fn set_attribute_node_ns(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        self.set_attribute_node(new_attribute)
    }

    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, local_name: &str) -> Vec<RefNode> {
        let mut results = Vec::default();
        if is_element(self) {
            let namespace_uri = namespace_uri.to_string();
            let local_name = local_name.to_string();
            let ref_self = self.borrow();
            if namespaced_name_match(
                ref_self.i_name.namespace_uri(),
                &ref_self.i_name.local_name(),
                &namespace_uri,
                &local_name,
            ) {
                results.push(self.clone());
            }
            for child_node in &ref_self.i_child_nodes {
                match as_element(child_node) {
                    Ok(ref_child) => results
                        .extend(ref_child.get_elements_by_tag_name_ns(&namespace_uri, &local_name)),
                    Err(_) => {
                        warn!("{}", MSG_INVALID_NODE_TYPE);
                        ()
                    }
                }
            }
        }
        results
    }

    fn has_attribute(&self, name: &str) -> bool {
        if is_element(self) {
            match Name::from_str(name) {
                Ok(name) => {
                    let ref_self = self.borrow();
                    if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
                        i_attributes.contains_key(&name)
                    } else {
                        warn!("{}", MSG_INVALID_EXTENSION);
                        false
                    }
                }
                Err(_) => {
                    warn!("{}: '{}'", MSG_INVALID_NAME, name);
                    false
                }
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            false
        }
    }

    fn has_attribute_ns(&self, namespace_uri: &str, local_name: &str) -> bool {
        if is_element(self) {
            match Name::new_ns(namespace_uri, local_name) {
                Ok(name) => {
                    let ref_self = self.borrow();
                    if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
                        i_attributes.contains_key(&name)
                    } else {
                        warn!("{}", MSG_INVALID_EXTENSION);
                        false
                    }
                }
                Err(_) => {
                    warn!("{}: '{}'", MSG_INVALID_NAME, local_name);
                    false
                }
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            false
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Entity for RefNode {
    fn public_id(&self) -> Option<String> {
        let ref_self = self.borrow();
        if let Extension::Entity { i_public_id, .. } = &ref_self.i_extension {
            i_public_id.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn system_id(&self) -> Option<String> {
        let ref_self = self.borrow();
        if let Extension::Entity { i_system_id, .. } = &ref_self.i_extension {
            i_system_id.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn notation_name(&self) -> Option<String> {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------

impl EntityReference for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Node for RefNode {
    type NodeRef = RefNode;

    fn name(&self) -> Name {
        let ref_self = self.borrow();
        ref_self.i_name.clone()
    }

    fn node_value(&self) -> Option<String> {
        let ref_self = self.borrow();
        ref_self.i_value.clone()
    }

    fn set_node_value(&mut self, value: &str) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        mut_self.i_value = Some(value.to_string());
        Ok(())
    }

    fn unset_node_value(&mut self) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        mut_self.i_value = None;
        Ok(())
    }

    fn node_type(&self) -> NodeType {
        let ref_self = self.borrow();
        ref_self.i_node_type.clone()
    }

    fn parent_node(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => None,
            Some(node) => node.clone().upgrade(),
        }
    }

    fn child_nodes(&self) -> Vec<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_child_nodes.clone()
    }

    fn first_child(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match ref_self.i_child_nodes.first() {
            None => None,
            Some(node) => Some(node.clone()),
        }
    }

    fn last_child(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match ref_self.i_child_nodes.first() {
            None => None,
            Some(node) => Some(node.clone()),
        }
    }

    fn previous_sibling(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => {
                warn!("{}", MSG_NO_PARENT_NODE);
                None
            }
            Some(parent_node) => {
                let parent_node = parent_node.clone();
                let parent_node = parent_node.upgrade()?;
                let ref_parent = parent_node.borrow();
                match ref_parent
                    .i_child_nodes
                    .iter()
                    .position(|child| child == self)
                {
                    None => None,
                    Some(index) => {
                        if index == 0 {
                            None
                        } else {
                            let sibling = ref_parent.i_child_nodes.get(index - 1);
                            sibling.map(|n| n.clone())
                        }
                    }
                }
            }
        }
    }

    fn next_sibling(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => {
                warn!("{}", MSG_NO_PARENT_NODE);
                None
            }
            Some(parent_node) => {
                let parent_node = parent_node.clone();
                let parent_node = parent_node.upgrade()?;
                let ref_parent = parent_node.borrow();
                match ref_parent
                    .i_child_nodes
                    .iter()
                    .position(|child| child == self)
                {
                    None => None,
                    Some(index) => {
                        let sibling = ref_parent.i_child_nodes.get(index + 1);
                        sibling.map(|n| n.clone())
                    }
                }
            }
        }
    }

    fn attributes(&self) -> HashMap<Name, RefNode, RandomState> {
        let ref_self = self.borrow();
        if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
            i_attributes.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            HashMap::default()
        }
    }

    fn owner_document(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_owner_document {
            None => None,
            Some(node) => node.clone().upgrade(),
        }
    }

    fn insert_before(&mut self, new_child: RefNode, ref_child: Option<RefNode>) -> Result<RefNode> {
        match ref_child {
            None => {
                let mut_node = as_element_mut(self)?;
                mut_node.append_child(new_child)
            }
            Some(ref_child) => {
                // TODO: see `append_child` for specifics
                let mut mut_self = self.borrow_mut();
                match mut_self
                    .i_child_nodes
                    .iter()
                    .position(|child| child == &ref_child)
                {
                    None => mut_self.i_child_nodes.push(new_child.clone()),
                    Some(position) => mut_self.i_child_nodes.insert(position, new_child.clone()),
                }
                Ok(new_child)
            }
        }
    }

    fn replace_child(&mut self, _new_child: RefNode, _old_child: &RefNode) -> Result<RefNode> {
        // TODO: see `append_child` for specifics
        unimplemented!()
    }

    fn remove_child(&mut self, old_child: Self::NodeRef) -> Result<Self::NodeRef> {
        unimplemented!()
    }

    // * Document -- Element (maximum of one), ProcessingInstruction, Comment, DocumentType (maximum of one)
    // * DocumentFragment -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
    // * DocumentType -- no children
    // * EntityReference -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
    // * Element -- Element, Text, Comment, ProcessingInstruction, CDATASection, EntityReference
    // * Attr -- Text, EntityReference
    // * ProcessingInstruction -- no children
    // * Comment -- no children
    // * Text -- no children
    // * CDATASection -- no children
    // * Entity -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
    // * Notation -- no children

    fn append_child(&mut self, new_child: RefNode) -> Result<RefNode> {
        if !is_child_allowed(self, &new_child) {
            return Err(Error::HierarchyRequest);
        }
        {
            //
            // CHECK: Raise `Error::WrongDocument` if `newChild` was created from a different
            // document than the one that created this node.
            let self_parent = &self.borrow().i_parent_node;
            let child_parent = &self.borrow().i_parent_node;
            if !match (self_parent, child_parent) {
                (None, None) => true,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (Some(self_parent), Some(child_parent)) => {
                    let self_parent = self_parent.clone().upgrade().unwrap();
                    let child_parent = child_parent.clone().upgrade().unwrap();
                    &self_parent == &child_parent
                }
            } {
                return Err(Error::WrongDocument);
            }
        }
        // TODO: Check to see if it is in the tree already, if so remove it
        // update child with references
        {
            let ref_self = self.borrow();
            let mut mut_child = new_child.borrow_mut();
            mut_child.i_parent_node = Some(self.to_owned().downgrade());
            mut_child.i_owner_document = ref_self.i_owner_document.clone();
        }
        let mut mut_self = self.borrow_mut();

        // TODO: generalize, for each parent type, is child allowed

        mut_self.i_child_nodes.push(new_child.clone());

        // TODO: deal with document fragment as special case

        Ok(new_child)
    }

    fn has_child_nodes(&self) -> bool {
        !self.child_nodes().is_empty()
    }

    fn clone_node(&self, _deep: bool) -> Option<RefNode> {
        unimplemented!()
    }

    fn normalize(&mut self) {
        unimplemented!()
    }

    fn is_supported(&self, feature: &str, version: &str) -> bool {
        get_implementation().has_feature(feature, version)
    }

    fn has_attributes(&self) -> bool {
        !self.attributes().is_empty()
    }
}

// ------------------------------------------------------------------------------------------------

impl Notation for RefNode {
    fn public_id(&self) -> Option<String> {
        let ref_self = self.borrow();
        if let Extension::Notation { i_public_id, .. } = &ref_self.i_extension {
            i_public_id.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn system_id(&self) -> Option<String> {
        let ref_self = self.borrow();
        if let Extension::Notation { i_system_id, .. } = &ref_self.i_extension {
            i_system_id.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl ProcessingInstruction for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Text for RefNode {
    fn split(&mut self, offset: usize) -> Result<RefNode> {
        let new_data = {
            let text = as_character_data_mut(self)?;
            let length = text.length();
            if offset >= length {
                String::new()
            } else {
                let count = length - offset;
                let new_data = text.substring(offset, count)?;
                text.delete(offset, count)?;
                new_data
            }
        };

        let mut new_node = {
            //
            // Create a new node and adjust contents
            //
            let mut_self = self.borrow_mut();
            match mut_self.i_node_type {
                NodeType::Text => {
                    let document = mut_self.i_owner_document.as_ref().unwrap();
                    Ok(NodeImpl::new_text(document.clone(), &new_data))
                }
                NodeType::CData => {
                    let document = mut_self.i_owner_document.as_ref().unwrap();
                    Ok(NodeImpl::new_cdata(document.clone(), &new_data))
                }
                _ => {
                    warn!("{}", MSG_INVALID_NODE_TYPE);
                    Err(Error::Syntax)
                }
            }?
        };

        let ref_self = self.borrow();
        Ok(match &ref_self.i_parent_node {
            None => RefNode::new(new_node),
            Some(parent) => {
                new_node.i_parent_node = Some(parent.clone());
                let new_node = RefNode::new(new_node);
                let parent = parent.clone();
                let mut parent = parent.upgrade().unwrap();
                let parent_element = as_element_mut(&mut parent)?;
                let self_node = as_character_data(self)?;
                let _ignored =
                    parent_element.insert_before(new_node.clone(), self_node.next_sibling())?;
                new_node
            }
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for RefNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.node_type() {
            NodeType::Element => {
                let element = self as &dyn Element<NodeRef = RefNode>;
                write!(f, "{}{}", XML_ELEMENT_START_START, element.name())?;
                for attr in element.attributes().values() {
                    write!(f, " {}", attr.to_string())?;
                }
                write!(f, "{}", XML_ELEMENT_START_END)?;
                for child in element.child_nodes() {
                    write!(f, "{}", child.to_string())?;
                }
                write!(
                    f,
                    "{}{}{}",
                    XML_ELEMENT_END_START,
                    element.name(),
                    XML_ELEMENT_END_END
                )
            }
            NodeType::Attribute => {
                let attribute = self as &dyn Attribute<NodeRef = RefNode>;
                write!(
                    f,
                    "{}=\"{}\"",
                    attribute.name(),
                    attribute.value().unwrap_or(String::new())
                )
            }
            NodeType::Text => {
                let char_data = self as &dyn CharacterData<NodeRef = RefNode>;
                match char_data.data() {
                    None => Ok(()),
                    Some(data) => write!(f, "{}", data),
                }
            }
            NodeType::CData => {
                let char_data = self as &dyn CharacterData<NodeRef = RefNode>;
                match char_data.data() {
                    None => Ok(()),
                    Some(data) => write!(f, "{} {} {}", XML_CDATA_START, data, XML_CDATA_END),
                }
            }
            NodeType::ProcessingInstruction => {
                let pi = self as &dyn ProcessingInstruction<NodeRef = RefNode>;
                match pi.data() {
                    None => write!(f, "{}{}{}", XML_PI_START, pi.target(), XML_PI_END),
                    Some(data) => {
                        write!(f, "{}{} {}{}", XML_PI_START, pi.target(), data, XML_PI_END)
                    }
                }
            }
            NodeType::Comment => {
                let char_data = self as &dyn CharacterData<NodeRef = RefNode>;
                match char_data.data() {
                    None => Ok(()),
                    Some(data) => write!(f, "{}{}{}", XML_COMMENT_START, data, XML_COMMENT_END),
                }
            }
            NodeType::Document => {
                let document = self as &dyn Document<NodeRef = RefNode>;
                match document.doc_type() {
                    None => (),
                    Some(doc_type) => write!(f, "{}", doc_type)?,
                }
                for child in self.child_nodes() {
                    write!(f, "{}", child.to_string())?;
                }
                match document.document_element() {
                    None => Ok(()),
                    Some(document_element) => write!(f, "{}", document_element),
                }
            }
            NodeType::DocumentType => {
                let doc_type = self as &dyn DocumentType<NodeRef = RefNode>;
                write!(f, "{} {}", XML_DOCTYPE_START, doc_type.name())?;
                match &doc_type.public_id() {
                    None => (),
                    Some(id) => {
                        write!(f, " {} \"{}\"", XML_DOCTYPE_PUBLIC, id)?;
                    }
                }
                match &doc_type.system_id() {
                    None => (),
                    Some(id) => {
                        write!(f, " {} \"{}\"", XML_DOCTYPE_SYSTEM, id)?;
                    }
                }
                write!(f, "{}", XML_DOCTYPE_END)
            }
            _ => Ok(()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const WILD_CARD: &str = "*";

fn tag_name_match(test: &String, against: &String) -> bool {
    let wild = &WILD_CARD.to_string();
    (test == against) || test == wild || against == wild
}

fn namespaced_name_match(
    test_ns: &Option<String>,
    test_local: &String,
    against_ns: &String,
    against_local: &String,
) -> bool {
    let wild = &WILD_CARD.to_string();
    match test_ns {
        None => {
            against_ns == wild
                && ((test_local == against_local) || test_local == wild || against_local == wild)
        }
        Some(test_ns) => {
            ((test_ns == against_ns) || test_ns == wild || against_ns == wild)
                && ((test_local == against_local) || test_local == wild || against_local == wild)
        }
    }
}

fn is_child_allowed(parent: &RefNode, child: &RefNode) -> bool {
    let self_node_type = { &parent.borrow().i_node_type };
    let child_node_type = { &child.borrow().i_node_type };
    match self_node_type {
        NodeType::Element => match child_node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::CData
            | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::Attribute => match child_node_type {
            NodeType::Text | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::Text => false,
        NodeType::CData => false,
        NodeType::EntityReference => match child_node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::CData
            | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::Entity => match child_node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::CData
            | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::ProcessingInstruction => false,
        NodeType::Comment => false,
        NodeType::Document => match child_node_type {
            NodeType::Element
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::DocumentType => true,
            _ => false,
        },
        NodeType::DocumentType => false,
        NodeType::DocumentFragment => match child_node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::CData
            | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::Notation => false,
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_document_node() -> RefNode {
        get_implementation()
            .create_document("http://example.org/", "root", None)
            .unwrap()
    }

    fn make_node(document: &mut RefNode, name: &str) -> RefNode {
        let document = as_document_mut(document).unwrap();
        let element = document.create_element(name).unwrap();
        let mut document_element = document.document_element().unwrap();
        let document_element = as_element_mut(&mut document_element).unwrap();
        let result = document_element.append_child(element.clone());
        assert!(result.is_ok());
        element
    }

    #[test]
    fn test_next_sibling() {
        //
        // Setup the tree
        //
        let mut document = make_document_node();
        let mut root_node = make_node(&mut document, "element");
        {
            let root_element = as_element_mut(&mut root_node).unwrap();

            for index in 1..6 {
                let child_node = make_node(&mut document, &format!("child-{}", index));
                let _ignore = root_element.append_child(child_node.clone());
            }
        }

        {
            assert_eq!(root_node.borrow().i_child_nodes.len(), 5);
        }

        //
        // Ask for siblings
        //
        let ref_root = root_node.borrow();
        let mid_node = ref_root.i_child_nodes.get(2).unwrap();
        let ref_mid = as_element(mid_node).unwrap();
        assert_eq!(ref_mid.name().to_string(), "child-3".to_string());

        let next_node = ref_mid.next_sibling().unwrap();
        let ref_next = as_element(&next_node).unwrap();
        assert_eq!(ref_next.name().to_string(), "child-4".to_string());

        let last_node = ref_next.next_sibling().unwrap();
        let ref_last = as_element(&last_node).unwrap();
        assert_eq!(ref_last.name().to_string(), "child-5".to_string());

        let no_node = ref_last.next_sibling();
        assert!(no_node.is_none());
    }

    #[test]
    fn test_previous_sibling() {
        let mut document = make_document_node();
        //
        // Setup the tree
        //
        let mut root_node = make_node(&mut document, "element");
        {
            let root_element = as_element_mut(&mut root_node).unwrap();

            for index in 1..6 {
                let child_node = make_node(&mut document, &format!("child-{}", index));
                let _ignore = root_element.append_child(child_node.clone());
            }
        }

        {
            assert_eq!(root_node.borrow().i_child_nodes.len(), 5);
        }

        //
        // Ask for siblings
        //
        let ref_root = root_node.borrow();
        let mid_node = ref_root.i_child_nodes.get(2).unwrap();
        let ref_mid = as_element(mid_node).unwrap();
        assert_eq!(ref_mid.name().to_string(), "child-3".to_string());

        let previous_node = ref_mid.previous_sibling().unwrap();
        let ref_previous = as_element(&previous_node).unwrap();
        assert_eq!(ref_previous.name().to_string(), "child-2".to_string());

        let first_node = ref_previous.previous_sibling().unwrap();
        let ref_first = as_element(&first_node).unwrap();
        assert_eq!(ref_first.name().to_string(), "child-1".to_string());

        let no_node = ref_first.previous_sibling();
        assert!(no_node.is_none());
    }
}
