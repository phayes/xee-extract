use xee_extract::{Extract, ExtractError, Extractor};

#[derive(Extract, Debug, PartialEq)]
struct SimpleVariableStruct {
    #[xee(xpath("$user_name"))]
    user_name: String,

    #[xee(xpath("$user_age"))]
    user_age: i32,

    #[xee(xpath("$is_active"))]
    is_active: bool,

    #[xee(xpath("$score"))]
    score: f64,
}

#[derive(Extract, Debug, PartialEq)]
struct PartialXPathStruct {
    #[xee(xpath("//user[@id = $user_id]/name/text()"))]
    user_name: String,

    #[xee(xpath("//user[@id = $user_id]/age/text()"))]
    user_age: i32,

    #[xee(xpath("//user[@id = $user_id]/@status"))]
    user_status: String,

    #[xee(xpath("//user[@id = $user_id]/scores/score[@type = $score_type]/text()"))]
    specific_score: f64,
}

#[derive(Extract, Debug, PartialEq)]
struct ComplexVariableStruct {
    #[xee(xpath("$company_name"))]
    company_name: String,

    #[xee(xpath("$employee_count"))]
    employee_count: u32,

    #[xee(xpath("$revenue"))]
    revenue: f64,

    #[xee(xpath("$is_public"))]
    is_public: bool,

    #[xee(xpath("$metadata_name"))]
    metadata_name: String,

    #[xee(xpath("$metadata_description"))]
    metadata_description: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct ComplexPartialXPathStruct {
    #[xee(xpath("//company[@id = $company_id]/name/text()"))]
    company_name: String,

    #[xee(xpath("//company[@id = $company_id]/employees[@dept = $department]/text()"))]
    dept_employee_count: i32,

    #[xee(xpath("//company[@id = $company_id]/revenue[@year = $fiscal_year]/text()"))]
    yearly_revenue: f64,

    #[xee(xpath("//company[@id = $company_id]/@status"))]
    company_status: String,

    #[xee(xpath("//company[@id = $company_id]/metadata[@type = $metadata_type]/text()"))]
    metadata_value: String,
}

#[derive(Extract, Debug, PartialEq)]
struct NestedVariableStruct {
    #[xee(xpath("$api_base_url"))]
    api_base_url: String,

    #[xee(xpath("$api_version"))]
    api_version: String,

    #[xee(xpath("$db_name"))]
    db_name: String,

    #[xee(xpath("$db_port"))]
    db_port: u16,
}

#[derive(Extract, Debug, PartialEq)]
struct ConditionalVariableStruct {
    #[xee(xpath("$user_id"))]
    user_id: String,

    #[xee(xpath("if ($is_admin) then 'admin' else 'user'"))]
    role: String,

    #[xee(xpath("if ($has_permission) then 'granted' else 'denied'"))]
    permission: String,

    #[xee(xpath("$default_language"))]
    default_language: String,
}

#[derive(Extract, Debug, PartialEq)]
struct ConditionalPartialXPathStruct {
    #[xee(xpath("//user[@id = $user_id]/@role"))]
    user_role: String,

    #[xee(xpath("//user[@id = $user_id]/permissions[@type = $permission_type]/@status"))]
    permission_status: String,

    #[xee(xpath("//user[@id = $user_id]/settings[@lang = $default_language]/@active"))]
    language_active: bool,

    #[xee(xpath("//user[@id = $user_id]/access[@level = 'admin']/@granted"))]
    access_granted: bool,
}

#[derive(Extract, Debug, PartialEq)]
struct VectorVariableStruct {
    #[xee(xpath("$numbers"))]
    numbers: Vec<i32>,

    #[xee(xpath("$strings"))]
    strings: Vec<String>,

    #[xee(xpath("$booleans"))]
    booleans: Vec<bool>,

    #[xee(xpath("$floats"))]
    floats: Vec<f64>,
}

#[derive(Extract, Debug, PartialEq)]
struct OptionalVariableStruct {
    #[xee(xpath("$required_field"))]
    required_field: String,

    #[xee(xpath("$optional_field"))]
    optional_field: Option<String>,

    #[xee(xpath("$optional_number"))]
    optional_number: Option<i32>,

