# Advent of Code - Day 04 - Scratchcards | filename: day_04_scratchcards.py | 12 Dec 2023
# required libraries
import re
import sys

# import the txt file and reads each line into a list of strings
lines = list(sys.stdin)

# my functions
def get_lists(your_string):
    # given input string, split at delimiter, separate the numbers, and convert the str lists to int lists
    a, b, c = re.split(r': | \|', your_string)

    int_card = re.findall(r'\d+', a)
    card_num = [int(digit) for digit in int_card]
    int_win = re.findall(r'\d+', b)
    winning_num = [int(digit) for digit in int_win]
    int_my = re.findall(r'\d+', c)
    my_num = [int(digit) for digit in int_my]

    return card_num, winning_num, my_num

# PART 1 AND 2
cards_worth = []  # for PART 1
card_count = [1] * len(lines)  # for PART 2: list of every card's number of wins > initalize as 1

for index, line in enumerate(lines):
    card_number, winning_numbers, my_numbers = get_lists(line)
    # PART 1: find how many points each card gets
    matches = []
    for x, y in enumerate(winning_numbers):
        for j in range(len(my_numbers)):
            if y == my_numbers[j]: matches.append(x)
    cards_worth.append(2 ** (len(matches) - 1)) if len(matches) > 0 else cards_worth.append(0)  # worth = 1 x 2^n-1

    # PART 2:
    for n in range(len(matches)):
        # update the count of the card at spot "index + n + 1" with the count of the card at "index"
        # why does this work? je ne sais pas...c'est le voodoo magic
        card_count[index + n + 1] += card_count[index]

print(card_count)
