use xee_extract::{Extract, Extractor};
use xee_xpath::Documents;

#[derive(Extract, Debug, PartialEq)]
struct UserProfile {
    #[xee(xpath("//user/@id"))]
    user_id: String,

    #[xee(xpath("//user/name/text()"))]
    name: String,

    #[xee(xpath("//user/email/text()"))]
    email: String,

    #[xee(xpath("//user/role/text()"))]
    role: String,
}

#[derive(Extract, Debug, PartialEq)]
struct UserPermissions {
    #[xee(xpath("//permissions/user/@id"))]
    user_id: String,

    #[xee(xpath("//permissions/access_level/text()"))]
    access_level: String,

    #[xee(xpath("//permissions/roles/role/text()"))]
    roles: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct CrossDocumentUser {
    #[xee(xpath("//user/@id"))]
    user_id: String,

    #[xee(xpath("//user/name/text()"))]
    name: String,

    #[xee(xpath("//user/email/text()"))]
    email: String,

    #[xee(xpath("//user/role/text()"))]
    role: String,

    // Cross-document extraction using doc() function
    #[xee(xpath("let $uid := //user/@id return doc('http://example.com/permissions.xml')//permissions[@user_id = $uid]/access_level/text()"))]
    access_level: String,

    #[xee(xpath("let $uid := //user/@id return doc('http://example.com/permissions.xml')//permissions[@user_id = $uid]/roles/role/text()"))]
    permissions_roles: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct SimpleCrossDocument {
    #[xee(xpath("//user/@id"))]
    user_id: String,

    #[xee(xpath("//user/name/text()"))]
    name: String,

    // Simple cross-document extraction without variables
    #[xee(xpath("doc('http://example.com/extra.xml')/extra/value/text()"))]
    extra_value: String,
}

#[derive(Extract, Debug, PartialEq)]
struct ProductCatalog {
    #[xee(xpath("//catalog/@id"))]
    catalog_id: String,

    #[xee(xpath("//catalog/name/text()"))]
    name: String,

    #[xee(xpath("//catalog/description/text()"))]
    description: String,
}

#[derive(Extract, Debug, PartialEq)]
struct ProductInventory {
    #[xee(xpath("//inventory/@id"))]
    inventory_id: String,

    #[xee(xpath("//inventory/stock_level/text()"))]
    stock_level: i32,

    #[xee(xpath("//inventory/price/text()"))]
    price: f64,
}

#[derive(Extract, Debug, PartialEq)]
struct ProductWithInventory {
    #[xee(xpath("//product[1]/@id"))]
    product_id: String,

    #[xee(xpath("//product[1]/name/text()"))]
    name: String,

    #[xee(xpath("//product[1]/description/text()"))]
    description: String,

    #[xee(xpath("//product[1]/category/text()"))]
    category: String,

    // Cross-document extraction
    #[xee(xpath("let $pid := //product[1]/@id return doc('http://example.com/inventory.xml')//inventory[@product_id = $pid]/stock_level/text()"))]
    stock_level: i32,

    #[xee(xpath("let $pid := //product[1]/@id return doc('http://example.com/inventory.xml')//inventory[@product_id = $pid]/price/text()"))]
    price: f64,
}

#[derive(Extract, Debug, PartialEq)]
struct OrderWithCustomer {
    #[xee(xpath("//order/@id"))]
    order_id: String,

    #[xee(xpath("//order/total/text()"))]
    total: f64,

    #[xee(xpath("//order/status/text()"))]
    status: String,

    // Cross-document customer info
    #[xee(xpath("let $cid := //order/customer_id/text() return doc('http://example.com/customers.xml')//customer[@id = $cid]/name/text()"))]
    customer_name: String,

    #[xee(xpath("let $cid := //order/customer_id/text() return doc('http://example.com/customers.xml')//customer[@id = $cid]/email/text()"))]
    customer_email: String,
}

#[derive(Extract, Debug, PartialEq)]
struct LibraryBook {
    #[xee(xpath("//book[1]/@id"))]
    book_id: String,

    #[xee(xpath("//book[1]/title/text()"))]
    title: String,

    #[xee(xpath("//book[1]/author/text()"))]
    author: String,

    #[xee(xpath("//book[1]/isbn/text()"))]
    isbn: String,

    // Cross-document availability info
    #[xee(xpath("let $bid := //book[1]/@id return doc('http://example.com/availability.xml')//book[@id = $bid]/available/text()"))]
    available: bool,

    #[xee(xpath("let $bid := //book[1]/@id return doc('http://example.com/availability.xml')//book[@id = $bid]/location/text()"))]
    location: String,
}

#[test]
fn test_simple_cross_document_extraction() {
    let mut documents = Documents::new();

    // Add user profile document
    let user_doc = documents
        .add_string(
            "http://example.com/users.xml".try_into().unwrap(),
            r#"<users>
                <user id="user123">
                    <name>John Doe</name>
                    <email>john@example.com</email>
                    <role>developer</role>
                </user>
            </users>"#,
        )
        .unwrap();

    // Add permissions document
    documents
        .add_string(
            "http://example.com/permissions.xml".try_into().unwrap(),
            r#"<permissions>
                <permissions user_id="user123">
                    <access_level>admin</access_level>
                    <roles>
                        <role>developer</role>
                        <role>admin</role>
                        <role>reviewer</role>
                    </roles>
                </permissions>
            </permissions>"#,
        )
        .unwrap();

    let extractor = Extractor::new();
    let result: CrossDocumentUser = extractor
        .extract_from_docs(&mut documents, &user_doc)
        .unwrap();

    assert_eq!(result.user_id, "user123");
    assert_eq!(result.name, "John Doe");
    assert_eq!(result.email, "john@example.com");
    assert_eq!(result.role, "developer");
    assert_eq!(result.access_level, "admin");
    assert_eq!(
        result.permissions_roles,
        vec!["developer", "admin", "reviewer"]
    );
}

#[test]
fn test_simple_direct_cross_document_extraction() {
    let mut documents = Documents::new();

    // Add user document
    let user_doc = documents
        .add_string(
            "http://example.com/users.xml".try_into().unwrap(),
            r#"<users>
                <user id="user123">
                    <name>John Doe</name>
                </user>
            </users>"#,
        )
        .unwrap();

    // Add extra document with simple value
    documents
        .add_string(
            "http://example.com/extra.xml".try_into().unwrap(),
            r#"<extra>
                <value>additional information</value>
            </extra>"#,
        )
        .unwrap();

    let extractor = Extractor::new();
    let result: SimpleCrossDocument = extractor
        .extract_from_docs(&mut documents, &user_doc)
        .unwrap();

    assert_eq!(result.user_id, "user123");
    assert_eq!(result.name, "John Doe");
    assert_eq!(result.extra_value, "additional information");
}

#[test]
fn test_product_catalog_with_inventory() {
    let mut documents = Documents::new();

    // Add product catalog document
    let catalog_doc = documents
        .add_string(
            "http://example.com/catalog.xml".try_into().unwrap(),
            r#"<catalog>
                <product id="prod001">
                    <name>Laptop Computer</name>
                    <description>High-performance laptop</description>
                    <category>Electronics</category>
                </product>
                <product id="prod002">
                    <name>Wireless Mouse</name>
                    <description>Ergonomic wireless mouse</description>
                    <category>Accessories</category>
                </product>
            </catalog>"#,
        )
        .unwrap();

    // Add inventory document
    documents
        .add_string(
            "http://example.com/inventory.xml".try_into().unwrap(),
            r#"<inventory>
                <inventory product_id="prod001">
                    <stock_level>15</stock_level>
                    <price>1299.99</price>
                </inventory>
                <inventory product_id="prod002">
                    <stock_level>42</stock_level>
                    <price>29.99</price>
                </inventory>
            </inventory>"#,
        )
        .unwrap();

    let extractor = Extractor::new();
    let result: ProductWithInventory = extractor
        .extract_from_docs(&mut documents, &catalog_doc)
        .unwrap();

    assert_eq!(result.product_id, "prod001");
    assert_eq!(result.name, "Laptop Computer");
    assert_eq!(result.description, "High-performance laptop");
    assert_eq!(result.category, "Electronics");
    assert_eq!(result.stock_level, 15);
    assert_eq!(result.price, 1299.99);
}

#[test]
fn test_order_with_customer_info() {
    let mut documents = Documents::new();

    // Add order document
    let order_doc = documents
        .add_string(
            "http://example.com/orders.xml".try_into().unwrap(),
            r#"<orders>
                <order id="order456">
                    <customer_id>cust789</customer_id>
                    <total>149.99</total>
                    <status>pending</status>
                </order>
            </orders>"#,
        )
        .unwrap();

    // Add customers document
    documents
        .add_string(
            "http://example.com/customers.xml".try_into().unwrap(),
            r#"<customers>
                <customer id="cust789">
                    <name>Jane Smith</name>
                    <email>jane@example.com</email>
                    <phone>555-1234</phone>
                </customer>
            </customers>"#,
        )
        .unwrap();

    let extractor = Extractor::new();
    let result: OrderWithCustomer = extractor
        .extract_from_docs(&mut documents, &order_doc)
        .unwrap();

    assert_eq!(result.order_id, "order456");
    assert_eq!(result.total, 149.99);
    assert_eq!(result.status, "pending");
    assert_eq!(result.customer_name, "Jane Smith");
    assert_eq!(result.customer_email, "jane@example.com");
}

#[test]
fn test_library_book_with_availability() {
    let mut documents = Documents::new();

    // Add books document
    let books_doc = documents
        .add_string(
            "http://example.com/books.xml".try_into().unwrap(),
            r#"<books>
                <book id="book001">
                    <title>The Rust Programming Language</title>
                    <author>Steve Klabnik</author>
                    <isbn>978-1593278281</isbn>
                </book>
                <book id="book002">
                    <title>Programming Rust</title>
                    <author>Jim Blandy</author>
                    <isbn>978-1491927281</isbn>
                </book>
            </books>"#,
        )
        .unwrap();

    // Add availability document
    documents
        .add_string(
            "http://example.com/availability.xml".try_into().unwrap(),
            r#"<availability>
                <book id="book001">
                    <available>true</available>
                    <location>Main Library - Floor 2</location>
                </book>
                <book id="book002">
                    <available>false</available>
                    <location>Main Library - Floor 2</location>
                </book>
            </availability>"#,
        )
        .unwrap();

    let extractor = Extractor::new();
    let result: LibraryBook = extractor
        .extract_from_docs(&mut documents, &books_doc)
        .unwrap();

    assert_eq!(result.book_id, "book001");
    assert_eq!(result.title, "The Rust Programming Language");
    assert_eq!(result.author, "Steve Klabnik");
    assert_eq!(result.isbn, "978-1593278281");
    assert_eq!(result.available, true);
    assert_eq!(result.location, "Main Library - Floor 2");
}

#[test]
fn test_multiple_documents_with_complex_relationships() {
    let mut documents = Documents::new();

    // Add departments document
    let dept_doc = documents
        .add_string(
            "http://example.com/departments.xml".try_into().unwrap(),
            r#"<departments>
                <department id="dept001">
                    <name>Engineering</name>
                    <manager_id>emp123</manager_id>
                    <location>Building A</location>
                </department>
                <department id="dept002">
                    <name>Marketing</name>
                    <manager_id>emp456</manager_id>
                    <location>Building B</location>
                </department>
            </departments>"#,
        )
        .unwrap();

    // Add employees document
    documents
        .add_string(
            "http://example.com/employees.xml".try_into().unwrap(),
            r#"<employees>
                <employee id="emp123">
                    <name>Alice Johnson</name>
                    <title>Engineering Manager</title>
                    <salary>120000</salary>
                </employee>
                <employee id="emp456">
                    <name>Bob Wilson</name>
                    <title>Marketing Director</title>
                    <salary>110000</salary>
                </employee>
            </employees>"#,
        )
        .unwrap();

    // Add projects document
    documents
        .add_string(
            "http://example.com/projects.xml".try_into().unwrap(),
            r#"<projects>
                <project id="proj001" department_id="dept001">
                    <name>Rust Compiler Optimization</name>
                    <budget>50000</budget>
                    <status>active</status>
                </project>
                <project id="proj002" department_id="dept002">
                    <name>Brand Campaign 2024</name>
                    <budget>75000</budget>
                    <status>planning</status>
                </project>
            </projects>"#,
        )
        .unwrap();

    // Define a struct for complex cross-document extraction
    #[derive(Extract, Debug, PartialEq)]
    struct DepartmentWithDetails {
        #[xee(xpath("//department[1]/@id"))]
        dept_id: String,

        #[xee(xpath("//department[1]/name/text()"))]
        name: String,

        #[xee(xpath("//department[1]/location/text()"))]
        location: String,

        // Cross-document manager info
        #[xee(xpath("let $mid := //department[1]/manager_id/text() return doc('http://example.com/employees.xml')//employee[@id = $mid]/name/text()"))]
        manager_name: String,

        #[xee(xpath("let $mid := //department[1]/manager_id/text() return doc('http://example.com/employees.xml')//employee[@id = $mid]/title/text()"))]
        manager_title: String,

        // Cross-document project info
        #[xee(xpath("let $did := //department[1]/@id return doc('http://example.com/projects.xml')//project[@department_id = $did]/name/text()"))]
        project_names: Vec<String>,
    }

    let extractor = Extractor::new();
    let result: DepartmentWithDetails = extractor
        .extract_from_docs(&mut documents, &dept_doc)
        .unwrap();

    assert_eq!(result.dept_id, "dept001");
    assert_eq!(result.name, "Engineering");
    assert_eq!(result.location, "Building A");
    assert_eq!(result.manager_name, "Alice Johnson");
    assert_eq!(result.manager_title, "Engineering Manager");
    assert_eq!(result.project_names, vec!["Rust Compiler Optimization"]);
}

#[test]
fn test_error_handling_for_missing_document() {
    let mut documents = Documents::new();

    // Add only the user document, but try to reference a non-existent permissions document
    let user_doc = documents
        .add_string(
            "http://example.com/users.xml".try_into().unwrap(),
            r#"<users>
                <user id="user123">
                    <name>John Doe</name>
                    <email>john@example.com</email>
                    <role>developer</role>
                </user>
            </users>"#,
        )
        .unwrap();

    let extractor = Extractor::new();
    let result: Result<CrossDocumentUser, _> =
        extractor.extract_from_docs(&mut documents, &user_doc);

    // Should fail because the permissions document doesn't exist
    assert!(result.is_err());
}

#[test]
fn test_document_without_uri_cannot_be_accessed_via_doc() {
    let mut documents = Documents::new();

    // Add a document without URI
    let user_doc = documents
        .add_string_without_uri(
            r#"<users>
                <user id="user123">
                    <name>John Doe</name>
                    <email>john@example.com</email>
                    <role>developer</role>
                </user>
            </users>"#,
        )
        .unwrap();

    // Add a document with URI that tries to reference the document without URI
    documents
        .add_string(
            "http://example.com/permissions.xml".try_into().unwrap(),
            r#"<permissions>
                <permissions user_id="user123">
                    <access_level>admin</access_level>
                </permissions>
            </permissions>"#,
        )
        .unwrap();

    // Define a struct that tries to access the document without URI
    #[derive(Extract, Debug, PartialEq)]
    struct UserWithPermissions {
        #[xee(xpath("//user/@id"))]
        user_id: String,

        #[xee(xpath("//user/name/text()"))]
        name: String,

        // This should fail because the user document has no URI
        #[xee(xpath("let $uid := //user/@id return doc('http://example.com/users.xml')//user[@id = $uid]/name/text()"))]
        cross_ref_name: String,
    }

    let extractor = Extractor::new();
    let result: Result<UserWithPermissions, _> =
        extractor.extract_from_docs(&mut documents, &user_doc);

    // Should fail because the user document has no URI and can't be accessed via doc()
    assert!(result.is_err());
}
