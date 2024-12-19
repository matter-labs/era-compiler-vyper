@external
def f():
    result: address = create_minimal_proxy_to(convert(0x42, address))
    return