struct Todo:
    text: String[100]
    completed: bool

todos: public(Todo[100])
count: uint256

@external
def create_(_text: String[100]):
    # 2 ways to initialize a struct
    # key value mapping
    self.todos[self.count] = Todo({text: _text, completed: False})
    self.count += 1

    # initialize an empty struct and then update it
    todo: Todo = empty(Todo)
    todo.text = _text
    # todo.completed initialized to false

    self.todos[self.count] = todo
    self.count += 1

@external
@view
def get(_index: uint256) -> (String[100], bool):
    todo: Todo = self.todos[_index]
    return (todo.text, todo.completed)

@external
def update(_index: uint256, _text: String[100]):
    self.todos[_index].text = _text

@external
def toggleCompleted(_index: uint256):
    self.todos[_index].completed = not self.todos[_index].completed