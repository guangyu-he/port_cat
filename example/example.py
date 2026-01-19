import asyncio
from port_cat import connect_mode, scan_mode


async def scan():
    return await scan_mode(
        host="192.168.178.1",
        scan_range="20-1024",
    )


if __name__ == "__main__":
    a = connect_mode(ports=[5432], host="10.96.0.20", timeout=10)
    b = asyncio.run(scan())
    pass
