//! Example 3: Custom ExtractValue
//! 
//! This example demonstrates how to implement custom ExtractValue for custom types
//! and complex data structures.

use std::str::FromStr;
use xee_extract::{Extractor, Extract};

/// Custom enum for user status
#[derive(Debug, PartialEq)]
enum UserStatus {
    Active,
    Inactive,
    Pending,
    Suspended,
}

impl FromStr for UserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "pending" => Ok(UserStatus::Pending),
            "suspended" => Ok(UserStatus::Suspended),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

// UserStatus automatically gets ExtractValue implementation via FromStr

/// Custom struct for coordinates
#[derive(Debug, PartialEq)]
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid coordinates format: {}", s));
        }
        
        let lat = parts[0].trim().parse::<f64>()
            .map_err(|_| format!("Invalid latitude: {}", parts[0]))?;
        let lon = parts[1].trim().parse::<f64>()
            .map_err(|_| format!("Invalid longitude: {}", parts[1]))?;
        
        Ok(Coordinates { latitude: lat, longitude: lon })
    }
}

// Coordinates automatically gets ExtractValue implementation via FromStr

/// Custom struct for date range
#[derive(Debug, PartialEq)]
struct DateRange {
    start_date: String,
    end_date: String,
}

impl FromStr for DateRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" to ").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid date range format: {}", s));
        }
        
        Ok(DateRange {
            start_date: parts[0].trim().to_string(),
            end_date: parts[1].trim().to_string(),
        })
    }
}

// DateRange automatically gets ExtractValue implementation via FromStr

/// Struct using custom types
#[derive(Extract, Debug, PartialEq)]
struct UserProfile {
    #[xee(xpath("//name/text()"))]
    name: String,

    #[xee(xpath("//status/text()"))]
    status: UserStatus,

    #[xee(xpath("//location/text()"))]
    location: Option<Coordinates>,

    #[xee(xpath("//subscription_period/text()"))]
    subscription_period: Option<DateRange>,

    #[xee(xpath("//tags/tag/text()"))]
    tags: Vec<String>,
}

/// Struct for complex user data
#[derive(Extract, Debug, PartialEq)]
struct ComplexUser {
    #[xee(xpath("//user/@id"))]
    id: String,

    #[xee(xpath("//user/status/text()"))]
    status: UserStatus,

    #[xee(xpath("//user/coordinates/text()"))]
    coordinates: Coordinates,

    #[xee(xpath("//user/active_period/text()"))]
    active_period: DateRange,
}

fn main() {
    // Example 1: User profile with custom types
    let profile_xml = r#"
        <profile>
            <name>Alice Johnson</name>
            <status>active</status>
            <location>37.7749, -122.4194</location>
            <subscription_period>2023-01-01 to 2023-12-31</subscription_period>
            <tags>
                <tag>premium</tag>
                <tag>verified</tag>
            </tags>
        </profile>
    "#;

    let extractor = Extractor::new();
    let profile: UserProfile = extractor.extract_one(profile_xml).unwrap();

    println!("User profile with custom types:");
    println!("  Name: {}", profile.name);
    println!("  Status: {:?}", profile.status);
    println!("  Location: {:?}", profile.location);
    println!("  Subscription Period: {:?}", profile.subscription_period);
    println!("  Tags: {:?}", profile.tags);
    println!();

    // Example 2: Complex user with all custom types
    let complex_xml = r#"
        <user id="U123">
            <status>pending</status>
            <coordinates>40.7128, -74.0060</coordinates>
            <active_period>2023-06-01 to 2023-12-31</active_period>
        </user>
    "#;

    let user: ComplexUser = extractor.extract_one(complex_xml).unwrap();

    println!("Complex user with custom types:");
    println!("  ID: {}", user.id);
    println!("  Status: {:?}", user.status);
    println!("  Coordinates: {:?}", user.coordinates);
    println!("  Active Period: {:?}", user.active_period);
    println!();

    // Example 3: Error handling for invalid custom types
    let invalid_xml = r#"
        <profile>
            <name>Invalid User</name>
            <status>invalid_status</status>
            <location>invalid, coordinates</location>
            <subscription_period>invalid date range</subscription_period>
        </profile>
    "#;

    let result = extractor.extract_one::<UserProfile>(invalid_xml);
    
    println!("Error handling for invalid custom types:");
    match result {
        Ok(profile) => println!("  Unexpected success: {:?}", profile),
        Err(e) => println!("  Expected error: {}", e),
    }

    // Example 4: User profile with missing optional custom types
    let minimal_xml = r#"
        <profile>
            <name>Minimal User</name>
            <status>inactive</status>
            <tags>
                <tag>basic</tag>
            </tags>
        </profile>
    "#;

    let profile: UserProfile = extractor.extract_one(minimal_xml).unwrap();

    println!("User profile with missing optional custom types:");
    println!("  Name: {}", profile.name);
    println!("  Status: {:?}", profile.status);
    println!("  Location: {:?}", profile.location);
    println!("  Subscription Period: {:?}", profile.subscription_period);
    println!("  Tags: {:?}", profile.tags);
} 