use self::super::error::*;
use self::super::name::*;
use self::super::rc_cell::*;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Attr` interface.
///
pub trait Attribute: Node {
    ///
    /// On retrieval, the value of the attribute is returned as a string.
    ///
    /// # Specification
    ///
    /// Character and general entity references are replaced with their values. See also the method
    /// [getAttribute](trait.Element.html#tymethod.get_attribute) on the [Element](trait.Element.html) interface.
    /// On setting, this creates a Text node with the unparsed contents of the string. I.e. any
    /// characters that an XML processor would recognize as markup are instead treated as literal
    /// text. See also the method setAttribute on the Element interface.
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    fn value(&self) -> Option<String> {
        Node::node_value(self)
    }
    ///
    /// Set the `value` for the node; see [value()](#tymethod.value).
    ///
    fn set_value(&mut self, value: &str) -> Result<()> {
        Node::set_node_value(self, value)
    }
    ///
    /// Set the `value` for the node to `None`; see [value()](#tymethod.value).
    ///
    fn unset_value(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    ///
    /// If this attribute was explicitly given a value in the original document, this is `true`;
    /// otherwise, it is `false`.
    ///
    /// # Specification
    ///
    /// Note that the implementation is in charge of this attribute, not the user. If the user
    /// changes the value of the attribute (even if it ends up having the same value as the default
    /// value) then the specified flag is automatically flipped to true. To re-specify the
    /// attribute as the default value from the DTD, the user must delete the attribute. The
    /// implementation will then make a new attribute available with specified set to false and
    /// the default value (if one exists).
    ///
    /// In summary:
    ///
    /// * If the attribute has an assigned value in the document then specified is `true`, and the
    ///   value is the assigned value.
    /// * If the attribute has no assigned value in the document and has a default value in the
    ///   DTD, then specified is `false`, and the value is the default value in the DTD.
    /// * If the attribute has no assigned value in the document and has a value of `#IMPLIED` in
    ///   the DTD, then the attribute does not appear in the structure model of the document.
    /// * If the `ownerElement` attribute is `null` (i.e. because it was just created or was set to
    ///   `null` by the various removal and cloning operations) specified is `true`.
    ///
    fn specified(&self) -> bool {
        true
    }
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `CDataSection` interface.
///
/// # Specification
///
/// CDATA sections are used to escape blocks of text containing characters that would otherwise be
/// regarded as markup. The only delimiter that is recognized in a CDATA section is the `"]]>"`
/// string that ends the CDATA section. CDATA sections cannot be nested. Their primary purpose is
/// for including material such as XML fragments, without needing to escape all the delimiters.
///
/// The `DOMString` attribute of the `Text` node holds the text that is contained by the CDATA
/// section. Note that this may contain characters that need to be escaped outside of CDATA
/// sections and that, depending on the character encoding ("charset") chosen for serialization,
/// it may be impossible to write out some characters as part of a CDATA section.
///
/// The CDATASection interface inherits from the [CharacterData](trait.CharacterData.html)
/// interface through the [Text](trait.Text.html) interface. Adjacent CDATASection nodes are not
/// merged by use of the normalize method of the [Node](trait.Node.html) interface.
///
/// **Note:** Because no markup is recognized within a CDATASection, character numeric references
/// cannot be used as an escape mechanism when serializing. Therefore, action needs to be taken
/// when serializing a CDATASection with a character encoding where some of the contained
/// characters cannot be represented. Failure to do so would not produce well-formed XML.
///
/// One potential solution in the serialization process is to end the CDATA section before the
/// character, output the character using a character reference or entity reference, and open a
/// new CDATA section for any further characters in the text node. Note, however, that some
/// code conversion libraries at the time of writing do not return an error or exception when a
/// character is missing from the encoding, making the task of ensuring that data is not corrupted
/// on serialization more difficult.
///
pub trait CDataSection: Text {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `CharacterData` interface.
///
/// # Specification
///
/// The `CharacterData` interface extends [Node](trait.Node.html) with a set of attributes and
/// methods for accessing character data in the DOM. For clarity this set is defined here rather
/// than on each object that uses these attributes and methods. No DOM objects correspond directly
/// to `CharacterData`, though [Text](trait.Text.html) and others do inherit the interface from it.
/// All offsets in this interface start from 0.
///
/// As explained in the `DOMString` interface, text strings in the DOM are represented in UTF-16,
/// i.e. as a sequence of 16-bit units. In the following, the term 16-bit units is used whenever
/// necessary to indicate that indexing on `CharacterData` is done in 16-bit units.
///
pub trait CharacterData: Node {
    ///
    /// The number of 16-bit units that are available through data and the `substringData` method
    /// below. This may have the value zero, i.e., `CharacterData` nodes may be empty.
    ///
    /// **Note:** This implementation drops the `_data` suffix from the methods for clarity.
    ///
    fn length(&self) -> usize {
        match self.data() {
            None => 0,
            Some(s) => s.len(),
        }
    }
    ///
    /// The character data of the node that implements this interface.
    ///
    /// # Specification
    ///
    /// The DOM implementation may not put arbitrary limits on the amount of data that may be
    /// stored in a `CharacterData` node. However, implementation limits may mean that the entirety
    /// of a node's data may not fit into a single `DOMString`. In such cases, the user may call
    /// `substringData` to retrieve the data in appropriately sized pieces.
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    /// **Exceptions on retrieval**
    ///
    /// * `DOMSTRING_SIZE_ERR`: Raised when it would return more characters than fit in a
    ///   `DOMString` variable on the implementation platform.
    ///
    fn data(&self) -> Option<String> {
        Node::node_value(self)
    }
    ///
    /// Set the `data` for the node; see [data()](#tymethod.data).
    ///
    fn set_data(&mut self, data: &str) -> Result<()> {
        Node::set_node_value(self, data)
    }
    ///
    /// Set the `data` for the node to `None`; see [data()](#tymethod.data).
    ///
    fn unset_data(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    ///
    /// Extracts a range of data from the node.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: Start offset of substring to extract.
    /// * `count` of type `unsigned long`: The number of 16-bit units to extract.
    ///
    /// **Return Value**
    ///
    /// * `DOMString`: The specified substring. If the sum of `offset` and `count` exceeds the
    ///   `length`, then all 16-bit units to the end of the data are returned.
    ///
    /// **Exceptions**
    ///
    /// * `INDEX_SIZE_ERR`: Raised if the specified `offset` is negative or greater than the
    ///   number of 16-bit units in data, or if the specified `count` is negative.
    /// * `DOMSTRING_SIZE_ERR`: Raised if the specified range of text does not fit into a `DOMString`.
    ///
    fn substring(&self, offset: usize, count: usize) -> Result<String>;
    ///
    /// Append the string to the end of the character data of the node.
    ///
    /// # Specification
    ///
    /// Upon success, data provides access to the concatenation of data and the DOMString specified.
    ///
    /// **Parameters**
    ///
    /// * `arg` of type `DOMString`: The DOMString to append.
    ///
    /// **Exceptions**
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn append(&mut self, data: &str) -> Result<()>;
    ///
    /// Insert a string at the specified 16-bit unit offset.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: The character offset at which to insert.
    /// * `arg` of type `DOMString`: The DOMString to insert.
    ///
    /// **Exceptions**
    ///
    /// * `INDEX_SIZE_ERR`: Raised if the specified `offset` is negative or greater than the number
    ///   of 16-bit units in data.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn insert(&mut self, offset: usize, data: &str) -> Result<()>;
    ///
    /// Remove a range of 16-bit units from the node. Upon success, data and length reflect the change.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: The offset from which to start removing.
    /// * `count` of type `unsigned long`: The number of 16-bit units to delete. If the sum of
    ///   `offset` and `count` exceeds `length` then all 16-bit units from offset to the end of
    ///   the data are deleted.
    ///
    /// **Exceptions**
    ///
    /// * `INDEX_SIZE_ERR`: Raised if the specified `offset` is negative or greater than the number
    ///   of 16-bit units in data, or if the specified `count` is negative.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn delete(&mut self, offset: usize, count: usize) -> Result<()>;
    ///
    /// Replace the characters starting at the specified 16-bit unit offset with the specified string.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: The offset from which to start replacing.
    /// * `count` of type `unsigned long`: The number of 16-bit units to replace. If the sum of
    ///   `offset` and `count` exceeds `length`, then all 16-bit units to the end of the data are
    ///   replaced; (i.e., the effect is the same as a remove method call with the same range,
    ///   followed by an append method invocation).
    /// * `arg` of type `DOMString`: The `DOMString` with which the range must be replaced.
    /// Exceptions
    ///
    /// INDEX_SIZE_ERR: Raised if the specified `offset` is negative or greater than the number
    ///   of 16-bit units in data, or if the specified `count` is negative.
    /// NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.
    ///
    fn replace(&mut self, offset: usize, count: usize, data: &str) -> Result<()>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Comment` interface.
///
/// # Specification
///
/// This interface inherits from [CharacterData](trait.CharacterData.html) and represents the
/// content of a comment, i.e., all the characters between the starting `'<!--'` and ending `'-->'`.
/// Note that this is the definition of a comment in XML, and, in practice, HTML, although some
/// HTML tools may implement the full SGML comment structure.
///
pub trait Comment: CharacterData {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Document` interface.
///
pub trait Document: Node {
    ///
    /// The Document Type Declaration (see [DocumentType](trait.DocumentType.html)) associated with
    /// this document.
    ///
    /// # Specification
    ///
    /// For HTML documents as well as XML documents without a document type
    /// declaration this returns `null`. The DOM Level 2 does not support editing the Document Type
    /// Declaration. `docType` cannot be altered in any way, including through the use of methods
    /// inherited from the [Node](trait.Node.html) interface, such as `insertNode` or `removeNode`.
    ///
    fn doc_type(&self) -> Option<RefNode>;
    ///
    /// This is a convenience attribute that allows direct access to the child node that is the
    /// root element of the document.
    ///
    /// # Specification
    ///
    /// For HTML documents, this is the element with the tagName `"HTML"`.
    ///
    fn document_element(&self) -> Option<RefNode>;
    ///
    /// The DOMImplementation object that handles this document.
    ///
    /// # Specification
    ///
    /// A DOM application may use objects from multiple implementations.
    ///
    fn implementation(&self) -> &Implementation;
    fn create_attribute(&self, name: &str) -> Result<RefNode>;
    fn create_attribute_with(&self, name: &str, value: &str) -> Result<RefNode>;
    fn create_attribute_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode>;
    fn create_cdata_section(&self, data: &str) -> Result<RefNode>;
    fn create_comment(&self, data: &str) -> RefNode;
    fn create_element(&self, tag_name: &str) -> Result<RefNode>;
    fn create_element_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode>;
    fn create_processing_instruction(&self, target: &str, data: Option<&str>) -> Result<RefNode>;
    fn create_text_node(&self, data: &str) -> RefNode;
    fn get_element_by_id(&self, id: &str) -> Option<RefNode>;
    fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<RefNode>;
    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, local_name: &str) -> Vec<RefNode>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DocumentFragment` interface (current unsupported).
///
/// # Specification
///
/// `DocumentFragment` is a "lightweight" or "minimal" [Document](trait.Document.html) object. It
/// is very common to want to be able to extract a portion of a document's tree or to create a new
/// fragment of a document. Imagine implementing a user command like cut or rearranging a document
/// by moving fragments around. It is desirable to have an object which can hold such fragments and
/// it is quite natural to use a Node for this purpose. While it is true that a `Document` object
/// could fulfill this role, a `Document` object can potentially be a heavyweight object, depending
/// on the underlying implementation. What is really needed for this is a very lightweight object.
/// `DocumentFragment` is such an object.
///
/// Furthermore, various operations -- such as inserting nodes as children of another Node -- may
/// take `DocumentFragment` objects as arguments; this results in all the child nodes of the
/// `DocumentFragment` being moved to the child list of this node.
///
/// The children of a `DocumentFragment` node are zero or more nodes representing the tops of any
/// sub-trees defining the structure of the document. `DocumentFragment` nodes do not need to be
/// well-formed XML documents (although they do need to follow the rules imposed upon well-formed
/// XML parsed entities, which can have multiple top nodes). For example, a `DocumentFragment`
/// might have only one child and that child node could be a [Text](trait.Text.html) node. Such a
/// structure model represents neither an HTML document nor a well-formed XML document.
///
/// When a `DocumentFragment` is inserted into a `Document` (or indeed any other `Node` that may
/// take children) the children of the `DocumentFragment` and not the `DocumentFragment` itself are
/// inserted into the `Node`. This makes the `DocumentFragment` very useful when the user wishes
/// to create nodes that are siblings; the `DocumentFragment` acts as the parent of these nodes
/// so that the user can use the standard methods from the [Node](trait.Node.html) interface, such as
/// `insertBefore` and `appendChild`.
///
pub trait DocumentFragment: Node {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DocumentType` interface.
///
/// # Specification
///
/// Each [Document](trait.Document.html) has a `doctype` attribute whose value is either `null` or
/// a `DocumentType` object. The `DocumentType` interface in the DOM Core provides an interface
/// to the list of entities that are defined for the document, and little else because the effect
/// of namespaces and the various XML schema efforts on DTD representation are not clearly
/// understood as of this writing.
///
/// The DOM Level 2 doesn't support editing `DocumentType` nodes.
///
pub trait DocumentType: Node {
    /// The public identifier of the external subset.
    fn public_id(&self) -> Option<String>;
    /// The system identifier of the external subset.
    fn system_id(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Element` interface.
///
pub trait Element: Node {
    fn get_attribute(&self, name: &str) -> Option<String>;
    fn set_attribute(&mut self, name: &str, value: &str) -> Result<()>;
    fn remove_attribute(&mut self, _name: &str) -> Result<()>;
    fn get_attribute_node(&self, name: &str) -> Option<RefNode>;
    fn set_attribute_node(&mut self, _new_attribute: RefNode) -> Result<RefNode>;
    fn remove_attribute_node(&mut self, _old_attribute: RefNode) -> Result<RefNode>;
    fn get_elements_by_tag_name(&self, _tag_name: &str) -> Vec<RefNode>;
    fn get_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<String>;
    fn set_attribute_ns(
        &mut self,
        namespace_uri: &str,
        qualified_name: &str,
        value: &str,
    ) -> Result<()>;
    fn remove_attribute_ns(&mut self, _namespace_uri: &str, _local_name: &str) -> Result<()>;
    fn get_attribute_node_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<RefNode>;
    fn set_attribute_node_ns(&mut self, _new_attribute: RefNode) -> Result<RefNode>;
    fn get_elements_by_tag_name_ns(&self, _namespace_uri: &str, _local_name: &str) -> Vec<RefNode>;
    fn has_attribute(&self, name: &str) -> bool;
    fn has_attribute_ns(&self, namespace_uri: &str, local_name: &str) -> bool;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Entity` interface (currently unsupported).
///
/// # Specification
///
/// This interface represents an entity, either parsed or unparsed, in an XML document. Note that
/// this models the entity itself not the entity declaration. Entity declaration modeling has
/// been left for a later Level of the DOM specification.
///
/// The nodeName attribute that is inherited from Node contains the name of the entity.
///
/// An XML processor may choose to completely expand entities before the structure model is passed
/// to the DOM; in this case there will be no `EntityReference` nodes in the document tree.
///
/// XML does not mandate that a non-validating XML processor read and process entity declarations
/// made in the external subset or declared in external parameter entities. This means that parsed
/// entities declared in the external subset need not be expanded by some classes of applications,
/// and that the replacement value of the entity may not be available. When the replacement value
/// is available, the corresponding `Entity` node's child list represents the structure of that
/// replacement text. Otherwise, the child list is empty.
///
/// The DOM Level 2 does not support editing `Entity` nodes; if a user wants to make changes to
/// the contents of an `Entity`, every related `EntityReference` node has to be replaced in the
/// structure model by a clone of the `Entity`'s contents, and then the desired changes must be
/// made to each of those clones instead. `Entity` nodes and all their descendants are readonly.
///
/// An `Entity` node does not have any parent.
///
/// **Note:** If the entity contains an unbound namespace prefix, the` namespaceURI` of the
/// corresponding node in the `Entity` node subtree is `null`. The same is true for
/// `EntityReference` nodes that refer to this entity, when they are created using the
/// `createEntityReference` method of the [Document](trait.Document.html) interface. The DOM
/// Level 2 does not support any mechanism to resolve namespace prefixes.
///
pub trait Entity: Node {
    fn public_id(&self) -> Option<String>;
    fn system_id(&self) -> Option<String>;
    fn notation_name(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `EntityReference` interface (currently unsupported).
///
/// # Specification
///
/// `EntityReference` objects may be inserted into the structure model when an entity reference
/// is in the source document, or when the user wishes to insert an entity reference. Note that
/// character references and references to predefined entities are considered to be expanded by
/// the HTML or XML processor so that characters are represented by their Unicode equivalent rather
/// than by an entity reference. Moreover, the XML processor may completely expand references to
/// entities while building the structure model, instead of providing `EntityReference` objects. If
/// it does provide such objects, then for a given `EntityReference` node, it may be that there is
/// no `Entity` node representing the referenced entity. If such an `Entity` exists, then the
/// subtree of the `EntityReference` node is in general a copy of the `Entity` node subtree.
/// However, this may not be true when an entity contains an unbound namespace prefix. In such a
/// case, because the namespace prefix resolution depends on where the entity reference is, the
/// descendants of the `EntityReference` node may be bound to different namespace URIs.
///
/// As for `Entity` nodes, `EntityReference` nodes and all their descendants are readonly.
///
pub trait EntityReference: Node {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Node` interface.
///
/// # Specification
///
/// The Node interface is the primary datatype for the entire Document Object Model. It represents
/// a single node in the document tree. While all objects implementing the Node interface expose
/// methods for dealing with children, not all objects implementing the Node interface may have
/// children. For example, Text nodes may not have children, and adding children to such nodes
/// results in a DOMException being raised.
///
/// The attributes nodeName, nodeValue and attributes are included as a mechanism to get at node
/// information without casting down to the specific derived interface. In cases where there is no
/// obvious mapping of these attributes for a specific nodeType (e.g., nodeValue for an Element or
/// attributes for a Comment), this returns null. Note that the specialized interfaces may contain
/// additional and more convenient mechanisms to get and set the relevant information.
///
/// The values of nodeName, nodeValue, and attributes vary according to the node type as follows:
///
///
/// | Interface               | nodeName                  | nodeValue                           | attributes   |
/// |-------------------------|---------------------------|-------------------------------------|--------------|
/// | `Attr`                  | name of attribute         | value of attribute                  | `None`       |
/// | `CDATASection`          | `"#cdata-section"`        | content of the CDATA Section        | `None`       |
/// | `Comment`               | `"#comment"`              | content of the comment              | `None`       |
/// | `Document`              | `"#document"`             | `None`                              | `None`       |
/// | `DocumentFragment`      | `"#document-fragment"`    | `None`                              | `None`       |
/// | `DocumentType`          | document type name        | `None`                              | `None`       |
/// | `Element`               | tag name                  | `None`                              | `HashMap`    |
/// | `Entity`                | entity name               | `None`                              | `None`       |
/// | `EntityReference`       | name of entity referenced | `None`                              | `None`       |
/// | `Notatio`n              | notation name             | `None`                              | `None`       |
/// | `ProcessingInstruction` | `target`                  | entire content excluding the target | `None`       |
/// | `Text`                  | `"#text"`                 | content of the text node            | `None`       |
///
pub trait Node {
    ///
    /// The name of this node, depending on its type; see the table above.
    ///
    fn name(&self) -> Name;
    ///
    /// The value of this node, depending on its type; see the table above. When it is defined to
    /// be `None`, setting it has no effect.
    ///
    /// # Specification
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    /// **Exceptions on retrieval**
    ///
    /// * `DOMSTRING_SIZE_ERR`: Raised when it would return more characters than fit in a DOMString
    /// variable on the implementation platform.
    ///
    fn node_value(&self) -> Option<String>;
    ///
    /// Set the `value` for the node; see [node_value()](#tymethod.node_value).
    ///
    fn set_node_value(&mut self, value: &str) -> Result<()>;
    ///
    /// Set the `value` for the node to `None`; see [node_value()](#tymethod.node_value).
    ///
    fn unset_node_value(&mut self) -> Result<()>;
    ///
    /// A code representing the type of the underlying object.
    ///
    fn node_type(&self) -> NodeType;
    ///
    /// The parent of this node. All nodes, except `Attr`, `Document`, `DocumentFragment`,
    /// `Entity`, and `Notation` may have a parent. However, if a node has just been created and not
    /// yet added to the tree, or if it has been removed from the tree, this is `None`.
    ///
    fn parent_node(&self) -> Option<RefNode>;
    ///
    /// A `Vec` that contains all children of this node. If there are no children,
    /// this is a `Vec` containing no nodes.
    ///
    fn child_nodes(&self) -> Vec<RefNode>;
    ///
    /// The first child of this node. If there is no such node, this returns `None`.
    ///
    fn first_child(&self) -> Option<RefNode>;
    ///
    /// The last child of this node. If there is no such node, this returns `None`.
    ///
    fn last_child(&self) -> Option<RefNode>;
    ///
    /// The node immediately preceding this node. If there is no such node, this returns `None`.
    ///
    fn previous_sibling(&self) -> Option<RefNode>;
    ///
    /// The node immediately following this node. If there is no such node, this returns `None`.
    ///
    fn next_sibling(&self) -> Option<RefNode>;
    ///
    /// A `HashMap` containing the attributes of this node (if it is an `Element`) or
    /// `None` otherwise.
    ///
    fn attributes(&self) -> HashMap<Name, RefNode>;
    ///
    /// The `Document` object associated with this node. This is also the `Document`
    /// object used to create new nodes. When this node is a `Document` or a `DocumentType` which is
    /// not used with any `Document` yet, this is `None`.
    ///
    fn owner_document(&self) -> Option<RefNode>;
    fn insert_before(&mut self, _new_child: RefNode, _ref_child: &RefNode) -> Result<RefNode>;
    fn replace_child(&mut self, _new_child: RefNode, _old_child: &RefNode) -> Result<RefNode>;
    ///
    /// Adds the node `newChild` to the end of the list of children of this node.
    ///
    /// # Specification
    ///
    /// If the `newChild` is already in the tree, it is first removed.
    ///
    /// **Parameters**
    ///
    /// * `newChild` of type `Node`: The node to add. If it is a `DocumentFragment` object, the
    ///   entire contents of the document fragment are moved into the child list of this node.
    ///
    /// **Return Value**
    ///
    /// `Node`: The node added.
    ///
    /// **Exceptions**
    ///
    /// * `HIERARCHY_REQUEST_ERR: Raised if this node is of a type that does not allow children of`
    ///   the type of the `newChild` node, or if the node to append is one of this node's ancestors.
    /// * `WRONG_DOCUMENT_ERR`: Raised if `newChild` was created from a different document than
    ///   the one that created this node.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn append_child(&mut self, new_child: RefNode) -> Result<RefNode>;
    fn has_child_nodes(&self) -> bool;
    fn clone_node(&self, _deep: bool) -> Option<RefNode>;
    fn normalize(&mut self);
    fn is_supported(&self, feature: String, version: String) -> bool;
    fn has_attributes(&self) -> bool;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Notation` interface (currently unsupported).
///
/// # Specification
///
/// This interface represents a notation declared in the DTD. A notation either declares, by name,
/// the format of an unparsed entity (see section 4.7 of the XML 1.0 specification), or is used
/// for formal declaration of processing instruction targets (see section 2.6 of the XML 1.0
/// specification). The `nodeName` attribute inherited from [Node](trait.Node.html) is set to the
/// declared name of the notation.
///
/// The DOM Level 1 does not support editing `Notation` nodes; they are therefore readonly.
///
/// A `Notation` node does not have any parent.
///
pub trait Notation: Node {
    ///
    /// The public identifier of this notation.
    ///
    /// # Specification
    ///
    /// If the public identifier was not specified, this is null.
    ///
    fn public_id(&self) -> Option<String>;
    ///
    /// The system identifier of this notation.
    ///
    /// # Specification
    ///
    /// If the system identifier was not specified, this is null.
    ///
    fn system_id(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `ProcessingInstruction` interface.
///
/// # Specification
///
/// The `ProcessingInstruction` interface represents a "processing instruction", used in XML as a
/// way to keep processor-specific information in the text of the document.
///
pub trait ProcessingInstruction: Node {
    ///
    /// The number of 16-bit units that are available through `data`.
    ///
    fn length(&self) -> usize {
        match self.data() {
            None => 0,
            Some(s) => s.len(),
        }
    }
    ///
    /// The content of this processing instruction.
    ///
    /// # Specification
    ///
    /// This is from the first non white space character after the target to the character
    /// immediately preceding the `'?>'`.
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    fn data(&self) -> Option<String> {
        Node::node_value(self)
    }
    ///
    /// Set the `data` for the node; see [data()](#tymethod.data).
    ///
    fn set_data(&mut self, data: &str) -> Result<()> {
        Node::set_node_value(self, data)
    }
    ///
    /// Set the `data` for the node to `None`; see [data()](#tymethod.data).
    ///
    fn unset_data(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    ///
    /// The target of this processing instruction.
    ///
    /// # Specification
    ///
    /// XML defines this as being the first token following the markup that begins the processing
    /// instruction.
    ///
    fn target(&self) -> String {
        Node::name(self).to_string()
    }
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Text` interface.
///
/// # Specification
///
/// The `Text` interface inherits from [CharacterData](trait.CharacterData.html) and represents the
/// textual content (termed character data in XML) of an [Element](trait.Element.html) or
/// [Attr](trait.Attribute.html). If there is no markup inside an element's content, the text is
/// contained in a single object implementing the `Text` interface that is the only child of the
/// element. If there is markup, it is parsed into the information items (elements, comments,
/// etc.) and `Text` nodes that form the list of children of the element.
///
/// When a document is first made available via the DOM, there is only one `Text` node for each
/// block of text. Users may create adjacent `Text` nodes that represent the contents of a given
/// element without any intervening markup, but should be aware that there is no way to represent
/// the separations between these nodes in XML or HTML, so they will not (in general) persist
/// between DOM editing sessions. The `normalize()` method on [Node](trait.Node.html) merges any
/// such adjacent `Text` objects into a single node for each block of text.
///
pub trait Text: CharacterData {
    ///
    /// Breaks this node into two nodes at the specified offset, keeping both in the tree as siblings.
    ///
    /// # Specification
    ///
    /// After being split, this node will contain all the content up to the offset point. A new
    /// node of the same type, which contains all the content at and after the offset point, is
    /// returned. If the original node had a parent node, the new node is inserted as the next
    /// sibling of the original node. When the offset is equal to the length of this node, the
    /// new node has no data.
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: The 16-bit unit offset at which to split, starting from 0.
    ///
    /// **Return Value**
    ///
    /// * `Text`: The new node, of the same type as this node.
    ///
    /// **Exceptions**
    ///
    /// * `INDEX_SIZE_ERR: Raised if the specified offset is negative or greater than the number of 16-bit units in data.
    /// * `NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.
    ///
    fn split(offset: usize) -> Result<RefNode>;
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DOMImplementation` interface.
///
#[derive(Debug)]
pub struct Implementation {}

// ------------------------------------------------------------------------------------------------

///
/// Internal DOM tree node owned reference
///
pub type RefNode = RcRefCell<NodeImpl>;

///
/// Internal DOM tree node weak reference
///
pub type WeakRefNode = WeakRefCell<NodeImpl>;

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `NodeType` set of constants.
///
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum NodeType {
    /// The node is an [Element](trait.Element.html)
    Element = 1,
    /// The node is an [Attribute](trait.Attribute.html)
    Attribute,
    /// The node is a [Text](trait.Text.html)
    Text,
    /// The node is a [CDataSection](trait.CDataSection.html)
    CData,
    /// The node is an `EntityReference`
    EntityReference,
    /// The node is an `Entity`
    Entity,
    /// The node is a [ProcessingInstruction](trait.ProcessingInstruction.html)
    ProcessingInstruction,
    /// The node is a [Comment](trait.Comment.html)
    Comment,
    /// The node is a [Document](trait.Document.html)
    Document,
    /// The node is a [DocumentType](trait.DocumentType.html)
    DocumentType,
    /// The node is a `DocumentFragment`
    DocumentFragment,
    /// The node is a `Notation`
    Notation,
}

// ------------------------------------------------------------------------------------------------

///
/// Internal container for DOM tree node data and state.
///
#[derive(Clone, Debug)]
pub struct NodeImpl {
    pub(crate) i_node_type: NodeType,
    pub(crate) i_name: Name,
    pub(crate) i_value: Option<String>,
    pub(crate) i_parent_node: Option<WeakRefNode>,
    pub(crate) i_owner_document: Option<WeakRefNode>,
    pub(crate) i_attributes: HashMap<Name, RefNode>,
    pub(crate) i_child_nodes: Vec<RefNode>,
    // for Document
    pub(crate) i_document_element: Option<RefNode>,
    pub(crate) i_document_type: Option<RefNode>,
}
