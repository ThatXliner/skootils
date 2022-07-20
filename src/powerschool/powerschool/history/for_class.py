import json
import sys

from powerschool.storage import HISTORY_DIR

COLOR = "rgb(132,230,248)"


def main():
    if len(sys.argv) == 2 and sys.argv[1] == "--test":
        print("Program is valid")
        return
    quarter = sys.argv[1]
    class_name = sys.argv[2]
    labels = []
    data = []
    for i, file in enumerate(sorted((HISTORY_DIR / quarter).iterdir()), start=1):
        labels.append(f"Scrape #{i}")
        scrape = json.loads(file.read_text())
        for period in scrape:
            if scrape[period]["class_name"] == class_name:
                data.append(
                    int(scrape[period]["quarter_info"]["overall_grade"]["percent"])
                )
                break

    print(
        json.dumps(
            {
                "labels": labels,
                "datasets": [
                    {
                        "label": class_name,
                        "data": data,
                        "fill": False,
                        "backgroundColor": COLOR,
                        "tension": 0,  # 0 = straight line. this is for bezier curve
                        # Connection line colors
                        "borderColor": COLOR,
                    }
                ],
            }
        )
    )


if __name__ == "__main__":
    main()
