pub fn get_systemd_service(app_name: &str, service_name: &str, user: &str, port: u16) -> String {
    format!(
        r#"[Unit]
Description={service_name}
After=network.target
StartLimitIntervalSec=0

[Service]
User={user}
Type=simple
Restart=always
RestartSec=1
ExecStart=/home/{user}/{app_name}/bin/{service_name} --port={port} --config=/home/{user}/{app_name}/etc/config.json

[Install]
WantedBy=default.target

"#,
        app_name = app_name,
        service_name = service_name,
        user = user,
        port = port
    )
}
