"""Logic for experimentation features"""
from collections import defaultdict
from typing import Dict, List, Tuple, TypedDict

from sympy import Matrix, symbols
from sympy.solvers.solveset import linsolve


class Scrape(TypedDict):
    total: float
    assignments: List[Tuple[str, float]]  # WEight * total ig


def get_weights(scrapes: List[Scrape]) -> Dict[str, float]:
    """Given total and assignments, find weights"""
    weight_symbols = {}
    for scrape in scrapes:
        for name, score in scrape["assignments"]:
            if name not in weight_symbols:
                weight_symbols[name] = symbols(name)
    eqs = []
    for scrape in scrapes:
        eq = defaultdict(float)
        for name, score in scrape["assignments"]:
            eq[name] += score
        eqs.append([eq[weight] for weight in weight_symbols] + [scrape["total"]])
    print(eqs)
    print(list(weight_symbols.keys()))
    output = linsolve(Matrix(eqs), list(weight_symbols.values()))
    assert len(output) == 1
    return dict(zip(weight_symbols.keys(), set(output).pop()))


def whatif(
    weights: Dict[str, float],
    prev_assignments: List[Tuple[str, float]],
    new_assignments: List[Tuple[str, float]],
) -> float:
    """Given weights and assignments, find total"""
    all_assignments = prev_assignments + new_assignments
    return sum(map(lambda x: x[1] * weights[x[0]], all_assignments)) / sum(
        weights.values()
    )


if __name__ == "__main__":
    print(whatif({"a": 0.25, "e": 0.5, "q": 0.25}, [("a", 1), ("q", 1)], [("e", 0)]))
    print(
        get_weights(
            [
                {"total": 1, "assignments": [("a", 1), ("q", 1), ("e", 1)]},
                {"total": 0.75, "assignments": [("a", 1), ("q", 0), ("e", 1)]},
                {"total": 0.5, "assignments": [("a", 1), ("q", 1), ("e", 0)]},
            ]
        )
    )
