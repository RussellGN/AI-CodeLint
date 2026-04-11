type Config = { theme: { darkMode: boolean } };

const original: Config = { theme: { darkMode: false } };
const copy = { ...original };

copy.theme.darkMode = true;
console.log(original.theme.darkMode);
