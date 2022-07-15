import re
from typing import NamedTuple, Optional

from attrs import define

MONTH2INT: dict[str, int] = {
    "jan": 1,
    "feb": 2,
    "mar": 3,
    "apr": 4,
    "may": 5,
    "jun": 6,
    "jul": 7,
    "aug": 8,
    "sep": 9,
    "oct": 10,
    "nov": 11,
    "dec": 12,
}

DATE_RE: re.Pattern[str] = re.compile(
    r"(?P<month>[a-z]+) (?P<day>\d+)", flags=re.IGNORECASE
)


class Date(NamedTuple):
    month: int
    day: int

    @classmethod
    def from_str(cls, x: str) -> "Date":
        """Transform x into a Date"""

        def normalize_month(x: str) -> int:
            return MONTH2INT[x.lower()[:3]]

        match = DATE_RE.search(x)
        if not match:
            raise ValueError(f"No date found for {x}")

        return cls(month=normalize_month(match["month"]), day=int(match["day"]))

    def __str__(self) -> str:
        MONTHS = [
            "Janurary",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ]
        return f"{MONTHS[self.month-1]} {self.day}"


CLASS_DAY_RE: re.Pattern[str] = re.compile(
    r"(?P<amonth>[a-z]+) (?P<aday>\d+)(?:/(?P<bmonth>[a-z]+)?[ ]?(?P<bday>\d+))?",
    flags=re.IGNORECASE,
)


@define(frozen=True)
class ClassDay:
    a_day: Date
    b_day: Optional[Date] = None

    def __hash__(self) -> int:
        return hash((self.a_day, self.b_day))

    def __str__(self) -> str:
        return str(self.a_day)  # TODO: Use PowerSchool

    def __eq__(self, other):  # type: ignore
        if isinstance(other, ClassDay):  # type: ignore
            return (self.a_day, self.b_day) == (other.a_day, other.b_day)
        if isinstance(other, Date):  # type: ignore
            return other in {self.a_day, self.b_day}
        return NotImplemented

    @classmethod
    def from_str(cls, x: str) -> "ClassDay":
        """Transform x into a ClassDay"""

        def normalize_month(x: str) -> int:
            return MONTH2INT[x.lower()[:3]]

        match = CLASS_DAY_RE.search(x)
        if not match:
            raise ValueError(f"No date found for {x}")
        a_month = normalize_month(match["amonth"])
        return cls(
            a_day=Date(month=a_month, day=int(match["aday"])),
            b_day=Date(
                month=normalize_month(match["bmonth"]) if match["bmonth"] else a_month,
                day=int(match["bday"]),
            )
            if match["bday"]
            else None,  # If "bday" doesn't exist, it's a C day
        )
