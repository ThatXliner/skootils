"""A browser-automation-based API for PowerSchool"""
import datetime
import re
import urllib.parse as urlparse
from collections import defaultdict
from dataclasses import dataclass, field
from typing import Dict, List, Optional
import requests
from bs4 import BeautifulSoup, SoupStrainer
from sympy import Symbol
from sympy.solvers.solveset import linsolve
from functools import reduce

import interprog

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
    session: requests.Session = field(init=False)
    home_table: BeautifulSoup = field(init=False)

    def __enter__(self) -> "PowerSchool":
        self.session = requests.Session()
        self.session.post(
            self.url + "/guardian/home.html",
            data={"account": self.username, "pw": self.password},
        ).raise_for_status()
        self.home_table = BeautifulSoup(
            self.session.get(self.url + "/guardian/home.html").text,
            features="html.parser",
            # parse_only=SoupStrainer(id="quickLookup"),
        ).find(id="quickLookup")
        # print(self.home_table)
        return self

    def __exit__(self, exc_type, exc_value, traceback) -> None:
        self.session.close()

    @staticmethod
    def _get_flags(score) -> int:
        # not including "is collected"
        # because that one can be false even
        # when we have full score.
        # it's weird :/
        FLAGS = {
            "late": 1 << 0,
            "exempt": 1 << 1,
            "missing": 1 << 2,
            "incomplete": 1 << 3,
            "absent": 1 << 4,
        }
        return reduce(
            lambda a, b: a | b,
            [value if score[f"is{key}"] else 0 for key, value in FLAGS.items()],
            0,
        )

    def _make_assignment(self, assignment):
        #   info = {
        #   "_name": "assignmentsection",
        #   "isscorespublish": true,
        #   "scoreentrypoints": 5.0,
        #   "weight": 1.0,
        #   "isscoringneeded": true,
        #   "sectionsdcid": 40915,
        #   "totalpointvalue": 5.0,
        #   "assignmentsectionid": 467070,
        #   "iscountedinfinalgrade": true,
        #   "duedate": "2022-02-02",
        #   "_assignmentscores": [
        #     {
        #       "scorepoints": 5.0,
        #       "islate": false,
        #       "_name": "assignmentscore",
        #       "isexempt": false,
        #       "studentsdcid": 18500,
        #       "scorelettergrade": "A+",
        #       "isabsent": false,
        #       "ismissing": false,
        #       "isincomplete": false,
        #       "whenmodified": "2022-02-08",
        #       "iscollected": false,
        #       "actualscorekind": "REAL_SCORE",
        #       "authoredbyuc": false,
        #       "scoreentrydate": "2022-02-08 18:40:46",
        #       "actualscoreentered": "5",
        #       "scorepercent": 100.0
        #     }
        #   ],
        #   "name": "Book Report Tone Square",
        #   "scoretype": "POINTS",
        #   "_id": 467070,
        #   "_assignmentcategoryassociations": [
        #     {
        #       "assignmentcategoryassocid": 323300,
        #       "_name": "assignmentcategoryassoc",
        #       "_teachercategory": {
        #         "_name": "teachercategory",
        #         "color": "2",
        #         "name": "Homework"
        #       },
        #       "_id": 323300
        #     }
        #   ]
        # }
        sections = assignment["_assignmentsections"]
        assert len(sections) == 1
        info = sections[0]
        scores = info["_assignmentscores"]
        assert len(scores) == 1
        score = scores[0]
        assignment_types = info["_assignmentcategoryassociations"]
        assert len(assignment_types) == 1
        assignment_type = assignment_types[0]
        assert info["scoreentrypoints"] == info["totalpointvalue"]
        return {
            "due_date": info["duedate"],
            "description": info.get("description"),
            "comment": info["_assignmentscorecomment"]["commentvalue"]
            if "_assignmentscorecomment" in info
            else None,
            "type": assignment_type["_teachercategory"]["name"],
            # types:
            # - Classwork
            # - Homework
            # - Participation
            # - Lab
            # - Project
            # - Quiz
            # - Test
            # - Progress Assessments
            # - Mastery Assessments
            "name": info["name"],
            # TODO: Flags (missing, late, etc)
            "flag": self._get_flags(score),
            "_raw": info,
            "score": {  # Can also be --/20 or smth like that
                "total": info["totalpointvalue"],
                "recieved": score.get("scorepoints"),
            },
            # these two does not exist when it's extra credit or extempted
            "percent": score.get(
                "scorepercent"  # , int(score["actualscoreentered"]) * 100
            ),
            "grade": score.get("scorelettergrade"),  # A+, B-, etc.
        }

    def get_quarters(self) -> List[str]:
        return [
            el.string
            for el in self.home_table.table.tr.select("th")
            if el.string[1].isdigit()
        ]

    # DEPRECATE: Don't allow None as an option
    def get(self, quarters: Optional[List[str]] = None) -> OutputType:
        soup = self.home_table
        output: OutputType = {}
        reporter = interprog.TaskManager()
        for quarter in quarters or ["Latest"]:
            reporter.add_task(quarter, total=-1)
        student_id = None
        section_ids = {}
        for quarter in quarters or [None]:
            reporter.start()
            offset = -3  # Because the rightmost contains "Absences" and "Tardies"
            heads = soup.table.tr.select("th")
            if quarter is not None:
                offset = -(len(heads) - [el.string for el in heads].index(quarter))

            current_quarter_name = heads[offset].string

            quarter_key = quarter or f"Latest ({current_quarter_name})"
            output[quarter_key] = {}
            _output = output[quarter_key]
            # Row of class info
            NAME_RE = re.compile(r"Email (\w+),\s*(\w+\.)\s*(\w+)")
            classes = soup.table.select("[id^='ccid_']")
            # TODO: Only classes that have grades for this quarter
            # (in other words, don't add to the output)
            reporter.set_total(len(classes))
            # TODO: parallelize

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

                current_quarter = row.select("td")[offset]
                current_grade = current_quarter.a
                current_grade_name = current_grade_percent = NOT_AVAILABLE
                assignments = []
                if current_grade:
                    _page = None
                    if student_id is None:
                        if _page is None:
                            _page = self.session.get(
                                "https://powerschool.vcs.net/guardian/"
                                + current_grade["href"]
                            ).text
                        student_id = int(
                            re.search(r"studentFRN.+?(\d+)", _page).group(1)[3:]
                        )
                    if class_name not in section_ids:
                        if _page is None:
                            _page = self.session.get(
                                "https://powerschool.vcs.net/guardian/"
                                + current_grade["href"]
                            ).text
                        section_ids[class_name] = int(
                            BeautifulSoup(
                                _page,
                                features="html.parser",
                                parse_only=SoupStrainer(attrs={"data-sectionid": True}),
                            ).div["data-sectionid"]
                        )
                    try:
                        current_grade_name = current_grade.contents[0]
                        current_grade_percent = current_grade.contents[-1]
                    except AttributeError:
                        current_grade_name = current_grade_percent = NOT_AVAILABLE

                    assignments.extend(
                        [
                            self._make_assignment(assignment)
                            for assignment in self.session.post(
                                "https://powerschool.vcs.net/ws/xte/assignment/lookup",
                                headers={
                                    "Referer": "https://powerschool.vcs.net/public"
                                },
                                json={
                                    # dunno why it's a list
                                    "section_ids": [section_ids[class_name]],
                                    "student_ids": [student_id],
                                    # 'Specify store_codes, or start_date and end_date'
                                    "store_codes": [current_quarter_name],
                                },
                            ).json()
                        ]
                    )

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
                reporter.increment()
            reporter.finish()
        return output
