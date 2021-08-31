// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wry-resize-click-propagation")
        .with_decorations(false)
        .build(&event_loop)
        .unwrap();

        let index_html = r#"
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
          </head>
          <body>
            <div>Window moves on resize</div>
            <script>
              document.addEventListener('mousedown', () => {
                  console.log('mousedown')
              })
              document.addEventListener('mouseup', () => {
                console.log('mouseup')
              })
              document.addEventListener('click', () => {
                console.log('click')
              })
            </script>
          </body>
        </html>"#;

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_html(index_html)
        .unwrap()
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
