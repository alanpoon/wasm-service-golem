use html_to_string_macro::html;
use crate::NewItem;
use crate::bindings::exports::golem::template::api::*;
use uuid::Uuid;
use chrono::{DateTime, NaiveDateTime, Utc};

pub fn add2(item:NewItem )->Result<String, String>{
    let title = item.title.trim();
        
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        let deadline = crate::unix_time_from_option_string(item.deadline)?;

        let id = Uuid::new_v4().to_string();

        let now = Utc::now().timestamp();

        let item = Item {
            id,
            title: title.to_string(),
            priority: item.priority,
            deadline,
            status: Status::Backlog,
            created_timestamp: now,
            updated_timestamp: now,
        };

        println!("New item created: {:?}", item);

        let result = item.clone();

        crate::with_state(|state| {
            state.items.insert(item.id.clone(), item);
        });
    let id = result.id;
    Ok(html! {
        <li hx-target="this" hx-swap="outerHTML">
            <input type="checkbox" { if result.status==Status::Done { "checked" } else { "" }} hx-post={ format!("./todos/{id}/toggle") }/>
            <label>{ result.title.as_str() }</label>
            <button class="delete" hx-delete={ format!("./todos/{id}") }></button>
        </li>
    })
}