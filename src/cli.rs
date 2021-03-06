use clap::clap_app;

pub fn matches() -> clap::ArgMatches {
    clap_app!(csk =>
        (version: "0.2")
        (author: "MJ. <tywf91@gmail.com>")
        (about: "Castor 命令行工具包")
        (@subcommand logcat => 
            (about: "日志查看器")
            (version: "0.1")
            (author: "MJ. <tywf91@gmail.com>")
            (@arg port: -p +required ... "设置串口端口地址")
            (@arg baud_rate: -b +required ... "波特率设置")
            (@arg log_level: -l ... "日志等级 默认: DEBUG")            
        )
        (@subcommand player =>
            (about: "测试播放器, 遍历文件加随机播放声音")
            (version: "0.1")
            (author: "MJ. <tywf91@gmail.com>")
            (@arg dir: -d +required ... "文件夹地址")
        )
    ).get_matches()
}