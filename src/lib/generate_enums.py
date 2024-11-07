import csv
import sys

def read_csv(input_path):
    with open(input_path, mode='r', encoding='utf-8') as file:
        reader = csv.DictReader(file)
        data = [row for row in reader]
    return data

def extract_unique_teams_and_tournaments(data):
    teams = set()
    tournaments = set()

    for row in data:
        teams.add(row['team_home'])
        teams.add(row['team_away'])
        tournaments.add(row['tournament_name'])

    return sorted(teams), sorted(tournaments)

def write_ts_const(output_path, teams, tournaments):
    with open(output_path, mode='w', encoding='utf-8') as file:
        file.write("export const Teams = [\n")
        for team in teams:
            file.write(f"    \"{team}\",\n")
        file.write("] as const;\n\n")

        file.write("export type Team = typeof Teams[number];\n\n")

        file.write("export const Tournaments = [\n")
        for tournament in tournaments:
            file.write(f"    \"{tournament}\",\n")
        file.write("] as const;\n\n")

        file.write("export type Tournament = typeof Tournaments[number];\n")

def main(input_path, output_path):
    data = read_csv(input_path)
    teams, tournaments = extract_unique_teams_and_tournaments(data)

    write_ts_const(output_path, teams, tournaments)

    print(f"Number of unique teams: {len(teams)}")
    print(f"Number of unique tournaments: {len(tournaments)}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python script.py <input_path> <output_path>")
        sys.exit(1)

    input_path = sys.argv[1]
    output_path = sys.argv[2]

    main(input_path, output_path)
