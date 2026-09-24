```
`static 全局内存
`a `b 续命 到最后一个出现改续命符的地方为止 标记这个值继续活不被清除

↓
Heap + ownership

dyn Trait
↓
动态多态

Box<dyn Trait>
↓
Heap 上的 Trait Object

Arc<dyn Trait>
↓
多个线程/任务共享 Trait Object

你使用标准 QUIC v1，仍然需要 TLS。
quic总是需要tls
wireguard 跨云私网
Tailscale wireguard管理层

apt install wireguard
private key
sudo wg genkey | sudo tee /etc/wireguard/server_private.key
iJvVQFKwZ89VahXnlSANHx+FvAP/N+1Eo24pPHOwpUE=

public key
sudo cat /etc/wireguard/server_private.key | wg pubkey | sudo tee /etc/wireguard/server_public.key
15kF4pYubG7sZm79T6lKv/6uGbiP3cKu2ovNlbczDnE=

sudo sh -c 'umask 077; wg genkey > /etc/wireguard/server_private.key'
sudo cat /etc/wireguard/server_private.key | wg pubkey | sudo tee /etc/wireguard/server_public.key

查看
sudo cat /etc/wireguard/server_public.key

sudo nano /etc/wireguard/wg0.conf

查看ipv6 是否有人用
ip -6 addr | grep -i 'fd10:10:10::1'
fd20:20:20::2


运行后添加ipv6
sudo route -n add -inet6 fd20:20:20::2 -interface utun7


启动 wg



