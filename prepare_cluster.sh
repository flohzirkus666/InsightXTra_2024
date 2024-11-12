#!/usr/bin/expect -f

spawn telnet console1.demo.netapp.com 10001
send -- "\r"
send -- "\x03"

expect "login"
send -- "admin\r"
expect "password"
send -- "Netapp1!\r"
expect "::>"
send -- "net int create -vserver Default -lif mgmt_auto -address 192.168.0.111 -netmask-length 24 -home-port e0c\r"

