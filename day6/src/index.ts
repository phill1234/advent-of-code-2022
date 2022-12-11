import * as fs from "fs";
const main = () => {
  const characters = fs.readFileSync("input.txt", "utf8");
  let globalIndex = 0;
  let actualStreak: string[] = [];
  
  while (true) {
    for (let i = globalIndex; i < characters.length; i++) {
      const character = characters[i];
      if (actualStreak.includes(character)) {
        actualStreak = [];
        globalIndex++;
        break;
      }
      actualStreak.push(character);
      if (actualStreak.length === 4) {
        return globalIndex + 4;
      }
    }
  }
};
console.log(main());
