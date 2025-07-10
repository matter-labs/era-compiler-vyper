@external
def foo() -> address:
    # create the bytes of an empty vyper contract
    return raw_create(x"61000361000f6000396100036000f35f5ffd855820cd372fb85148700fa88095e3492d3f9f5beb43e555e5ff26d95f5a6adc36f8e6038000a1657679706572830004020033")