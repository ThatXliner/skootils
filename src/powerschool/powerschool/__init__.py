"""A browser-automation-based API for PowerSchool"""
from data49.web import Browser, expected_conditions, By
from typing import Dict, TYPE_CHECKING, Optional
import urllib.parse as urlparse
from collections import defaultdict
from sympy import Symbol
from sympy.solvers.solveset import linsolve

if TYPE_CHECKING:
    from typing_extensions import TypedDict

import re
from dataclasses import dataclass
import datetime

__version__ = "0.1.0"


@dataclass
class Auth:
    url: str
    username: str
    password: str


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
        sum([symbols[name] * v for name, v in a1.values()]),
        sum([symbols[name] * v for name, v in a2.values()]),
        symbols[weight],
    )


@dataclass
class PowerSchool:
    url: str
    username: str
    password: str
    _info: Optional = None

    def can_i_tank_it(
        self,
        for_period: str,
        assignment_type: str,
        points: float,
        current_offset: int,
        offset_1: int,
        offset_2: int,
        tank_standard: float = 93.0,
    ):
        info_1 = self.full_info_scrape(offset_1)
        info_2 = self.full_info_scrape(offset_2)
        weight = calculate_weights(info_1[for_period], info_2[for_period])[
            assignment_type
        ]
        # c = map(lambda x:,self.full_info_scrape(current_offset)[for_period]['scores'])
        # return calc_grade() >= tank_standard

    @classmethod
    def from_auth(cls, auth: Auth):
        return cls(auth.url, auth.username, auth.password)

    @property
    def info(self):
        if self._info is None:
            self._info = self.full_info_scrape()
        return self._info

    @info.deleter
    def _(self):
        self._info = None

    def full_info_scrape(self, offset: int = 3):
        # TODO: Figure offset... and like make it user-friendly
        # def teachers(self) -> Dict[str, Dict[str, str]]:
        with Browser(self.url) as driver:
            driver.query_selector("#fieldAccount").type(self.username)
            driver.query_selector("#fieldPassword").type(self.password)
            driver.query_selector("#btn-enter-sign-in").click()
            driver.wait(
                until=expected_conditions.visibility_of_element_located(
                    (By.ID, "quickLookup")
                )
            )
            soup = driver.query_selector("#quickLookup").soup()

            output: Dict[str, Dict[str, str]] = {}

            current_quarter_name = soup.table.tbody.tr.select(
                f"th:nth-last-child({offset})"
            )[0].string

            for row in soup.table.select("[id^='ccid_']"):
                period = row.td.string
                misc = row.find("td", align="left")
                class_name = misc.contents[0].strip()
                email_info = misc.find_all("a")[-1]
                email = email_info["href"][len("mailto:") :]
                name = " ".join(
                    re.search(r"Email (\w+),\s*(\w+\.)", email_info.get_text()).group(
                        2, 1
                    )
                )
                current_quarter = row.select(f"td:nth-last-child({offset})")[0]

                current_grade = current_quarter.a
                if current_grade:
                    try:
                        current_grade_name = current_grade.contents[0]
                        current_grade_percent = current_grade.contents[-1]
                    except AttributeError:
                        current_grade_name = current_grade_percent = "N/A"

                    driver.driver.get(
                        urlparse.urljoin(
                            driver.driver.current_url, current_grade["href"]
                        )
                    )
                    scores = driver.wait(
                        expected_conditions.visibility_of_element_located(
                            (By.ID, "scoreTable")
                        )
                    )
                    assignments = []
                    for score_row in scores.soup().tbody("tr")[1:-1]:
                        assignment_info = score_row("td")
                        # try:
                        assignments.append(
                            {
                                "due_date": datetime.datetime.strptime(
                                    assignment_info[0].get_text(), "%m/%d/%Y"
                                ),
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
                                "score": assignment_info[  # Can also be --/20 or smth like that
                                    10
                                ]
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
                        )
                        # except IndexError:  # TODO: Better error message
                        #     pass
                    driver.driver.back()

                output[period.upper()] = {
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
            return output

    def teachers(self):
        raise NotImplementedError
        # return self.info[]
