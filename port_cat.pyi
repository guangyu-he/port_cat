from typing import List


class ConnectionInfo:
    host: str
    port: int
    timeout: int
    recv_buffer_size: int
    send_buffer_size: int
    keepalive: bool
    reuse_address: bool
    remote_ip: str
    remote_port: int
    service: str | None


def connect_mode(
        ports: List[int],
        host: str,
        timeout: int = 5,
) -> List[ConnectionInfo]:
    ...
