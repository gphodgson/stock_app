slint::include_modules!();
use polygon_service::service::PolygonService;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let service = PolygonService{
        client: PolygonService::create_client().unwrap()
    };

    let test_value = service.get_ticker_data("AAPL").await;
    let test2 = test_value.err().unwrap().to_string();

    let ui = MainWindow::new()?;
    ui.set_winHeight(50);
    ui.set_winWidth(300);

    ui.on_create_chart_window({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_winHeight(ui.get_winHeight() + 100);
        }
    });

    ui.run()
}
