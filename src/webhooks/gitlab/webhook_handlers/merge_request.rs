use actix_web::web;
use serde::Deserialize;
use ureq::serde_json;

#[derive(Debug, Deserialize)]
struct MergeRequestEvent {
    user: User,
    object_attributes: MergeRequestDetails,
}

#[derive(Debug, Deserialize)]
struct MergeRequestDetails {
    title: String,
    url: String,
    source_branch: String,
    target_branch: String,
    action: String,
}

#[derive(Debug, Deserialize)]
struct User {
    name: String,
}

// TODO also implement environment name

pub fn handle_merge_request_event(body: &web::Bytes) -> String {
    let merge_request_event: MergeRequestEvent = serde_json::from_slice(body).unwrap();
    let merge_request_details = &merge_request_event.object_attributes;
    let url = &merge_request_details.url;
    let title = &merge_request_details.title;
    let source_branch = &merge_request_details.source_branch;
    let target_branch = &merge_request_details.target_branch;
    let action = &merge_request_details.action;
    let sender = &merge_request_event.user.name;

    if action == "open" {
        format!(
            "<b>{sender}</b> opened a new merge request <a href=\"{url}\">{title}</a> \
             from <code>{source_branch}</code> to <code>{target_branch}</code>\n",
        )
    } else if action == "update" {
        format!(
            "<b>{sender}</b> updated merge request <a href=\"{url}\">{title}</a> \
             from <code>{source_branch}</code> to <code>{target_branch}</code>\n",
        )
    } else if action == "merge" {
        format!(
            "<b>{sender}</b> merged merge request <a href=\"{url}\">{title}</a> \
             from <code>{source_branch}</code> to <code>{target_branch}</code>\n",
        )
    } else if action == "close" {
        format!(
            "<b>{sender}</b> closed merge request <a href=\"{url}\">{title}</a> \
             from <code>{source_branch}</code> to <code>{target_branch}</code>\n",
        )
    } else if action == "reopen" {
        format!(
            "<b>{sender}</b> reopened merge request <a href=\"{url}\">{title}</a> \
             from <code>{source_branch}</code> to <code>{target_branch}</code>\n",
        )
    } else {
        String::new()
    }
}
