#include <_stdio.h>
#include <cassert>
#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

namespace day2 {

struct Input {
  vector<vector<int>> list;
};

Input parseInput() {

  Input input;
  ifstream file("../src/days/2/input.txt");
  assert(file.is_open());

  for (string line; getline(file, line);) {
    string s;

    stringstream ss(line);

    vector<int> numbers;

    while (getline(ss, s, ' ')) {

      numbers.push_back(stoi(s));
    }
    input.list.push_back(numbers);
  }

  file.close();
  return input;
}

bool isValidDifference(int a, int b) {
  if (abs(a - b) < 1 || abs(a - b) > 3) {
    return false;
  }
  return true;
}

bool isSafe(vector<int> row) {
  bool isSafe = true;
  bool isAcending = row[0] < row[1];

  for (int i = 1; i < row.size(); i++) {
    int prevNumber = row[i - 1];
    int current = row[i];

    if (!isSafe)
      break;

    if (!isValidDifference(prevNumber, current)) {
      isSafe = false;
      break;
    }
    if (isAcending && current < prevNumber) {
      isSafe = false;
      break;
    }
    if (!isAcending && current > prevNumber) {
      isSafe = false;
      break;
    }
  }
  return isSafe;
}

int solvePart1(Input input) {
  int result = 0;
  for (vector<int> row : input.list) {
    if (isSafe(row)) {
      result++;
    }
  }
  return result;
}
int solvePart2(Input input) {
  int result = 0;
  for (vector<int> row : input.list) {

    if (isSafe(row)) {
      result++;
    } else {
      for (int i = 0; i < row.size(); i++) {
        vector<int> copy = row;
        copy.erase(copy.begin() + i);
        if (isSafe(copy)) {
          result++;
          break;
        }
      }
    }
  }
  return result;
}

void solver() {

  Input input = parseInput();
  int result1 = solvePart1(parseInput());
  cout << "Part 1: " << result1 << endl;
  int result2 = solvePart2(parseInput());
  cout << "Part 2: " << result2 << endl;
}
} // namespace day2
