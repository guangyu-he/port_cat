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
    """
    :param ports: A list of port numbers to connect to
    :param host: The host to connect to
    :param timeout: Connection timeout in seconds, default is 5
    :return:
    """
    ...


async def scan_mode(
        host: str,
        scan_range: str = "0-65535",
) -> list[int]:
    """
    :param host: The host to scan
    :param scan_range: The range of ports to scan, default is "0-65535"
    :return:
    """
    ...
