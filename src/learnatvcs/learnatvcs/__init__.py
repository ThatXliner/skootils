"""Awesome lesson plan utilities"""

import functools
import json
import re
from typing import ClassVar, Optional

from attrs import define
from data49 import web

import selenium
from learnatvcs import process as process_data

from .highlighter import highlight

HEADLESS = False  # TODO: get it to work with headless


@functools.lru_cache()
def _clean(x: str) -> str:
    return str(process_data.clean_html(process_data.to_soup(x)))


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


@define(frozen=True)
class Date:
    month: int
    day: int
    DATE_RE: ClassVar[re.Pattern[str]] = re.compile(
        r"(?P<amonth>[a-z]+) (?P<aday>\d+)(?:/(?P<bmonth>[a-z]+)?[ ]?(?P<bday>\d+))?",
        flags=re.IGNORECASE,
    )

    MONTH2INT: ClassVar[dict[str, int]] = {
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

    @classmethod
    def from_str(cls, x: str) -> "Date":
        """Transform x into a Date"""

        def normalize_month(x: str) -> int:
            return cls.MONTH2INT[x.lower()[:3]]

        # TODO: A day/B day config. Use data from powerschool module
        match = cls.DATE_RE.search(x)
        if not match:
            raise ValueError(f"No date found for {x}")
        return cls(month=normalize_month(match["amonth"]), day=int(match["aday"]))

    def __str__(self) -> str:
        return f"{MONTHS[self.month-1]} {self.day}"


#
# @define
# class LessonPlan:
#     class_name: str
#     # class_name: Class
#     day: Date
#     content: BeautifulSoup = field(converter=process_data.to_soup)
#
#     @property
#     @functools.lru_cache()
#     def summary(self) -> str:
#         # XXX: this is waay too oversimplified. lmao
#         return ai(process_data.extract_useful(self.content))
#
#     def extract_assignments(self):
#         ...
#
#     def highlight_dates(self):
#         ...


# FIXME: Restructure (again, ugh) to be similar to lesson plan strucure?
# or keep it like this: the space-optimized way, at the cost of information
# (or in this case, the lack of knowledge of information loss)
RawOutput = dict[Date, dict[str, str]]
# RawOutput = dict[Date, dict[str, Optional[str]]]

# TODO: not class
@define
class Bot:  # TODO: CACHE
    # prog: interprog.ProgressReporter = interprog.MockProgressReporter()
    # browser: web.Browser = field(default)
    # TODO: parallelize
    # def get_lesson_plans(self, **kwargs) -> list[LessonPlan]:
    #     raw_data = self.run(**kwargs)
    #     return [
    #         LessonPlan(class_name, date, content)
    #         for date, contents in raw_data.items()
    #         for class_name, content in contents.items()
    #     ]

    def get_json(self, **kwargs) -> str:
        raw_data = self.run(**kwargs)
        # TODO: also highlight contents
        return json.dumps({str(date): content for date, content in raw_data.items()})

    def run(self, for_dates: Optional[list[Date]] = None) -> RawOutput:
        """Get raw scrape data"""
        # TODO: Choose quarters
        output: RawOutput = {}
        tasks = [
            {"finished": False, "name": "Log in"},
        ]
        print(json.dumps(tasks))
        # self.prog.assign(interprog.tasks.Status("Log in"))

        with web.Browser(
            "https://learn.vcs.net/",
            driver=web.get_browser(
                headless=HEADLESS,
                arguments=(
                    "user-data-dir=/Users/bryanhu/projects/skootils/src/learnatvcs/selenium",
                ),
            ),
        ) as browser:
            tasks[0]["finished"] = None
            print(json.dumps(tasks))
            ### Log in ###
            browser.query_selector(".login-btn").click()
            browser.query_selector(
                "div.potentialidp:nth-child(1) > a:nth-child(1)"
            ).click()
            tasks[0]["finished"] = True
            print(json.dumps(tasks))
            ### For every class... ###
            # TODO: Assert #side-panel-button exists and not "which account"
            # for future auto setup
            browser.query_selector("#side-panel-button").click()
            raw_links = browser.query_selector_all(
                "#inst6206 > div > div > ul > li > div > a"
            )[2:-1]
            links = [link["href"] for link in raw_links]
            class_names = [link["title"] for link in raw_links]
            tasks.extend(
                [
                    {
                        "finished": [0, len(for_dates)]
                        if for_dates and len(for_dates) > 1
                        else False,
                        "name": f"Scrape {name}",  # XXX: or just `name`? "Scrape" seems a bit excessive
                    }
                    for name in class_names
                ]
            )
            print(json.dumps(tasks))
            cur_task = 1
            for link, class_name in zip(links, class_names):
                browser.go(to=link)
                try:
                    browser.query_selector('a[title^="Lesson"]').click()
                    browser.query_selector_all(".activity.book.modtype_book a")[
                        -1
                    ].click()
                except (selenium.common.exceptions.NoSuchElementException, IndexError):
                    tasks[cur_task]["finished"] = "No lesson plans"
                    print(json.dumps(tasks))
                    cur_task += 1
                    continue
                ### ...go to that day's lesson plans... ###
                dates_found = {
                    Date.from_str(date["innerText"]): date
                    for date in browser.query_selector_all(
                        ".book_toc a,.book_toc strong"
                    )
                }
                if tasks[cur_task]["finished"] is False:
                    tasks[cur_task]["finished"] = None
                    print(json.dumps(tasks))
                for date in for_dates or [list(dates_found)[-1]]:
                    if date not in output:
                        output[date] = {}  # or should we use a list?
                    # elif for_dates is None and None not in output:
                    #     output[None] = {}
                    chosen = [
                        link_title
                        for link_title in dates_found
                        # TODO: Ok but like what if the date given in param
                        # was b day
                        if link_title == date
                    ]
                    if len(chosen) > 1:
                        tasks[cur_task][
                            "finished"
                        ] = f"Ambigous lesson plan dates for date {date}"
                        cur_task += 1
                        print(json.dumps(tasks))
                        # output[date][class_name] = None
                        continue
                    if len(chosen) == 0:
                        tasks[cur_task]["finished"] = f"No lesson plans for {date}"
                        cur_task += 1
                        print(json.dumps(tasks))
                        # output[date][class_name] = None
                        continue
                    # TODO: If the element is not clickable (cannot see)
                    # because the window is too small, we should
                    # scroll to the element first
                    ### ...and scrape it ###
                    content = browser.css("section#region-main > div[role='main']")
                    output[date if for_dates is not None else None][class_name] = str(
                        highlight(
                            process_data.clean_html(
                                process_data.to_soup(content["innerHTML"])
                            )
                        )
                    )
                    if tasks[cur_task]["finished"] is None:
                        tasks[cur_task]["finished"] = True
                    else:
                        tasks[cur_task]["finished"][0] += 1
                    print(json.dumps(tasks))
                tasks[cur_task]["finished"] = True
                print(json.dumps(tasks))
                cur_task += 1
        return output


__version__ = "0.1.0"
