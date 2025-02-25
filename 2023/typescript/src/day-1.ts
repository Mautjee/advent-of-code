const testInputPart1 = `
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
`;
const testInputPart2 = `
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
`;

const puzzleInput = await Bun.file('inputs/day-1.txt').text();


const day1Part1 = () => {
  const input = formatInput(puzzleInput);

  const answer = input.map((line) => {
    const nums = line.replace(/\D/g, "");
    if (nums.length === 2) {
      return nums;
    }
    if (nums.length === 1) {
      return nums.concat(nums);
    }
    return nums.charAt(0).concat(nums.charAt(nums.length - 1));
  }).reduce((acc, curr) => {
    return acc + parseInt(curr);
  },0);

  console.log(answer);
};

const day1Part2 = () => {
  const input = formatInput(puzzleInput);

  const allnumbers = input.map((line) => {
    const replace = line.replace(/one|two|three|four|five|six|seven|eight|nine/gi, (x)=> {
      switch (x) {
        case 'one':
          return '1';
        case 'two':
          return '2';
        case 'three':
          return '3';
        case 'four':
          return '4';
        case 'five':
          return '5';
        case 'six':
          return '6';
        case 'seven':
          return '7';
        case 'eight':
          return '8';
        case 'nine':
          return '9';
        default:
          return '';
      }
    });
    const nums = replace.replace(/\D/g, "");
    if (nums.length === 2) {
      return nums;
    }
    if (nums.length === 1) {
      return nums.concat(nums);
    }
    return nums.charAt(0).concat(nums.charAt(nums.length - 1));
  }).reduce((acc, curr) => {
    return acc + parseInt(curr);
  },0);

  console.log(allnumbers);
}

const formatInput = (input: string) => {
  return input.trim().split('\n');
}

// day1Part1();
day1Part2();
