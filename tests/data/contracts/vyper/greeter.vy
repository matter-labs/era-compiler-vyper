# @version ^0.4.0

greet: public(String[100])

@deploy
def __init__():
    self.greet = "Hello World"
