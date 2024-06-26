# @version ^0.4.0

@external
@view
def test(hash: bytes32, v: uint256, r:uint256, s:uint256) -> address:
    return ecrecover(hash, v, r, s)
