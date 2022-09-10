pub fn get_systemd_service(
    app_name: &str,
    service_name: &str,
    user: &str,
    host: &str,
    port: u16,
) -> String {
    format!(
        r#"[Unit]
Description={app_name} {service_name}
After=network.target
StartLimitIntervalSec=0

[Service]
User={user}
Type=simple
Restart=always
RestartSec=1
WorkingDirectory=/home/{user}/{app_name}
ExecStart=/usr/bin/bash -c 'cargo run --bin {service_name} --release --host={host} --port={port} --config=etc/config.json'

StandardError=file:/home/{user}/{app_name}/log/{app_name}_{service_name}.log
StandardOutput=file:/home/{user}/{app_name}/log/{app_name}_{service_name}.log
StandardInput=null

[Install]
WantedBy=default.target

"#,
        app_name = app_name,
        service_name = service_name,
        user = user,
        host = host,
        port = port
    )
}
