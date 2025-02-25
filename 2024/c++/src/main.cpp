#include <_stdio.h>
#include <cassert>
#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>

using namespace std;

struct Input {
  vector<int> left;
  vector<int> right;
};

Input parseInput() {
  Input input;
  ifstream file("../src/days/1/input.txt");
  assert(file.is_open());

  for (string line; getline(file, line);) {
    size_t startWhitespace = line.find_first_of(' ');
    size_t endWhitespace = line.find_last_of(' ');

    string leftNumber = line.substr(0, startWhitespace);
    string rightNumber = line.substr(endWhitespace + 1);

    input.left.push_back(stoi(leftNumber));
    input.right.push_back(stoi(rightNumber));
  }

  file.close();
  return input;
};

int solvePart1(Input input) {
  sort(input.left.begin(), input.left.end());
  sort(input.right.begin(), input.right.end());

  int total = 0;

  for (int row = 0; row < input.left.size(); row++) {
    int rowResult = input.left[row] - input.right[row];
    total += abs(rowResult);
  }
  return total;
}

int solvePart2(Input input) {

  map<int, int> rightData;

  for (int val : input.right) {
    rightData[val]++;
  }
  int total = 0;

  for (int val : input.left) {
    total += val * rightData[val];
  }

  return total;
}

int main() {
  parseInput();
  int result1 = solvePart1(parseInput());
  cout << "Part 1: " << result1 << endl;
  int result2 = solvePart2(parseInput());
  cout << "Part 2: " << result2 << endl;
}
