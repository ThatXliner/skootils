import json
import sys
from collections import defaultdict, deque

from powerschool.storage import HISTORY_DIR


def main():
    if len(sys.argv) == 2 and sys.argv[1] == "--test":
        print("Program is valid")
        return
    quarter = sys.argv[1]
    labels = []
    colors = deque(
        [  # Even looks good on dark mode!
            "rgb(128,35,146)",
            "rgb(222,77,134)",
            "rgb(124,144,219)",
            "rgb(132,230,248)",
            "rgb(77,121,118)",
            "rgb(165,248,211)",
            "rgb(72,155,81)",
            "rgb(233,176,77)",
            "rgb(223,73,73)",
        ]
    )

    def pickColor():
        colors.rotate()
        return colors[0]

    classes = defaultdict(lambda: ([], pickColor()))
    # TODO: multithreading for maximum efficiency
    for i, file in enumerate(sorted((HISTORY_DIR / quarter).iterdir()), start=1):
        labels.append(f"Scrape #{i}")
        scrape = json.loads(file.read_text())
        for period in scrape:
            try:
                classes[scrape[period]["class_name"]][0].append(
                    int(scrape[period]["quarter_info"]["overall_grade"]["percent"])
                )
            except ValueError:
                pass
    print(
        json.dumps(
            {
                "labels": labels,
                "datasets": [
                    {
                        "label": k,
                        "data": v[0],
                        "fill": False,
                        "backgroundColor": v[1],
                        "tension": 0,  # 0 = straight line. this is for bezier curve
                        # Connection line colors
                        "borderColor": v[1][:-1] + ", 0.4)",  # slightly transparent
                        # "borderColor": v[1],  # solid
                        # [not specified] # Gray
                    }
                    for k, v in classes.items()
                    if v[0]
                ],
            }
        )
    )


if __name__ == "__main__":
    main()
