use befu_macros::command;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct HelloResponse {
    pub message: String,
}

/// Returns a greeting for the given name.
#[command]
pub(crate) fn hello(name: String) -> HelloResponse {
    HelloResponse { message: format!("Hello {name}") }
}

// ── Complex struct support (Phase 6) ────────────────────────────

/// A geographic coordinate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoCoord {
    pub lat: f64,
    pub lng: f64,
}

/// A single address with nested geo data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
    pub coords: GeoCoord,
}

/// A user profile with nested structs and collections.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfile {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub active: bool,
    pub tags: Vec<String>,
    pub scores: Vec<f64>,
    pub address: Address,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// Accepts a complex nested struct and echoes it back with computed fields.
#[command(name = "user.profile.echo")]
pub(crate) fn user_profile_echo(profile: UserProfile) -> UserProfileEchoResponse {
    let tag_count = profile.tags.len();
    let avg_score = if profile.scores.is_empty() {
        0.0
    } else {
        profile.scores.iter().sum::<f64>() / profile.scores.len() as f64
    };

    UserProfileEchoResponse {
        profile,
        computed: ComputedFields { tag_count, avg_score },
    }
}

/// Response containing the echoed profile plus server-computed fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputedFields {
    pub tag_count: usize,
    pub avg_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfileEchoResponse {
    pub profile: UserProfile,
    pub computed: ComputedFields,
}

/// Accepts a Vec of nested items and returns aggregate stats.
#[command(name = "data.aggregate")]
pub(crate) fn data_aggregate(items: Vec<DataItem>) -> AggregateResponse {
    let count = items.len();
    let total: f64 = items.iter().map(|i| i.value).sum();
    let categories: Vec<String> = {
        let mut cats: Vec<String> = items.iter().map(|i| i.category.clone()).collect();
        cats.sort();
        cats.dedup();
        cats
    };

    AggregateResponse {
        count,
        total,
        average: if count > 0 { total / count as f64 } else { 0.0 },
        categories,
        items,
    }
}

/// A single data item with nested tags.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataItem {
    pub label: String,
    pub value: f64,
    pub category: String,
    pub nested_tags: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AggregateResponse {
    pub count: usize,
    pub total: f64,
    pub average: f64,
    pub categories: Vec<String>,
    pub items: Vec<DataItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hello_text() {
        assert_eq!(hello("Developer".to_string()).message, "Hello Developer");
    }

    #[test]
    fn user_profile_echo_computes_fields() {
        let profile = UserProfile {
            id: 1,
            name: "Alice".into(),
            email: "alice@example.com".into(),
            active: true,
            tags: vec!["rust".into(), "wasm".into(), "mobile".into()],
            scores: vec![95.0, 87.5, 92.0],
            address: Address {
                street: "123 Main St".into(),
                city: "Portland".into(),
                country: "US".into(),
                coords: GeoCoord { lat: 45.5152, lng: -122.6784 },
            },
            metadata: std::collections::HashMap::from([
                ("role".into(), serde_json::json!("engineer")),
                ("level".into(), serde_json::json!(3)),
            ]),
        };

        let resp = user_profile_echo(profile.clone());
        assert_eq!(resp.profile, profile);
        assert_eq!(resp.computed.tag_count, 3);
        assert!((resp.computed.avg_score - 91.5).abs() < f64::EPSILON);
    }

    #[test]
    fn data_aggregate_computes_stats() {
        let items = vec![
            DataItem {
                label: "A".into(),
                value: 10.0,
                category: "x".into(),
                nested_tags: vec![vec!["a1".into(), "a2".into()]],
            },
            DataItem {
                label: "B".into(),
                value: 20.0,
                category: "y".into(),
                nested_tags: vec![vec!["b1".into()], vec!["b2".into(), "b3".into()]],
            },
        ];

        let resp = data_aggregate(items);
        assert_eq!(resp.count, 2);
        assert!((resp.total - 30.0).abs() < f64::EPSILON);
        assert!((resp.average - 15.0).abs() < f64::EPSILON);
        assert_eq!(resp.categories, vec!["x", "y"]);
    }
}
