# @version ^0.3.0
@external
def test(addr: address) -> bool:
    return addr.is_contract
