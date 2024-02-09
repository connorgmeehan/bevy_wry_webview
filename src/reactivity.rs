use crate::*;

pub struct WebViewReactivityPlugin;

impl Plugin for WebViewReactivityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::on_webview_reposition_resize,
                Self::on_webview_redirect,
                Self::on_window_resize,
            ),
        );
    }
}

impl WebViewReactivityPlugin {
    fn on_webview_reposition_resize(
        registry: NonSendMut<WebViewRegistry>,
        query: Query<
            (&WebViewHandle, &GlobalTransform, &Node),
            (With<WebViewMarker>, Changed<GlobalTransform>),
        >,
    ) {
        for (handle, position, node) in query.iter() {
            let size = node.size();
            handle.map(|x| {
                registry.get(x).map(|webview| {
                    webview.set_bounds(wry::Rect {
                        x: (position.translation().x - size.x / 2.0) as i32,
                        y: (position.translation().y - size.y / 2.0) as i32,
                        width: size.x as u32,
                        height: size.y as u32,
                    });
                })
            });
        }
    }

    fn on_webview_redirect(
        registry: NonSendMut<WebViewRegistry>,
        query: Query<
            (&WebViewHandle, &WebViewLocation),
            (With<WebViewMarker>, Changed<WebViewLocation>),
        >,
    ) {
        for (handle, location) in query.iter() {
            handle.map(|x| {
                registry.get(x).map(|webview| match location {
                    WebViewLocation::Url(url) => webview.load_url(url),
                    WebViewLocation::Html(_html) => {
                        // TODO Implement HTML loading past builder
                    }
                })
            });
        }
    }

    fn on_window_resize(
        e: EventReader<WindowResized>,
        registry: NonSendMut<WebViewRegistry>,
        query: Query<(&WebViewHandle, &Node, &GlobalTransform), With<WebViewHandle>>,
    ) {
        if !e.is_empty() {
            for (handle, size, position) in &query {
                let size = size.size();
                let final_position = (
                    (position.translation().x - size.x / 2.0) as i32,
                    (position.translation().y - size.y / 2.0) as i32,
                );
                handle.map(|x| registry.get(x)).flatten().map(|webview| {
                    webview.set_bounds(wry::Rect {
                        x: final_position.0,
                        y: final_position.1,
                        width: size.x as u32,
                        height: size.y as u32,
                    });
                });
            }
        }
    }
}
