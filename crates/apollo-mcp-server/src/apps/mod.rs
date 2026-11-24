pub(crate) mod app;
pub(crate) mod manifest;
pub(crate) mod resource;
pub(crate) mod tool;

pub(crate) use app::App;
pub(crate) use manifest::load_from_path;

/// The query parameter that scopes a request to a single app.
pub(crate) const APP_QUERY_PARAM: &str = "app";

/// Reads the app name out of a request's query string.
///
/// The auth middleware and the tool dispatcher both decide what a request
/// targets, so they read this parameter through one function to keep those two
/// answers from drifting apart.
pub(crate) fn app_param_from_query(query: Option<&str>) -> Option<String> {
    query.and_then(|query| {
        url::form_urlencoded::parse(query.as_bytes())
            .find(|(key, _)| key == APP_QUERY_PARAM)
            .map(|(_, value)| value.into_owned())
    })
}

/// Checks that app names and app tool names are unique across `apps`.
///
/// A connection that serves every app at once resolves a resource by its app
/// name and a tool by its tool name, so a repeated name would make one of the
/// two unreachable.
pub(crate) fn ensure_unique_names(apps: &[App]) -> Result<(), String> {
    let mut app_names = std::collections::HashSet::new();
    let mut tool_owners = std::collections::HashMap::new();

    for app in apps {
        if !app_names.insert(app.name.as_str()) {
            return Err(format!(
                "apps.expose_all requires unique app names, but more than one app is named {}",
                app.name
            ));
        }
        for tool in &app.tools {
            if let Some(owner) = tool_owners.insert(tool.tool.name.as_ref(), app.name.as_str()) {
                return Err(format!(
                    "apps.expose_all requires unique app tool names, but apps {owner} and {} both define a tool named {}",
                    app.name, tool.tool.name
                ));
            }
        }
    }

    Ok(())
}
