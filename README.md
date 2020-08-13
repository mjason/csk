# Castor 芯片命令行工具集
> version: 0.1

## logcat 工具

```shell
> csk logcat -p /dev/tty.wchusbserial14210 -b 115200 -l info
```
- -p: 串口端口
- -b: 波特率
- -l: 日志等级: trace, debug, info, warn, error