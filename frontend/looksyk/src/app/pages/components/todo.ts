export function isTodoTodoBlock(x: string): boolean {
  return x.startsWith("[ ] ");
}

export function isTodoDoneBlock(x: string): boolean {
  return x.startsWith("[x] ");
}

export function computeNewTodoState(x: Todo, originalText: string) {
  if (x.isChecked) {
    return replaceAt(originalText, 1, " ");
  } else {
    return replaceAt(originalText, 1, "x");
  }
}

export function chopTodo(x: string): string {
  return x.substring(4);
}


export function replaceAt(string: string, index: number, replacement: string) {
  return string.substring(0, index) + replacement + string.substring(index + replacement.length);
}

export interface Todo {
  isTodo: boolean,
  isChecked: boolean
}

export const TODO_TODO: Todo = {
  isChecked: false,
  isTodo: true
}

export const TODO_DONE: Todo = {
  isChecked: true,
  isTodo: true
}
