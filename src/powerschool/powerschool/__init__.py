"""A browser-automation-based API for PowerSchool"""
import datetime
import re
import urllib.parse as urlparse
from collections import defaultdict
from dataclasses import dataclass, field
from typing import Dict, List, Optional
import json

from data49.web import Browser, BrowserContext, By, expected_conditions, get_browser
from sympy import Symbol
from sympy.solvers.solveset import linsolve

__version__ = "0.1.0"


def calculate_weight(info_1, info_2, weight) -> dict:
    def eval_score(s):
        a, b = s.split("/")
        try:
            return float(a) / int(b)
        except:
            return 0

    a1 = defaultdict(int)
    a2 = defaultdict(int)
    for a in info_1["scores"]:
        a1[a["type"]] += eval_score(a["score"])
    for a in info_2["scores"]:
        a2[a["type"]] += eval_score(a["score"])
    symbols = {name: Symbol(name) for name in set(a1.keys()).union(set(a2.keys()))}

    return linsolve(
        sum(symbols[name] * v for name, v in a1.values()),
        sum(symbols[name] * v for name, v in a2.values()),
        symbols[weight],
    )


NOT_AVAILABLE = "N/A"

OutputType = Dict[str, Dict[str, str]]


@dataclass
class PowerSchool:
    url: str
    username: str
    password: str
    browser: Browser = field(init=False)
    driver: BrowserContext = field(init=False)
    home_table: BrowserContext = field(init=False)

    # def can_i_tank_it(
    #     self,
    #     for_period: str,
    #     assignment_type: str,
    #     points: float,
    #     current_offset: int,
    #     offset_1: int,
    #     offset_2: int,
    #     tank_standard: float = 93.0,
    # ):
    #     info_1 = self.full_info_scrape(offset_1)
    #     info_2 = self.full_info_scrape(offset_2)
    #     weight = calculate_weights(info_1[for_period], info_2[for_period])[
    #         assignment_type
    #     ]
    #     # c = map(lambda x:,self.full_info_scrape(current_offset)[for_period]['scores'])
    #     # return calc_grade() >= tank_standard

    # @property
    # def info(self):
    #     if self._info is None:
    #         self._info = self.full_info_scrape()
    #     return self._info
    #
    # @info.deleter
    # def _(self):
    #     self._info = None
    def _login(self) -> None:
        self.driver.query_selector("#fieldAccount").type(self.username)
        self.driver.query_selector("#fieldPassword").type(self.password)
        self.driver.query_selector("#btn-enter-sign-in").click()
        self.home_table = self.driver.css("#quickLookup").soup()

    def __enter__(self) -> "PowerSchool":
        self.browser = Browser(self.url)
        self.driver = self.browser.open()
        self._login()
        return self

    def __exit__(self, exc_type, exc_value, traceback) -> None:
        self.browser.close()

    def _convert_asignment_row(self, score_row):
        assignment_info = score_row("td")
        date = datetime.datetime.strptime(assignment_info[0].get_text(), "%m/%d/%Y")
        return {
            "due_date": datetime.date(date.year, date.month, date.day).isoformat(),
            "type": assignment_info[1].get_text().strip(),
            # types:
            # - Classwork
            # - Homework
            # - Participation
            # - Lab
            # - Quiz
            # - Test
            # - Progress Assessments
            # - Mastery Assessments
            "name": assignment_info[2].get_text().strip(),
            # TODO: Flags
            "score": assignment_info[10]  # Can also be --/20 or smth like that
            .span.get_text()
            .strip(),
            "percent": float(assignment_info[11].get_text().strip())
            if assignment_info[11].get_text().strip()
            else None,
            "grade": assignment_info[12]
            .get_text()
            .strip(),  # A+, B-, etc. Can be empty since 'score' may
            # be empty
        }

    def get_quarters(self) -> List[str]:
        return [
            el.string
            for el in self.home_table.table.tbody.tr.select(f"th")
            if el.string[1].isdigit()
        ]

    # DEPRECATE: Don't allow None as an option
    def get(self, quarters: Optional[List[str]] = None) -> OutputType:
        soup = self.home_table
        output: OutputType = {}
        real_prog = {
            "type": "progress",
            "content": {q: [0, -1] for q in (quarters or [])},
        }
        for quarter in quarters or [None]:

            offset = -3  # Because the rightmost contains "Absences" and "Tardies"
            heads = soup.table.tbody.tr.select(f"th")
            if quarter is not None:
                offset = -(len(heads) - [el.string for el in heads].index(quarter))

            current_quarter_name = heads[offset].string

            quarter_key = quarter or f"Latest ({current_quarter_name})"
            output[quarter_key] = {}
            _output = output[quarter_key]

            # fetch('https://powerschool.vcs.net/ws/xte/assignment/lookup', {method: 'POST', headers: {"Content-Type": "application/json"}, body: '{"section_ids":[47018],"student_ids":[18500],"store_codes":["S2"]}'})
            #   .then(response => response.json())
            #   .then(data => console.log(data));
            # Row of class info
            NAME_RE = re.compile(r"Email (\w+),\s*(\w+\.)\s*(\w+)")
            classes = soup.table.select("[id^='ccid_']")
            # TODO: Only classes that have grades for this quarter
            real_prog["content"][current_quarter_name] = [0, len(classes)]
            progress = real_prog["content"][current_quarter_name]
            for row in classes:
                period = row.td.string
                misc = row.find("td", align="left")
                class_name = misc.contents[0].strip()
                # Email and teacher name
                email_info = misc("a")[-1]
                email = email_info["href"][len("mailto:") :]
                name_info = NAME_RE.search(email_info.get_text())
                last_name = name_info.group(1)
                title = name_info.group(2)
                # first_name = name_info.group(3)
                name = title + " " + last_name

                current_quarter = row.select(f"td")[offset]
                current_grade = current_quarter.a
                current_grade_name = current_grade_percent = NOT_AVAILABLE
                assignments = []
                if current_grade:
                    try:
                        current_grade_name = current_grade.contents[0]
                        current_grade_percent = current_grade.contents[-1]
                    except AttributeError:
                        current_grade_name = current_grade_percent = NOT_AVAILABLE

                    self.driver.go(
                        urlparse.urljoin(
                            self.driver.driver.current_url, current_grade["href"]
                        )
                    )
                    # TODO: Use raw fetch
                    scores = self.driver.wait(
                        expected_conditions.visibility_of_element_located(
                            (By.ID, "scoreTable")
                        ),
                        up_to=42.069,
                    )
                    for score_row in scores.soup().tbody("tr")[1:-1]:
                        assignments.append(self._convert_asignment_row(score_row))
                    self.driver.back()

                _output[period.upper()] = {  # TODO: Change format
                    "name": name,
                    "class_name": class_name,
                    "email": email,
                    "quarter_info": {
                        "name": current_quarter_name,
                        "overall_grade": {
                            "name": current_grade_name,
                            "percent": current_grade_percent,
                        },
                        "scores": assignments,
                    },
                }
                progress[0] += 1
                print(json.dumps(real_prog))
        return output

    def teachers(self):
        raise NotImplementedError
        # return self.info[]
