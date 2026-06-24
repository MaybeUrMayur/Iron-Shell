use sysinfo::{System};

pub fn get_welcome_screen() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    
    // Convert uptime to hours and mins
    let uptime_secs = System::uptime();
    let uptime_hours = uptime_secs / 3600;
    let uptime_mins = (uptime_secs % 3600) / 60;
    let uptime_str = format!("{} hours, {} mins", uptime_hours, uptime_mins);

    // CPU info
    let cpu_name = sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Unknown CPU");
    
    // Mem info
    let total_mem = sys.total_memory() as f64 / 1_073_741_824.0;
    let used_mem = sys.used_memory() as f64 / 1_073_741_824.0;
    let mem_percent = if sys.total_memory() > 0 { (sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0) as u64 } else { 0 };

    let logo = vec![
        r#"   ______          "#,
        r#"  |  ____|         "#,
        r#"  | |__            "#,
        r#"  |  __|           "#,
        r#"  | |              "#,
        r#"  |_|              "#,
        r#"                   "#,
        r#"                   "#,
    ];

    let info = vec![
        format!("OS      {} {}", os_name, os_version),
        format!("HOST    {}", host_name),
        format!("UPTIME  {}", uptime_str),
        format!("SHELL   iron_shell 0.1.0"),
        String::new(),
        format!("SYSTEM STATUS"),
        format!("CPU     {}", cpu_name),
        format!("MEM     {:.2} GiB / {:.2} GiB ({}%)", used_mem, total_mem, mem_percent),
    ];

    let mut output = String::new();
    output.push('\n');
    for i in 0..std::cmp::max(logo.len(), info.len()) {
        let l = if i < logo.len() { logo[i] } else { "                   " };
        let i_str = if i < info.len() { &info[i] } else { "" };
        output.push_str(&format!("{}   {}\n", l, i_str));
    }
    
    output.push('\n');
    output.push_str("  » \"This is our world now... the world of the electron and the switch, the beauty of the baud.\" - The Mentor\n");
    output.push_str("  ● ● ● ● ● ● ●\n\n");
    
    output
}