    #[xee(xpath("$optional_bool"))]
    optional_bool: Option<bool>,
}

#[test]
fn test_basic_variable_binding() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default()
        .bind_value("user_name", "John Doe")
        .bind_value("user_age", 30)
        .bind_value("is_active", true)
        .bind_value("score", 95.5);

    let result: SimpleVariableStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.user_name, "John Doe");
    assert_eq!(result.user_age, 30);
    assert_eq!(result.is_active, true);
    assert_eq!(result.score, 95.5);
}

#[test]
fn test_partial_xpath_variable_binding() {
    let xml = r#"
        <root>
            <users>
                <user id="123" status="active">
                    <name>John Doe</name>
                    <age>30</age>
                    <scores>
                        <score type="math">95.5</score>
                        <score type="science">88.2</score>
                    </scores>
                </user>
                <user id="456" status="inactive">
                    <name>Jane Smith</name>
                    <age>25</age>
                    <scores>
                        <score type="math">92.1</score>
                        <score type="science">94.7</score>
                    </scores>
                </user>
            </users>
        </root>
    "#;

    let extractor = Extractor::default()
        .bind_value("user_id", "123")
        .bind_value("score_type", "math");

    let result: PartialXPathStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.user_name, "John Doe");
    assert_eq!(result.user_age, 30);
    assert_eq!(result.user_status, "active");
    assert_eq!(result.specific_score, 95.5);
}

#[test]
fn test_complex_variable_binding() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default()
        .bind_value("company_name", "Tech Corp")
        .bind_value("employee_count", 500)
        .bind_value("revenue", 1000000.0)
        .bind_value("is_public", false)
        .bind_value("metadata_name", "Tech Corp")
        .bind_value("metadata_description", "Leading technology company");

    let result: ComplexVariableStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.company_name, "Tech Corp");
    assert_eq!(result.employee_count, 500);
    assert_eq!(result.revenue, 1000000.0);
    assert_eq!(result.is_public, false);
    assert_eq!(result.metadata_name, "Tech Corp");
    assert_eq!(
        result.metadata_description,
        Some("Leading technology company".to_string())
    );
}

#[test]
fn test_complex_partial_xpath_variable_binding() {
    let xml = r#"
        <root>
            <companies>
                <company id="C001" status="active">
                    <name>Tech Corp</name>
                    <employees dept="engineering">150</employees>
                    <employees dept="marketing">75</employees>
                    <employees dept="sales">200</employees>
                    <revenue year="2023">1000000.0</revenue>
                    <revenue year="2024">1200000.0</revenue>
                    <metadata type="industry">Technology</metadata>
                    <metadata type="founded">2010</metadata>
                </company>
                <company id="C002" status="inactive">
                    <name>Old Corp</name>
                    <employees dept="engineering">50</employees>
                    <revenue year="2023">500000.0</revenue>
                    <metadata type="industry">Manufacturing</metadata>
                </company>
            </companies>
        </root>
    "#;

    let extractor = Extractor::default()
        .bind_value("company_id", "C001")
        .bind_value("department", "engineering")
        .bind_value("fiscal_year", "2023")
        .bind_value("metadata_type", "industry");

    let result: ComplexPartialXPathStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.company_name, "Tech Corp");
    assert_eq!(result.dept_employee_count, 150);
    assert_eq!(result.yearly_revenue, 1000000.0);
    assert_eq!(result.company_status, "active");
    assert_eq!(result.metadata_value, "Technology");
}

#[test]
fn test_nested_variable_binding() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default()
        .bind_value("api_base_url", "https://api.example.com")
        .bind_value("api_version", "v1.0")
        .bind_value("db_name", "production_db")
        .bind_value("db_port", 5432);

    let result: NestedVariableStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.api_base_url, "https://api.example.com");
    assert_eq!(result.api_version, "v1.0");
    assert_eq!(result.db_name, "production_db");
    assert_eq!(result.db_port, 5432);
}

#[test]
fn test_conditional_variable_binding() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default()
        .bind_value("user_id", "user123")
        .bind_value("is_admin", true)
        .bind_value("has_permission", false)
        .bind_value("default_language", "en");

    let result: ConditionalVariableStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.user_id, "user123");
    assert_eq!(result.role, "admin");
    assert_eq!(result.permission, "denied");
    assert_eq!(result.default_language, "en");
}

