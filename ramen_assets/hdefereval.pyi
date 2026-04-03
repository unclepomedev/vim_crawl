from typing import Callable, TypeVar

_T = TypeVar("_T")

def executeInMainThreadWithResult(func: Callable[[], _T]) -> _T: ...
