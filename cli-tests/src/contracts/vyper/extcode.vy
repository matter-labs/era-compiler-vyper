# @version ^0.4.0

@external
def test(addr: address) -> bool:
    return addr.is_contract
