use mongodb::{Client, bson::{doc, oid::ObjectId}};
use crate::models::menu::{Menu, MenuItem, MenuItemChild};

pub async fn sync_configuration() {
    let client = Client::with_uri_str("mongodb://root:password@localhost:27017")
        .await
        .unwrap();
    let db = client.database("configuration");

    let menu_collection = db.collection::<Menu>("menu");
    let children_collection = db.collection::<MenuItemChild>("menuItemsChildren");

    // Cleanup static menus
    menu_collection.delete_many(doc! { "static_flag": true }).await.unwrap();
    children_collection.delete_many(doc! { "static_flag": true }).await.unwrap();

    // Create ObjectIds
    let configuration_menu_id = ObjectId::new();
    let ai_workflow_id = ObjectId::new();
    let services_id = ObjectId::new();
    let text_analysis_id = ObjectId::new();
    let text_analysis_services_id = ObjectId::new();

    // Insert main menu
    let menu = Menu {
        id: configuration_menu_id,
        name: "Configuration".to_string(),
        static_flag: true,
        menu_items: vec![
            MenuItem { id: ai_workflow_id, name: "AI-Workflow".to_string() },
            MenuItem { id: services_id, name: "Services".to_string() },
        ],
    };
    menu_collection.insert_one(&menu).await.unwrap();
    println!("Inserted menu");

    // Insert children
    let children = vec![
        MenuItemChild {
            id: text_analysis_id,
            name: "AI-TextAnalysis".to_string(),
            parent_id: ai_workflow_id,
            static_flag: true,
        },
        MenuItemChild {
            id: text_analysis_services_id,
            name: "AI-TextAnalysis-Services".to_string(),
            parent_id: services_id,
            static_flag: true,
        },
    ];
    children_collection.insert_many(children).await.unwrap();
    println!("Inserted child menu items");
}
