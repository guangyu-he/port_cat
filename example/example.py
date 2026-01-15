from port_cat import connect_mode

if __name__ == "__main__":
    a = connect_mode(ports=[80], host="www.google.com", timeout=10)
    pass
