"""Awesome lesson plan utilities"""
import re
from pathlib import Path
from typing import NamedTuple, Optional
from platformdirs import user_data_dir
import selenium
from attrs import define
from data49 import web

import interprog
from learnatvcs import process as process_data

from .highlighter import highlight

HEADLESS = False  # TODO: get it to work with headless
APP_NAME = "Skootils"
APP_AUTHOR = "Bryan Hu"

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


# FIXME: Restructure (again, ugh) to be similar to lesson plan strucure?
# or keep it like this: the space-optimized way, at the cost of information
# (or in this case, the lack of knowledge of information loss)
RawOutput = dict[Optional[str], dict[str, str]]


def _lesson_plan_pipeline(contents: str) -> str:
    # TODO Give highlighting function what date is 'today'
    return str(highlight(process_data.clean_html(process_data.to_soup(contents))))


def scrape(for_dates: Optional[list[Date]] = None) -> RawOutput:
    """Get raw scrape data"""
    # TODO: Choose quarters
    output: RawOutput = {}
    reporter = interprog.TaskManager()
    reporter.add_task("Log in")

    with web.Browser(
        "https://learn.vcs.net/",
        driver=web.get_browser(
            headless=HEADLESS,
            arguments=(
                f"user-data-dir={Path(user_data_dir(APP_NAME,APP_AUTHOR))/'learnatvcs'/'.webdriver_profile'}",
            ),
        ),
    ) as browser:
        reporter.start()
        # Log in
        browser.query_selector(".login-btn").click()
        browser.query_selector("div.potentialidp:nth-child(1) > a:nth-child(1)").click()
        reporter.finish()
        # For every class...
        # TODO: Assert #side-panel-button exists and not "which account"
        # for future auto setup
        browser.query_selector("#side-panel-button").click()
        raw_links = browser.query_selector_all(
            "#inst6206 > div > div > ul > li > div > a"
        )[2:-1]
        links = [link["href"] for link in raw_links]
        class_names = [link["title"] for link in raw_links]
        for name in class_names:
            reporter.add_task(
                f"Scrape {name}",
                total=len(for_dates) if for_dates and len(for_dates) > 1 else None,
            )
        for link, class_name in zip(links, class_names):
            reporter.start()
            browser.go(to=link)
            try:
                browser.query_selector('a[title^="Lesson"]').click()
                browser.query_selector_all(".activity.book.modtype_book a")[-1].click()
            except (selenium.common.exceptions.NoSuchElementException, IndexError):
                reporter.error("No lesson plans")
                continue
            # ...go to that day's lesson plans...
            date2link = {
                ClassDay.from_str(date["innerText"]): date.get("href")
                for date in browser.query_selector_all(".book_toc a,.book_toc strong")
            }
            for date in for_dates or [list(date2link)[-1]]:
                day_key = str(date) if for_dates is not None else None
                if day_key not in output:
                    output[day_key] = {}  # or should we use a list?
                chosen = [
                    date2link[link_date] for link_date in date2link if link_date == date
                ]
                if len(chosen) > 1:
                    # Should never happen
                    assert False, f"Ambigous lesson plan dates for date {date}"
                if len(chosen) == 0:
                    # TODO: Error reporting
                    # output[day_key][class_name] = None
                    # output[day_key][class_name] = f"No lesson plans for {date}"
                    reporter.increment(silent=True)
                    continue
                if chosen[0] is not None:
                    browser.go(to=chosen[0])
                # ...and scrape it
                content = browser.css("section#region-main > div[role='main']")
                output[day_key][class_name] = _lesson_plan_pipeline(
                    content["innerHTML"]
                )
                reporter.increment(silent=True)
            reporter.finish()
    return output


def mock(for_dates: Optional[list] = None) -> str:  # type: ignore
    """how to kill a mockingbird?"""
    import time

    reporter = interprog.TaskManager()
    reporter.add_task("Log in")
    reporter.start()
    time.sleep(1)
    reporter.finish()
    class_names = ["English", "Math", "Science", "History"]
    for name in class_names:
        reporter.add_task(
            f"Scrape {name}",
            total=len(for_dates) if for_dates and len(for_dates) > 1 else None,  # type: ignore
        )
    reporter.start()
    time.sleep(1)
    reporter.increment(silent=True)
    time.sleep(2)
    reporter.error("Lmao get good")
    for _ in class_names[1:]:
        reporter.start()
        time.sleep(0.2)
        for _ in for_dates or [None]:  # type: ignore
            time.sleep(1)
            reporter.increment(silent=True)
        reporter.finish()
    return (Path(__file__).parent.parent / "example.json").read_text()


__version__ = "0.1.0"
