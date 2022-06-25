from typing import List, TypedDict, Optional, Union, Tuple, Dict
from dataclasses import dataclass, field
import json


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
        print(json.dumps(self.tasks))

    def add_task(self, name: str, total: Optional[int] = None) -> None:
        """Enqueue a task"""
        self.tasks.append({"name": name, "progress": False})
        if total is not None:
            self._totals[name] = total
        self._output()

    def start(self) -> None:
        """Start the next task"""
        task = self.tasks[self.task_counter]
        if task["name"] in self._totals:
            self.tasks[self.task_counter]["progress"] = [0, self._totals[task["name"]]]
        else:
            self.tasks[self.task_counter]["progress"] = None
        self._output()

    def increment(self) -> None:
        """Increment a bar task"""
        task = self.tasks[self.task_counter]
        assert isinstance(task["progress"], list)
        if task["progress"][0] >= task["progress"][1]:
            raise ValueError("Maxed out")
        task["progress"][0] += 1
        self._output()

    def finish(self) -> None:
        """Mark a task as finished"""
        task = self.tasks[self.task_counter]
        task["progress"] = True
        if task["name"] in self._totals:
            del self._totals[task["name"]]
        self.task_counter += 1
        self._output()

    def error(self, message: str) -> None:
        """Mark a task as errored with a reason"""
        task = self.tasks[self.task_counter]
        task["progress"] = message
        if task["name"] in self._totals:
            del self._totals[task["name"]]
        self.task_counter += 1
        self._output()


__version__ = "0.1.0"
