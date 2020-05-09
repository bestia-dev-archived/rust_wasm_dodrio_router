//! router_mod - A simple `#`-fragment local router for dodrio vdom and html templating
//! This is the trait module. It is a lib crate.
//! It does not know anything about the data model of the project.
//! That is abstracted away with field get/set methods to implement.
//! All the implementation for a project are isolated in the
//! project module router_impl_mod.
//! I couldn't abstract it away from dodrio vdom. It is still a dependency.

//use crate::*;

use rust_wasm_websys_utils::websysmod;

use dodrio::VdomWeak;
use unwrap::unwrap;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;

/// methods for Router, some required to be implemented,
/// other with default implementation in this file
/// Traits cannot have fields. They must use access methods for every single field.
/// dodrio is used for event handling, so I must
pub trait RouterTrait {
    // region: fields get/set methods to be implemented for specific project
    fn get_file_name_to_fetch_from_dodrio(root: &mut dyn dodrio::RootRender) -> &str;
    fn set_file_name_to_fetch_from_dodrio(
        file_name_to_fetch: String,
        root: &mut dyn dodrio::RootRender,
        vdom: VdomWeak,
    ) -> String;
    fn set_fetched_file(
        resp_body_text: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static>;
    // endregion: methods to be implemented

    // region: default implementations methods
    /// Start the router.
    fn start_router(&self, vdom: VdomWeak) {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.router_data.file_name_to_fetch in sync with the `#` fragment.
        let on_hash_change = Box::new(move || {
            let location = websysmod::window().location();
            let mut short_route = unwrap!(location.hash());
            if short_route.is_empty() {
                short_route = "index".to_owned();
            }
            // websysmod::debug_write("after .hash");
            wasm_bindgen_futures::spawn_local({
                let vdom_on_next_tick = vdom.clone();
                async move {
                    let _ = vdom_on_next_tick
                        .with_component({
                            let vdom = vdom_on_next_tick.clone();
                            // Callback fired whenever the URL hash fragment changes.
                            // Keeps the rrc.router_data.file_name_to_fetch in sync with the `#` fragment.
                            move |root| {
                                let short_route = short_route.clone();
                                // If the rrc file_name_to_fetch already matches the event's
                                // short_route, then there is nothing to do (ha). If they
                                // don't match, then we need to set the rrc' file_name_to_fetch
                                // and re-render.
                                if Self::get_file_name_to_fetch_from_dodrio(root) != short_route {
                                    // the function that recognizes routes and urls
                                    let url = Self::set_file_name_to_fetch_from_dodrio(
                                        short_route,
                                        root,
                                        vdom.clone(),
                                    );
                                    // I cannot simply await here because this closure is not async
                                    spawn_local({
                                        let vdom_on_next_tick = vdom.clone();
                                        async move {
                                            //websysmod::debug_write(&format!("fetch {}", &url));
                                            let resp_body_text: String =
                                                websysmod::fetch_response(url).await;
                                            // set values in rrc is async.
                                            unwrap!(
                                                vdom_on_next_tick
                                                    .with_component({
                                                        Self::set_fetched_file(resp_body_text)
                                                    })
                                                    .await
                                            );
                                            vdom.schedule_render();
                                        }
                                    });
                                }
                            }
                        })
                        .await;
                }
            });
        });
        self.set_on_hash_change_callback(on_hash_change);
    }

    fn set_on_hash_change_callback(&self, mut on_hash_change: Box<dyn FnMut()>) {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.router_data.file_name_to_fetch in sync with the `#` fragment.
        // Call it once to handle the initial `#` fragment.
        on_hash_change();

        // Now listen for hash changes forever.
        //
        // Note that if we ever intended to unmount our app, we would want to
        // provide a method for removing this router's event listener and cleaning
        // up after ourselves.
        let on_hash_change = Closure::wrap(on_hash_change);
        websysmod::window()
            .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
            .unwrap_throw();
        on_hash_change.forget();
    }

    // endregion: generic methods

    // region: associated functions that don't need self

    /// get the first param after hash in local route after dot
    /// example #p03.1234 -> 1234
    fn get_url_param_in_hash_after_dot(short_route: &str) -> &str {
        // I cannot test associated methods. So I make a normal private fn.
        private_get_url_param_in_hash_after_dot(short_route)
    }
    // endregion: associated function
}

/// I cannot test associated methods. So I make a normal private fn.
fn private_get_url_param_in_hash_after_dot(short_route: &str) -> &str {
    let mut spl = short_route.split('.');
    unwrap!(spl.next());
    unwrap!(spl.next())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_url_param_in_hash_after_dot() {
        let x = private_get_url_param_in_hash_after_dot("#p03.1234");
        assert_eq!("1234", x);
    }
}
