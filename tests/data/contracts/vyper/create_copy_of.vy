@external
def f():
    result: address = create_copy_of(convert(0x42, address))
    return