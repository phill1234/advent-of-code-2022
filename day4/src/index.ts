import * as fs from "fs";
function part1() {
  // read file line by line
  const file = fs.readFileSync("input.txt", "utf8");
  let result = 0;

  // get symbol for new line
  const newLine = String.fromCharCode(10);
  const lines = file.split(`${newLine}`);

  for (const line of lines) {
    // Range A
    // split line at ,
    const [rangeA, rangeB] = line.split(",");
    // split range at -
    const [startA, endA] = rangeA.split("-");
    // convert to number
    const minNumberRangeA = parseInt(startA);
    const maxNumberRangeA = parseInt(endA);

    // Range B
    // split range at -
    const [startB, endB] = rangeB.split("-");
    // convert to number
    const minNumberRangeB = parseInt(startB);
    const maxNumberRangeB = parseInt(endB);

    // check if it is in range
    // then B is maybe in A
    if (
      maxNumberRangeA >= maxNumberRangeB &&
      minNumberRangeA <= minNumberRangeB
    ) {
      result++;
    }
    // then A is maybe in B
    else if (
      maxNumberRangeA <= maxNumberRangeB &&
      minNumberRangeA >= minNumberRangeB
    ) {
      result++;
    }
  }
  console.log("PART1: ", result);
}

function part2() {
  // read file line by line
  const file = fs.readFileSync("input.txt", "utf8");
  let result = 0;

  // get symbol for new line
  const newLine = String.fromCharCode(10);
  const lines = file.split(`${newLine}`);

  for (const line of lines) {
    // Range A
    // split line at ,
    const [rangeA, rangeB] = line.split(",");
    // split range at -
    const [startA, endA] = rangeA.split("-");
    // convert to number
    const minNumberRangeA = parseInt(startA);
    const maxNumberRangeA = parseInt(endA);

    // Range B
    // split range at -
    const [startB, endB] = rangeB.split("-");
    // convert to number
    const minNumberRangeB = parseInt(startB);
    const maxNumberRangeB = parseInt(endB);

    const rangeAList = range(minNumberRangeA, maxNumberRangeA);
    const rangeBList = range(minNumberRangeB, maxNumberRangeB);

    for (const entry of rangeAList) {
      if (rangeBList.includes(entry)) {
        result++;
        break;
      }
    }
  }
  console.log("PART2: ", result);
}

function range(start: number, end: number) {
  const result = [];
  for (let i = start; i <= end; i++) {
    result.push(i);
  }
  return result;
}

function part1WithLists() {
  // read file line by line
  const file = fs.readFileSync("input.txt", "utf8");
  let result = 0;

  // get symbol for new line
  const newLine = String.fromCharCode(10);
  const lines = file.split(`${newLine}`);

  for (const line of lines) {
    // Range A
    // split line at ,
    const [rangeA, rangeB] = line.split(",");
    // split range at -
    const [startA, endA] = rangeA.split("-");
    // convert to number
    const minNumberRangeA = parseInt(startA);
    const maxNumberRangeA = parseInt(endA);

    // Range B
    // split range at -
    const [startB, endB] = rangeB.split("-");
    // convert to number
    const minNumberRangeB = parseInt(startB);
    const maxNumberRangeB = parseInt(endB);

    const rangeAList = range(minNumberRangeA, maxNumberRangeA);
    const rangeBList = range(minNumberRangeB, maxNumberRangeB);

    let isIn = true;
    let startRange = null;
    let compareRange = null;
    if (rangeAList.length >= rangeBList.length) {
      startRange = rangeBList;
      compareRange = rangeAList;
    } else {
      startRange = rangeAList;
      compareRange = rangeBList;
    }

    for (const entry of startRange) {
      if (!compareRange.includes(entry)) {
        isIn = false;
        break;
      }
    }
    if (isIn) {
      result++;
    }
  }
  console.log("PART1 with lists: ", result);
}

part1();
part1WithLists();
part2();
