import json
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple, TypedDict, Union


class TaskType(TypedDict):
    """
    For progress:
    General types
        False = hasn't started
        str = error
        True = finished

    Spinners
        None = in progress
    Progress bars
        (x, y) = in progress (x out of y tasks finished)
    """

    name: str
    progress: Union[str, Optional[bool], List[int]]  # , List["TaskType"]]


JsonOutput = List[TaskType]


@dataclass
class TaskManager:
    tasks: JsonOutput = field(default_factory=list)
    _totals: Dict[str, int] = field(default_factory=dict)

    task_counter: int = 0

    def _output(self) -> None:
        # TODO: A buffer that flushes at an interval
        print(json.dumps(self.tasks))

    def _current_task(self) -> TaskType:
        return self.tasks[self.task_counter]

    def set_total(self, new_total: int) -> None:
        """Set total of the current bar task"""
        task = self._current_task()
        if not isinstance(task["progress"], list):
            raise AttributeError("Only bar-type tasks may have a total")
        task["progress"][1] = new_total
        self._output()

    def add_task(self, name: str, total: Optional[int] = None) -> None:
        """Enqueue a task"""
        self.tasks.append({"name": name, "progress": False})
        if total is not None:
            self._totals[name] = total
        self._output()

    def start(self) -> None:
        """Start the next task"""
        task = self._current_task()
        if task["name"] in self._totals:
            task["progress"] = [0, self._totals[task["name"]]]
        else:
            task["progress"] = None
        self._output()

    def increment(self, by: int = 1, silent: bool = True) -> None:
        """Increment a bar task"""
        task = self._current_task()
        if not isinstance(task["progress"], list):
            if silent:
                # Or do self.finish?
                return
            raise TypeError("Task is a spinner")

        if task["progress"][0] >= task["progress"][1]:
            raise ValueError("Maxed out")
        task["progress"][0] += by
        self._output()

    def finish(self) -> None:
        """Mark a task as finished"""
        task = self._current_task()
        task["progress"] = True
        if task["name"] in self._totals:
            del self._totals[task["name"]]
        self.task_counter += 1
        self._output()

    def error(self, message: str) -> None:
        """Mark a task as errored with a reason"""
        task = self._current_task()
        task["progress"] = message
        if task["name"] in self._totals:
            del self._totals[task["name"]]
        self.task_counter += 1
        self._output()


__version__ = "0.1.0"