#[test]
fn test_conditional_partial_xpath_variable_binding() {
    let xml = r#"
        <root>
            <users>
                <user id="user123" role="admin">
                    <permissions type="read" status="granted"/>
                    <permissions type="write" status="denied"/>
                    <settings lang="en" active="true"/>
                    <settings lang="es" active="false"/>
                    <access level="admin" granted="true"/>
                    <access level="user" granted="false"/>
                </user>
                <user id="user456" role="user">
                    <permissions type="read" status="granted"/>
                    <permissions type="write" status="granted"/>
                    <settings lang="en" active="true"/>
                    <access level="admin" granted="false"/>
                    <access level="user" granted="true"/>
                </user>
            </users>
        </root>
    "#;

    let extractor = Extractor::default()
        .bind_value("user_id", "user123")
        .bind_value("permission_type", "read")
        .bind_value("default_language", "en")
        .bind_value("is_admin", true);

    let result: ConditionalPartialXPathStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.user_role, "admin");
    assert_eq!(result.permission_status, "granted");
    assert_eq!(result.language_active, true);
    assert_eq!(result.access_granted, true);
}

#[test]
fn test_optional_variable_binding() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default()
        .bind_value("required_field", "always present")
        .bind_value("optional_field", "present")
        .bind_value("optional_number", 42)
        .bind_value("optional_bool", true);

    let result: OptionalVariableStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.required_field, "always present");
    assert_eq!(result.optional_field, Some("present".to_string()));
    assert_eq!(result.optional_number, Some(42));
    assert_eq!(result.optional_bool, Some(true));
}

#[test]
fn test_different_data_types() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default()
        .bind_value("user_name", "John Doe")
        .bind_value("user_age", 30i32)
        .bind_value("user_age_long", 30i64)
        .bind_value("user_age_unsigned", 30u32)
        .bind_value("score", 95.5f64)
        .bind_value("score_single", 95.5f32)
        .bind_value("is_active", true)
        .bind_value("is_student", false);

    // Test that different numeric types work
    let result: SimpleVariableStruct = extractor.extract_from_str(xml).unwrap();
    assert_eq!(result.user_age, 30);
}

#[test]
fn test_variable_binding_with_named_extraction() {
    let xml = r#"
        <root>
            <data>
                <value>test_value</value>
            </data>
        </root>
    "#;

    let extractor = Extractor::default()
        .bind_value("prefix", "test_")
        .bind_value("suffix", "_value");

    #[derive(Extract, Debug, PartialEq)]
    struct NamedExtractionStruct {
        #[xee(xpath("concat($prefix, 'data', $suffix)"))]
        combined_value: String,
    }

    let result: NamedExtractionStruct = extractor.extract_from_str(xml).unwrap();
    assert_eq!(result.combined_value, "test_data_value");
}

#[test]
fn test_variable_binding_error_handling() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default().bind_value("user_name", "John Doe");
    // Missing required variables

    let result = extractor.extract_from_str::<SimpleVariableStruct>(xml);
    assert!(result.is_err());

    if let Err(ExtractError { error, .. }) = result {
        // Should be a field extraction error for missing variables
        assert!(matches!(
            error,
            xee_extract::Error::FieldExtract(_) | xee_extract::Error::SpannedError(_)
        ));
    }
}

#[test]
fn test_variable_binding_with_complex_xpath() {
    let xml = r#"
        <root>
            <users>
                <user id="1" name="Alice"/>
                <user id="2" name="Bob"/>
                <user id="3" name="Charlie"/>
            </users>
        </root>
    "#;

    let extractor = Extractor::default()
        .bind_value("min_id", 2)
        .bind_value("max_id", 3)
        .bind_value("search_name", "Bob");

    #[derive(Extract, Debug, PartialEq)]
    struct ComplexXPathStruct {
        #[xee(xpath("count(//user[@id >= $min_id and @id <= $max_id])"))]
        count_in_range: i32,

        #[xee(xpath("//user[@name = $search_name]/@id"))]
        found_user_id: String,

        #[xee(xpath("//user[@id = $min_id]/@name"))]
        min_user_name: String,
    }

    let result: ComplexXPathStruct = extractor.extract_from_str(xml).unwrap();
    assert_eq!(result.count_in_range, 2);
    assert_eq!(result.found_user_id, "2");
    assert_eq!(result.min_user_name, "Bob");
}

