use web_sys::{window, MediaQueryList};

pub struct MediaQueryService {}

impl MediaQueryService {
    pub fn watch_media_query_list(query: &str, event_type: &str, listener: &::js_sys::Function) {
        Self::get_media_query_list(query)
            .add_event_listener_with_callback(event_type, listener)
            .unwrap();
    }

    pub fn get_media_query_list(query: &str) -> MediaQueryList {
        window()
            .unwrap()
            .match_media(query)
            .expect("MediaQueryList not exported from web-sys.")
            .unwrap()
    }
}