#[test]
fn test_variable_binding_with_functions_and_partial_xpath() {
    let xml = r#"
        <root>
            <products>
                <product id="P001" category="electronics" price="299.99">
                    <name>Laptop</name>
                    <specs>
                        <spec type="cpu">Intel i7</spec>
                        <spec type="ram">16GB</spec>
                        <spec type="storage">512GB SSD</spec>
                    </specs>
                    <reviews>
                        <review rating="5">Great laptop!</review>
                        <review rating="4">Good performance</review>
                        <review rating="3">Average</review>
                    </reviews>
                </product>
                <product id="P002" category="books" price="19.99">
                    <name>Programming Book</name>
                    <specs>
                        <spec type="pages">400</spec>
                        <spec type="language">English</spec>
                    </specs>
                    <reviews>
                        <review rating="5">Excellent book!</review>
                        <review rating="4">Very helpful</review>
                    </reviews>
                </product>
            </products>
        </root>
    "#;

    let extractor = Extractor::default()
        .bind_value("product_id", "P001")
        .bind_value("spec_type", "cpu")
        .bind_value("min_rating", 4)
        .bind_value("category_filter", "electronics");

    #[derive(Extract, Debug, PartialEq)]
    struct FunctionXPathStruct {
        #[xee(xpath("//product[@id = $product_id and @category = $category_filter]/name/text()"))]
        product_name: String,

        #[xee(xpath("//product[@id = $product_id]/specs/spec[@type = $spec_type]/text()"))]
        spec_value: String,

        #[xee(xpath(
            "count(//product[@id = $product_id]/reviews/review[@rating >= $min_rating])"
        ))]
        high_rating_count: i32,

        #[xee(xpath("//product[@id = $product_id]/@price"))]
        product_price: String,

        #[xee(xpath("//product[@id = $product_id]/reviews/review[@rating = max(//product[@id = $product_id]/reviews/review/@rating)]/text()"))]
        highest_rating_review: String,
    }

    let result: FunctionXPathStruct = extractor.extract_from_str(xml).unwrap();
    assert_eq!(result.product_name, "Laptop");
    assert_eq!(result.spec_value, "Intel i7");
    assert_eq!(result.high_rating_count, 2);
    assert_eq!(result.product_price, "299.99");
    assert_eq!(result.highest_rating_review, "Great laptop!");
}

#[test]
fn test_variable_binding_with_string_operations() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default()
        .bind_value("first_name", "John")
        .bind_value("last_name", "Doe")
        .bind_value("separator", " ")
        .bind_value("title", "Mr.");

    #[derive(Extract, Debug, PartialEq)]
    struct StringOpsStruct {
        #[xee(xpath("concat($first_name, $separator, $last_name)"))]
        full_name: String,

        #[xee(xpath("concat($title, ' ', $first_name, ' ', $last_name)"))]
        formal_name: String,

        #[xee(xpath("string-length($first_name)"))]
        first_name_length: i32,
    }

    let result: StringOpsStruct = extractor.extract_from_str(xml).unwrap();
    assert_eq!(result.full_name, "John Doe");
    assert_eq!(result.formal_name, "Mr. John Doe");
    assert_eq!(result.first_name_length, 4);
}

#[test]
fn test_variable_binding_with_numeric_operations() {
    let xml = r#"<root></root>"#;

    let extractor = Extractor::default()
        .bind_value("base_price", 100.0)
        .bind_value("tax_rate", 0.08)
        .bind_value("discount", 0.10)
        .bind_value("quantity", 3);

    #[derive(Extract, Debug, PartialEq)]
    struct NumericOpsStruct {
        #[xee(xpath("$base_price * $quantity"))]
        subtotal: f64,

        #[xee(xpath("$base_price * $quantity * $tax_rate"))]
        tax_amount: f64,

        #[xee(xpath("$base_price * $quantity * (1 - $discount)"))]
        discounted_total: f64,

        #[xee(xpath("round($base_price * $quantity * (1 + $tax_rate))"))]
        total_with_tax: f64,
    }

    let result: NumericOpsStruct = extractor.extract_from_str(xml).unwrap();
    assert_eq!(result.subtotal, 300.0);
    assert_eq!(result.tax_amount, 24.0);
    assert_eq!(result.discounted_total, 270.0);
    assert_eq!(result.total_with_tax, 324.0);
}
